from gql import gql, Client
from gql.transport.requests import RequestsHTTPTransport
import os
import dotenv
import datetime
from typing import Optional, Dict, Any, List

dotenv.load_dotenv()

leetcode_api_key = os.getenv("LEETCODE_API_KEY")
leetcode_session = f"LEETCODE_SESSION={leetcode_api_key or ''}"

headers = {
    "Cookie": leetcode_session
}

transp = RequestsHTTPTransport(url="https://leetcode.com/graphql", headers=headers, use_json=True)

client = Client(
    transport=transp,
    fetch_schema_from_transport=False,
)

def fetch_first_solve_date(username: str, title_slug: str) -> Optional[datetime.datetime]:
    """
    Fetches the date a user first solved a specific problem on LeetCode.

    Args:
        username: The LeetCode username.
        title_slug: The problem's title slug.

    Returns:
        The datetime object representing the first solve date, or None if not found.
    """
    query = gql(
        """
        query userFirstSolveDate($username: String!, $titleSlug: String!) {
            matchedUser(username: $username) {
                username
            }
            questionSubmissionList(
                limit: 1000
                offset: 0
                questionSlug: $titleSlug
                status: 10
            ) {
                submissions {
                    id
                    timestamp
                    status
                    titleSlug
                }
            }
        }
        """
    )

    params = {
        "username": username,
        "titleSlug": title_slug
    }

    result = client.execute(query, variable_values=params)

    first_solve_date = None
    if result and result.get("questionSubmissionList") and result["questionSubmissionList"].get("submissions"):
        submissions = result["questionSubmissionList"]["submissions"]
        if submissions:
            submissions_with_dates = []
            for sub in submissions:
                try:
                    timestamp_int = int(sub["timestamp"])
                    submission_date = datetime.datetime.fromtimestamp(timestamp_int)
                    submissions_with_dates.append((submission_date, sub))
                except (ValueError, TypeError) as e:
                    print(f"Error converting timestamp: {sub['timestamp']}, Error: {e}")
                    continue
            if submissions_with_dates:
                submissions_with_dates.sort(key=lambda x: x[0])
                first_solve_date = submissions_with_dates[0][0]

    return first_solve_date


def fetch_problem_info(title_slug: str) -> Dict[str, Any]:
    """
    Fetches problem difficulty and topic tags for a specific problem.

    Args:
        title_slug: The problem's title slug.

    Returns:
         A dictionary containing the problem difficulty and topic tags.
    """
    query = gql(
        """
        query problemInfo($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                difficulty
                topicTags {
                    name
                    slug
                }
            }
        }
        """
    )

    params = {
        "titleSlug": title_slug
    }

    result = client.execute(query, variable_values=params)
    
    problem_info = {}
    if result and result.get("question"):
        problem_info["difficulty"] = result["question"].get("difficulty")
        problem_info["topicTags"] = result["question"].get("topicTags", [])
    return problem_info

def fetch_user_info(username: str) -> Dict[str, Any]:
    """
    Fetches user ranking and other profile information.

    Args:
        username: The LeetCode username.

    Returns:
        A dictionary containing the user's ranking.
    """
    query = gql(
        """
        query userInfo($username: String!) {
            matchedUser(username: $username) {
                profile {
                    ranking
                }
            }
        }
        """
    )

    params = {
        "username": username
    }

    result = client.execute(query, variable_values=params)
    
    user_info = {}
    if result and result.get("matchedUser") and result["matchedUser"].get("profile"):
        user_info["ranking"] = result["matchedUser"]["profile"].get("ranking")
    return user_info

if __name__ == "__main__":
    username = "liam-wirth"  # Replace with your LeetCode username
    title_slug = "two-sum"  # Replace with your LeetCode problem slug

    first_solve = fetch_first_solve_date(username, title_slug)
    if first_solve:
        print(f"First Solve Date for {title_slug}: {first_solve}")
    else:
        print(f"No accepted submissions found for {title_slug} by {username}")

    problem_info = fetch_problem_info(title_slug)
    print(f"Problem Info for {title_slug}: {problem_info}")

    user_info = fetch_user_info(username)
    print(f"User Info for {username}: {user_info}")

