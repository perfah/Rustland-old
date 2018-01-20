use io::process_all_current_jobs;
use common::job::JobType;
use wmstate::{WM_STATE, PENDING_JOBS, FINALIZED_JOBS};
use common::definitions::{SOCKET_PORT, SOCKET_DETERMINANT};
use serde_json;

use std::io::{Write, BufReader, BufWriter, BufRead};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::str;

lazy_static! {
    pub static ref TCP_SOCKET: Option<Mutex<TcpListener>> = 
        match TcpListener::bind(format!("localhost:{}", SOCKET_PORT)) {
            Ok(working_listener) => Some(Mutex::new(working_listener)),
            Err(e) => None
        };
}


pub fn handle_client(mut stream: TcpStream) {
    println!("Client {} connected. Now ready accept jobs it.", stream.peer_addr().unwrap());

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    loop{
        let mut input_buffer = Vec::<u8>::new();
        reader.read_until(SOCKET_DETERMINANT, &mut input_buffer);
        if input_buffer.pop() == None { break; }

        if let Ok(mut pending_jobs) = PENDING_JOBS.lock(){
            match str::from_utf8(input_buffer.as_slice()){
                Ok(v) => {
                    if let Ok(job) = serde_json::from_str(v){
                        pending_jobs.push(job);
                    }
                },
                Err(e) => { panic!("ERROR = {}", e) }
            }
        }

        process_all_current_jobs();

        while let Some(job) = FINALIZED_JOBS.try_lock().unwrap().pop(){
            writer.write_all(serde_json::to_string(&job).unwrap().as_bytes());
            writer.write(&[SOCKET_DETERMINANT]);
        }
        writer.flush();
    }
}
