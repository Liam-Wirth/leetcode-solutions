use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type ProblemTable = HashMap<i32, Problem>;
use std::fs;
use std::path::PathBuf;
#[derive(Debug, Serialize, Deserialize)]
struct Problem {
    name: String,
    languages: Vec<String>,
    solved_dates: Vec<String>,
}

// Define a function to format problem name
fn snake_case_to_words(text: Vec<&str>) -> String {
    // Split the snake_case string into words

    // Capitalize the first letter of each word
    let mut capitalized_words = Vec::new();
    for word in text {
        capitalized_words
            .push(word.to_uppercase().chars().next().unwrap().to_string() + &word[1..]);
    }

    // Join the capitalized words back into a string with spaces
    capitalized_words.join(" ")
}
fn ignore_directory(dir_name: &str) -> bool {
    matches!(dir_name, "." | "./" | "./." | "target" | "writeups")
}
fn parse_rs(fname: &str, prob: &mut Option<Problem>) -> (i32, Problem) {
    let mut problem_num: i32 = 0;
    prob = match prob {
        Some(mut problem) => {
            problem.languages.push("Rust".to_string());

            &mut Some(problem)
        }
        None => &mut Some(Problem {
            name: "".to_string(),
            languages: Vec::new(),
            solved_dates: Vec::new(),
        }),
    };
    if let Some(fname) = fname.strip_suffix(".rs") {
        let parts: Vec<&str> = fname.split('_').collect();
        if parts.len() >= 2 {
            prob.name = snake_case_to_words(parts[..parts.len() - 1].to_vec());
            if let Ok(problenum) = parts[parts.len() - 1].parse::<i32>() {
                problem_num = problenum;
            }
        }
    }

    (problem_num, prob.unwrap())
}

fn proccess_dirs(dir_path: &PathBuf) -> ProblemTable {
    let mut table = ProblemTable::new();
    for entry in fs::read_dir(dir_path).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                //Sort out directories I don't want to waste time looking at:
                if let Some(dir_name) = path.file_name() {
                    if let Some(dir_str) = dir_name.to_str() {
                        if ignore_directory(dir_str) {
                            println!("Ignoring Directory: {}", dir_str);
                            continue;
                        } else {
                            println!("Processing Sub-Directory: {}", dir_str);
                            let sub_table = proccess_dirs(&path);
                            table.extend(sub_table);
                        }
                    }
                }
            }
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if let Some(extension) = path.extension() {
                    match extension.to_str() {
                        Some("rs") => {
                            println!("Rust File {:?}", file_name);
                        }
                        Some("c") | Some("cpp") => {
                            println!("C File {:?}", file_name);
                        }
                        Some("java") => {
                            println!("Java File {:?}", file_name);
                        }
                        Some("py") => {
                            println!("Python File {:?}", file_name);
                        }
                        Some("rb") => {
                            println!("Ruby File {:?}", file_name);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    table
}

fn main() {
    // Initialize an empty vector to store Markdown table entries
    let mut table_entries: Vec<(i32, String, &str)> = Vec::new();
    // Iterate over the directories
    //TODO: We are going to need to figure out how/where this binary will be called from, for now I'll just use relative paths
    //if let Some(file_name_str) = file_name_str.strip_suffix(".rs") {
    //    let parts: Vec<&str> = file_name_str.split('_').collect();
    //    if parts.len() >= 2 {
    //        let problem_name = snake_case_to_words(parts[..parts.len() - 1].to_vec());
    //        if let Ok(problem_number) = parts[parts.len() - 1].parse::<i32>() {
    //            table_entries.push((problem_number, problem_name, "Rust"));
    //        }
    //    }
    //}
    // Sort the table entries by problem number in ascending order
    // table_entries.sort_by(|a, b| a.0.cmp(&b.0));

    // // Generate the Markdown table string
    // let mut markdown_table = String::new();
    // markdown_table.push_str("| Problem Name | Problem Number | Language |\n");
    // markdown_table.push_str("|--------------|----------------|----------|\n");
    // for entry in &table_entries {
    //     markdown_table.push_str(&format!("| {} | {} | {} |\n", entry.1, entry.0, entry.2));
    // }

    // // Read the content of the README.md file
    // let mut readme_content = fs::read_to_string("README.md").expect("Failed to read README.md");

    // // Find the position of the last occurrence of the end marker for the table
    // if let Some(end_marker_position) = readme_content.rfind("|----|----|----|") {
    //     // Replace the content from the end marker position to the end with the updated Markdown table
    //     let updated_content = format!(
    //         "{}{}",
    //         &readme_content[..end_marker_position],
    //         markdown_table
    //     );
    //     // Write the updated content back to the README.md file
    //     fs::write("README.md", updated_content).expect("Failed to write to README.md");
    // }
}
