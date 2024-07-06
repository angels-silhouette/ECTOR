use std::env;

fn encrypt(key: u128, string: String) -> String {
    

    return output;
}

fn main() {
    let mut working_dir: String = String::from("");
    let mut switches: Vec<char> = Vec::new();
    let mut flags: Vec<String> = Vec::new();
    let mut arguments: String = String::from("");

    {
        let args: Vec<String> = env::args().collect();

        for arg in args.iter().enumerate() {
            if arg.0 == 0 {
                working_dir = arg.1.clone();
            } else if arg.1.starts_with("--") {
                println!("This program does not accept any flags");
            } else if arg.1.starts_with("-") {
                for item in arg.1.chars() {
                    switches.push(item.clone());
                }
            } else {
                arguments = arg.1.clone();
            }
        }
    }

    if switches.contains('e') {
        
    }
}
