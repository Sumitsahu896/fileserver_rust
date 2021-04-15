// This fucntion prints the sentence that contains the text to be searched

pub fn search_f(mut contents: &str, search_text: &str)  -> String {
	let mut response = String::from(" ");

    loop {
        let index_u = contents.find('.');
        if index_u == None {
            break;
        } else {
            let index = contents.chars().position(|c| c == '.');

            let index=match index{
                None =>{
                    eprintln!("Error index is null");
                    break
                } ,
                Some(index)=> index
            };
            


            let sub_string = &contents[0..index + 1];
    
            if sub_string.contains(search_text.trim()) {
                println!("found: {}\n", sub_string);
                response.push_str(sub_string.trim());
                response.push('\t');
            }
            contents = &contents[index +1..];
        }

        
    }
    //response.to_string()
    response
}

pub fn search_s(mut contents: &str, search_text: &str, file_name: &str) -> String  {
    let mut response = String::from(" ");
    loop {
       
        let index_u = contents.find('.');
        if index_u == None {
            break;
        } else {
            let index = contents.chars().position(|c| c == '.');

            let index=match index{
                None =>{
                    eprintln!("Error index is null");
                    break
                } ,
                Some(index)=> index
            };


            let sub_string = &contents[0..index + 1];
            if sub_string.contains(search_text.trim()) {
                //println!("{}: {}", file_name, sub_string);
                
                response.push_str(file_name.trim());
                response.push_str(":");
                response.push_str(sub_string.trim());
                response.push('\n');
            }
            contents = &contents[index + 1..];
        }
    }
    response
}