// server to listen for commands from client threads
use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::{Read, Write, Error};

// listen for messages from a client
fn connection_thread(mut stream: TcpStream) -> Result<(), Error> {
	println!("Received a connection from: {}", 
		stream.peer_addr()?);	// network address of client unwrap to OK if
	let mut buffer = [0; 256]; 	// zero buffer
	loop {
		let bytes_read = stream.read(&mut buffer)?; // unwrap to Ok if read from stream successful
		if bytes_read == 0 {	// no more to read
			return Ok(());
		}
		
		let cmd_line = String::from_utf8_lossy(&buffer[..]);
		println!("cmd_line from buffer: {}", cmd_line);
		
		// split the command line into tokens: tokens[0] .. tokens[params-1]
		
		let tokens: Vec<&str> = cmd_line.trim().split_whitespace().collect();
    		
    		
		match tokens[0] {
			"search" => {
				println!("SEARCH HERE !!!");
				//assert_eq!(tokens.len(), 4);
			},
			
			_ => 
			{
				println!("catch all for now")
				
			},	
		} // end match on cmd
		
		// echo data back for now
		stream.write(&buffer[..bytes_read])?;
			
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
