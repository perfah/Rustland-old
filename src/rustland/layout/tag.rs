use std::vec;
use std::collections::HashMap;
use std::sync::MutexGuard;
use std::cell::RefCell;
use wmstate::WMState;

use common::definitions::{LayoutElemID, ViewPID, ElementReference};
use common::definitions::ElementReference::*;
use common::job::Job;
use wmstate::PENDING_JOBS;
use layout::element::LayoutElement;
use layout::LayoutTree;

use wlc::WeakView;

pub struct TagRegister{
    // bindings between tags and n LayoutElements  
    bindings: HashMap<String, Vec<LayoutElemID>>,

    // bindings between element ids and View PID:s
    pub view_bindings: HashMap<WeakView, LayoutElemID>,

    pub view_pid_bindings: HashMap<ViewPID, LayoutElemID>,

    // closure functions (values) determining whether LayoutElements can be addressed by specific tags (keys) 
    tag_conditions: HashMap<String, Box<Fn(LayoutElemID, &WMState) -> bool>>
}

impl TagRegister{
    pub fn init() -> TagRegister{
        TagRegister{
            bindings: HashMap::new(),
            view_bindings: HashMap::new(),
            view_pid_bindings: HashMap::new(),
            tag_conditions: HashMap::new()
        }
    }

    pub fn address_element(&self, reference: ElementReference) -> Vec<LayoutElemID> {
        match reference
        {
            ElementID(elem_id) => { vec![elem_id] },
            ViewPID(view_pid) => {
                if let Some(elem_id) = self.view_pid_bindings.get(&view_pid){
                    vec![*elem_id]
                }
                else{
                    Vec::new()
                }  
            },
            Tag(tag) => {
                if let Some(element_ids) = self.bindings.get(&tag){
                    element_ids.clone()
                }
                else{
                    Vec::new()
                }  
            }
        }

    }

    pub fn address_element_by_tag(&self, tag: String) -> Vec<LayoutElemID> {
        match self.bindings.get(&tag)
        {
            Some(elements) => elements.clone(),
            None => Vec::new()
        }  
    }

    pub fn address_tags_by_element(&self, elem_id: LayoutElemID) -> Vec<String>{
        let mut tags = Vec::<String>::new();
        
        for (tag, tag_elements) in &self.bindings{
            if tag_elements.contains(&elem_id){
                tags.push(tag.clone());
            }
        }

        tags
    }

    pub fn tag_element(&mut self, tag: &str, elem_id: LayoutElemID) {
        let unoccupied_tag = 
            if self.bindings.contains_key(&tag.to_string()){
                let mut counter = 2;
                let mut name_attempt: String;

                while {
                    name_attempt = format!("{}_{}", tag.to_string(), counter);
                    counter += 1;

                    self.bindings.contains_key(&name_attempt)
                } {}

                name_attempt
            }
            else{
                tag.to_string()
            };
        
        let tag_elements = self.bindings.entry(unoccupied_tag).or_insert(Vec::new());

        if !tag_elements.contains(&elem_id){
            tag_elements.push(elem_id);
        }
    }

    pub fn tag_element_on_condition<F: Fn(LayoutElemID, &WMState) -> bool + 'static>(&mut self, tag: &str, condition: F) {
        self.tag_conditions.insert(String::from(tag), Box::new(condition));
    }

    pub fn untag_element(&mut self, elem_id: LayoutElemID){
        for (tag, mut tag_elements) in &mut self.bindings{
            match tag_elements.binary_search(&elem_id)
            {
                Ok(index) => { tag_elements.remove(index); },
                _ => {}
            }
        }
    }

    pub fn remove_tag(&mut self, tag: &str, include_conditions: bool){
        self.bindings.remove(&tag.to_string());
    }

    pub fn refresh_tag_statuses(wm_state: &mut WMState){
        let mut elements_ids = Vec::new();

        for elem_id in wm_state.tree.get_all_element_ids(){
            elements_ids.push(elem_id.clone())
        }
        
        for (tag, det) in &wm_state.tree.tags.tag_conditions {
            for candidate_id in &elements_ids{
                if det(*candidate_id, wm_state) { 
                    let tag_elements = wm_state.tree.tags.bindings.entry(tag.clone()).or_insert(Vec::new());

                    if !tag_elements.contains(&candidate_id) {
                        tag_elements.push(*candidate_id);
                    }
                }
                else {
                    let tag_elements = wm_state.tree.tags.bindings.entry(tag.clone()).or_insert(Vec::new());

                    match tag_elements.binary_search(&candidate_id)
                    {
                        Ok(index) => { tag_elements.remove(index); },
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn handle_element_swap(&mut self, elem1_id: LayoutElemID, elem2_id: LayoutElemID){
        for tag_elements in self.bindings.values_mut(){
            for i in 0..tag_elements.len(){
                if tag_elements[i] == elem1_id{
                    tag_elements[i] = elem2_id;
                }
                else if tag_elements[i] == elem2_id{
                    tag_elements[i] = elem1_id;
                }
            }
        }
    }
}
