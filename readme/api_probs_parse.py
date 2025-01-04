import json
import pickle

file_path = "../assets/api_probs.json"         # your massive JSON
output_file_path = "../assets/filtered_probs.pkl"
json_output_file_path = "../assets/filtered_probs.json"

with open(file_path, "r") as f:
    data = json.load(f)

filtered_data = {}
for entry in data.get("stat_status_pairs", []):
    stat = entry.get("stat", {})
    # Convert to string so that "88" in problem_entries matches "88" in filtered_data
    frontend_id = str(stat.get("frontend_question_id"))
    if frontend_id:
        filtered_data[frontend_id] = {
            "question_id": stat.get("question_id"),
            "question__title": stat.get("question__title"),
            "question__title_slug": stat.get("question__title_slug"),
        }

with open(output_file_path, "wb") as f:
    pickle.dump(filtered_data, f)

with open(json_output_file_path, "w") as f:
    json.dump(filtered_data, f, indent=4)

print(len(filtered_data))

