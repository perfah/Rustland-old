use std::convert::From;
use std::fmt;

use definitions::{LayoutElemID, ElementReference};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum JobType{
    NA,
    FOCUS,
    INSERT_WRKSPC,
    RUN_APP,
    SEND_TREE,
    MOVE_TO
}


impl fmt::Display for JobType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}", match self
        {
            NA => "N/A",
            FOCUS => "Set the focus on an element",
            INSERT_WRKSPC => "Insert a workspace at the focused location",
            RUN_APP => "Run application",
            SEND_TREE => "Tree view",
            MOVE_TO => "Element move"
        })
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Job{
    pub job_type: JobType,
    pub main_ref: Option<ElementReference>,
    pub contextual_refs: Vec<ElementReference>,
    pub anonymous_args: Vec<String>, 
    pub generated_result: Result<String, String>
}

impl Job{
    pub fn init(job_type: JobType, main_ref: Option<ElementReference>, contextual_refs: Vec<ElementReference>) -> Self{
        Job{
            job_type: job_type,
            main_ref: main_ref,
            contextual_refs: contextual_refs,
            anonymous_args: Vec::new(),
            generated_result: Err("No generated result.".to_string())
        }
    }
}

impl Default for Job {
    fn default() -> Self { 
        Job{
            job_type: JobType::NA, 
            main_ref: None, 
            contextual_refs: Vec::new(), 
            anonymous_args: Vec::new(),
            generated_result: Err("No generated result.".to_string())
        }
    }
}

