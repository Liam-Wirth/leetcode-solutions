from typing import Union, Optional, Dict, Any, List
import os
import pickle
from dotenv.main import load_dotenv
import json
import datetime
import dotenv
import sys
import git


# Add the directory containing api_testing.py to the Python path
sys.path.append(os.path.join(os.path.dirname(__file__), "readme"))

from readme.api_testing import fetch_first_solve_date, fetch_problem_info, fetch_user_info
from readme.utils import (
    create_markdown_table,
)
from readme.parser import parse_solution_file

good = dotenv.load_dotenv()  # Load environment variables from .env

if not good:
    print(
        "ERROR: Please get your leetcode session tokkey and update it within the .env file in the project root. Needs to be with the key LEETCODE_API_KEY"
    )
else:
    print(os.getenv("LEETCODE_API_KEY"))

leetcode_api_key = os.getenv("LEETCODE_API_KEY")

use_json: bool = True
revisit_count: int = 0
do_extra_git_searches: bool = False
prob_path: str = ""
scan_todo: bool = False

ProblemEntry = dict[str, str | dict[str, str] | float | bool | list[str] | None]

problem_entries: dict[str, ProblemEntry] = {}
user_rankings: list[dict[str, Union[str, int]]] = []

if use_json:
    serialized: str = "assets/problems.json"
    rankings_serialized: str = "assets/rankings.json"
    if os.path.exists(serialized):
        with open(serialized, "r") as f:
            problem_entries = json.load(f)
    else:
        problem_entries = {}
        do_extra_git_searches = True
    if os.path.exists(rankings_serialized):
        with open(rankings_serialized, "r") as f:
            user_rankings = json.load(f)
    else:
        user_rankings = []
else:
    serialized: str = "assets/problems.pkl"
    rankings_serialized: str = "assets/rankings.pkl"
    if os.path.exists(serialized):
        with open(serialized, "rb") as f:
            problem_entries = pickle.load(f)
    else:
        problem_entries = {}
        do_extra_git_searches = True
    if os.path.exists(rankings_serialized):
        with open(rankings_serialized, "rb") as f:
            user_rankings = pickle.load(f)
    else:
        user_rankings = []


problem_name: str = ""
language: str = ""
repo_path: str = ""
repo = git.Repo(repo_path)


# Iterate over the directories
for root, dirs, files in os.walk("."):
    for file in files:
        file_path: str = os.path.join(root, file)
        problem_entries, revisit_count = parse_solution_file(
            file_path, problem_entries, do_extra_git_searches, scan_todo, repo_path
        )

# Fetch user ranking and update history
username = "liam-wirth"  # Replace with your LeetCode username
user_info = fetch_user_info(username)
current_ranking = user_info.get("ranking")

if current_ranking is not None:
    if not user_rankings or user_rankings[-1].get("ranking") != current_ranking:
        user_rankings.append(
            {"timestamp": datetime.datetime.now().isoformat(), "ranking": current_ranking}
        )

# Sort the problem entries by problem number in ascending order
sorted_problem_entries = sorted(problem_entries.items(), key=lambda x: int(x[0]))

# Generate the Markdown table string
markdown_table, revisit_count = create_markdown_table(sorted_problem_entries)


# Read the content of the README.md file
with open("README.md", "r") as file:
    content: list[str] = file.readlines()

# Find the position of the heading "List of problems solved"
start_index: int = 0
for index, line in enumerate(content):
    if "List of problems solved" in line:
        start_index = index + 1
        break
# Calculate problems solved by language
language_counts = {}
for entry in problem_entries.values():
    for language in entry["languages"]:
        language_counts[language] = language_counts.get(language, 0) + 1

# Generate Mermaid pie chart syntax
mermaid_chart = "```mermaid\npie title Problems Solved by Language\n"
for language, count in language_counts.items():
    mermaid_chart += f'    "{language}": {count}\n'
mermaid_chart += "```\n"
print(mermaid_chart)
# Extract the content before the table
if start_index != 0:
    before_table: str = "".join(content[:start_index])

    # Write the content before the table back to the README.md file
    with open("README.md", "w") as file:
        file.write(before_table)
        file.write('# Chart:')
        file.write(mermaid_chart)
        file.write('\n')

    # Write the updated content back to the README.md file
    with open("README.md", "a") as file:
        file.write('### Problems Marked "Revisit": ')
        file.write(str(revisit_count))
        file.write("\n")
        file.write(markdown_table)

# Ensure the assets directory exists
assets_dir = os.path.dirname(serialized)
os.makedirs(assets_dir, exist_ok=True)

# serialize and such
if use_json:
    with open(serialized, "w") as f:
        json.dump(problem_entries, f)
    with open(rankings_serialized, "w") as f:
        json.dump(user_rankings, f)
else:
    with open(serialized, "wb") as f:
        pickle.dump(problem_entries, f)
    with open(rankings_serialized, "wb") as f:
        pickle.dump(user_rankings, f)
