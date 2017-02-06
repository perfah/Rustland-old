pub mod workspace;
pub mod window;
pub mod arrangement;
pub mod segmentation;
pub mod rules;

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
use definitions::{LayoutElemID};
use layout::segmentation::*;
use layout::rules::*;

pub enum LayoutElement
{
    // Unallocated space in the layout
    None,

    // A reactive container with exactly one child element that can be turned on or off
    Workspace(workspace::Workspace),

    // A segmentation of multiple child elements
    Segm(segmentation::Segmentation),
    
    // 
    Window(window::Window)
}


impl fmt::Debug for LayoutElement{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &LayoutElement::None => "",
            &LayoutElement::Workspace(ref workspace) => "Workspace",
            &LayoutElement::Segm(ref segmentation) => "Segmentation",
            &LayoutElement::Window(ref window) => "Title [geometry]"
        })
    }
}

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

    // 
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
            outer_geometry: outer_geometry,
            rule_set: rule_set,
            view_assoc: HashMap::new()
        };

        //Place root 
        let parent_id = tree.spawn_element();
        let parent_element = LayoutElement::Segm(Segmentation::init(&mut tree, no_monitors, Orientation::Horizontal));
        tree.swap_element(parent_id, parent_element);
        
        tree
    }

    pub fn arrange(&self)
    {
        arrangement::arrange(&self, PARENT_ELEMENT, self.outer_geometry);
    }

    pub fn lookup_element(&self, id: LayoutElemID) -> Option<RefMut<LayoutElement>>{   
        // is option really necessary
        
        match self.elements.get(&id)
        {
            Some(element) => Some(element.borrow_mut()),
            None => { panic!("Element out of reach.") }
        }
    }

    pub fn lookup_element_from_view(&self, view_pid: i32) -> LayoutElemID{
        match self.view_assoc.get(&view_pid)
        {
            Some(element_id) => *element_id,
            None => { panic!("Element not found!"); }
        }
    }

    pub fn swap_element(&mut self, id: LayoutElemID, new_element: LayoutElement) -> Option<RefCell<LayoutElement>>
    {
        self.elements.insert(
            id, 
            RefCell::new(new_element)
        )
    }
    pub fn swap_cell(&mut self, id: LayoutElemID, new_cell: RefCell<LayoutElement>) -> Option<RefCell<LayoutElement>>
    {
        match *(new_cell.borrow()){
            LayoutElement::Window(ref window) => { 
                if let Some(view) = window.get_view(){
                    self.view_assoc.insert(view.get_pid(), id); 
                }
            }
            _ => {}
        }

        let old_cell = self.elements.insert(
            id, 
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

    pub fn last_window_id(&self) -> LayoutElemID{
        let mut i = self.active_id - 1;

        while {
            if let Some(a) = self.elements.get(&i)
            {   
                match *a.borrow(){
                    LayoutElement::Window(ref window) => { false }
                    _ => { true }
                }
            }
            else {
                panic!("Nothing found!!")
            }
        }{
            i -= 1;
        };

        i
    }

    pub fn get_rule_set(&self) -> &RefCell<Box<RuleSet>>{
        &self.rule_set
    }
}

impl fmt::Debug for LayoutTree{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a")
        
    }
}

pub extern fn on_output_resolution(output: WlcOutput, _from: &Size, _to: &Size) {
    
}
