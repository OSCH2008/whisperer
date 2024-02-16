mod kem;
mod tcp;

use tcp::{
    vector as vect,
    StreamReader
};
use std::{
    net::TcpListener,
    io::Write,
    thread,
    sync::{Arc, Mutex}
};
use screen_info::DisplayInfo;
use eframe::egui;

const KEY_SIZE: usize = 16;
const TEST_IP: &'static str = "192.168.40.126:9998";

fn main() {
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    handles.push(thread::spawn(request_handler_thread));
    
    handles.push(thread::spawn(|| {
        tcp::check_availability(TEST_IP).unwrap();
        
        let public_key = vect::rand_byte_vector(KEY_SIZE);
        let recv_key = tcp::send_public_key(TEST_IP, public_key.clone()).unwrap();
        
        let base_key = vect::rand_byte_vector(KEY_SIZE);
        let private_key = vect::and_vector(base_key.clone(), recv_key);
        
        let combined_key = vect::and_vector(base_key, public_key);
        tcp::send_mixed_key(TEST_IP, combined_key).unwrap();

        let message = "you will be forever alone";
        tcp::encrypted_send(TEST_IP, message, private_key).unwrap();
    }));
    

    for handle in handles {
        handle.join().unwrap();
    }
}

fn request_handler_thread() {
    let port = TcpListener::bind("0.0.0.0:9998").unwrap();

    let base_key = Arc::new(vect::rand_byte_vector(KEY_SIZE));
    let private_key: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));

    for req in port.incoming() {
        let base_key = Arc::clone(&base_key);
        let private_key = Arc::clone(&private_key);

        thread::spawn(move || {
            let mut stream = req.unwrap();

            stream.parse_incoming(|stream, protocol, data| match protocol {
                tcp::Protocol::PublicKey => {
                    let combined_key = vect::and_vector(base_key.to_vec(), data);
                    stream.write_all(&[combined_key.as_slice(), &[255u8]].concat()).unwrap();
                },
                tcp::Protocol::CombineKey => {
                    let mut mutex = private_key.lock().unwrap();
                    *mutex = vect::and_vector(base_key.to_vec(), data);
                    drop(mutex);

                    stream.write_all(&[0u8]).unwrap();
                },
                tcp::Protocol::Message => {
                    let key = {
                        let mutex = private_key.lock().unwrap();
                        mutex.clone()
                    };

                    let author = stream.peer_addr().unwrap().to_string();

                    let message = kem::decrypt(data, key);
                    let message = vect::bytes_to_string(message);
                    stream.write_all(&[0u8]).unwrap();

                    println!("{author}: {message}");
                },
                tcp::Protocol::Unknown => stream.write_all(&[0u8]).unwrap()
            });
        });
    }
}

#[inline]
fn calculate_center_screen(x: u32, y: u32) -> (f32, f32) {
    let display = DisplayInfo::all().unwrap();
    let mut res: Option<(u32, u32)> = None;
    for info in display {
        match info.is_primary {
            true => {
                res = Some((info.width, info.height));
                break;
            },
            false => ()
        }
    }
    let size = res.expect("No primary monitor");
    (
        {size.0 / 2 - {x / 2}} as f32,
        {size.1 / 2 - {y / 2}} as f32
    )
}