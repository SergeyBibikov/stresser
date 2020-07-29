extern crate serde_json;
extern crate serde;

use std::io::prelude::*;
use native_tls::TlsConnector;
use std::net::TcpStream;
use std::string::String;


pub fn get_req(path: &String, domain: &String, port: &String){
    let mut connection = TcpStream::connect(format!("{}:{}",&domain,port)).unwrap();
    let temp = create_get_req(&path, &domain);
    let request = temp.as_bytes();
    for _ in 0..100{
        connection.write(request).unwrap();
    }   
}

pub fn tls_get_req(path: String, domain: String, port: String){
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