import os

# Define a function to format problem name
def format_problem_name(name):
    return name.replace('-', ' ').title()

# Initialize an empty dictionary to store problem entries
problem_entries = {}
problem_number=0
problem_name =""
language=""
# Iterate over the directories
for root, dirs, files in os.walk("."):
    for file in files:
        problem_number=0
        problem_name =""
        language=""
        # Check if the file is a solution file
        if file.endswith((".rs", ".rb", ".java")):
            if file == "lib.rs" or file == "main.rs":
                continue
            else:
                if file.endswith(".rs"):
                    # Split filename by underscores
                    parts = file.split('_')
                    # Extract problem name (all chunks except the last one)
                    problem_name = ' '.join(parts[:-1])
                    problem_name = format_problem_name(problem_name)
                    # Extract problem number (last chunk without the ".rs" extension)
                    problem_number = parts[-1].split('.')[0]
                    # Append entry to the dictionary of problem entries
                    language = "Rust"
                elif file.endswith(".rb"):
                    # Split filename by dots
                    parts = file.split('.')
                    # Extract problem number (first chunk)
                    problem_number = parts[0]
                    # Extract problem name (all chunks except the first and last)
                    problem_name = ' '.join(parts[1:-1])
                    problem_name = format_problem_name(problem_name)
                    # Append entry to the dictionary of problem entries
                    language = "Ruby"
                elif file.endswith(".java"):
                    print(file)
                    problem_number = file.split(".")[0]
                    parts = file.split(".")[1]
                    parts = format_problem_name(parts)
                    print(parts)
                    problem_name = format_problem_name(parts)
                    language = "Java"
                    print("Java Found")

                if problem_number in problem_entries:
                    # Update existing entry with the language
                    problem_entries[problem_number][1].append(language)
                else:
                    # Create new entry with problem name and language
                    problem_entries[problem_number] = [problem_name, [language]]

# Sort the problem entries by problem number in ascending order
sorted_problem_entries = sorted(problem_entries.items(), key=lambda x: x[0])

# Generate the Markdown table string
markdown_table = "| Problem Name | Problem Number | Language |\n|--------------|----------------|----------|\n"
for problem_number, entry in sorted_problem_entries:
    languages = ', '.join(entry[1])
    markdown_table += f"| {entry[0]} | {problem_number} | {languages} |\n"

# Read the content of the README.md file
with open("README.md", "r") as file:
    content = file.read()

    # Find the position of the last occurrence of the end marker for the table
    end_marker_position = content.rfind("|----|----|----|")

    # Replace the content from the end marker position to the end with the updated Markdown table
    updated_content = content[:end_marker_position] + markdown_table + content[end_marker_position:]

# Write the updated content back to the README.md file
with open("README.md", "w") as file:
    file.write(updated_content)

