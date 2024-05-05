import os
import pickle
# Define a function to format problem name
def format_problem_name(name):
    return name.replace('-', ' ').title()

# Initialize an empty dictionary to store problem entries
serialized = "./assets/problems.pkl"
if os.path.exists(serialized):
    with open(serialized, "rb") as f:
        problem_entries = pickle.load(f)
else:
    problem_entries = {}


problem_name =""
language=""



# Iterate over the directories
for root, dirs, files in os.walk("."):
    for file in files:
        problem_number=0
        problem_name =""
        language=""
        writeup=False
        writeup_path=""
        # Check if the file is a solution file
        if file.endswith((".rs", ".rb", ".java", ".py", ".md", ".c", ".cpp")):
            if file == "lib.rs" or file == "main.rs" or file == "update_readme.py":
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
                    problem_number = file.split(".")[0]
                    parts = file.split(".")[1]
                    parts = format_problem_name(parts)
                    problem_name = format_problem_name(parts)
                    language = "Java"
                elif file.endswith(".py"):
                    # Split filename by underscores
                    parts = file.split('_')
                    # Extract problem name (all chunks except the last one)
                    problem_name = ' '.join(parts[:-1])
                    problem_name = format_problem_name(problem_name)
                    # Extract problem number (last chunk without the ".rs" extension)
                    problem_number = parts[-1].split('.')[0]
                    # Append entry to the dictionary of problem entries
                    language = "Python"
                elif file.endswith(".md") and file != "README.md":
                    problem_number=file.split('.')[0]
                    writeup=True
                    writeup_path= os.path.join("assets/writeups/",file)
                elif file.endswith(".c"):
                    # Split filename by dots
                    parts = file.split('.')
                    # Extract problem number (first chunk)
                    problem_number = parts[0]
                    # Extract problem name (all chunks except the first and last)
                    problem_name = ' '.join(parts[1:-1])
                    problem_name = format_problem_name(problem_name)
                    # Append entry to the dictionary of problem entries
                    language = "C"
                elif file.endswith(".cpp"):
                    # Split filename by dots
                    parts = file.split('.')
                    # Extract problem number (first chunk)
                    problem_number = parts[0]
                    # Extract problem name (all chunks except the first and last)
                    problem_name = ' '.join(parts[1:-1])
                    problem_name = format_problem_name(problem_name)
                    # Append entry to the dictionary of problem entries
                    language = "CPP"
                if problem_number in problem_entries:
                    #this if else statement should help
                    if language in problem_entries[problem_number][1]:
                        problem_entries[problem_number][0] = problem_name #shouldn't have to do this, but maybe it's why I'm having problems
                        continue
                    else:
                        print(f"Found new language {language}, for {problem_number}")
                        # Update existing entry with the language
                        problem_entries[problem_number][1].append(language)
                        #if problem entry is added, but does not currently have a writeup linked? add it
                        if writeup:
                            problem_entries[problem_number][2] = True
                            problem_entries[problem_number][3] = writeup_path
                elif problem_number != 0:
                    # Create new entry with problem name and language
                    # TODO: Probably would be more ideal to just have this (writeup stuff) be a conditional entry in the table that 
                    # exists only if its needed, instead of adding unneccessary extra keys 
                    if writeup:
                        problem_entries[problem_number] = [problem_name, [language], True, writeup_path]
                    else: 
                        print("No writeup! appending false, and an empty value")
                        problem_entries[problem_number] = [problem_name, [language], False, ""]
                    
                    languages = ', '.join(problem_entries[problem_number][1])
                    print(f"{problem_number} {problem_entries[problem_number][0]} | {languages}")
                else:
                    continue

# Sort the problem entries by problem number in ascending order
sorted_problem_entries = sorted(problem_entries.items(), key=lambda x: int(x[0]))

# Generate the Markdown table string
markdown_table = "| Problem Number | Problem Name | Language | Writeup/Solution? |\n|--------------|----------------|----------|----------|\n"
for problem_number, entry in sorted_problem_entries:
    if entry[1]:
        languages = ', '.join(filter(None, entry[1]))
    else:
        languages = ""
    if entry[2]:
        print(entry[3])
        markdown_table += f"| {problem_number} | {entry[0]} | {languages} | [Yes]({entry[3]})|\n"
    else:
        markdown_table += f"| {problem_number} | {entry[0]} | {languages} | No |\n"

# Read the content of the README.md file
with open("README.md", "r") as file:
    content = file.readlines()

# Find the position of the heading "List of problems solved"
start_index = 0
for index, line in enumerate(content):
    if "List of problems solved" in line:
        # Add 2 to the index to skip the header and the underline below it
        start_index = index + 1
        break

# Extract the content before the table
if start_index != 0 : 
    before_table = "".join(content[:start_index])

# Write the content before the table back to the README.md file
    with open("README.md", "w") as file:
        file.write(before_table)

# Write the updated content back to the README.md file
    with open("README.md", "a") as file:
        file.write(markdown_table)

#serialize and such
    with open(serialized, "wb") as f:
        pickle.dump(problem_entries, f)



