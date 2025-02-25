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
from rich.progress import (
    Progress,
    SpinnerColumn,
    TextColumn,
    BarColumn,
    TimeElapsedColumn,
)
import concurrent.futures  # New: for multithreading

# Add the directory containing api_testing.py to the Python path
sys.path.append(os.path.join(os.path.dirname(__file__), "readme"))

from readme.api_testing import (
    fetch_first_solve_date,
    fetch_problem_info,
    fetch_user_info,
)
from readme.utils import create_markdown_table
from readme.parser import parse_solution_file
from readme.update_dates import update_dates

FORCE_UPDATE_DATE = False

good = dotenv.load_dotenv()  # Load environment variables from .env
if not good:
    print("ERROR: Please update your .env file with LEETCODE_API_KEY")
else:
    # old debug statement, disregard
    pass

leetcode_api_key = os.getenv("LEETCODE_API_KEY")

use_json: bool = True
revisit_count: int = 0
do_extra_git_searches: bool = False
scan_todo: bool = False

ProblemEntry = dict[str, str | dict[str, str] | float | bool | list[str] | None]

problem_entries: dict[str, ProblemEntry] = {}
user_rankings: list[dict[str, Union[str, int]]] = []
user_points: list[dict[str, Union[str, int]]] = []

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
    problems_ser: str = "assets/problems.json"
    ranking_ser: str = "assets/rankings.json"
    points_ser: str = "assets/points.json"
    if os.path.exists(problems_ser):
        with open(problems_ser, "r") as f:
            problem_entries = json.load(f)
            print("Loaded problem entries, length: ", len(problem_entries))
    else:
        problem_entries = {}
        do_extra_git_searches = True
    if os.path.exists(ranking_ser):
        with open(ranking_ser, "r") as f:
            user_rankings = json.load(f)
    else:
        user_rankings = []
    if os.path.exists(points_ser):
        with open(points_ser, "r") as f:
            user_points = json.load(f)
    else:
        user_points = []
else:
    problems_ser: str = "assets/problems.pkl"
    ranking_ser: str = "assets/rankings.pkl"
    if os.path.exists(problems_ser):
        with open(problems_ser, "rb") as f:
            problem_entries = pickle.load(f)
    else:
        problem_entries = {}
        do_extra_git_searches = True
    if os.path.exists(ranking_ser):
        with open(ranking_ser, "rb") as f:
            user_rankings = pickle.load(f)
    else:
        user_rankings = []

repo_path: str = ""  # or the actual path to your repo
repo = git.Repo(repo_path)

with Progress(
    SpinnerColumn(),
    TextColumn("[progress.description]{task.description}"),
    BarColumn(),
    TimeElapsedColumn(),
) as progress:
    # 1) Main parse task
    parse_task = progress.add_task("[cyan]Parsing Files...", total=1)
    # 1a) Subtask for individual files
    parse_subtask = progress.add_task("[cyan]Processing individual files...", total=0)

    # 2) User ranking
    user_task = progress.add_task("[green]Fetching user data...", total=1)

    # 3) Dates
    dates_task = progress.add_task("[yellow]Processing dates...", total=1)
    # 3a) Subtask for problem dates
    dates_subtask = progress.add_task("[yellow]Processing problem dates...", total=0)

    # 4) Table
    table_task = progress.add_task("[magenta]Generating table...", total=1)

    # 5) Writing
    write_task = progress.add_task("[blue]Writing files...", total=1)

    # Now run your logic, updating the tasks in that same order
    # ...


    # --- 3) Parse each solution file concurrently ---
    # First, gather all file paths
    file_paths = []
    for root, dirs, files in os.walk("."):
        for file in files:
            file_paths.append(os.path.join(root, file))

    total_files = len(file_paths)
    # Adjust the parse_subtask total to match the number of files
    progress.update(parse_subtask, total=total_files)

    # Define a wrapper for processing a single file.
    # Note: Modify parse_solution_file to work independently and return results.
    def process_file(file_path: str):
        # Each call returns a tuple: (dict of new problem entries, revisit_count for this file)
        return parse_solution_file(file_path, {}, do_extra_git_searches, scan_todo, repo_path)

    results = []
    with concurrent.futures.ThreadPoolExecutor() as executor:
        futures = {executor.submit(process_file, fp): fp for fp in file_paths}
        for future in concurrent.futures.as_completed(futures):
            try:
                new_entries, new_revisit = future.result()
                results.append((new_entries, new_revisit))
            except Exception as e:
                progress.log(f"Error processing {futures[future]}: {e}")
            progress.advance(parse_subtask)
    # Merge all results into the main dictionary
    for new_entries, new_revisit in results:
        problem_entries.update(new_entries)
        revisit_count += new_revisit
    progress.update(parse_task, completed=1)

    # --- 4) Optionally fetch user ranking ---
    progress.update(user_task, description="[green]Fetching user ranking...")
    username = "liam-wirth"
    user_info = fetch_user_info(username)
    current_ranking = user_info.get("ranking")
    from readme.api_testing import fetch_user_points

    current_points = fetch_user_points()
    if current_ranking is not None:
        if not user_rankings or user_rankings[-1].get("ranking") != current_ranking:
            user_rankings.append({
                "timestamp": datetime.datetime.now().isoformat(),
                "ranking": current_ranking,
            })
    if current_points > 0:
        if not user_points or user_points[-1].get("points") != current_points:
            user_points.append({
                "timestamp": datetime.datetime.now().isoformat(),
                "points": current_points,
            })
    progress.update(user_task, completed=1)

    # --- 5) Attach slug & fetch solve dates if needed ---
    total_entries = len(problem_entries)
    progress.update(dates_subtask, total=total_entries)
    for prob_id, entry in problem_entries.items():
        # Make sure ID is in filtered_probs (i.e. has a slug)
        if prob_id in filtered_probs:
            entry["slug"] = filtered_probs[prob_id]["question__title_slug"]
            # If date missing or equals float('inf'), attempt fetch
            if not entry.get("date") or entry["date"] == float("inf"):
                slug = entry["slug"]
                solve_date = fetch_first_solve_date(username, slug)
                if solve_date:
                    entry["date"] = solve_date.timestamp()
                    progress.log(f"Fetched official solve date for {prob_id} (slug={slug})")
                else:
                    progress.log(f"No official solve date for {prob_id} (slug={slug})")
        else:
            progress.log(f"[WARNING] Problem ID {prob_id} not found in filtered_probs.pkl")
        progress.advance(dates_subtask)
    update_dates(problem_entries, username, FORCE_UPDATE_DATE)
    progress.update(dates_task, completed=1)

    # --- 6) Sort & create the markdown table ---
    progress.update(table_task, description="[magenta]Generating markdown table...")
    sorted_problem_entries = sorted(problem_entries.items(), key=lambda x: int(x[0]))
    markdown_table, revisit_count = create_markdown_table(sorted_problem_entries)
    progress.update(table_task, completed=1)

    # --- 7) Update the README.md with the newly generated table ---
    progress.update(write_task, description="[blue]Writing files...")
    with open("README.md", "r") as file:
        content: List[str] = file.readlines()

    start_index: int = 0
    for index, line in enumerate(content):
        if "List of problems solved".lower() in line.lower():
            start_index = index + 1
            break
    # progress.log(start_index)

    # Count solutions per language (for pie chart)
    language_counts = {}
    for entry in problem_entries.values():
        for lang in entry["languages"]:
            language_counts[lang] = language_counts.get(lang, 0) + 1

    mermaid_chart = "```mermaid\npie title Problems Solved by Language\n"
    for lang, count in language_counts.items():
        mermaid_chart += f'    "{lang}": {count}\n'
    mermaid_chart += "```\n"
    # progress.log(mermaid_chart)

    subprocess.run(["python", "assets/graphrank.py"])
    if start_index != 0:
        before_table: str = "".join(content[:start_index])
        with open("README.md", "w") as file:
            file.write(before_table)
            file.write("# Chart:\n")
            file.write(mermaid_chart)
            file.write("\n![Ranking Graph](assets/rankings_plot.png)\n\n")
            file.write('### Problems Marked "Revisit": ')
            file.write(str(revisit_count))
            file.write("\n")
            file.write(markdown_table)
    # --- 8) Serialize everything back out ---
    assets_dir = os.path.dirname(problems_ser)
    os.makedirs(assets_dir, exist_ok=True)

    if use_json:
        with open(problems_ser, "w") as f:
            json.dump(problem_entries, f)
        with open(ranking_ser, "w") as f:
            json.dump(user_rankings, f)
        with open(points_ser, "w") as f:
            json.dump(user_points, f)
    else:
        with open(problems_ser, "wb") as f:
            pickle.dump(problem_entries, f)
        with open(ranking_ser, "wb") as f:
            pickle.dump(user_rankings, f)
        with open(points_ser, "wb") as f:
            pickle.dump(user_points, f)
    progress.update(write_task, completed=1)

