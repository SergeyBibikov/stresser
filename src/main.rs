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
        request_threads.push(std::thread::spawn(move ||{
            get_req(path,domain,port);
        }));
    }
    for j in request_threads{
        j.join().unwrap();
    }
    print!("Done");

}


fn get_req(path: String, domain: String, port: String){
    let domain_copy = domain.clone();
    let temp = create_get_req(path, domain);
    let request = temp.as_bytes();
    let mut connection = TcpStream::connect(format!("{}:{}",domain_copy,port)).unwrap();
    for _ in 0..100{
        connection.write(request).unwrap();
    }
    //C:\Users\Sergey\VSW\r_ust\jsonparse\src\req.json    
}



fn tls_get_req(path: String, domain: String){

    let connector = TlsConnector::new().unwrap();
    let tcp_stream = TcpStream::connect("bash.im:443").unwrap();
    let mut tls_stream = connector.connect("bash.im", tcp_stream).unwrap();

    let mut result:Vec<u8> = Vec::new();
    let request = create_get_req(path,domain);
    tls_stream.write(request.as_bytes()).unwrap();
    //tls_stream.write(b"GET / HTTP/1.1\r\nHost: bash.im\r\n\r\n").unwrap();
    tls_stream.write(b"GET / HTTP/1.0\r\nConnection: close\r\n\r\n").unwrap();
    tls_stream.read_to_end(&mut result).unwrap();

    println!("{}",std::string::String::from_utf8(result).unwrap());
}

fn create_get_req(path: String, domain: String)-> String {
    let mut request: Vec<u8> = Vec::new();
    let mut get = Vec::from("GET /");
    let mut p_ath = Vec::from(path);
    let mut http = Vec::from(" HTTP/1.1\r\n");
    let mut host = Vec::from("Host: ");
    let mut d_omain = Vec::from(domain);
    let mut ending = Vec::from("\r\n\r\n");


    request.append(&mut get);
    request.append(&mut p_ath);
    request.append(&mut http);
    request.append(&mut host);
    request.append(&mut d_omain);
    request.append(&mut ending);
    String::from_utf8(request).unwrap()
    //println!("{:?}",request);
    //print!("{}",String::from_utf8(request).unwrap());
}

fn create_post_request(path: String, domain: String, body:String)->String{
    let mut request: Vec<u8> = Vec::new();
    let mut get = Vec::from("POST /");
    let mut p_ath = Vec::from(path);
    let mut http = Vec::from(" HTTP/1.1\r\n");
    let mut host = Vec::from("Host: ");
    let mut d_omain = Vec::from(domain);
    let mut line_end = Vec::from("\r\n");
    let mut content = Vec::from("Content-type: application/json");
    let mut ending = Vec::from("\r\n\r\n");
    let mut b_ody = Vec::from(body);

    request.append(&mut get);
    request.append(&mut p_ath);
    request.append(&mut http);
    request.append(&mut host);
    request.append(&mut d_omain);
    request.append(&mut line_end);
    request.append(&mut content);
    request.append(&mut ending);
    request.append(&mut b_ody);
    String::from_utf8(request).unwrap()
}

fn post_req(path: String, domain: String, port: String, body: String){
    let inner_dom = domain.clone();
    let temp = create_post_request(path, domain, body);
    let request = temp.as_bytes();   
    let mut connection = TcpStream::connect(format!("{}:{}",inner_dom,port)).unwrap();
    connection.write(request).unwrap();
    
    let mut result: Vec<u8> = Vec::new();
    connection.read_to_end(&mut result).unwrap();    // DEBUG!
    print!("{}", String::from_utf8(result).unwrap());
    
}

fn tls_post_req(){
    let connector = TlsConnector::new().unwrap();
    let tcp_stream = TcpStream::connect("reqres.in:443").unwrap();
    let mut tls_stream = connector.connect("reqres.in", tcp_stream).unwrap();
    let request = "POST /api/user HTTP/1.1\r\nHost: reqres.in\r\nContent-type: application/json\r\n\r\n{\"name\":\"Jack\",\"job\":\"helper\"}".as_bytes();
    tls_stream.write(request).unwrap();

    let mut result:Vec<u8> = Vec::new();
    tls_stream.read_to_end(&mut result).unwrap();
    println!("{}",std::string::String::from_utf8(result).unwrap());

}

fn read_from_console(){
    print!("Please set path something! ");
    io::stdout().flush().unwrap();

    let mut path = String::new();    
    std::io::stdin().read_line(&mut path).expect("error: unable to read user input");
    
    //String::from(path.trim())
    //print!("You entered \"{}\"",input.trim());
    
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