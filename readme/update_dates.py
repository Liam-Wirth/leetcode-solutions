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

# leetcode/readme/date_updater.py
import datetime
import math
from typing import Dict, Any

from readme.api_testing import fetch_first_solve_date

def update_dates(
    problem_entries: Dict[str, Any], 
    username: str, 
    force_update: bool = False
) -> None:
    """
    If force_update=False:
      - Only re-check problems with date == None, float('inf'), or your known placeholders (optional).
    If force_update=True:
      - Re-check *all* problems that have a slug from the LeetCode API.
        If the fetched date differs from the currently stored date (beyond microsecond rounding),
        we update it.
    """
    for prob_id, entry in problem_entries.items():
        slug = entry.get("slug")
        if not slug:
            # No slug => can't do an API lookup
            continue

        current_ts = entry.get("date")  # This might be None, float('inf'), or a float.

        if force_update:
            # Force re-check all
            new_dt = fetch_first_solve_date(username, slug)
            if new_dt is not None:
                # Convert to an *integer* to avoid float microsecond issues
                new_ts = int(new_dt.timestamp())
                if current_ts is None or math.isclose(float(current_ts), new_ts, abs_tol=0.999) is False:
                    old_ts = current_ts  # capture old before overwriting
                    entry["date"] = new_ts
                    old_str = ("None" if old_ts is None 
                               else str(datetime.datetime.fromtimestamp(float(old_ts))))
                    print(f"[update_dates] [FORCE] Problem {prob_id}: date updated from {old_str} to {new_dt}")
                # else: old and new are effectively the same => do nothing
            else:
                print(f"[update_dates] [FORCE] No accepted submission found for {prob_id}")
        else:
            # Not force-update => only fix "missing" or obviously invalid
            if current_ts is None or current_ts == float("inf"):
                new_dt = fetch_first_solve_date(username, slug)
                if new_dt is not None:
                    entry["date"] = int(new_dt.timestamp())
                    print(f"[update_dates] Problem {prob_id}: date set to {new_dt}")
                else:
                    print(f"[update_dates] No accepted submission found for {prob_id}")
            # else: date is presumably valid => skip

