extern crate serde_json;
extern crate serde;
mod posts;
mod gets;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io;
use std::{fs,str};
use std::io::prelude::*;
use native_tls::TlsConnector;
use std::net::TcpStream;
use std::string::String;
use posts::*;
use gets::*;

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
}

fn main(){
    let init_data = init().unwrap();   
    let mut request_threads = vec![];
   
   let counter: usize = init_data[3].as_str().parse::<usize>().unwrap();
    for _ in 0..counter{
        let path = init_data[0].clone();
        let domain = init_data[1].clone();
        let port = init_data[2].clone();
        let body = init_data[5].clone();
        request_threads.push(std::thread::spawn(move ||{
            post_req(&path,&domain,&port,&body);
        }));
    }
    for j in request_threads{
            j.join().unwrap();
        }
    print!("Done");

}


fn init() -> Result<[String;6]>{
    let start_message = r#" Hi! You need to provide two paths to start your test: the locations of you config file and the request body file.
 Both paths should be absolute and should not be put between quotation marks.
 Cheers!"#;
    print!("{}",start_message);
    //Paths init
    let mut req_data_path: String = String::new(); 
    let mut req_body_path: String = String::new(); 

    print!("\n Please enter the config file location ");
    io::stdout().flush().unwrap();  
    io::stdin().read_line(&mut req_data_path).expect("No access to standard io");
    
    print!(" Please enter the request body location ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut req_body_path).expect("No access to standard io");

    //Request data serialization
    let temp = fs::read(req_data_path.as_str().trim()).unwrap();
    let data_to_serialize: &str = str::from_utf8(&temp).unwrap();
    //let d = str::from_utf8(&data_to_serialize).unwrap();
    let p: Request = serde_json::from_str(data_to_serialize)?;
    println!("\n Start sending {} {} requests to the path /{} of {}:{}.\n Body path: {}. Press Ctrl+C to exit",
             p.request_num, p.request_type, p.path, p.domain, p.port, req_body_path.trim());
    let array_to_return: [String;6] =[p.path.clone(),
                                      p.domain.clone(),
                                      p.port.clone(),
                                      p.request_num.clone(),
                                      p.request_type.clone(),
                                      req_body_path
                                      ];

    Ok(array_to_return)
}