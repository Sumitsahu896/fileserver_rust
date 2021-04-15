// server to listen for commands from client threads
use std::env;
mod search;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::ops::{Bound, RangeBounds};

use std::io::{Read, Write, Error};

// listen for messages from a client
fn connection_thread(mut stream: TcpStream) -> Result<(), Error> {
	println!("Received a connection from: {}", 
		stream.peer_addr()?);	// network address of client unwrap to OK if
	let mut buffer = [0; 1024]; 	// zero buffer
	
	loop {
		let bytes_read = stream.read(&mut buffer)?; // unwrap to Ok if read from stream successful
		if bytes_read == 0 {	// no more to read
			return Ok(());
			
		}
		
		let cmd_line = String::from_utf8_lossy(&buffer[0..bytes_read]);
		//println!("cmd_line from buffer: {}", cmd_line);
		
		// split the command line into tokens: tokens[0] .. tokens[params-1]
		
		let tokens: Vec<&str> = cmd_line.trim().split_whitespace().collect();
    		let mut args: Vec<String> = cmd_line.trim().split_whitespace().map(str::to_string).collect();
    		
		match tokens[0] {
			"search" => {
				println!("SEARCH HERE !!!");
				//assert_eq!(tokens.len(), 4);
				let mut path = PathBuf::new();
    				path.push("./users_client/");
    				path.push("diane/");
    				let file_name;
        			let search_text;
        			if args[1] == "-f" && args.iter().any(|i| i=="-s"){
        				println!("f and s found\n");
            				if args.len() < 5 {
                				println!("Please insert the correct format and try again! The format is “search -f [FILE NAME] -s [TEXT]” or “search -f [FILE NAME] -s [TEXT]”");
                				//break;
            				}
            				file_name = &args[2];
            				path.push(file_name);
					
            				let search_text_args: Vec<_> = args.drain(4..).collect();
            			
            				search_text = search_text_args.join(" ");
            				
            				
            			// fix this error handling of file open	
            			//	let file = fs::File::open(path);
            			 	let mut file = fs::File::open(path).expect("Can't open file!");
            				
            				let mut contents = String::new();

             				
            			/*	let mut file=match file {
                				Ok(file) => file,
                				Err(error) => {
                    					eprintln!("Problem opening the file: {}",error);
                    					break
                				}
             				};
				*/
            				file.read_to_string(&mut contents).map_err(
                				|err| println!("{:?}", err)
            
            					).ok();
            				
            				
            				let mut response = search::search_f(&contents, &search_text);
            			
            				response.push('\n');
            				println!("Response from search_f: {}\n", response);
            				
            				write!(stream, "{}",&response).unwrap();
            	
            				
            				//response = contents; // this sends
            			
					//write!(stream, "{}", &response).unwrap(); // this sends
					
					//stream.write(&buffer[..bytes_read])?;
            				//break;
             	
            			}
        			
			},
			
			_ => 
			{
				
				// echo data back for now
				stream.write(&buffer[..bytes_read])?;
				
			},	
		} // end match on cmd
		
		// echo data back for now
//		stream.write(&buffer[..bytes_read])?;
			
	} // end loop on commands
		
	
}

fn main() {

	let listener = TcpListener::bind("127.0.0.1:2000") // client to connect to this port : 2000
				.expect("Unable to bind"); // return listener or panic
	let mut authenticated_user = false;
	// incoming is iterator on connected streams
	// loop on incoming client connections
	for stream in listener.incoming() {
		match stream {
			Err(e) => {	eprintln!("failed: {}", e) }
			Ok(stream) => {
				thread::spawn(move || { //spawn thread on connection
					connection_thread(stream)
					.unwrap_or_else(|error| eprintln!("{:?}", error));
				});
			}
		}
	}
}


trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}


