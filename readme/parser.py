from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
import os

from readme.utils import commit_date_info, format_problem_name, tagswag

class Language(Enum):
    RUST = "Rust"
    RUBY = "Ruby"
    JAVA = "Java"
    PYTHON = "Python"
    C = "C"
    CPP = "CPP"
    MARKDOWN = "Markdown"
    RACKET = "Racket"
    SQL = "SQL"
    JS = "JavaScript"
    ELIXIR = "Elixir"

@dataclass
class FileConfig:
    extension: str
    language: Optional[Language] = None
    is_writeup: bool = False
    parse_format: str = "underscore"  # underscore or dot

SUPPORTED_FILES: Dict[str, FileConfig] = {
    ".rs": FileConfig(".rs", Language.RUST, parse_format="underscore"),
    ".rb": FileConfig(".rb", Language.RUBY, parse_format="dot"),
    ".java": FileConfig(".java", Language.JAVA, parse_format="dot"),
    ".py": FileConfig(".py", Language.PYTHON, parse_format="underscore"),
    ".md": FileConfig(".md", is_writeup=True, parse_format="dot"),
    ".c": FileConfig(".c", Language.C, parse_format="dot"),
    ".cpp": FileConfig(".cpp", Language.CPP, parse_format="dot"),
    ".rkt": FileConfig(".rkt", Language.RACKET, parse_format="underscore"),
    ".sql": FileConfig(".sql", Language.SQL, parse_format="underscore"),
    ".js": FileConfig(".js", Language.JS, parse_format="underscore"),
    ".ex": FileConfig(".ex", Language.ELIXIR, parse_format="underscore")
}

SKIP_FILES = {"lib.rs", "main.rs", "update_readme.py", "README.md", "update_dates.py", "utils.py", "api_testing.py", "api_probs_parse.py", "parser.py"}

def get_file_config(file_path: str) -> Optional[FileConfig]:
    """Get file configuration based on extension"""
    ext = Path(file_path).suffix
    return SUPPORTED_FILES.get(ext)

def parse_problem_number(file_name: str, parse_format: str) -> Optional[int]:
    """Extract problem number from filename based on parse format"""
    try:
        match parse_format:
            case "underscore":
                return int(file_name.split("_")[-1].split(".")[0])
            case "dot":
                return int(file_name.split(".")[0])
    except ValueError:
        # print(f"Skipping file due to invalid problem number: {file_name}")
        return None

def parse_problem_name(file_name: str, file_config: FileConfig) -> str:
    """Extract and format problem name from filename"""
    match file_config.parse_format:
        case "underscore":
            parts = file_name.split("_")
            return format_problem_name(" ".join(parts[:-1]))
        case "dot":
            parts = file_name.split(".")
            if file_config.language == Language.JAVA:
                return format_problem_name(parts[1])
            return format_problem_name(" ".join(parts[1:-1]))

def parse_solution_file(
    file_path: str,
    problem_entries: Dict[str, Any],
    do_extra_git_searches: bool,
    scan_todo: bool,
    repo_path: str
) -> Tuple[Dict[str, Any], int]:
    """
    Parses a solution file and extracts problem information.
    """
    file_name = Path(file_path).name
    if file_name in SKIP_FILES:
        return problem_entries, 0

    file_config = get_file_config(file_path)
    if not file_config:
        return problem_entries, 0

    tags: List[str] = []
    if scan_todo:
        with open(file_path, "r") as f:
            if "TODO:" in f.read():
                tags.append("Revisit")

    problem_number = parse_problem_number(file_name, file_config.parse_format)
    if problem_number is None:
        return problem_entries, 0

    problem_name = parse_problem_name(file_name, file_config)
    problem_id = str(problem_number)
    revisit_count = 0

    # Create entry data
    entry_data = {
        "name": problem_name,
        "languages": {},
        "date": float("inf"),
        "writeup": file_config.is_writeup,
        "Tags": tags if tags else None
    }

    if file_config.is_writeup:
        entry_data["writeup_path"] = os.path.join("assets/writeups/", file_name)
    elif file_config.language:
        entry_data["languages"][file_config.language.value] = file_path
        if commit_date := commit_date_info(repo_path, file_path):
            entry_data["date"] = commit_date

    # Update or create problem entry
    if existing := problem_entries.get(problem_id):
        if do_extra_git_searches and existing["date"] is not None:
            if commit_date := commit_date_info(repo_path, file_path):
                if commit_date < float(existing["date"]):
                    existing["date"] = commit_date

        if file_config.language and file_config.language.value not in existing["languages"]:
            existing["languages"][file_config.language.value] = file_path

        if file_config.is_writeup:
            existing.update({
                "writeup": True,
                "writeup_path": entry_data["writeup_path"],
                "Tags": tags if tags else None
            })
            if scan_todo:
                revisit_count = tagswag(problem_entries, revisit_count, tags, problem_id)

        if not existing["name"]:
            existing["name"] = problem_name

    else:
        problem_entries[problem_id] = entry_data
        if tags:
            revisit_count = tagswag(problem_entries, revisit_count, tags, problem_id)

    return problem_entries, revisit_count
