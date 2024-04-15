use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::Path;
// Define a function to format problem name
fn snake_case_to_words(text: Vec<&str>) -> String {
    // Split the snake_case string into words
    let words: Vec<&str> = text.split('_').collect();

    // Capitalize the first letter of each word
    let mut capitalized_words = Vec::new();
    for word in words {
        capitalized_words
            .push(word.to_uppercase().chars().next().unwrap().to_string() + &word[1..]);
    }

    // Join the capitalized words back into a string with spaces
    capitalized_words.join(" ")
}

fn main() {
    // Initialize an empty vector to store Markdown table entries
    let mut table_entries: Vec<(i32, String, &str)> = Vec::new();

    // Iterate over the directories
    for entry in fs::read_dir(".").expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if let Some(file_name_str) = file_name_str.strip_suffix(".rs") {
                    let parts: Vec<&str> = file_name_str.split('_').collect();
                    if parts.len() >= 2 {
                        let problem_name =
                            snake_case_to_words(&parts[..parts.len() - 1].to_vec());

                        if let Ok(problem_number) = parts[parts.len() - 1].parse::<i32>() {
                            table_entries.push((problem_number, problem_name, "Rust"));
                        }
                    }
                }
            }
        }
    }

    // Sort the table entries by problem number in ascending order
    table_entries.sort_by(|a, b| a.0.cmp(&b.0));

    // Generate the Markdown table string
    let mut markdown_table = String::new();
    markdown_table.push_str("| Problem Name | Problem Number | Language |\n");
    markdown_table.push_str("|--------------|----------------|----------|\n");
    for entry in &table_entries {
        markdown_table.push_str(&format!("| {} | {} | {} |\n", entry.1, entry.0, entry.2));
    }

    // Read the content of the README.md file
    let mut readme_content = fs::read_to_string("README.md").expect("Failed to read README.md");

    // Find the position of the last occurrence of the end marker for the table
    if let Some(end_marker_position) = readme_content.rfind("|----|----|----|") {
        // Replace the content from the end marker position to the end with the updated Markdown table
        let updated_content = format!(
            "{}{}",
            &readme_content[..end_marker_position],
            markdown_table
        );
        // Write the updated content back to the README.md file
        fs::write("README.md", updated_content).expect("Failed to write to README.md");
    }
}
