use std::env;
mod search;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let file_name;
    let search_text;
    let args_length = args.len() - 1;

    if args_length < 2 {
        println!("Please insert the correct format and try again! The format is “search -f [FILE NAME] -s [TEXT]” or “search -f [FILE NAME] -s [TEXT]”");
        return;
    }

    let query = &args[1];

    let mut path = PathBuf::new();
    path.push("./users_client/");
    path.push("sumit/");

    // If the query is search, check for further arguments and perform search operation
    if query == "search" {
        if args[2] == "-f" {
            if args_length < 5 {
                println!("Please insert the correct format and try again! The format is “search -f [FILE NAME] -s [TEXT]” or “search -f [FILE NAME] -s [TEXT]”");
                return;
            }

            file_name = &args[3];
            path.push(file_name);

            let search_text_args: Vec<_> = args.drain(5..).collect();
            search_text = search_text_args.join(" ");

            let mut file = fs::File::open(path).expect("Can't open file!");

            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("Opps! Can not read the file...");

            search::search_f(&contents, &search_text);
        } else if args[2] == "-s" {
            if args_length < 3 {
                println!("Please insert the correct format and try again! The format is “search -f [FILE NAME] -s [TEXT]” or “search -f [FILE NAME] -s [TEXT]”");
                return;
            }
            let search_text_args: Vec<_> = args.drain(3..).collect();
            search_text = search_text_args.join(" ");

            for entry_res in fs::read_dir(path).unwrap() {
                let entry = entry_res.unwrap();
                let this_file_name_buf = entry.file_name();
                let this_file_name = this_file_name_buf.to_str().unwrap();
                let mut file = fs::File::open(entry.path()).expect("Can't open file!");
                let mut contents = String::new();

                file.read_to_string(&mut contents)
                    .expect("Opps! Can not read the file...");

                search::search_s(&contents, &search_text, &this_file_name);
            }
        } else {
            println!("Please specify the correct arguments!");
        }
    }
}
