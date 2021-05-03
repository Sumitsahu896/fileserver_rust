use std::env;
mod search;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {

    loop{

    
    let mut args: Vec<String> = env::args().collect();
    let args_length = args.len() - 1;
    if args_length == 0 {
        println!("Invalid argument; There is no argument error");
        break;
    }

    let mut path = PathBuf::new();
    path.push("./users_client/");
    path.push("sumit/");
    let query = &args[1];




    if query == "search" {

        let file_name;
        let search_text;




        if args[2] == "-f" && args.iter().any(|i| i=="-s"){
            if args_length < 5 {
                println!("Please insert the correct format and try again! The format is “search -f [FILE NAME] -s [TEXT]” or “search -f [FILE NAME] -s [TEXT]”");
                break;
            }

            file_name = &args[3];
            path.push(file_name);

            let search_text_args: Vec<_> = args.drain(5..).collect();
            search_text = search_text_args.join(" ");

            let file = fs::File::open(path);
            let mut contents = String::new();


            let mut file=match file {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Problem opening the file: {}",error);
                    break
                }
             };

            file.read_to_string(&mut contents).map_err(
                |err| println!("{:?}", err)
            
            ).ok();
            
   

            search::search_f(&contents, &search_text);
            break;
        } else if args[2] == "-s" {
            if args_length < 3 {
                println!("Please insert the correct format and try again! The format is “search -f [FILE NAME] -s [TEXT]” or “search -f [FILE NAME] -s [TEXT]”");
                return;
            }
            let search_text_args: Vec<_> = args.drain(3..).collect();
            search_text = search_text_args.join(" ");

            
            let r_directory=fs::read_dir(path);

            let r_directory=match r_directory {
                Err(e) =>{
                    eprintln!("Path problem :{:?}", e);
                    break
                } 
                Ok(r_directory) => (r_directory)
            };



                for entry_res in r_directory{
            
                let entry = entry_res;

                //if entry.is_err() { continue; }


                let entry=match entry{
                    Err(e) =>{
                        eprintln!("{:?}", e);
                        break
                    } ,
                    Ok(entry) => entry
                };

                let this_file_name_buf = entry.file_name();
                let this_file_name = this_file_name_buf.to_str();



                let this_file_name=match this_file_name{
                    None =>{
                        eprintln!("error");
                        break
                    } ,
                    Some(this_file_name)=> this_file_name.to_string()
                };
                
              
                // let this_file_name=match this_file_name{
                //     Err(e) =>{
                //         eprintln!("{:?}", e);
                //         break
                //     } ,
                //     Ok(this_file_name) => this_file_name
                // };






                let file = fs::File::open(entry.path());

                let mut file=match file {
                    Ok(file) => file,
                    Err(error) => {
                        eprintln!("Problem opening the file: {}",error);
                        break
                    }
                 };


                let mut contents = String::new();

                file.read_to_string(&mut contents).map_err(
                    |err| println!("{:?}", err)
                
                ).ok();

                search::search_s(&contents, &search_text, &this_file_name);
               
            }
            break;
        } else {
            println!("Search error! Please specify the correct arguments!");
            break;
        }
    }
  }


 
}
