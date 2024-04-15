import os

# Define a function to format problem name
def format_problem_name(name):
    return name.replace('-', ' ').title()

# Initialize an empty list to store Markdown table entries
table_entries = []

# Iterate over the directories
for root, dirs, files in os.walk("."):
    for file in files:
        # Check if the file is a Rust solution file
        if file.endswith(".rs"):
            if file == "lib.rs" or "main.rs":
                continue
            else:
                # Split filename by underscores
                parts = file.split('_')
                # Extract problem name (all chunks except the last one)
                problem_name = ' '.join(parts[:-1])
                problem_name = format_problem_name(problem_name)
                # Extract problem number (last chunk without the ".rs" extension)
                problem_number = parts[-1].split('.')[0]

                # Append entry to the list of Markdown table entries
                table_entries.append((int(problem_number), problem_name, "Rust"))
                print(problem_number)
                print(problem_name)
                print(" ")

# Sort the table entries by problem number in ascending order
table_entries.sort()

# Generate the Markdown table string
markdown_table = "| Problem Name | Problem Number | Language |\n|--------------|----------------|----------|\n"
for entry in table_entries:
    markdown_table += f"| {entry[1]} | {entry[0]} | {entry[2]} |\n"

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
