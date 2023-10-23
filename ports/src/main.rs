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
