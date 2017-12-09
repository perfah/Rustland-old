use std::path::PathBuf;
use std::time;
use std::thread::{JoinHandle, spawn, sleep};
use std::sync::atomic::AtomicBool;

use common::definitions::FPS;
use layout::LayoutTree;
use io::process_all_current_jobs;
use io::tcp_server::{TCP_SOCKET, handle_client};
use wmstate::{WM_STATE, ACTIVE_TRANSITIONS};

use image;
use image::RgbaImage;
use wlc::event_loop::{event_loop_add_timer,TimerCallback};

pub fn schedule_wallpaper_init(path: PathBuf) ->  JoinHandle<RgbaImage> {
    println!("Initializing wallpaper...");

    spawn(move || {
        let path_copy = path.clone();

        match image::open(path){
            Ok(ref img) => img.to_rgba(),
            Err(_) => panic!(format!("{}: {}", "File path to wallpaper not found", path_copy.to_str().unwrap()))
        }
    })
    
}

pub fn schedule_job_routine(){
    spawn(||{
        let delta = 1000 / FPS;

        loop{
            // Continous processing of jobs
            sleep(time::Duration::from_millis(delta));
            process_all_current_jobs();
        }
    });
}
         
pub fn schedule_animator_routine(){
    spawn(|| {
        let delta = 1000 / FPS;

        loop{
            if let Ok(ref mut active_transitions) = ACTIVE_TRANSITIONS.try_lock(){    
                if !active_transitions.is_empty(){     
                    if let Ok(mut wm_state) = WM_STATE.try_write(){  

                        // Continues to step forward each transition
                        for transition in active_transitions.iter_mut(){
                            transition.next(&mut wm_state.tree, delta);
                        }

                        // Completed transition should not be in the list
                        active_transitions.retain(|ref transition| transition.is_ongoing());

                        // A layout refresh is necessary for the changes to apply
                        LayoutTree::refresh(&mut wm_state);
                    }
                }
            }

            sleep(time::Duration::from_millis(delta));
        }
    });
}


pub fn schedule_tcp_routine(){
    spawn(||{
        if let Some(ref tcp_socket) = *TCP_SOCKET{
            if let Ok(ref socket_instance) = tcp_socket.lock(){
                for stream in socket_instance.incoming() {
                    match stream {
                        Ok(mut stream) => {
                            handle_client(stream);
                        }
                        Err(e) => { /* connection failed */ }
                    }
                }
            }
        }
    });
}