use rand::Rng;
use std::char;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::io::{Error, Read};
use std::io::SeekFrom;
use std::net::TcpStream;
use std::path::PathBuf;
use std::env;
use std::ops::{Bound, RangeBounds};
use std::str;
extern crate chrono;
use chrono::Local;

fn encryption(encrypted_text: &mut Vec<char>, private_key: &mut u8, public_key: &mut u8) {
    let mut rng = rand::thread_rng();
    let modulo: u8 = 64;
    let date = Local::now().format("%d%m%Y%H%M%S").to_string();
    *private_key = rng.gen_range(1..modulo);
    *public_key = modulo - (*private_key);

    let encrypted_text_temp: Vec<char> = date.chars().collect();

    for c in &encrypted_text_temp {
        //println!("{}:{}", *c,((*c as u8+*public_key)% modulo) as char);
        encrypted_text.push(((*c as u8 + *public_key) % modulo) as char);
    }
}

fn encryptedTextt(encrypted_text: &mut Vec<char>, private_key: &u8, public_key: &u8) {
    let modulo: u8 = 64;
    let date = Local::now().format("%d%m%Y%H%M%S").to_string();
    let encrypted_text_temp: Vec<char> = date.chars().collect();
    for c in &encrypted_text_temp {
        //println!("{}:{}", *c,((*c as u8+*public_key)% modulo) as char);
        encrypted_text.push(((*c as u8 + *public_key) % modulo) as char);
    }
}

fn main() {
    let date = Local::now().format("%d-%m-%Y-%H-%M-%S").to_string();
    println!("{}", date);

    println!("\nWelcome to fileserver in Rust!\n");
    println!("\nPlease enter the fileserver's connection port (2000 or 7070): ");

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read input");
    let conn_port: u16 = match line.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("Must enter a valid u16 number, 0-65535");
            return;
        }
    };

    println!("conn_port::{}", conn_port);

    let mut authenticated_user = false;
    let mut guest_user = false;
    let mut username = String::new();
    //match TcpStream::connect("localhost:2000") {
    match TcpStream::connect(("127.0.0.1", conn_port)) {
        Ok(mut stream) => {
            println!("Connected to the server!");

            loop {
                if authenticated_user {
                    loop {
                        let mut cmd_user = String::new(); // string for user input
                        let mut buffer: Vec<u8> = Vec::new(); // u8 vector for server responses
                        println!("\n::Commands: \n -- search [-f filename] -s text\n -- write -f|-a|-n filename\n -- send\n -- receive\n -- list files\n -- logout\n");
                        match io::stdin().read_line(&mut cmd_user) {
                            Ok(_n) => println!("{}", cmd_user),

                            Err(error) => {
                                println!("Unable to read from stdin: {}", error);
                                continue;
                            }
                        }
                        let tokens: Vec<&str> = cmd_user.trim().split_whitespace().collect();
                        let cmd = cmd_user.split_whitespace().next().unwrap_or("");
                        println!("cmd: {}", cmd);

                        match cmd {
                            "search" => {
                                match stream.write(cmd_user.as_bytes()) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to send command to server: {}", err);
                                        break;
                                    }
                                }

                                let mut reader = BufReader::new(&stream);

                                //  match reader.read_until(b'\n', &mut buffer) {
                                match reader.read_until(b'\r', &mut buffer) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to read into buffer: {}", err);
                                        break;
                                    }
                                }

                                let buffer = match str::from_utf8(&buffer) {
                                    Ok(buffer) => buffer,
                                    Err(err) => {
                                        println!("Could not write buffer as string: {}", err);
                                        break;
                                    }
                                };

                                print!("found in search:\n{}", buffer);
                            }
                            "write" => {
                                //send cmd to server
                                match stream.write(cmd_user.as_bytes()) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to send command to server: {}", err);
                                        break;
                                    }
                                }

                                //read response from server
                                let mut reader = BufReader::new(&stream);

                                match reader.read_until(b'\n', &mut buffer) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to read into buffer: {}", err);
                                        break;
                                    }
                                }

                                let buffer = match str::from_utf8(&buffer) {
                                    Ok(buffer) => buffer,
                                    Err(err) => {
                                        println!("Could not write buffer as string: {}", err);
                                        break;
                                    }
                                };

                                println!("{}", buffer);

                                if buffer == "Problem finding file\n" {
                                } else {
                                    //send text to server
                                    let mut res = String::new();
                                    match io::stdin().read_line(&mut res) {
                                        Ok(n) => println!("From stdin: {}", res),
                                        Err(error) => {
                                            println!("Unable to read from stdin: {}", error);
                                            continue;
                                        }
                                    }

                                    println!("Sent to server: {}", res.trim());

                                    match stream.write(res.as_bytes()) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            println!("Unable to send text to server: {}", err);
                                            break;
                                        }
                                    }
                                }
                            }
                            "send" => {
                                let mut path = PathBuf::new();
                                let splitcmd: Vec<&str> = cmd_user.split(" ").collect();
                                path.push("./users_client");
                                path.push(&username);
                                path.push(&splitcmd[1].trim());

                                //verify if file exists
                                let contents = write_file_to_string(&path.display().to_string());
                                if contents != "Problem opening the file"{

                                    //file exists, send cmd
                                    match stream.write(cmd_user.as_bytes()) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            println!("Unable to send command to server: {}", err);
                                            break;
                                        }
                                    }

                                    let mut reader = BufReader::new(&stream);

                                    match reader.read_until(b'\n', &mut buffer) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            println!("Unable to read into buffer: {}", err);
                                            break;
                                        }
                                    }

                                    let buffer = match str::from_utf8(&buffer) {
                                        Ok(buffer) => buffer,
                                        Err(err) => {
                                            println!("Could not write buffer as string: {}", err);
                                            break;
                                        }
                                    };

                                    if buffer == "request file size\n"{

                                        let mut filebytes: Vec<u8> = Vec::new();

                                        match File::open(&path){
                                            Ok(mut f) => {
                                                match f.read_to_end(&mut filebytes){
                                                    Ok(_)=>(),
                                                    Err(err)=>{
                                                        println!("Unable to read file: {}", err);
                                                        break
                                                    }

                                                };
                                                ()
                                            },
                                            Err(err) => {
                                                println!("Unable to open into file: {}", err);
                                                break
                                            }
                                        };

                                        let mut file_size = filebytes.len().to_string();
                                        file_size.push_str("\n");

                                        match stream.write(&file_size.as_bytes()){
                                            Ok(_) => (),
                                            Err(err) => {
                                                println!("Unable to send size to server: {}", err);
                                                break;
                                            }
                                        }

                                        let mut buf: Vec<u8> = Vec::new();
                                        let mut read = BufReader::new(&stream);

                                        match read.read_until(b'\n', &mut buf) {
                                            Ok(_) => (),
                                            Err(err) => {
                                                println!("Unable to read into buffer: {}", err);
                                                break;
                                            }
                                        }
        
                                        let buf = match str::from_utf8(&buf) {
                                            Ok(buffer) => buffer,
                                            Err(err) => {
                                                println!("Could not write buffer as string: {}", err);
                                                break;
                                            }
                                        };

                                        if buf == "request file\n"{
                                            match stream.write(&filebytes){
                                                Ok(_) => (),
                                                Err(err) => {
                                                    println!("Unable to send bytes to server: {}", err);
                                                    break;
                                                }
                                            }
                                        }
                                        else{
                                            println!("Could not request file\n");
                                        }

                                    }
                                    else{
                                        println!("Could not request file size\n");
                                    }
                                }
                                else{
                                    println!("The file has not been created.");
                                }
                            }
                            "receive" => {
                                match stream.write(cmd_user.as_bytes()) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to send command to server: {}", err);
                                        break;
                                    }
                                }
                                let mut reader = BufReader::new(&stream);

                                match reader.read_until(b'\n', &mut buffer) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to read into buffer: {}", err);
                                        break;
                                    }
                                }

                                let buffer = match str::from_utf8(&buffer) {
                                    Ok(buffer) => buffer,
                                    Err(err) => {
                                        println!("Could not write buffer as string: {}", err);
                                        break;
                                    }
                                };

                                if buffer == "file found\n"{
                                    let mut path = PathBuf::new();
                                    let splitcmd: Vec<&str> = cmd_user.split(" ").collect();
                                    path.push("./users_client");
                                    path.push(&username);
                                    path.push(&splitcmd[1].trim());

                                    let mut file = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .open(&path)
                                    .unwrap();    

                                    match write!(&stream, "{}", &"request file size\n"){
                                        Ok(_) => {
                                              println!("requested file size");
                                              ()
                                        },
                                        Err(err) => {
                                              println!("Unable to send command to client: {}", err);
                                        }
                                    }

                                    let mut reader = BufReader::new(&stream);
                                    let mut file_size_buff = Vec::new();
            
                                    match reader.read_until(b'\n', &mut file_size_buff) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            println!("Unable to read into buffer: {}", err);
                                        }
                                    }
                                    file_size_buff.pop();
            
                                    //define file size
                                    let file_size = str::from_utf8(&file_size_buff).unwrap().parse::<usize>().unwrap();
            
                                    //request file bytes
                                    match write!(&stream, "{}", &"request file\n"){
                                          Ok(_) => {
                                                println!("requested file");
                                                ()
                                          },
                                          Err(err) => {
                                                println!("Unable to send command to client: {}", err);
                                          }
                                    }

                                    let mut file_bytes = vec![0u8; file_size];
                                    match reader.read_exact(&mut file_bytes) {
                                          Ok(_) => (),
                                          Err(err) => {
                                              println!("Unable to read into buffer: {}", err);
                                          }
                                      }
            
                                    //write to file
                                    match file.write(&file_bytes){
                                          Ok(_) => (),
                                          Err(err) => {
                                              println!("Could not write file bytes to file: {}", err);
                                          }
                                    };
            
                                    println!("Write successful\n");

                                }
                                else{
                                    println!("File not found on server!")
                                }


                            },
                            "list" => {
                                println!("directory listing::");
                                match stream.write(cmd_user.as_bytes()) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to send command to server: {}", err);
                                        break;
                                    }
                                }

                                let mut reader = BufReader::new(&stream);

                                match reader.read_until(b'\r', &mut buffer) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to read into buffer: {}", err);
                                        break;
                                    }
                                }

                                let buffer = match str::from_utf8(&buffer) {
                                    Ok(buffer) => buffer,
                                    Err(err) => {
                                        println!("Could not write buffer as string: {}", err);
                                        break;
                                    }
                                };

                                print!("{}\nThat's all folks! \n", buffer);
                            }
                            "logout" => {
                                println!("match: logout");

                                match write!(&stream, "{}", &"logout\n") {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to send command to server: {}", err);
                                        break;
                                        //return Err(err);
                                    }
                                }

                                match stream.shutdown(std::net::Shutdown::Both) {
                                    Ok(_) => {
                                        println!("GOOD BYE!!");
                                        return;
                                    }
                                    Err(err) => {
                                        println!("cannot shutdown: {}", err);
                                        break;
                                    }
                                }
                            }
                            _ => {
                                println!("invalid command::");
                                println!("\n::Commands: \n::search \n:: write\n:: send\n:: receive\n:: list files\n:: logout\n")
                            }
                        } // end match on cmd
                    } // end command loop
                } // end authenticated_user

                println!(
                    "\n:: Registered users: login username \n:: Guests: connect guest\n:: logout\n"
                );
                let mut input = String::new(); // string for user input
                                               //let mut buffer: Vec<u8> = Vec::new(); 	// u8 vector for server responses
                match io::stdin().read_line(&mut input) {
                    // read line of input from user
                    Ok(_) => {
                        println!(">> {}", input);
                    }
                    Err(e) => println!("Error reading input; Please try again \n{:?}", e),
                }
                if input.trim() == "connect guest" || guest_user {
                    if !guest_user {
                        println!(":: Guest Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
                        guest_user = true;
                    }

                    loop {
                        let mut cmd_guest = String::new(); // string for user input
                                                           //let mut buffer = [0 as u8; 8]; // using 8 byte buffer
                        let mut buffer: Vec<u8> = Vec::new(); // u8 vector for server responses
                        let mut msg = String::new();
                        //io::stdin().read_line(&mut cmd_guest)	// read line of input from user
                        //	.expect("Failed to read from stdin");

                        match io::stdin().read_line(&mut cmd_guest) {
                            // read line of input from user
                            Ok(_) => {
                                //println!("You entered: {}",input);
                                println!("You entered: {}", cmd_guest);
                            }
                            Err(e) => println!("Error reading input; Please try again \n{:?}", e),
                        }

                        let cmd = cmd_guest.trim();

                        //println!("guest cmd: {}", cmd);

                        // guest command options

                        let tokens: Vec<&str> = cmd_guest.trim().split_whitespace().collect();
                        // let cmd = cmd_guest
                        // .split_whitespace()
                        // .next()
                        // .unwrap_or("");
                        //    println!("cmd: {}", cmd);

                        //	match tokens.len() {
                        //	3 => {
                        match tokens[0] {
                            "create" => {
                                if tokens.len() == 3 && tokens[1] == "user" {
                                    match stream.write(cmd_guest.as_bytes()) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            println!("Unable to send command to server: {}", err);
                                            break;
                                        }
                                    }
                                    let mut reader = BufReader::new(&stream);

                                    match reader.read_line(&mut msg) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            println!("Unable to read into buffer: {}", err);
                                            break;
                                        }
                                    }

                                    if msg.trim() == "request public key" {
                                        match fs::create_dir_all("./users_client/") {
                                            Err(why) => {
                                                println!("! {:?}", why.kind());
                                                break;
                                            }
                                            Ok(_) => {
                                                println!("users_client directory is created.");
                                            }
                                        }

                                        println!("response back: {}", msg.trim());
                                        //now create public,private and encrypted
                                        let mut user_path = String::from("./users_client/");

                                        user_path.push_str(tokens[2]); //tokens[2] is username

                                        match fs::create_dir(&user_path) {
                                            Err(why) => {
                                                println!("! {:?}", why.kind());
                                                break;
                                            }
                                            Ok(_) => {
                                                let mut private_key: u8 = 0;
                                                let mut public_key: u8 = 0;
                                                let mut encrypted_text: Vec<char> = Vec::new();

                                                encryption(
                                                    &mut encrypted_text,
                                                    &mut private_key,
                                                    &mut public_key,
                                                );

                                                //let pub_st=public_key.to_string().push_str("\n");

                                                match write!(&stream, "{}{}", &public_key, "\n") {
                                                    Ok(_) => (),
                                                    Err(err) => {
                                                        println!(
                                                            "Unable to send command to server: {}",
                                                            err
                                                        );
                                                        break;
                                                        //return Err(err);
                                                    }
                                                }
                                                let mut reader = BufReader::new(&stream);
                                                let mut msg2 = String::from("");
                                                match reader.read_line(&mut msg2) {
                                                    Ok(_) => (),
                                                    Err(err) => {
                                                        println!(
                                                            "Unable to read into buffer: {}",
                                                            err
                                                        );
                                                        break;
                                                    }
                                                }
                                                //println!("arizali mesaj icerigi:{}",&msg2);
                                                if msg2.trim() == "request encrypted" {
                                                    println!("response back: {}", msg2.trim());
                                                    let encrypted_string = encrypted_text
                                                        .iter()
                                                        .cloned()
                                                        .collect::<String>();

                                                    match write!(
                                                        stream,
                                                        "{}{}",
                                                        &encrypted_string, "\n"
                                                    ) {
                                                        Ok(_) => (),
                                                        Err(err) => {
                                                            println!("Unable to send command to server: {}", err);
                                                            break;
                                                            //return Err(err);
                                                        }
                                                    }

                                                    let mut reader = BufReader::new(&stream);
                                                    let mut msg3 = String::from("");
                                                    match reader.read_line(&mut msg3) {
                                                        Ok(_) => (),
                                                        Err(err) => {
                                                            println!(
                                                                "Unable to read into buffer: {}",
                                                                err
                                                            );
                                                            break;
                                                        }
                                                    }

                                                    if msg3.trim()=="User creation successful. Please login" {
                                                            				println!("response back: {}, username is: {}",msg3.trim(),tokens[2]);

                                                            				//writing pub ,private and encrypted to the clients file system.
                                                            				match File::create([&mut user_path, "/txt.enc"].join("")){
											Ok(mut f) => {
												match f.write_all(encrypted_string.as_bytes()){
													Ok(_)=>(),
													Err(err)=>{
														println!("Unable to write into file: {}", err);
														break
													}

												};
												()
											},
											Err(err) => {
												println!("Unable to write into file: {}", err);
												break
											}
										};

										match File::create([&mut user_path, "/txt.pub"].join("")){
											Ok(mut f) => {
												match f.write_all(public_key.to_string().as_bytes()){
													Ok(_)=>(),
													Err(err)=>{
														println!("Unable to write into file: {}", err);
														break
													}

												};
												()
											},
											Err(err) => {
												println!("Unable to write into file: {}", err);
												break
											}
										};


										match File::create([&mut user_path, "/txt.pri"].join("")){
											Ok(mut f) => {
												match f.write_all(private_key.to_string().as_bytes()){
													Ok(_)=>(),
													Err(err)=>{
														println!("Unable to write into file: {}", err);
														break
													}

												};
												()
											},
											Err(err) => {
												println!("Unable to write into file: {}", err);
												break
											}
										};



										//user_enc_file.write_all(encrypted_text.as_bytes());
									}
                                                }
                                            } //
                                        }
                                    } else {
                                        println!("Error! response back: {}", msg.trim());
                                    }
                                } else {
                                    // token0 is create - token 1 != user

                                    println!(
                                        ":To create a user account enter -> create user username"
                                    );

                                    println!(":: Guest Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
                                    //println!(":: Commands: \n\t -- create user username\n");
                                }

                                //	},
                                //	_ => {
                                //		println!("invalid guest command \n");
                                //		println!(":: Guest Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
                                //		//println!(":: Commands: \n\t -- create user username\n");
                                //	}
                            } // end token 0 is create

                            //		}, // end 3 parameters
                            //		2 => {
                            "show" => {
                                match cmd {
                                    "show users" => {
                                        println!("match: show users");

                                        match write!(&stream, "{}", &"show users\n") {
                                            Ok(_) => {
                                                let mut reader = BufReader::new(&stream);
                                                let mut msg11 = String::from("");
                                                match reader.read_line(&mut msg11) {
                                                    Ok(_) => (),
                                                    Err(err) => {
                                                        println!(
                                                            "Unable to read into buffer: {}",
                                                            err
                                                        );
                                                        //break;
                                                    }
                                                }

                                                println!("Users: {}", msg11.trim());
                                            }
                                            Err(err) => {
                                                println!(
                                                    "Unable to send command to server: {}",
                                                    err
                                                );
                                                //break
                                                //return Err(err);
                                            }
                                        }
                                    }
                                    "show active" => {
                                        println!("match: show active");

                                        match write!(&stream, "{}", &"show active\n") {
                                            Ok(_) => {
                                                let mut reader = BufReader::new(&stream);
                                                let mut msg13 = String::from("");
                                                match reader.read_line(&mut msg13) {
                                                    Ok(_) => (),
                                                    Err(err) => {
                                                        println!(
                                                            "Unable to read into buffer: {}",
                                                            err
                                                        );
                                                        //break;
                                                    }
                                                }
                                                println!("Active Users: {}", msg13.trim());
                                            }
                                            Err(err) => {
                                                println!(
                                                    "Unable to send command to server here: {}",
                                                    err
                                                );
                                                //break
                                                //return Err(err);
                                            }
                                        }
                                    }
                                    _ => {
                                        println!("show active | show users \n");
                                        println!(":: Guest Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
                                    }
                                }
                            } // end show for token[0]
                            //		}, // end match on 2 parameters
                            //		1 => {
                            //			match tokens[0] {
                            "logout" => {
                                authenticated_user = false;
                                guest_user = false;
                                break;
                            }
                            _ => {
                                println!("invalid guest command \n");
                                println!(":: Guest Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
                            } //				}
                        } // end match on token[0] of guest commands
                          //	_ => {
                          //		println!("invalid guest command\n");
                          //		println!("USERNAME must not contain space");
                          //		println!("USERNAME must be one word length");
                          //		println!(":: Guest Commands: \n\t -- create user username\n\t -- show users \n\t -- show active\n\t -- logout\n");
                          //		continue

                        //	}
                        //				}
                    } // end loop for guest commands
                } else if input.substring(0, 5) == "login" {
                    // ... code for authentication

                    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

                    if tokens.len() != 2 {
                        println!("::error!!!! wrong login command\n");
                        break;
                    } else {
                        println!("::LOGIN SCREEN .....\n");
                        let mut user_path = String::from("users_client/");

                        user_path.push_str(tokens[1]); //tokens[2] is username

                        match stream.write(input.as_bytes()) {
                            Ok(_) => (),
                            Err(err) => {
                                println!("Unable to send command to server: {}", err);
                                break;
                            }
                        }
                        let mut reader = BufReader::new(&stream);

                        let mut msg8 = String::from("");
                        match reader.read_line(&mut msg8) {
                            Ok(_) => (),
                            Err(err) => {
                                println!("Unable to read into buffer: {}", err);
                                break;
                            }
                        }
                        if msg8.trim() == "request encrypted" {
                            println!("response back: {}", msg8.trim());
                            let oldencrypted_text =
                                write_file_to_string(&[&user_path, "/txt.enc"].join(""));
                            let private_key =
                                write_file_to_string(&[&user_path, "/txt.pri"].join(""));
                            let public_key =
                                write_file_to_string(&[&user_path, "/txt.pub"].join(""));
                            //println!("my enkripsiyon {}",oldencrypted_text.len());

                            match write!(&stream, "{}{}", &oldencrypted_text, "\n") {
                                Ok(_) => (),
                                Err(err) => {
                                    println!("Unable to send command to server: {}", err);
                                    break;
                                    //return Err(err);
                                }
                            }

                            let mut reader = BufReader::new(&stream);
                            let mut msg10 = String::from("");
                            match reader.read_line(&mut msg10) {
                                Ok(_) => (),
                                Err(err) => {
                                    println!("Unable to read into buffer: {}", err);
                                    break;
                                }
                            }

                            if msg10.trim() == "Old encrypted matched.Requesting new encrypted" {
                                println!("response back: {}", msg10.trim());
                                //println!("{},{}",private_key,public_key);
                                let private_key: u8 = private_key.parse().unwrap();
                                let public_key: u8 = public_key.parse().unwrap();

                                let mut new_encrypted_text: Vec<char> = Vec::new();
                                encryptedTextt(&mut new_encrypted_text, &private_key, &public_key);

                                let new_encrypted_string =
                                    new_encrypted_text.iter().cloned().collect::<String>();

                                match write!(stream, "{}{}", &new_encrypted_string, "\n") {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to send command to server: {}", err);
                                        break;
                                        //return Err(err);
                                    }
                                }

                                let mut reader = BufReader::new(&stream);
                                let mut msg11 = String::from("");
                                match reader.read_line(&mut msg11) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("Unable to read into buffer: {}", err);
                                        break;
                                    }
                                }

                                if msg11.trim() == "Successful authentication" {

                                    username.push_str(tokens[1]);
                                    println!("response back: {}", msg11.trim());

                                    let mut user_enc_file =
                                        File::create([&mut user_path, "/txt.enc"].join(""));

                                    match user_enc_file {
                                        Ok(mut user_enc_file) => match user_enc_file.set_len(0) {
                                            Ok(_) => {
                                                match user_enc_file.write_all(
                                                    new_encrypted_string.trim().as_bytes(),
                                                ) {
                                                    Ok(_) => (),
                                                    Err(err) => {
                                                        println!(
                                                            "Unable to read into buffer: {}",
                                                            err
                                                        );
                                                        break;
                                                    }
                                                }
                                            }
                                            Err(err) => {
                                                println!("Unable to read into buffer: {}", err);
                                                break;
                                            }
                                        },
                                        Err(err) => {
                                            println!("error opening encrypted: {}", err);
                                            break;
                                        }
                                    }

                                    authenticated_user = true;
                                    guest_user = false;
                                } else {
                                    println!("error in authentication process");
                                }
                            } else if msg10.trim() == "login failed.Wrong encrypted" {
                                println!("response back: {}", msg10.trim());
                            }
                        } else if msg8.trim() == "User is not created. Please try again." {
                            println!("response back: {}", msg8.trim());
                        }

                        // let _username = tokens[1];
                        // authenticated_user=true;
                        // guest_user=false;
                    }
                } else if input.substring(0, 6) == "logout" {
                    break;
                } else {
                    // no match on command

                    println!("\nInvalid command {:?}. Please enter:\n\tconnect guest ||  login username\n", input);
                }
            } // end login or connect loop

            /*
             * loop to process authenticated user commands:
             * 	write, search, send, receive, list files
             */
        }
        Err(e) => {
            println!("Couldn't connect to server...\nPlease make sure server is up and receiving connections.");
        }
    } // end tcp connected
}

fn write_file_to_string(path1: &String) -> String {
    let file = fs::File::open(&path1);
    let mut contents = String::new();

    let mut file = match file {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Problem opening the file: {}", error);
            return "Problem opening the file".to_string();
        }
    };

    file.read_to_string(&mut contents)
        .map_err(|err| println!("{:?}", err))
        .ok();

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
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
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

// let date = Local::now().format("%d%m%Y%H%M%S").to_string();
// let mut rng = rand::thread_rng();
// let modulo:u8=64;
// let private_key:u8=rng.gen_range(1..modulo);
// let public_key:u8= modulo- (private_key);
// let mut encrypted_text:Vec<char>=Vec::new();

// let encrypted_text_temp:Vec<char>=date.chars().collect();
// for c in &encrypted_text_temp {
// 	//println!("{}:{}", c,(c.to_digit(10).unwrap()+ public_key)%modulo);
// 	//println!("{}:{}", c,(((c.to_digit(10).unwrap()) as u8+public_key)% modulo) as char);
// 	println!("{}:{}", *c,((*c as u8+public_key)% modulo) as char);

// 	//encrypted_text[i]=char::from_digit((c.to_digit(10).unwrap()+public_key)% modulo, 10).unwrap();
// 	encrypted_text.push(((*c as u8+public_key)% modulo) as char);
// 	//(c.to_digit(10).unwrap()+public_key)% modulo,10;

//   }

//   let encryptedString=encrypted_text.iter().cloned().collect::<String>();  //returned encrypted string
// to check
//   let mut server_encrypted_text:Vec<char>=Vec::new();

//   for c in encrypted_text {

//       server_encrypted_text.push(((c as u8+private_key)% modulo) as char);
//   }

//   //println!("hiiiii: {}",server_encrypted_text.iter().cloned().collect::<String>());
//   let serverstring=server_encrypted_text.iter().cloned().collect::<String>();

//   println!("{}",serverstring);

//   const RADIX: u32 = 10;
//   let x = "134";
//   println!("{}", x.chars().map(|c| c.to_digit(RADIX).unwrap()).sum::<u32>());
