extern crate serde_json;
extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io;
use std::{fs,str};
use std::io::prelude::*;
use native_tls::TlsConnector;
use std::net::TcpStream;
//use std::io::stdout;
use std::string::String;
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
   
   let counter: usize = init_data[3].as_str().parse::<usize>().unwrap()/100usize;
    for _ in 0..counter{
        let path = init_data[0].clone();
        let domain = init_data[1].clone();
        let port = init_data[2].clone();
        request_threads.push(std::thread::spawn(move ||{
            get_req(&path,&domain,&port);
        }));
    }
    for j in request_threads{
            j.join().unwrap();
        }
    print!("Done");

}


fn get_req(path: &String, domain: &String, port: &String){
    let mut connection = TcpStream::connect(format!("{}:{}",&domain,port)).unwrap();
    let temp = create_get_req(&path, &domain);
    let request = temp.as_bytes();
    for _ in 0..100{
        connection.write(request).unwrap();
    }   
}

fn tls_get_req(path: String, domain: String, port: String){
    let connector = TlsConnector::new().unwrap();
    let tcp_stream = TcpStream::connect(format!("{}:{}",&domain,&port)).unwrap();
    let mut tls_stream = connector.connect(&domain, tcp_stream).unwrap();

    let temp = create_get_req(&path, &domain);
    let request = temp.as_bytes();
    tls_stream.write(request).unwrap();
}

fn create_get_req(path: &String, domain: &String)-> String {
    let mut request = String::from("GET /");
    request.push_str(path);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: ");
    request.push_str(domain);
    request.push_str("\r\n");
    request.push_str("Connection: keep-alive");
    request.push_str("\r\n\r\n");
    request
}

fn create_post_request(path: &String, domain: &String, body: &String) -> String{
    let mut request: String = "POST /".to_string();
    request.push_str(path);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: ");
    request.push_str(domain);
    request.push_str("\r\n");
    request.push_str("Content-Type: application/json\r\n");
    request.push_str("Connection: keep-alive");
    request.push_str("\r\n\r\n");
    request.push_str(body);
    request
}

fn post_req(path: String, domain: String, port: String, body: String){
    let temp = create_post_request(&path, &domain, &body);
    let request = temp.as_bytes();
    let mut connection = TcpStream::connect(format!("{}:{}",&domain,port)).unwrap();
    for _ in 0..100{
        connection.write(request).unwrap();
    }    
}

fn tls_post_req(path: String, domain: String, port: String, body: String){
    let connector = TlsConnector::new().unwrap();
    let tcp_stream = TcpStream::connect(format!("{}:{}",&domain,port)).unwrap();
    let mut tls_stream = connector.connect(&domain, tcp_stream).unwrap();
    let temp = create_post_request(&path,&domain,&body);
    let request = temp.as_bytes();
    tls_stream.write(request).unwrap();
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