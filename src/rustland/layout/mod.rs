pub mod arrangement;
pub mod element;
pub mod rules;
pub mod tag;

use std::cmp;
use std::fmt;
use std::collections::HashMap;
use std::cell::*;
use std::cell::*;
use std::sync::RwLock;
use std::rc::Rc;

use rustwlc::handle::*;
use rustwlc::types::*;
use wmstate::*;
use definitions::{LayoutElemID, MAX_WORKSPACES_LIMIT};
use layout::element::LayoutElement;
use layout::element::segmentation::*;
use layout::element::workspace::*;
use layout::element::segmentation::*;
use layout::element::window::*;

use layout::rules::*;
use layout::tag::*;

pub const PARENT_ELEMENT: LayoutElemID = 0;

pub struct LayoutTree{
    // the currently active workspace
    active_workspace: u16,

    // the index of the last added element
    active_id: LayoutElemID,

    // the available workspaces of the layout 
    elements: HashMap<LayoutElemID, RefCell<LayoutElement>>,

    // the complete geometrical surface of all monitors 
    outer_geometry: Geometry,

    // tag register used to give names to layout elements  
    pub tags: TagRegister,

    // rule_set: yet to be implemented
    rule_set: RefCell<Box<RuleSet>>,

    // a layout element for each WlcView PID
    view_assoc: HashMap<i32, LayoutElemID> 
}

impl LayoutTree {
    pub fn init(outer_geometry: Geometry, no_monitors: u16, rule_set: RefCell<Box<RuleSet>>) -> Self{
        const default_workspace: u16 = 1; 
        assert!(default_workspace <= 1 as u16, "The minimum number of workspaces required are {}", default_workspace);

        let mut tree = LayoutTree{
            active_id: PARENT_ELEMENT,  
            active_workspace: default_workspace,
            elements: HashMap::new(),
            tags: TagRegister::init(),
            outer_geometry: outer_geometry,
            rule_set: rule_set,
            view_assoc: HashMap::new()
        };

        //Place root 
        let parent_id = tree.spawn_element();
        tree.tags.tag_element_on_condition("root", |elem_id| elem_id == PARENT_ELEMENT);
        let parent_element = Segmentation::init(&mut tree, no_monitors, Orientation::Horizontal);
        for child_id in parent_element.get_children(){
            let workspace = Workspace::init(&mut tree, MAX_WORKSPACES_LIMIT);
            tree.swap_element(*child_id, LayoutElement::Workspace(workspace));
        }
        
        tree.swap_element(parent_id, LayoutElement::Segm(parent_element));

        tree
    }

    pub fn refresh(&mut self)
    {
        let elements = self.get_all_element_ids();
        self.tags.refresh_tag_statuses(elements);

        arrangement::arrange(self, PARENT_ELEMENT, self.outer_geometry);
    }

    pub fn lookup_element(&self, elem_id: LayoutElemID) -> Option<RefMut<LayoutElement>>{   
        match self.elements.get(&elem_id)
        {
            Some(element) => Some(element.borrow_mut()),
            None => { panic!("Element out of reach.") }
        }
    }
    pub fn lookup_element_by_tag(&self, tag: String) -> Vec<RefMut<LayoutElement>>{   
        let mut element_references = Vec::<RefMut<LayoutElement>>::new();
        
        for elem_id in self.tags.address_element_by_tag(tag){
            match self.elements.get(&elem_id)
            {
                Some(element) => { element_references.push(element.borrow_mut()) },
                None => { panic!("Element out of reach.") }
            }
        }

        element_references
    }
    pub fn lookup_element_from_view(&self, view_pid: i32) -> LayoutElemID{
        match self.view_assoc.get(&view_pid)
        {
            Some(element_id) => *element_id,
            None => { panic!("Element not found!"); }
        }
    }

    pub fn swap_element(&mut self, elem_id: LayoutElemID, new_element: LayoutElement) -> Option<RefCell<LayoutElement>>
    {
        self.swap_cell(elem_id, RefCell::new(new_element))
    }
    pub fn swap_cell(&mut self, elem_id: LayoutElemID, new_cell: RefCell<LayoutElement>) -> Option<RefCell<LayoutElement>>
    {
        match *(new_cell.borrow()){
            LayoutElement::Window(ref window) => { 
                if let Some(view) = window.get_view(){
                    self.view_assoc.insert(view.get_pid(), elem_id); 
                }
            }
            _ => {}
        }

        let old_cell = self.elements.insert(
            elem_id, 
            new_cell
        );

        if let Some(ref old_element) = old_cell{
            match *(old_element.borrow()){
                LayoutElement::Window(ref window) => { 
                    if let Some(view) = window.get_view(){    
                        self.view_assoc.remove(&view.get_pid()); 
                    }
                }
                _ => {}
            }
        }

        old_cell
    }

    pub fn spawn_element(&mut self) -> LayoutElemID{
        self.elements.insert(self.active_id, RefCell::new(LayoutElement::None));
        self.active_id += 1;

        return self.active_id - 1;
    } 

    pub fn root(&self) -> RefMut<LayoutElement>{
        match self.lookup_element(PARENT_ELEMENT)
        {
            Some(parent) => parent,
            None => { panic!("Root not found!"); }
        }
    }

    pub fn get_all_element_ids(&self) -> Vec<LayoutElemID>{
        let mut elements_ids = Vec::new();

        for elem_id in self.elements.keys() {
            elements_ids.push(elem_id.clone())
        }

        elements_ids
    }

    pub fn last_window_id(&self) -> Option<LayoutElemID>{
        let mut i = self.active_id - 1;
        
        while {
            if let Some(a) = self.elements.get(&i)
            {   
                match *a.borrow(){
                    LayoutElement::Window(ref window) => { return Some(i) }
                    _ => { true }
                }
            }
            else {
                false
            }
        }{
            match i{
                0 => { break; },
                _ => { i -= 1; }
            }
        };

        None        
    }

    pub fn get_outer_geometry(&self) -> Geometry{
        self.outer_geometry
    }

    pub fn set_outer_geometry(&mut self, new_geometry: Geometry){
        self.outer_geometry = new_geometry;
    }

    pub fn get_rule_set(&self) -> &RefCell<Box<RuleSet>>{
        &self.rule_set
    }
}

impl fmt::Display for LayoutTree{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut indentation_whtspcs = 0;
        arrangement::tree(self, f, PARENT_ELEMENT, &mut indentation_whtspcs);

        writeln!(f)
    }
}

pub extern fn on_output_resolution(output: WlcOutput, _from: &Size, _to: &Size) {
    let mut wm_state = WM_STATE.write().unwrap();

    wm_state.tree.set_outer_geometry(Geometry::new(Point::origin(), *_to));

    println!("Updated resolution: {}", _to);
}
