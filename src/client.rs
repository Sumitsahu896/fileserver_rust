use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};
use std::ops::{Bound, RangeBounds};

fn main() {
	println!("\nWelcome to fileserver in Rust!\n");
	
	let mut authenticated_user=false;
	match TcpStream::connect("localhost:2000") {
	
	Ok(mut stream) => {
    			println!("Connected to the server!");
	
			
	loop {
		println!("\n:: Registered users: login username \n:: Guests: connect guest\n:: quit\n");
		let mut input = String::new();	// string for user input
		//let mut buffer: Vec<u8> = Vec::new(); 	// u8 vector for server responses
		match io::stdin().read_line(&mut input) {	// read line of input from user
			Ok(_) => {
				println!(">> {}",input);
			},
			Err(e) => println!("Error reading input; Please try again \n{:?}",e)
		
		}
		if input.trim() == "connect guest" {
			println!(":: Guest Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
			loop {
			
			
			
				let mut cmd_guest = String::new();	// string for user input
				
				//io::stdin().read_line(&mut cmd_guest)	// read line of input from user
				//	.expect("Failed to read from stdin");
				
				match io::stdin().read_line(&mut cmd_guest) {	// read line of input from user
					Ok(_) => {
						//println!("You entered: {}",input);
                                    println!("You entered: {}",cmd_guest);
					},
					Err(e) => println!("Error reading input; Please try again \n{:?}",e)
		
				}
				
				
				let cmd = cmd_guest.trim();
				
				//println!("guest cmd: {}", cmd);
				
				// guest command options
				
				let tokens: Vec<&str> = cmd_guest.trim().split_whitespace().collect();
    		
    	
    				match tokens.len() {
    				3 => {
    					match tokens[0] {
					"create" => {
						println!("user account created");
						authenticated_user = true
					},
					_ => {
						println!("invalid guest command\n");
						println!(":: Commands: \n\t -- create user username\n");
					}
					}
    				
    				},
    				2 => {
    					match cmd {
					"show users" => {
						println!("match: show users")
				
					},
					"show active" => {
						println!("match: show active")
				
					},
					_ => {
						println!("invalid guest command\n");
						println!(":: Guest Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
					}
					}
    				
    				},
    				1 => {
    					match tokens[0] {
					"logout" => {
						authenticated_user = false;
						break
					},
					_ => {
						println!("invalid guest command\n");
						println!(":: Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
					}
					}
    				
    				},
    				_ => {
    					println!("invalid guest command\n");
					println!(":: Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
					continue
    				
    				}
    				}
    			
			} // end loop for guest commands
			
		 } else if input.substring(0,5) == "login" {
		
			// ... code for authentication
			
			let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    		
    	
    			if tokens.len() != 2 {
    				println!("::login username\n");
    				
    			} else {
    				println!(":: authenticating .....\n");
				//let username = tokens[1];
                        let _username = tokens[1];
				authenticated_user=true;
				break

			}
		} else if input.substring(0,4) == "quit" {
			break;
		} else { // no match on command
			
			println!("\nInvalid command {:?}. Please enter:\n\tconnect guest ||  login username\n", input);
		}

	} // end login or connect loop
	
	/*
	 * loop to process authenticated user commands:
	 * 	write, search, send, receive, list files
	 */
	 if authenticated_user {
	 	loop {
			let mut cmd_user = String::new();	// string for user input
			let mut buffer: Vec<u8> = Vec::new(); 	// u8 vector for server responses
			println!("\n::Commands: \n -- search [-f filename] -s text\n -- write\n -- send\n -- receive\n -- list files\n -- logout\n");
			match io::stdin().read_line(&mut cmd_user) {
    				Ok(_n) => println!("{}", cmd_user),
    				
    				Err(error) => {
    					println!("Unable to read from stdin: {}", error);
    					continue;
    				}
			}
			let tokens: Vec<&str> = cmd_user.trim().split_whitespace().collect();
			let cmd = cmd_user
    				.split_whitespace()
    				.next()
    				.unwrap_or("");
			println!("cmd: {}", cmd);
		
			match cmd {
			"search" => {
				
				//match stream.write(cmd_user.as_bytes()) {
					//Ok(_n) => println!("sent command to server"),
					//Err(e) => {
					//	println!("Unable to send command to server: {}, e);
					//	continue;
					//}
				//}
				//stream.write(cmd_user.as_bytes()) 
					//.expect("Failed to write to server");



                        match stream.write(cmd_user.as_bytes()) {
                              Ok(_) => (),
                              Err(err) => {
                                    println!("Unable to send command to server: {}", err);
                                    break;
                              }
                                       
                              }
	
			
				let mut reader = BufReader::new(&stream);
		
				// reader.read_until(b'\n', &mut buffer)	// read_until reads the data in buffer
				// 	.expect("Could not read into buffer");
				// print!("echo: {}", str::from_utf8(&buffer)	// write buffer converted 
				// 	.expect("Could not write buffer as string"))
                        match reader.read_until(b'\n', &mut buffer) {
                              Ok(_) => (),
                              Err(err) => {
                                    println!("Unable to read into buffer: {}", err);
                                    break;
                              }
                                 
                             }

                        let buffer=match str::from_utf8(&buffer){
                              Ok(buffer) => buffer,
                              Err(err) => {
                                    println!("Could not write buffer as string: {}", err);
                                    break;
                              }
                                 
                             };

                  

                        print!("found in search: {}",buffer);                      
				
			},
			"write" => {
				println!("match: write")
				
			},
			"send" => {
				println!("match: send")
				
			},
			"receive" => {
				println!("match: receive")
				
			},
			"list" => 
			{
				println!("match: list")
				
			},
			"logout" => 
			{
				println!("match: logout");
				break;
				
			},
			_ => 
			{
				println!("invalid command::");
				println!("\n::Commands: \n::search \n:: write\n:: send\n:: receive\n:: list files\n:: logout\n")
			},	
			} // end match on cmd

	  	} // end command loop
	  } // end authenticated_user	
	  
	}, 
	Err(e) => {
    		println!("Couldn't connect to server...\nPlease make sure server is up and receiving connections.");
	}
} // end tcp connected
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