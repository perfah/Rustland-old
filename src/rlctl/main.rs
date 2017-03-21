use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{Write, BufReader, BufWriter, BufRead};
use std::iter::Peekable;
use std::str;

#[macro_use]
pub extern crate serde_derive;
#[macro_use]
pub extern crate serde_json;

extern crate common;
use common::job::{Job, JobType};
use common::definitions::{SOCKET_DETERMINANT, TAG_PREFIX};

fn parse_job_type(repr: String) -> Option<JobType>{
    match repr.as_ref()
    {
        "focus" => Some(JobType::FOCUS),
        "runapp" => Some(JobType::RUN_APP),
        "tree" => Some(JobType::SEND_TREE),
        "moveto" => Some(JobType::MOVE_TO),
        _ => { 
            println!("Unknown command.");  
            return None;
        }
    }
} 

fn generate_job_from_args(mut args: Peekable<env::Args>) -> Option<Job>{
    let mut job = Job::default();

    args.next();

    if let Some(head_tag_or_cmd) = args.next(){
        if head_tag_or_cmd.contains(TAG_PREFIX){
            job.head_tag = Some(head_tag_or_cmd.trim().replace(TAG_PREFIX, ""));
        }
        else{
            match parse_job_type(head_tag_or_cmd){
                Some(job_type) => job.job_type = job_type,
                None => return None
            }
        }
    }
    
    if job.head_tag.is_some(){
        if let Some(command) = args.next(){
            println!("{}", command);

            match parse_job_type(command){
                Some(job_type) => job.job_type = job_type,
                None => return None
            }
        }
        else{
            println!("No arguments given.");
            return None;
        }
    }

    while let Some(arg) = args.next(){
        job.contextual_tags.push(arg);
    }

    return Some(job);
}

fn show_usage(){
    println!("Usage: rlctl [PRIORITY_TAG] COMMAND [TAGS/ARGS]");
    println!();
    println!("Commands:");
    println!(r#"    - tree: Sends back a list of elements in the window layout in a tree like format.
    - runapp: Executes an application to start in the focused position of the layout.
    - moveto: Moves an element in the layout to another place. 
    "#);
}

fn main(){
    for arg in env::args() {
        if arg.contains("--help")
        {
            show_usage();
            return;
        }
    }

    if let Ok(mut stream) = TcpStream::connect("localhost:4451"){
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        if let Some(job) = generate_job_from_args(env::args().peekable()){
            writer.write_all(serde_json::to_string(&job).unwrap().as_bytes());
            writer.write(&[SOCKET_DETERMINANT]);
            
            if let Err(e) = writer.flush(){
                println!("Error occured while writing to server: {}", e)
            }
        
            let mut input_buffer = Vec::<u8>::new();
            reader.read_until(SOCKET_DETERMINANT, &mut input_buffer);
            if input_buffer.pop() == None { return; }
            
            match str::from_utf8(input_buffer.as_slice()){
                Ok(v) => {
                    if let Ok(job) = serde_json::from_str::<Job>(v) {
                        match job.generated_result
                        {
                            Ok(result) => println!("{}", result),
                            Err(e) => println!("{}", e)
                        }
                    }
                },
                _ => {}
            }
        }
    }
    else{
        println!("Could not connect to Rustland compositor/server. Are you sure it's running?");
    }
}

