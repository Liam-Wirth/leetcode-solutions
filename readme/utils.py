import os
import git
import datetime
from typing import List, Optional, Dict, Any

def format_problem_name(name: str) -> str:
    """Formats a problem name by replacing hyphens with spaces and capitalizing words."""
    return name.replace("-", " ").title()


def commit_date_info(repo_path: str, file_path: str, debug: bool = False) -> Optional[float]:
    """
    Retrieves the earliest commit date for a file from git history.
    
    Args:
        repo_path: Path to git repository
        file_path: Path to file to check
        debug: Whether to print debug information
        
    Returns:
        Timestamp of earliest commit or None
    """
    repo = git.Repo(repo_path)
    if os.path.basename(file_path) != "README.md":
        commits = list(repo.iter_commits(paths=file_path, reverse=True))
        date: float = float("inf") 
        pretty_date: datetime.datetime | None = None
        preferred_commit: git.Commit | None = None
        
        if debug:
            print(f"Filename {file_path}, Commits found: {len(commits)}")
            
        for commit in commits:
            tmp = commit.authored_datetime.timestamp()
            if tmp < date:
                date = tmp
                pretty_date = commit.authored_datetime 
                preferred_commit = commit
                
        if preferred_commit is not None and pretty_date is not None:
            if debug:
                print(f"Filename: {file_path}  Date: {pretty_date}, Commit Message {preferred_commit.message}")
            if date < float("inf"):
                return date
                
        return datetime.datetime.now().timestamp()
        
    return None


def tagswag(problem_entries: Dict[str, Any], revisit_count: int, tags: List[str], probnum: str) -> int:
    """Adds tags to a problem entry and updates the revisit count."""
    if problem_entries[probnum]["Tags"] is not None:
        if len(tags) >= 1:
            for t in tags:
                print(f"Tag found! {t}")
                if t not in problem_entries[probnum]["Tags"]:  # type: ignore
                    problem_entries[probnum]["Tags"].append(t)  # type: ignore
                    revisit_count += 1
    return revisit_count


def create_markdown_table(sorted_problem_entries: List[Any]) -> str:
    """Generates a markdown table from the problem entries."""
    markdown_table: str = (
        "| Problem Number | Problem Name | Language | Estimated Solved Date| WriteSolution? |\n"
        "|--------------|----------------|---------|-----------------------|----------------|\n"
    )
    revisit_count = 0

    for problem_number, entry in sorted_problem_entries:
        # If date is None or inf, we assume "No accepted submission found on API"
        if not entry["date"] or entry["date"] == float("inf"):
            datestring = "No accepted submission found on API"
        else:
            datestring = datetime.datetime.fromtimestamp(float(entry["date"])).strftime(
                "%B %d, %Y"
            )

        # Increase revisit_count if it has any tags (like "Revisit")
        if entry["Tags"] is not None:
            revisit_count += 1

        # List out languages with links
        languages: str = ""
        if entry["languages"]:
            for language in entry["languages"]:  # type: ignore
                temp = f"[{language}]({entry['languages'][language]})"
                languages += temp + ", "

        # Show a link if there's a writeup
        if entry.get("writeup"):
            markdown_table += (
                f"| {problem_number} | {entry['name']} | {languages} | "
                f"{datestring} | [Yes]({entry['writeup_path']})|\n"
            )
        else:
            markdown_table += (
                f"| {problem_number} | {entry['name']} | {languages} | "
                f"{datestring} | No |\n"
            )

    return markdown_table, revisit_count

