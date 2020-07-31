extern crate serde_json;
extern crate serde;
mod posts;
mod gets;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io;
use std::{fs,str};
use std::io::prelude::*;
use std::string::String;
use posts::*;
use gets::*;

//use std::string::String;
/*  Debug block!
    let mut response: Vec<u8> = vec![];
    tls_stream.read_to_end(&mut response).unwrap();
    print!("\n{}",String::from_utf8(response).unwrap());*/

    /*TODO
    1. добавить ветви для выбора get vs post
    2. добавить ветви для выбора http vs https*/

#[derive(Serialize, Deserialize)]
struct Request {
    protocol: String,
    path: String,
    domain: String,
    port: String,
    request_type: String,
    request_num: String,
    headers: String,    
    path_to_body: String,
}

fn main(){
    let req_d = init().unwrap();    
    let mut request_threads = vec![];
    let temp_req_body = fs::read(req_d.path_to_body).unwrap();
    let req_body = String::from_utf8(temp_req_body).unwrap();
    for _ in 0..1{
        let path = req_d.path.clone();
        let domain = req_d.domain.clone();
        let port = req_d.port.clone();
        let headers = req_d.headers.clone();
        let body = req_body.clone();
        request_threads.push(std::thread::spawn(move ||{
            //get_req(&path,&domain,&port,&headers);
            tls_post_req(&path,&domain,&port,&body,&headers);                
        }));
    }
    //for j in request_threads{j.join().unwrap();} 
    //print!("Done");

}


fn init() -> Result<Request>{
    let start_message = r#" Hi! You need to provide two paths to start your test: the locations of you config file and the request body file.
 Both paths should be absolute and should not be put between quotation marks.
 Cheers!"#;
    print!("{}",start_message);
    //Paths init
    let mut req_data_path: String = String::new(); 

    print!("\n Please enter the config file location ");
    io::stdout().flush().unwrap();  
    io::stdin().read_line(&mut req_data_path).expect("No access to standard io");
    
    //Request data serialization
    let temp = fs::read(req_data_path.as_str().trim()).unwrap();
    let data_to_serialize: &str = str::from_utf8(&temp).unwrap();
    let req: Request = serde_json::from_str(data_to_serialize)?;
    println!("\n Start sending {} {} requests to the path /{} of {}:{} using {}.\n Body path: {}. Press Ctrl+C to exit",
             req.request_num, req.request_type, req.path, req.domain, req.port,req.protocol, req.path_to_body.trim());

    Ok(req)
}