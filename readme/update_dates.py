# leetcode/readme/date_updater.py
import datetime
from typing import Dict, Any
from readme.api_testing import fetch_first_solve_date

PLACEHOLDER_DATES = {
    # "January 16, 2023" example
    datetime.datetime(2023, 1, 16),
    datetime.datetime(2024, 1, 25),
    datetime.datetime(2024, 1, 8),
    datetime.datetime(2024, 1, 29),
    # Add more if you want
}

def is_placeholder_date(ts: float) -> bool:
    """
    Returns True if the timestamp corresponds to any known placeholder date 
    in PLACEHOLDER_DATES. Otherwise, False.
    """
    dt = datetime.datetime.fromtimestamp(ts)
    for ph in PLACEHOLDER_DATES:
        if (dt.year == ph.year and dt.month == ph.month and dt.day == ph.day):
            return True
    return False

def update_dates(
    problem_entries: Dict[str, Any], 
    username: str, 
    force_update: bool = False
) -> None:
    """
    If force_update = False:
        - Re-check problems that:
          1) Have date == None or float('inf'), or
          2) Date is a known placeholder (like January 16, 2023).
    If force_update = True:
        - Re-check all problems that have a slug.
          If the new date from the API is different from the old, update it.
    """
    for prob_id, entry in problem_entries.items():
        slug = entry.get("slug")
        if not slug:
            # No slug => can't do an API lookup
            continue

        current_date = entry.get("date")

        # Force update => check every problem with a slug
        if force_update:
            solve_dt = fetch_first_solve_date(username, slug)
            if solve_dt is not None:
                new_ts = solve_dt.timestamp()
                # If we either don't have a date or the new date differs
                if current_date is None or float(current_date) != new_ts:
                    old_str = (
                        "None" 
                        if current_date is None 
                        else str(datetime.datetime.fromtimestamp(float(current_date)))
                    )
                    entry["date"] = new_ts
                    print(
                        f"[update_dates] [FORCE] Updated date for problem {prob_id}"
                        f" from {old_str} to {solve_dt}"
                    )
                else:
                    # The new date == old date, or we choose not to update if identical
                    pass  
            else:
                print(f"[update_dates] [FORCE] No accepted submission found for {prob_id}")

        else:
            # Not force-updating => only fix missing or placeholders
            if current_date is None or current_date == float("inf") or (
                isinstance(current_date, (int, float)) and is_placeholder_date(float(current_date))
            ):
                solve_dt = fetch_first_solve_date(username, slug)
                if solve_dt is not None:
                    entry["date"] = solve_dt.timestamp()
                    print(
                        f"[update_dates] Updated date for problem {prob_id} to {solve_dt}"
                    )
                else:
                    print(f"[update_dates] No accepted submission found for {prob_id}")
            # else: date looks valid and not placeholder => do nothing

