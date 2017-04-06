pub mod physical;
pub mod tcp_server;

use common::job::{Job, JobType};
use wmstate::{WM_STATE, PENDING_JOBS, FINALIZED_JOBS};
use layout::element::LayoutElement;
use layout::arrangement;

use std::ops::DerefMut;
use std::borrow::BorrowMut;
use std::process::Command;

fn govern_current_job(){
    if let Ok(mut pending_jobs) = PENDING_JOBS.lock(){
        if let Ok(mut finalized_jobs) = FINALIZED_JOBS.lock(){
            while let Some(mut job) = pending_jobs.pop(){
                let result = process_job(&job);
                job.generated_result = result;

                match job.generated_result 
                {
                    Ok(ref expected_result) => println!("Notice: A job issued by a client has been processed: {}", job.job_type),
                    Err(ref e) => println!("Couldn't process job request: {}, cause: {}", job.job_type, e.to_lowercase())
                }

                finalized_jobs.push(job);
            }
        }     
    }
}

fn process_job(job: &Job) -> Result<String, String>{
    match job.job_type
    {
        JobType::NA => Err(String::from("Invalid job.")),
        JobType::FOCUS => {
            Err(String::from("Unimplemented :("))
        },
        JobType::INSERT_WRKSPC => {
            Err(String::from("Unimplemented :("))
        },
        JobType::RUN_APP => {
            if let Some(application) = job.contextual_tags.first().cloned(){
                let _ = Command::new("sh")
                    .arg("-c")
                    .arg(application)
                    .spawn()
                    .unwrap_or_else(|e| {
                        panic!("Can't spawn process!");
                    });
            }

            Ok(String::from("Application started."))
        },
        JobType::SEND_TREE => Ok(format!("{}", WM_STATE.write().unwrap().tree)),
        JobType::MOVE_TO => {
            let mut wmstate = WM_STATE.write().unwrap();

            if job.head_tag.is_none() || job.contextual_tags.is_empty(){
                Err(String::from("Move what, to where?"))
            }
            else{
                let carry_id = wmstate.tree.tags.address_element_by_tag(job.head_tag.clone().unwrap_or(String::new())).first().cloned().unwrap_or(0);
                let dest_id = wmstate.tree.tags.address_element_by_tag(job.contextual_tags.first().cloned().unwrap_or(String::new())).first().cloned().unwrap_or(0);

                arrangement::move_element(&mut wmstate, carry_id, dest_id)
            }
        }
    }
}