use std::vec;
use std::collections::HashMap;

use definitions::LayoutElemID;
use layout::LayoutTree;

pub struct TagRegister{
    // bindings between tags and n number of LayoutElements  
    bindings: HashMap<String, Vec<LayoutElemID>>,

    // closure functions (values) determining whether LayoutElements can be addressed by specific tags (keys) 
    tag_conditions: HashMap<String, Box<Fn(LayoutElemID) -> bool>>
}

impl TagRegister{
    pub fn init() -> TagRegister{
        TagRegister{
            bindings: HashMap::new(),
            tag_conditions: HashMap::new()
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
        let tag_elements = self.bindings.entry(tag.to_string()).or_insert(Vec::new());

        if !tag_elements.contains(&elem_id){
            tag_elements.push(elem_id);
        }
    }

    pub fn tag_element_on_condition<F: Fn(LayoutElemID) -> bool + 'static>(&mut self, tag: &str, condition: F) {
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

    pub fn refresh_tag_statuses(&mut self, element_candidates: Vec<LayoutElemID>){
        for (tag, det) in &self.tag_conditions {
            let tag_elements = self.bindings.entry(tag.clone()).or_insert(Vec::new());

            for cand_id in &element_candidates{
                if det(*cand_id) { 
                    if !tag_elements.contains(&cand_id) {
                        tag_elements.push(*cand_id);
                    }
                }
                else {
                    match tag_elements.binary_search(&cand_id)
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
