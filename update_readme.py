# FIX: There's a bug where the list won't update if it's already in the json? like if it finds two new
# files at the same time, (IE one solution in rust, another for the same prob in python),
# it will only append the one that gets checked in the logic conditions first,
# (Python in the example I gave) the only workaround/fix rn is to wipe the problems.json file
from json.encoder import INFINITY
import os
import pickle
import git
import json
import datetime


# Define a function to format problem name
def format_problem_name(name):
    return name.replace("-", " ").title()


# TODO: Add functionality to this script so that it will replace just the name of the problem with a
# Markdown fancy link []() (<- like that)


use_json = True
revisit_count = 0
do_extra_git_searches = False
prob_path = ""
# Initialize an empty dictionary to store problem entries
# NOTE: Only run this once, because this is a very costly thing to run otherwise
scan_todo = False
if use_json:
    serialized = "./assets/problems.json"
    if os.path.exists(serialized):
        with open(serialized, "r") as f:
            problem_entries = json.load(f)
    else:
        problem_entries = {}
        do_extra_git_searches = True

else:
    serialized = "./assets/problems.pkl"

    if os.path.exists(serialized):
        with open(serialized, "rb") as f:
            problem_entries = pickle.load(f)
    else:
        problem_entries = {}
        do_extra_git_searches = True

problem_name = ""
language = ""
repo = git.Repo(".")


def commit_date_info(file):
    if file != "README.md":
        commits = list(repo.iter_commits(paths=file_path, reverse=True))
        date = INFINITY
        pretty_date = None
        prefered_commit = None
        print(f"Filename {file}, Commits found: {len(commits)}")
        for commit in commits:
            tmp = commit.authored_datetime.timestamp()
            if tmp < date:
                date = tmp
                pretty_date = commit.authored_datetime
                prefered_commit = commit
        if prefered_commit is not None and pretty_date is not None:
            print(
                f"Filename: {file}  Date: {pretty_date}, Commit Message {prefered_commit.message}"
            )
            if date < INFINITY:
                return date
        else:
            return datetime.datetime.now().timestamp()
    else:
        return None


def tagswag(tags, probnum):
    # NOTE: Only call this function AFTER the key is in the dictionary
    if problem_entries[probnum]["Tags"] != None:
        if len(tags) >= 1:
            for t in tags:
                print(f"Tag found! {t}")
                if t not in problem_entries[probnum]["Tags"]:
                    problem_entries[probnum]["Tags"].append(t)
                    revisit_count += 1
            else:
                return None



# Iterate over the directories
for root, dirs, files in os.walk("."):
    for file in files:
        problem_number = 0
        file_path = os.path.join(root, file)
        problem_name = ""
        language = ""
        writeup = False
        writeup_path = ""
        date = None
        repo = git.Repo(".")
        tags = []
        # Check if the file is a solution file
        if file.endswith((".rs", ".rb", ".java", ".py", ".md", ".c", ".cpp")):
            if file == "lib.rs" or file == "main.rs" or file == "update_readme.py":
                continue
            else:
                if scan_todo:
                    with open(file_path, "r") as f:
                        content = f.read()
                        if "TODO:" in content:
                            tags.append("Revisit")
                if file.endswith(".rs"):
                    # Split filename by underscores
                    parts = file.split("_")
                    # Extract problem name (all chunks except the last one)
                    problem_name = " ".join(parts[:-1])
                    problem_name = format_problem_name(problem_name)
                    # Extract problem number (last chunk without the ".rs" extension)
                    problem_number = parts[-1].split(".")[0]
                    # Append entry to the dictionary of problem entries
                    language = "Rust"
                    
                elif file.endswith(".rb"):
                    # Split filename by dots
                    parts = file.split(".")
                    # Extract problem number (first chunk)
                    problem_number = parts[0]
                    # Extract problem name (all chunks except the first and last)
                    problem_name = " ".join(parts[1:-1])
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
                    parts = file.split("_")
                    # Extract problem name (all chunks except the last one)
                    problem_name = " ".join(parts[:-1])
                    problem_name = format_problem_name(problem_name)
                    # Extract problem number (last chunk without the ".rs" extension)
                    problem_number = parts[-1].split(".")[0]
                    # Append entry to the dictionary of problem entries
                    language = "Python"

                elif file.endswith(".md") and file != "README.md":
                    problem_number = file.split(".")[0]
                    writeup = True
                    writeup_path = os.path.join("assets/writeups/", file)
                elif file.endswith(".c"):
                    # Split filename by dots
                    parts = file.split(".")
                    # Extract problem number (first chunk)
                    problem_number = parts[0]
                    # Extract problem name (all chunks except the first and last)
                    problem_name = " ".join(parts[1:-1])
                    problem_name = format_problem_name(problem_name)
                    # Append entry to the dictionary of problem entries
                    language = "C"
                elif file.endswith(".cpp"):
                    # Split filename by dots
                    parts = file.split(".")
                    # Extract problem number (first chunk)
                    problem_number = parts[0]
                    # Extract problem name (all chunks except the first and last)
                    problem_name = " ".join(parts[1:-1])
                    problem_name = format_problem_name(problem_name)
                    # Append entry to the dictionary of problem entries
                    language = "CPP"
                
                
                
                
                
                

                # If the problem number is already in the dictionary
                if problem_number in problem_entries:
                    if do_extra_git_searches:
                            if problem_entries[problem_number]["date"] is not None:
                                tmp = commit_date_info(file)
                                if tmp != None:
                                    if tmp < problem_entries[problem_number]["date"]:
                                        problem_entries[problem_number]["date"] = tmp
                    # If it's a writeup, update writeup flag and path
                    if (language not in problem_entries[problem_number]["languages"] and language != ""):
                        problem_entries[problem_number]['languages'][language] = file_path
                    if writeup:
                        if scan_todo:
                            tagswag(tags, problem_number)
                        else:
                            tags = None
                        problem_entries[problem_number]["writeup"] = True
                        problem_entries[problem_number]["writeup_path"] = writeup_path
                    # Update problem name if it's not already set or if its value is an empty string
                    if (
                        not problem_entries[problem_number]["name"]
                        or problem_entries[problem_number]["name"] == ""
                    ):
                        problem_entries[problem_number]["name"] = problem_name
                    # Update the list of Languages just in case:
                    if language not in problem_entries[problem_number]["languages"] and language != "":
                        problem_entries[problem_number]['languages'][language] = file_path
                # If the problem number is not in the dictionary
                elif problem_number != 0:
                    # Create a new entry
                    if writeup:
                        # If it's a writeup, initialize languages as an empty list
                        # problem_entries[problem_number] = [problem_name, [], True, writeup_path]
                        if len(tags) <= 0:
                            tags = None
                        problem_entries[problem_number] = {
                            "name": problem_name,
                            "languages": {},
                            "date": INFINITY,
                            "writeup": True,
                            "writeup_path": writeup_path,
                            "Tags": tags,
                        }
                        print(problem_entries[problem_number]["languages"])

                    else:
                        # If it's not a writeup, initialize languages with the current language
                        # problem_entries[problem_number] = [problem_name, [language], False]
                        # Also, initialize with the first found date, further added entries will only update this value if the found date is less than the date that is currently within the table

                        with open(file_path, "r") as f:
                            content = f.read()
                            if "TODO:" in content:
                                tags.append("Revisit")
                        if len(tags) <= 0 or tags == None:
                            tags = None
                        date = commit_date_info(file)
                        if language == "":
                            temp = {}
                        else:
                            temp = {language: file_path}
                        problem_entries[problem_number] = {
                            "name": problem_name,
                            "languages": temp,
                            "date": date,
                            "writeup": False,
                            "Tags": tags,
                        }
                        tagswag(tags, problem_number)

                else:
                    continue

# Sort the problem entries by problem number in ascending order
sorted_problem_entries = sorted(problem_entries.items(), key=lambda x: int(x[0]))

# Generate the Markdown table string
markdown_table = "| Problem Number | Problem Name | Language | Estimated Solved Date| Writeup/Solution? |\n|--------------|----------------|---------|----------|----------|\n"
for problem_number, entry in sorted_problem_entries:
    datestring = "Unknown (Based on Git Log)"
    if entry["date"]:
        jan15 = datetime.datetime.combine(
            datetime.date(2023, 1, 15), datetime.time()
        ).timestamp()
        jan16 = datetime.datetime.combine(
            datetime.date(2023, 1, 16), datetime.time()
        ).timestamp()
        if entry["date"] == 1673906820:
            datestring = "Unknown (Based on Git Log)"
        else:
            datestring = datetime.datetime.fromtimestamp(entry["date"]).strftime(
                "%B %d, %Y"
            )
    if entry["Tags"] != None:
        revisit_count += 1
    if entry["languages"]:
       languages = "" 
    for language in entry["languages"]:
        temp ="["+language+"]("+entry["languages"][language]+")"
        print(temp)
        languages += ", "+temp
    if entry["writeup"]:
        markdown_table += f"| {problem_number} | {entry['name']} | {languages} | {datestring} | [Yes]({entry['writeup_path']})|\n"
    else:
        markdown_table += f"| {problem_number} | {entry['name']} | {languages} | {datestring} | No |\n"

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
if start_index != 0:
    before_table = "".join(content[:start_index])

    # Write the content before the table back to the README.md file
    with open("README.md", "w") as file:
        file.write(before_table)

    # Write the updated content back to the README.md file
    with open("README.md", "a") as file:
        file.write('### Problems Marked "Revisit": ')
        file.write(str(revisit_count))
        file.write("\n")
        file.write(markdown_table)

# serialize and such
if use_json:
    with open(serialized, "w") as f:
        json.dump(problem_entries, f)
else:
    with open(serialized, "wb") as f:
        pickle.dump(problem_entries, f)
