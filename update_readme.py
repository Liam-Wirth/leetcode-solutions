# TODO: Would be nice to update the api calls to check and provide a website link to the solution as well, visible on leetcode instead of just the repo/local solution
import subprocess
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
from readme.utils import create_markdown_table
from readme.parser import parse_solution_file
from readme.update_dates import update_dates

FORCE_UPDATE_DATE = False

good = dotenv.load_dotenv()  # Load environment variables from .env
if not good:
    print("ERROR: Please update your .env file with LEETCODE_API_KEY")
else:
    # print(os.getenv("LEETCODE_API_KEY"))
    # old debug statement, disregard lol
    pass    

leetcode_api_key = os.getenv("LEETCODE_API_KEY")

use_json: bool = True
revisit_count: int = 0
do_extra_git_searches: bool = False
scan_todo: bool = False

ProblemEntry = dict[str, str | dict[str, str] | float | bool | list[str] | None]

problem_entries: dict[str, ProblemEntry] = {}
user_rankings: list[dict[str, Union[str, int]]] = []

# --- 1) Load filtered_probs (with string keys) ---
filtered_probs_path = "assets/filtered_probs.pkl"
filtered_probs = {}
if os.path.exists(filtered_probs_path):
    with open(filtered_probs_path, "rb") as f:
        filtered_probs = pickle.load(f)
else:
    print(f"Warning: {filtered_probs_path} not found; validations will be skipped.")

# --- 2) Load existing problem_entries & user_rankings ---
if use_json:
    serialized: str = "assets/problems.json"
    rankings_serialized: str = "assets/rankings.json"
    if os.path.exists(serialized):
        with open(serialized, "r") as f:
            problem_entries = json.load(f)
            print("Loaded problem entries, length: ", len(problem_entries))
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

repo_path: str = ""  # or the actual path to your repo
repo = git.Repo(repo_path)

# --- 3) Parse each solution file ---
for root, dirs, files in os.walk("."):
    for file in files:
        file_path: str = os.path.join(root, file)
        problem_entries, revisit_count = parse_solution_file(
            file_path, problem_entries, do_extra_git_searches, scan_todo, repo_path
        )

# --- 4) Optionally fetch user ranking ---
username = "liam-wirth"
user_info = fetch_user_info(username)
current_ranking = user_info.get("ranking")
if current_ranking is not None:
    if not user_rankings or user_rankings[-1].get("ranking") != current_ranking:
        user_rankings.append(
            {"timestamp": datetime.datetime.now().isoformat(), "ranking": current_ranking}
        )

# --- 5) Attach slug & fetch solve dates if needed ---
for prob_id, entry in problem_entries.items():
    # Make sure ID is in filtered_probs (i.e. has a slug)
    if prob_id in filtered_probs:
        entry["slug"] = filtered_probs[prob_id]["question__title_slug"]
        # If date missing or == float('inf'), attempt fetch
        if not entry.get("date") or entry["date"] == float("inf"):
            slug = entry["slug"]
            solve_date = fetch_first_solve_date(username, slug)
            if solve_date:
                entry["date"] = solve_date.timestamp()
                print(f"Fetched official solve date for {prob_id} (slug={slug})")
            else:
                print(f"No official solve date for {prob_id} (slug={slug})")
    else:
        print(f"[WARNING] Problem ID {prob_id} not found in filtered_probs.pkl")

update_dates(problem_entries, username, FORCE_UPDATE_DATE)

# --- 6) Sort & create the markdown table ---
sorted_problem_entries = sorted(problem_entries.items(), key=lambda x: int(x[0]))
markdown_table, revisit_count = create_markdown_table(sorted_problem_entries)

# --- 7) Update the README.md with the newly generated table ---
with open("README.md", "r") as file:
    content: list[str] = file.readlines()

start_index: int = 0
for index, line in enumerate(content):
    if "List of problems solved" in line:
        start_index = index + 1
        break

# Count # solutions per language (pie chart)
language_counts = {}
for entry in problem_entries.values():
    for lang in entry["languages"]:
        language_counts[lang] = language_counts.get(lang, 0) + 1

mermaid_chart = "```mermaid\npie title Problems Solved by Language\n"
for lang, count in language_counts.items():
    mermaid_chart += f'    "{lang}": {count}\n'
mermaid_chart += "```\n"
print(mermaid_chart)

subprocess.run(['python', 'assets/graphrank.py'])
if start_index != 0:
    before_table: str = "".join(content[:start_index])
    with open("README.md", "w") as file:
        file.write(before_table)

        file.write('# Chart:\n')
        file.write(mermaid_chart)
        file.write('\n![Ranking Graph](assets/rankings_plot.png)\n')
        file.write('\n')
        file.write('### Problems Marked "Revisit": ')
        file.write(str(revisit_count))
        file.write("\n")

    with open("README.md", "a") as file:
        file.write(markdown_table)

# --- 8) Serialize everything back out ---
assets_dir = os.path.dirname(serialized)
os.makedirs(assets_dir, exist_ok=True)

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

