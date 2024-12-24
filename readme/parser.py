import os
from typing import List, Dict, Any, Tuple
from readme.utils import format_problem_name, commit_date_info, tagswag

def parse_solution_file(
    file_path: str,
    problem_entries: Dict[str, Any],
    do_extra_git_searches: bool,
    scan_todo: bool,
    repo_path: str
) -> Tuple[Dict[str, Any], int]:
    """Parses a solution file and extracts problem information."""
    problem_number: int = 0
    file_name: str = os.path.basename(file_path)
    problem_name: str = ""
    language: str = ""
    writeup: bool = False
    writeup_path: str = ""
    date: float | None = None
    tags: List[str] = []
    revisit_count = 0

    if file_name.endswith((".rs", ".rb", ".java", ".py", ".md", ".c", ".cpp")):
        if file_name in ("lib.rs", "main.rs", "update_readme.py"):
            return problem_entries, revisit_count
        
        if scan_todo:
            with open(file_path, "r") as f:
                content = f.read()
                if "TODO:" in content:
                    tags.append("Revisit")

        if file_name.endswith(".rs"):
            parts: List[str] = file_name.split("_")
            problem_name = " ".join(parts[:-1])
            problem_name = format_problem_name(problem_name)
            try:
                problem_number = int(parts[-1].split(".")[0])
            except ValueError:
                print(f"Skipping file due to invalid problem number: {file_name}")
                return problem_entries, revisit_count
            language = "Rust"

        elif file_name.endswith(".rb"):
            parts = file_name.split(".")
            try:
                problem_number = int(parts[0])
            except ValueError:
                print(f"Skipping file due to invalid problem number: {file_name}")
                return problem_entries, revisit_count
            problem_name = " ".join(parts[1:-1])
            problem_name = format_problem_name(problem_name)
            language = "Ruby"

        elif file_name.endswith(".java"):
            try:
                problem_number = int(file_name.split(".")[0])
            except ValueError:
                print(f"Skipping file due to invalid problem number: {file_name}")
                return problem_entries, revisit_count
            parts = file_name.split(".")[1]
            parts = format_problem_name(parts)
            problem_name = format_problem_name(parts)
            language = "Java"

        elif file_name.endswith(".py"):
            parts = file_name.split("_")
            problem_name = " ".join(parts[:-1])
            problem_name = format_problem_name(problem_name)
            try:
                problem_number = int(parts[-1].split(".")[0])
            except ValueError:
                print(f"Skipping file due to invalid problem number: {file_name}")
                return problem_entries, revisit_count
            language = "Python"

        elif file_name.endswith(".md") and file_name != "README.md":
            try:
                problem_number = int(file_name.split(".")[0])
            except ValueError:
                print(f"Skipping file due to invalid problem number: {file_name}")
                return problem_entries, revisit_count
            writeup = True
            writeup_path = os.path.join("assewriteu", file_name)

        elif file_name.endswith(".c"):
            parts = file_name.split(".")
            try:
                problem_number = int(parts[0])
            except ValueError:
                print(f"Skipping file due to invalid problem number: {file_name}")
                return problem_entries, revisit_count
            problem_name = " ".join(parts[1:-1])
            problem_name = format_problem_name(problem_name)
            language = "C"

        elif file_name.endswith(".cpp"):
            parts = file_name.split(".")
            try:
                problem_number = int(parts[0])
            except ValueError:
                print(f"Skipping file due to invalid problem number: {file_name}")
                return problem_entries, revisit_count
            problem_name = " ".join(parts[1:-1])
            problem_name = format_problem_name(problem_name)
            language = "CPP"


        if str(problem_number) in problem_entries:
            if do_extra_git_searches:
                if problem_entries[str(problem_number)]["date"] is not None:
                    tmp = commit_date_info(repo_path, file_path)
                    if tmp is not None:
                        if tmp < float(problem_entries[str(problem_number)]["date"]):  # type: ignore
                            problem_entries[str(problem_number)]["date"] = tmp

            if language not in problem_entries[str(problem_number)]["languages"] and language != "":  # type: ignore
                problem_entries[str(problem_number)]["languages"][language] = file_path  # type: ignore

            if writeup:
                if scan_todo:
                    revisit_count = tagswag(problem_entries, revisit_count, tags, str(problem_number))
                else:
                    tags = None
                problem_entries[str(problem_number)]["writeup"] = True
                problem_entries[str(problem_number)][
                    "writeup_path"
                ] = writeup_path

            if (
                not problem_entries[str(problem_number)]["name"]
                or problem_entries[str(problem_number)]["name"] == ""
            ):
                problem_entries[str(problem_number)]["name"] = problem_name

            if language not in problem_entries[str(problem_number)]["languages"] and language != "":  # type: ignore
                problem_entries[str(problem_number)]["languages"][language] = file_path  # type: ignore

        elif problem_number != 0:
            if writeup:
                if len(tags) <= 0:
                    tags = None
                problem_entries[str(problem_number)] = {
                    "name": problem_name,
                    "languages": {},
                    "date": float("inf"),
                    "writeup": True,
                    "writeup_path": writeup_path,
                    "Tags": tags,
                }
                print(problem_entries[str(problem_number)]["languages"])
            else:
                with open(file_path, "r") as f:
                    content = f.read()
                    if "TODO:" in content:
                        tags.append("Revisit")
                if len(tags) <= 0 or tags is None:
                    tags = None
                date = commit_date_info(repo_path, file_path)
                temp: Dict[str, str] = {language: file_path} if language else {}
                problem_entries[str(problem_number)] = {
                    "name": problem_name,
                    "languages": temp,
                    "date": date if date is not None else float("inf"),
                    "writeup": False,
                    "Tags": tags,
                }
                revisit_count = tagswag(problem_entries, revisit_count, tags, str(problem_number))
    
    return problem_entries, revisit_count

