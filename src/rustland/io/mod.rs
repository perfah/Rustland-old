use std::process::Command;

use common::definitions::ElementReference;
use common::job::{Job, JobType};
use wmstate::{WM_STATE, PENDING_JOBS, FINALIZED_JOBS};
use layout::element::LayoutElement;
use layout::arrangement;
use layout::tag::TagRegister;
use layout::LayoutTree;

use wlc::ViewState;

pub mod physical;
pub mod tcp_server;

pub fn process_all_current_jobs(){
    if let Ok(mut pending_jobs) = PENDING_JOBS.lock(){
        if let Ok(mut finalized_jobs) = FINALIZED_JOBS.lock(){
            while let Some(mut job) = pending_jobs.pop(){
                let result = process_job(&job);
                job.generated_result =  result;

                match job.generated_result 
                {
                    Ok(ref expected_result) => println!("Notice: A job described as '{}' has been processed.", format!("{}", job.job_type).to_lowercase()),
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
        JobType::NA => { panic!("WTF") }
        JobType::FOCUS => {
            let mut wm_state = WM_STATE.write().unwrap();
            if let Some(ref main_ref) = job.main_ref{
                if let Some(target_element_id) = wm_state.tree.tags.address_element(main_ref.clone()).first().cloned(){            
                    for (view_id, elem_id) in wm_state.tree.tags.view_bindings.iter(){
                        if let Some(mut element) = wm_state.tree.lookup_element(*elem_id){
                            match *element
                            {
                                LayoutElement::Window(ref mut window) => {
                                    if let ElementReference::ViewPID(view_pid_to_focus_on) = *main_ref{
                                        if let Some(view) = window.get_view().as_mut(){
                                            if view.pid() == view_pid_to_focus_on{
                                                view.set_state(ViewState::Activated, true);
                                                break;
                                            }
                                            else{
                                                view.set_state(ViewState::Activated, false);
                                            }
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                    
                    // update tag cache
                    wm_state.tree.focused_id = target_element_id;
                    TagRegister::refresh_tag_statuses(&mut wm_state);

                    Ok(String::from("Focused changed."))
                }
                else{
                    Err(String::from("That element does not exist in the layout."))
                }
            }
            else{
                Err(String::from("Focus on what?"))
            }
        },
        JobType::LAYOUT_REFRESH => {
            let mut wm_state = WM_STATE.write().unwrap();
            LayoutTree::refresh(&mut wm_state);

            Ok(String::from("Layout refreshed."))
        },
        JobType::WORKSPACE_INSERT => {
            Err(String::from("Unimplemented :("))
        },
        JobType::RUN_APP => {
            if let Some(application) = job.anonymous_args.first().cloned(){
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
            let mut wm_state = WM_STATE.write().unwrap();

            if job.main_ref.is_none() || job.contextual_refs.is_empty(){
                Err(String::from("Move what, to where?"))
            }
            else{
                let carry_id = wm_state.tree.tags.address_element(job.main_ref.clone().unwrap_or(ElementReference::ElementID(0))).first().cloned().unwrap_or(0);
                let dest_id = wm_state.tree.tags.address_element(job.contextual_refs.first().cloned().unwrap_or(ElementReference::ElementID(0))).first().cloned().unwrap_or(0);

                arrangement::move_element(&mut wm_state, carry_id, dest_id)
            }
        }
    }
}