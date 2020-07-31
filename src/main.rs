extern crate serde_json;
extern crate serde;
mod posts;
mod gets;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io;
use std::{fs,str};
use std::io::prelude::*;
//use native_tls::TlsConnector;
//use std::net::TcpStream;
use std::string::String;
use posts::*;
use gets::*;
//use std::time::SystemTime;

//use std::string::String;
/*  Debug block!
    let mut buf:[u8;10000] = [0u8; 10000];
    connection.read(&mut buf).unwrap();
    let strd = std::str::from_utf8(&buf).unwrap();
    println!("{}",String::from(strd));*/

    /*TODO
    1. tls_get_request - потестить
    2. */

#[derive(Serialize, Deserialize)]
struct Request {
    path: String,
    domain: String,
    port: String,
    request_num: String,
    request_type: String,
    path_to_body: String,
}

fn main(){
    let req_d = init().unwrap();    
    let mut request_threads = vec![];

    for _ in 0..2{
        let path = req_d.path.clone();
        let domain = req_d.domain.clone();
        let port = req_d.port.clone();
        //let body = init_data[5].clone();
        request_threads.push(std::thread::spawn(move ||{
            get_req(&path,&domain,&port);
                
        }));
    }
    for j in request_threads{j.join().unwrap();} 
    //print!("Done");
    //C:\Users\Sergey\VSW\r_ust\jsonparse\src\req.json

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
    println!("\n Start sending {} {} requests to the path /{} of {}:{}.\n Body path: {}. Press Ctrl+C to exit",
             req.request_num, req.request_type, req.path, req.domain, req.port, req.path_to_body.trim());

    Ok(req)
}