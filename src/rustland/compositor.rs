use std::sync::RwLock;
use std::io::Write;
use std::process::Command;
use std::io::Read;
use std::thread::{spawn, sleep};
use std::time;        
use std::ops::DerefMut;
use std::env::home_dir;
use std::path::PathBuf;

use common::definitions::{WM_FORWARD_EVENT_TO_CLIENT, WM_CATCH_EVENT};
use common::job::{Job, JobType};
use common::definitions::{FPS, ElementReference};
use config::Config;
use io::physical::InputDevice;
use layout::element::{LayoutElement, LayoutElementProfile};
use layout::element::window::Window;
use layout::element::grid::Direction;
use layout::element::bisect::Orientation;
use layout::LayoutTree;
use layout::arrangement::tree;

use utils::geometry::{PointExt, SizeExt, GeometryExt};
use wmstate::{WMState, WM_STATE, PENDING_JOBS, FINALIZED_JOBS, ACTIVE_TRANSITIONS};
use async::{schedule_wallpaper_init, schedule_job_routine, schedule_animator_routine, schedule_tcp_routine};
use sugars::solid_color::SolidColor;

use wlc::{Callback, Key, KeyState, Point, Size, Geometry, ButtonState, View, Output, Visibility, Modifier, Modifiers, Button, terminate};
use wlc::render::{RenderOutput, RenderView};
use wlc::TouchType;
use wlc::ResizeEdge::Flags;
use egli;
use gl;
use thread_tryjoin::TryJoinHandle;

pub struct Compositor;

impl Callback for Compositor {
    fn compositor_ready(&mut self){  
        gl::load_with(|s| egli::egl::get_proc_address(s) as *const _);
        gl::Viewport::load_with(|s| egli::egl::get_proc_address(s) as *const _);

        if let Ok(mut wm_state) = WM_STATE.write(){
            // Initialize config:
            if let Some(valid_config) = Config::load_from_file(Config::file_path()){
                wm_state.config = valid_config;
            }

            // Initialize layout tree:
            wm_state.tree = wm_state.config.layout.construct_tree();
            LayoutTree::refresh(&mut wm_state);

            // Initialize input device:
            wm_state.input_dev = Some(InputDevice::init());
            
            // Initalize additional graphics:
            let program = wm_state.init_graphics_program(); 
            wm_state.render_background();
            let grid_tag = wm_state.config.layout.grid_tag.clone();
            for mut sub in wm_state.tree.lookup_element_by_tag("sub".to_string())  {
                if let LayoutElementProfile::Padding(ref mut padding) = sub.profile {
                    //padding.apply_frame(wm_state.graphics_program.as_ref().unwrap(), 1.0f32);
                }
            }

            if let Some(wallpaper_path) = wm_state.config.background.wallpaper_path.clone(){
                wm_state.next_wallpaper_image = Some(schedule_wallpaper_init(PathBuf::from(wallpaper_path)));
            }

            if let Some(rgba) = wm_state.config.background.color_for_gl(){
                wm_state.solid_color = Some(SolidColor::new(rgba.0, rgba.1, rgba.2, 1.0f32));
            }
        }

        schedule_job_routine();              
        schedule_tcp_routine();    
        schedule_animator_routine();    
    }

    fn output_render_pre(&mut self, output: &mut RenderOutput) {        
        if let Ok(mut wm_state) = WM_STATE.write(){
            let image_loaded = match wm_state.next_wallpaper_image {
                Some(ref handle) => true, //handle.try_join().is_ok(),
                None => false
            };

            if image_loaded { wm_state.refresh_wallpaper(); }

            wm_state.render_background();
        }
    }       


    fn output_resolution(&mut self, output: &Output, old_res: Size, new_res: Size) {
        if let Ok(mut wm_state) = WM_STATE.write() {
            wm_state.tree.set_outer_geometry(Geometry::new(Point::origin(), new_res));
        }

        println!("Updated resolution: {:?}", new_res);
    }

    fn view_created(&mut self, view: &View) -> bool {
        if let Ok(mut wm_state) = WM_STATE.write(){
            view.set_visibility(view.output().visibility());
            view.bring_to_front();
            view.focus();    

            if view.view_type().is_empty(){
                let mut window = Window::init_dummy();
                window.attach_view(view.weak_reference());
                
                if view.view_type().is_empty(){
                    let mut layout_policy = wm_state.tree.layout_policy.clone();
                    let window_elem_id = layout_policy.seat_window(&mut wm_state.tree); 
                    
                    layout_policy.decorate_window(&mut wm_state, window_elem_id);
                    
                    wm_state.tree.layout_policy = layout_policy;

                    wm_state.tree.reserve_element_identity(window_elem_id, LayoutElementProfile::Window(window));  
                    
                    let tag = format!("{}", window_elem_id);

                    if !tag.is_empty(){
                        wm_state.tree.tags.tag_element(tag.as_ref(), window_elem_id);
                    }

                    if let Ok(mut pending_jobs) = PENDING_JOBS.try_lock(){
                        pending_jobs.push(Job::init_unconditional(JobType::LAYOUT_REFRESH));
                    } 
                }
            }
        }

        WM_CATCH_EVENT
    }
    
    fn view_destroyed(&mut self, view: &View){
        if let Ok(mut wm_state) = WM_STATE.write() {
            let mut layout_policy = wm_state.tree.layout_policy.clone();
            if let Some(element_ident) = wm_state.tree.lookup_element_from_view(view){
                layout_policy.detach_window(&mut wm_state.tree, element_ident);
                wm_state.tree.remove_view_binding_to(element_ident);
            }
        }
    }

    fn view_focus(&mut self, view: &View, focused: bool) {
        if focused && view.view_type().is_empty(){
            if let Ok(mut pending_jobs) = PENDING_JOBS.try_lock(){
                pending_jobs.push(Job::init(JobType::FOCUS, Some(ElementReference::ViewPID(view.pid())), Vec::new()));
            }  
        }
    }

    //fn view_request_move(&mut self, view: &View, origin: Point) {}
    //fn view_request_geometry(&mut self, _view: &View, _geometry: Geometry) {}

    fn keyboard_key(&mut self, view: Option<&View>, _time: u32, modifiers: Modifiers, sym: Key, state: KeyState) -> bool {
        use std::process::Command;

        if state == KeyState::Pressed {
            if let Ok(mut wm_state) = WM_STATE.write() {
                let meta_view_key = wm_state.config.keyboard.meta_view_key;

                //Press F3 for tree view
                if sym == Key::F3{
                    println!();
                    println!("~ Layout structure ~\n{}", wm_state.tree);
                    
                    return WM_CATCH_EVENT;
                }

                //Press F5 to force an update to the arrangement
                if sym == Key::F4{
                    if let Ok(mut pending_jobs) = PENDING_JOBS.try_lock(){
                        pending_jobs.push(Job::init_unconditional(JobType::LAYOUT_REFRESH));
                    } 
                    return WM_CATCH_EVENT;
                }
                if wm_state.config.keyboard.mod_key_is_pressed(modifiers.mods) {
                    // Window manager catch modifier
                    if let Some(&root_ident) = wm_state.tree.tags.address_element_by_tag(wm_state.config.layout.root_tag.clone()).first() {
                        if sym == meta_view_key {
                            if let Some(&jumper_ident) = wm_state.tree.tags.address_element_by_tag(wm_state.config.layout.jumper_tag.clone()).first() {
                                wm_state.tree.animate_property(jumper_ident, "offset_x", 0f32, false, 250);
                                wm_state.tree.animate_property(jumper_ident, "offset_y", 0f32, false, 250);
                            }
                            wm_state.tree.animate_property(root_ident, "inner_scale_x", 1f32 / wm_state.config.layout.grid_width() as f32, false, 125);
                            wm_state.tree.animate_property(root_ident, "inner_scale_y", 1f32 / wm_state.config.layout.grid_height() as f32, false, 125);
                        }
                        else {
                            wm_state.tree.animate_property(root_ident, "inner_scale_x", 1.0f32, false, 125);
                            wm_state.tree.animate_property(root_ident, "inner_scale_y", 1.0f32, false, 125);
                        }
                    }

                    let display_geometry = wm_state.tree.get_outer_geometry();
                    let mut new_workspace_offset = None;
                    if sym == Key::Left || sym == Key::Right || sym == Key::Up || sym == Key::Down {

                        let mut pre = None;
                        let mut post = None;
                        let animation_time = 500;

                        if let Some(mut element) = wm_state.tree.lookup_element_by_tag(wm_state.config.layout.grid_tag.clone()).first_mut() {
                            match element.profile{
                                LayoutElementProfile::Grid(ref mut grid) => {
                                    pre = Some(grid.get_active_child_id());
                                    
                                    grid.switch_to_subspace_in_direction(
                                        match sym{
                                            Key::Left => Direction::LEFT,
                                            Key::Right => Direction::RIGHT,
                                            Key::Up => Direction::UP,
                                            Key::Down => Direction::DOWN,
                                            _ => panic!("The number of key check are more than the possible direction.")
                                        }
                                    );

                                    post = Some(grid.get_active_child_id());
                                    wm_state.tree.animate_property_explicitly(post.unwrap(), "frame_opacity", 0.0f32, 1.0f32, false, animation_time, 0);
                                    new_workspace_offset = Some(grid.get_offset_geometry(display_geometry, Geometry::zero(), grid.active_subspace() as u16, &mut (1.0f32, 1.0f32)));
                                }
                                _ => { panic!("Expected element to be a workspace.") }
                            }
                        }
                        
                        if let Some(&jumper_ident) = wm_state.tree.tags.address_element_by_tag(wm_state.config.layout.jumper_tag.clone()).first() {
                            if let Some(geometry) = new_workspace_offset{
                                wm_state.tree.animate_property(jumper_ident, "offset_x", -geometry.origin.x as f32, false, 300);
                                wm_state.tree.animate_property(jumper_ident, "offset_y", -geometry.origin.y as f32, false, 300);

                                let zoom_magnitude = (wm_state.tree.get_outer_geometry().size.w / 10) as f32;
                                //wm_state.tree.animate_property(jumper_ident, "gap_size", zoom_magnitude, false, 125);
                                //wm_state.tree.animate_property_explicitly(jumper_ident, "gap_size", zoom_magnitude, 0f32 as f32, false, 125, 126);
                            }
                        }
                        if let Ok(mut pending_jobs) = PENDING_JOBS.try_lock(){
                            pending_jobs.push(Job::init_unconditional(JobType::LAYOUT_REFRESH));
                        } 

                        return WM_CATCH_EVENT;
                    }

                    if let Some(v) = view  {
                        if sym == Key::C {
                            v.close();
                            
                            return WM_CATCH_EVENT;
                        } 
                    }

                    if sym == Key::Esc {
                        terminate();  
                        return WM_CATCH_EVENT;
                    }
                }

                if let Some(matching_hotkey_executable) = wm_state.config.keyboard.matching_hotkey(modifiers.mods, sym) {
                    Command::new("sh")
                        .arg("-c")
                        .arg(matching_hotkey_executable)
                        .spawn();

                    return WM_CATCH_EVENT;
                }
            }
        }

        return WM_FORWARD_EVENT_TO_CLIENT;
    }

    fn pointer_button(&mut self, view: Option<&View>, _time: u32, modifiers: Modifiers, button: Button, state: ButtonState, origin: Point) -> bool {
        if let Ok(mut wm_state) = WM_STATE.write() {
            if let Some(ref mut input_dev) = wm_state.input_dev{
                input_dev.left_click = ButtonState::Released;
                input_dev.right_click = ButtonState::Released;
            
                if state == ButtonState::Pressed {
                    match button{
                        Button::Left => input_dev.left_click = ButtonState::Pressed,
                        Button::Right => input_dev.right_click = ButtonState::Pressed,
                        _ => {  }
                    }

                    if let Some(v) = view {
                        v.focus();
                        
                        if modifiers.mods.contains(Modifier::Ctrl) {
                            return WM_CATCH_EVENT;
                        }
                    }
                }
                else {
                    input_dev.resize = None;
                } 
            }
        }

        WM_FORWARD_EVENT_TO_CLIENT
    }

    fn pointer_motion(&mut self, _view: Option<&View>, _time: u32, point: Point) -> bool {
        if let Ok(mut wm_state) = WM_STATE.write() {
            let &mut WMState {ref tree, ref mut input_dev, ..} = wm_state.deref_mut();

            let (mut dx, mut dy) = (0, 0);
            let mut active_right_click = false;
            if let &mut Some(ref mut dev) = input_dev{
                dx = point.x - dev.mouse_location.x;
                dy = point.y - dev.mouse_location.y;

                dev.mouse_travel(
                    Point{
                        x: dx,
                        y: dy
                    }
                );

                if let Some( (element_ident, orientation) ) = dev.resize {
                    // Handle window resizing
                    let (x, y, w, h) = {
                        let parent = tree.parent_of(element_ident);
                        let parent_geometry = tree.geometry_of(parent).unwrap();

                        (
                            parent_geometry.origin.x as f32, 
                            parent_geometry.origin.y as f32,
                            parent_geometry.size.w as f32, 
                            parent_geometry.size.h as f32
                        )
                    };

                    tree.animate_property (
                        element_ident, 
                        "ratio", 
                        match orientation { 
                            Orientation::Horizontal => (point.x as f32 - x) / w, 
                            Orientation::Vertical => (point.y as f32 - y) / h
                        },
                        false, 
                        1
                    );
                }

                active_right_click = dev.right_click == ButtonState::Pressed;
            }
        }

        // Note: Forward is REQUIRED for input to be registered by clients
        WM_FORWARD_EVENT_TO_CLIENT
    }

    fn view_request_resize(&mut self, view: &View, edges: Flags, origin: Point) { 
        if let Ok(mut wm_state) = WM_STATE.write() {
            let &mut WMState {ref tree, ref mut input_dev, ..} = wm_state.deref_mut();
            
            if let Some(elem_ident) = tree.lookup_element_from_view(view) {
                let parent_id = tree.parent_of(elem_ident);

                input_dev.as_mut().unwrap().resize = match tree.lookup_element(parent_id).unwrap().profile {
                    LayoutElementProfile::Bisect(ref bisect) => Some( (parent_id, bisect.orientation) ),
                    _ => None
                };
            }
            
        }
    }
}
