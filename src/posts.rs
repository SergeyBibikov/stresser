use std::io::prelude::*;
use native_tls::TlsConnector;
use std::net::TcpStream;
use std::string::String;


pub fn post_req(path: &String, domain: &String, port: &String, body: &String, headers: &String){
    let mut connection = TcpStream::connect(format!("{}:{}",domain,port)).unwrap();
    let temp = create_post_request(path, domain, body, headers);
    let request = temp.as_bytes();    
    connection.write(request).unwrap();
}

pub fn tls_post_req(path: &String, domain: &String, port: &String, body: &String, headers: &String){
    let connector = TlsConnector::new().unwrap();
    let tcp_stream = TcpStream::connect(format!("{}:{}",domain,port)).unwrap();
    let mut tls_stream = connector.connect(domain, tcp_stream).unwrap();
    let temp = create_post_request(path,domain,body,headers);
    let request = temp.as_bytes();
    tls_stream.write(request).unwrap();
}

fn create_post_request(path: &String, domain: &String, body: &String, headers: &String) -> String{
    let mut request: String = "POST /".to_string();
    request.push_str(path);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: ");
    request.push_str(domain);
    request.push_str("\r\n");
    for h in headers.lines(){
        request.push_str(h);
        request.push_str("\r\n");
    }
    request.push_str("\r\n\r\n");
    request.push_str(body);
    request
}