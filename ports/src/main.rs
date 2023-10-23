extern crate chrono;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::result::Result;
use std::str;

use chrono::prelude::*;

const PUT_HTML_HERE: &str = "/var/www/";

const TEXT_404: &str = "
<html>
<head><title>404 Not Found</title></head>
<body bgcolor='white'>
<center><h1>404 Not Found</h1></center>
</body>
</html>";

const TEXT_500: &str = "
<html>
<head><title>500 Internal Server Error</title></head>
<body bgcolor='white'>
<center><h1>500 Internal Server Error</h1></center>
</body>
</html>";

fn create_filename(s: String) -> String {
    let part: String = s[1..].to_string();

    if part.find('.').is_none() {
        part + ".html"
    } else {
        part
    }
}

fn load_file(filename: String) -> Result<File, String> {
    // you could pass in '../../rootfile' in here
    match File::open(format!("{}{}", PUT_HTML_HERE, filename)) {
        Err(why) => return Err(format!("no {}", why).to_string()),
        Ok(file) => return Ok(file),
    }
}

fn read_stream<'a>(stream: &'a mut TcpStream) -> String {
    let mut buf = String::new();
    let mut b = [0; 1];
    while stream.read(&mut b).is_ok() {
        buf += str::from_utf8(&b).unwrap();
        if buf.ends_with("\r\n\r\n") {
            break;
        }
    }
    println!("handle this: {:?}", buf);
    let mut parts = buf.split(' ');
    let _method = parts.next(); // http method: GET / POST / PUT
    let url = parts.next().unwrap(); // PATH
    return url.to_string();
}
