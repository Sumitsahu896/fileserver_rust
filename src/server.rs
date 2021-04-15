// server to listen for commands from client threads
use std::env;
mod search;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::ops::{Bound, RangeBounds};
use std::sync::{Arc, Mutex};

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
            let mut path = PathBuf::new();
            path.push("./users_server/");
            path.push("irfan/");
		match tokens[0] {
			"search" => {
				println!("SEARCH HERE !!!");
	
    				let file_name;
        			let search_text;
        			if args[1] == "-f" && args.iter().any(|i| i=="-s"){
        				println!("f and s found\n");
            				if args.len() < 5 {
                				println!("Please insert the correct format and try again! The format is “search -f [FILE NAME] -s [TEXT]” or “search -f [FILE NAME] -s [TEXT]”");
                				//break;
            				}else{

                                   
            				file_name = &args[2];
            			
                                    path.push(file_name);
                             
					      let search_text_args: Vec<_> = args.drain(4..).collect();
            			      search_text = search_text_args.join(" ");

                                    //let mut file=Arc::new(Mutex::new(fs::File::open(path))).lock().unwrap();
                           
            				
            				let  contents= write_file_to_string(&path);

                                    if contents=="Problem opening the file"{

                                       println!("{}",contents);
                                    }else{
                                         
                                          let mut response = search::search_f(&contents, &search_text);

            				       //response.push('\n');
            				       println!("Response from search_f: {}", response);
                                           response.push('\n');
					             //write!(stream, "{}", &response).unwrap();
                                           match write!(stream, "{}", &response){
                                                Ok(_) => (),
                                                Err(err) => {
                                                      println!("Unable to send command to server: {}", err);
                                                      return Err(err);
                                                }
                                                         
                                                }
                                         
                                    }

                                    // (|| {
            				// match file {
                                    //             Ok(file) => {
                                                    
                                                        
                                    //             file.read_to_string(&mut contents).map_err(
                                    //                         |err| println!("{:?}", err)
                                      
                                    //                               ).ok();
                                                              
                                    //             search::search_f(&contents, &search_text);
                                    //             return;            
                                    //              // Ok!
                                                      
                                    //               },
                                    //             Err(error)=> {
                                                      
                                    //                   eprintln!("Problem opening the file: {}",error);
                                    //                   return;
                                                      
                                    //                 }
                                    //             }
                                    //       })();
                                            
            				
            				//break;
                                    }
            			 }else if args[1] != "-f" && args[1]=="-s"{
                                    println!("search -s found\n");
                                    if args.len() < 3 {
                                          println!("Please insert the correct format and try again! The format is “search -f [FILE NAME] -s [TEXT]” or “search -f [FILE NAME] -s [TEXT]”");
                                         
                                    }
                                    else{
                                          let search_text_args: Vec<_> = args.drain(2..).collect();
                                          search_text = search_text_args.join(" ");
                
                                          //let mut path_string=String::from("./users_server/irfan/");
   


                                          //let r_directory=fs::read_dir(path2).unwrap();
                                          let r_directory=fs::read_dir(path);
                                          let r_directory=match r_directory {
                                           Err(e) =>{
                                                eprintln!("Path problem :{:?}", e);
                                                return Err(e)
                                                } 
                                            Ok(r_directory) => (r_directory)
                                           };
                                           let mut answer=String::from("");
                                           for entry_res in r_directory.filter_map(Result::ok){
      
                                                let entry = entry_res;
                                      
                                                //if entry.is_err() { continue; }
                                      
                                                // let entry=match entry{
                                                //     _ =>{
                                                //         eprintln!("error reading directory");
                                                //         break;
                                                //     } ,
                                                //     Ok(entry) => entry
                                                // };
                                      
                                                let this_file_name_buf = entry.file_name();
                                                let this_file_name = this_file_name_buf.to_str();
                                      
                                                let this_file_name=match this_file_name{
                                                    None =>{
                                                        eprintln!("error");
                                                        break
                                                    } ,
                                                    Some(this_file_name)=> this_file_name.to_string()
                                                };
                                               
                  
                                               let  contents= write_file_to_string(&entry.path());

                                               if contents=="Problem opening the file"{
      
                                                println!("{}",contents);
                                                }else{
                                                   
                                                      let mut response =  search::search_s(&contents, &search_text, &this_file_name);
                                                      //println!("Response from search_s: {}", response);
                                                      response.push('\n');
                                                      answer.push_str(response.as_str());
                                                      
                                               }
                                             
                                            
                                                
                                          


                                    }
                                    println!("Response from search_s: {}", answer);
                                    //write!(stream, "{}", &answer).unwrap();


                                    match write!(stream, "{}", &answer){
                                          Ok(_) => (),
                                          Err(err) => {
                                                println!("Unable to send command to server: {}", err);
                                                return Err(err);
                                          }
                                                   
                                          }
                                    
                              }
                        }
        			
			},
			
			_ => 
			{
				println!("catch all for now");
                        // echo data back for now
			


                        match stream.write(&buffer[..bytes_read]) {
                              Ok(_) => (),
                              Err(err) => {
                                    println!("Unable to send command to server: {}", err);
                                    return Err(err)
                              }
                                       
                              }
				
			},	
		} // end match on cmd
		
		// echo data back for now
		//stream.write(&buffer[..bytes_read])?;

            // match stream.write(&buffer[..bytes_read]) {
            //       Ok(_) => (),
            //       Err(err) => return Err(err)
            //   }
			
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







fn write_file_to_string(path1:&PathBuf)-> String
{
      let file = fs::File::open(&path1);
      let mut contents = String::new();

       let mut file=match file {
          Ok(file) => file,
          Err(error) => {
                    eprintln!("Problem opening the file: {}",error);
                  return "Problem opening the file".to_string();
             
          }
       };

       file.read_to_string(&mut contents).map_err(
            |err| println!("{:?}", err)

                  ).ok();
              
     
      contents.to_string()
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