use std::sync::RwLock;
use std::io::Write;
use std::process::Command;
use std::io::Read;
use std::thread::{spawn, sleep};
use std::time;        
use std::ops::DerefMut;

use common::definitions::{WM_FORWARD_EVENT_TO_CLIENT, WM_CATCH_EVENT};
use common::job::{Job, JobType};
use common::definitions::{FPS, ElementReference};
use io::physical::InputDevice;
use layout::element::{LayoutElement, LayoutElementProfile};
use layout::element::window::Window;
use layout::element::grid::Direction;
use layout::LayoutTree;
use utils::geometry::{PointExt, SizeExt, GeometryExt};
use wmstate::{WMState, WM_STATE, PENDING_JOBS, FINALIZED_JOBS, ACTIVE_TRANSITIONS};
use async::{schedule_wallpaper_init, schedule_job_routine, schedule_animator_routine, schedule_tcp_routine};

use wlc::{Callback, Key, KeyState, Point, Size, Geometry, ButtonState, View, Output, Visibility, Modifier, Modifiers, Button, terminate};
use wlc::render::{RenderOutput, RenderView};
use egli;
use gl;
use thread_tryjoin::TryJoinHandle;

pub struct Compositor;

impl Callback for Compositor {
    fn compositor_ready(&mut self){  
        gl::load_with(|s| egli::egl::get_proc_address(s) as *const _);
        gl::Viewport::load_with(|s| egli::egl::get_proc_address(s) as *const _);

        if let Ok(mut wm_state) = WM_STATE.write(){
            wm_state.input_dev = Some(InputDevice::init());
            let program = wm_state.init_graphics_program();
            LayoutTree::refresh(&mut wm_state);
            
            // Temporary hardcoding of wallpaper file path (until compositor configuration is available):
            use std::env::home_dir;
            let mut wallpaper_path = home_dir().unwrap();
            wallpaper_path.push("wallpaper.jpg");

            wm_state.next_wallpaper_image = Some(schedule_wallpaper_init(wallpaper_path));
        }

        schedule_job_routine();              
        schedule_tcp_routine();    
        schedule_animator_routine();         
    }

    fn output_render_pre(&mut self, output: &mut RenderOutput) {        
        if let Ok(mut wm_state) = WM_STATE.write(){
            let image_loaded = match wm_state.next_wallpaper_image {
                Some(ref handle) => handle.try_join().is_ok(),
                None => false
            };
            if image_loaded { wm_state.refresh_wallpaper(); }

            wm_state.render_wallpaper();
        }
    }


    fn output_resolution(&mut self, output: &Output, old_res: Size, new_res: Size) {
        let mut wm_state = WM_STATE.write().unwrap();

        wm_state.tree.set_outer_geometry(Geometry::new(Point::origin(), new_res));

        println!("Updated resolution: {:?}", new_res);
    }

    fn view_created(&mut self, view: &View) -> bool {
        let mut wm_state = WM_STATE.write().unwrap();

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

                if let Ok(mut pending_jobs) = PENDING_JOBS.lock(){
                    pending_jobs.push(Job::init_unconditional(JobType::LAYOUT_REFRESH));
                } 
            }
        }

        WM_CATCH_EVENT
    }
    
    //fn view_destroyed(&mut self, view: &View);

    fn view_focus(&mut self, view: &View, focused: bool) {
        if focused && view.view_type().is_empty(){
            if let Ok(mut pending_jobs) = PENDING_JOBS.lock(){
                pending_jobs.push(Job::init(JobType::FOCUS, Some(ElementReference::ViewPID(view.pid())), Vec::new()));
            }  
        }
    }

    //fn view_request_move(&mut self, view: &View, origin: Point) {}
    //fn view_request_resize(&mut self, view: &View, edges: ResizeEdge::Flags, origin: Point) {}
    //fn view_request_geometry(&mut self, _view: &View, _geometry: Geometry) {}

    fn keyboard_key(&mut self, view: Option<&View>, _time: u32, modifiers: Modifiers, sym: Key, state: KeyState) -> bool {
        use std::process::Command;

        if state == KeyState::Pressed {
            let wm_state = WM_STATE.write().unwrap();

            //Press F3 for tree view
            if sym == Key::F3{
                println!();
                println!("~ Layout structure ~\n{}", wm_state.tree);
                
                return WM_CATCH_EVENT;
            }

            //Press F5 to force an update to the arrangement
            if sym == Key::F5{
                if let Ok(mut pending_jobs) = PENDING_JOBS.lock(){
                    pending_jobs.push(Job::init_unconditional(JobType::LAYOUT_REFRESH));
                } 
                return WM_CATCH_EVENT;
            }

            if modifiers.mods == Modifier::Ctrl {
                // Window manager catch modifier
                if let Some(&root_ident) = wm_state.tree.tags.address_element_by_tag("root".to_string()).first() {
                    if sym == Key::Tab {
                        if let Some(&jumper_ident) = wm_state.tree.tags.address_element_by_tag("workspace_selection".to_string()).first() {
                            wm_state.tree.animate_property(jumper_ident, "offset_x", 0f32, false, 250);
                            wm_state.tree.animate_property(jumper_ident, "offset_y", 0f32, false, 250);
                        }
                        wm_state.tree.animate_property(root_ident, "inner_scale", 0.33f32, false, 125);
                    }
                    else {

                        wm_state.tree.animate_property(root_ident, "inner_scale", 1.0f32, false, 125);
                    }
                }

                let display_geometry = wm_state.tree.get_outer_geometry();
                let mut new_workspace_offset = None;
                if sym == Key::Left || sym == Key::Right || sym == Key::Up || sym == Key::Down {
                    if let Some(element) = wm_state.tree.lookup_element(2) {
                        match *element.get_profile_mut(){
                            LayoutElementProfile::Grid(ref mut wrkspc) => {
                                wrkspc.switch_to_subspace_in_direction(
                                    match sym{
                                        Key::Left => Direction::LEFT,
                                        Key::Right => Direction::RIGHT,
                                        Key::Up => Direction::UP,
                                        Key::Down => Direction::DOWN,
                                        _ => panic!("The number of key check are more than the possible direction.")
                                    }
                                );
                                
                                new_workspace_offset = Some(wrkspc.get_offset_geometry(display_geometry, Geometry::zero(), wrkspc.get_active_subspace() as u16, &mut 1.0f32));
                            }
                            _ => { panic!("Expected element to be a workspace.") }
                        }
                    }
                    
                    if let Some(&jumper_ident) = wm_state.tree.tags.address_element_by_tag("workspace_selection".to_string()).first() {
                        if let Some(geometry) = new_workspace_offset{
                            wm_state.tree.animate_property(jumper_ident, "offset_x", -geometry.origin.x as f32, false, 250);
                            wm_state.tree.animate_property(jumper_ident, "offset_y", -geometry.origin.y as f32, false, 250);

                            let zoom_magnitude = (wm_state.tree.get_outer_geometry().size.w / 10) as f32;
                            wm_state.tree.animate_property(jumper_ident, "gap_size", zoom_magnitude, false, 125);
                            wm_state.tree.animate_property_after_delay(jumper_ident, "gap_size", zoom_magnitude, 0f32 as f32, false, 125, 126);
                        }
                    }
                    if let Ok(mut pending_jobs) = PENDING_JOBS.lock(){
                        pending_jobs.push(Job::init_unconditional(JobType::LAYOUT_REFRESH));
                    } 

                    return WM_CATCH_EVENT;
                }

                if let Some(v) = view  {
                    if sym == Key::C {
                        v.close();
                        
                        return WM_CATCH_EVENT;
                    } 
                    else if sym == Key::Tab {
                        v.send_to_back();
                        //get_topmost_view(v.output(), 0).unwrap().focus();
                        return WM_CATCH_EVENT;
                    }
                }

                if sym == Key::Esc {
                    terminate();
                    return WM_CATCH_EVENT;
                }
                else if sym == Key::Space {
                    let _ = Command::new("sh")
                        .arg("-c")
                        .arg("/usr/bin/dmenu_run")
                        .spawn()
                        .unwrap_or_else(|e| {
                            panic!("Can't spawn process!");
                        });

                    return WM_CATCH_EVENT;
                }
            }
        }

        return WM_FORWARD_EVENT_TO_CLIENT;
    }

    fn pointer_button(&mut self, view: Option<&View>, _time: u32, modifiers: Modifiers, button: Button, state: ButtonState, origin: Point) -> bool {
        let mut wm_state = WM_STATE.write().unwrap();
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
        }

        WM_FORWARD_EVENT_TO_CLIENT
    }

    fn pointer_motion(&mut self, _view: Option<&View>, _time: u32, point: Point) -> bool {
        let mut wm_state = WM_STATE.write().unwrap();

        let (mut dx, mut dy) = (0, 0);
        let mut active_right_click = false;
        if let Some(ref mut input_dev) = wm_state.input_dev{
            dx = point.x - input_dev.mouse_location.x;
            dy = point.y - input_dev.mouse_location.y;

            input_dev.mouse_travel(
                Point{
                    x: dx,
                    y: dy
                }
            );

            active_right_click = input_dev.right_click == ButtonState::Pressed;
        }

        // Note: Forward is REQUIRED for input to be registered by clients
        WM_FORWARD_EVENT_TO_CLIENT
    }
}
