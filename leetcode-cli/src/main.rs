use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
enum Lang {
    Rust,
    Python,
    Java,
    C,
    CPP,
    JS,
    Ruby,
    Go,
}

impl Default for Lang {
    fn default() -> Self {
        Self::Rust
    }
}

const LEETCODE_REPO_PATH: &str = "~/Projects/coding-problems/leetcode-solutions/";

fn change_to_leetcode_directory(path: &str) -> Result<(), String> {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(format!("Failed to get current directory: {}", err)),
    };

    if current_dir != Path::new(path) {
        if let Err(err) = env::set_current_dir(path) {
            return Err(format!("Failed to change directory to {}: {}", path, err));
        }
    }

    Ok(())
}
fn format_problem_name_snake(name: &str) -> String {
    name.to_lowercase().replace(" ", "_")
}

fn clean_response(response: &str) -> String {
    response
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter(|c| !c.is_ascii_punctuation())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn open_vim(filename: &str) {
    println!("Open {} in Vim? (y/n)", filename);
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    let cleaned_response = clean_response(&response);
    if cleaned_response == "y" || cleaned_response == "yes" {
        Command::new("nvim")
            .arg(&filename)
            .status()
            .expect("Failed to open file in editor");
    }
}

///Too damn lazy to rewrite the python script in rust, so cope
fn update_readme() -> std::io::Result<()> {
    if let Err(err) = change_to_leetcode_directory(LEETCODE_REPO_PATH) {
        eprintln!("Error: {}", err);
    }
    let mut response = String::new();
    println!("Add to readme?");
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    let cleaned_response = clean_response(&response);
    if cleaned_response == "y" || cleaned_response == "yes" {
        // Call Python script to update README
        Command::new("python3")
            .arg("update_readme.py")
            .status()
            .expect("Failed to update readme");
    }
    Ok(())
}

fn do_stuff_snake_case() -> (String, String) {
    let mut probname = String::new();
    let mut problem_number = String::new();
    println!("Name of the problem?");
    io::stdin().read_line(&mut probname).expect("FAIL!");
    let probname = clean_response(&probname.as_str());
    let fname = format_problem_name_snake(probname.as_str());

    println!("problem number:");
    io::stdin()
        .read_line(&mut problem_number)
        .expect("Failed to read line");
    let cleaned_number = clean_response(&problem_number);

    return (fname, cleaned_number);
}

fn main() -> ! {
    if let Err(err) = change_to_leetcode_directory(LEETCODE_REPO_PATH) {
        eprintln!("Error: {}", err);
    }
    //we are now in the leetcode repo
    //
    loop {
        println!("What language are we working in? (Rust is default)");
        println!("0: Rust");
        println!("1: Python");
        println!("2; C");
        println!("3; CPP");
        println!("4, Java");
        println!("5; JS");
        println!("6, Go");
        println!("7, Ruby");
        let mut choice: &str = "";
        let mut probname = String::new();
        let mut problem_number = String::new();

        io::stdin()
            .read_line(&mut choice.to_string())
            .expect("Fail!");

        let choice = clean_response(choice);

        match choice.to_lowercase().as_str() {
            "1" | "python" | "py" => {
                //python logic is very simmilar to the rust one
                println!("I mean python is kinda cool.... \nI guess....");
                std::env::set_current_dir("./python/").expect("Python Subdirectory not found :(");
                let (probname, problem_number) = do_stuff_snake_case();
                let filename = format!("{}_{}.py", probname.trim(), problem_number.trim());

                let _ = std::fs::File::create(&filename);
                open_vim(filename.as_str());
                let _ = update_readme();
            }
            "2" | "c" => {}
            "3" | "cpp" => {}
            "4" | "java" => {}
            "5" | "js" | "javascript" => {}
            "6" | "go" => {}
            "7" | "ruby" | "rb" => {
                println!("I'd be lying if I said I didn't think this was crazy, but Ruby it is");
                std::env::set_current_dir("./Ruby/").expect("Ruby Subdirectory Not Found");
                // TODO:


            }
            "0" | "rust" | "rs" | _ => {
                println!("Rust it is!");
                std::env::set_current_dir("./rust/src/").expect("Rust Subdirectory not found :(");
                println!("Name of the problem?");
                io::stdin().read_line(&mut probname).expect("FAIL!");
                let probname = clean_response(&probname.as_str());
                let fname = format_problem_name_snake(probname.as_str());

                println!("problem number:");
                io::stdin()
                    .read_line(&mut problem_number)
                    .expect("Failed to read line");
                let cleaned_number = clean_response(&problem_number);

                let filename = format!("{}_{}.rs", fname.trim(), cleaned_number.trim());
                let modname = format!("{}_{};", fname.trim(), cleaned_number.trim());
                let _ = std::fs::File::create(&filename);

                let mut lib_rs = std::fs::OpenOptions::new()
                    .append(true)
                    .open("lib.rs")
                    .expect("Failed to open lib.rs");
                writeln!(lib_rs, "mod {};", modname).expect("Failed to write to lib.rs");

                println!("File created: {}", filename);
                println!("mod {} added to lib.rs", filename);

                open_vim(&filename);
                let _ = update_readme();
            }
        }
    }
}
