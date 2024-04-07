#!/bin/bash
# Function to format problem name
format_problem_name() {
    name=$(echo "$1" | tr ' ' '_')
    echo "$name"
}
clean_response() {
    response=$(echo "$1" | tr -d '[:space:]' | tr -d '[:punct:]' | tr '[:upper:]' '[:lower:]')
    echo "$response"
}

# Ask user for the problem name
echo "problem name:"
read problem_name
formatted_name=$(format_problem_name "$problem_name")

# Ask user for the problem number
echo "problem number:"
read problem_number
cleaned_number=$(clean_response "$problem_number")

# Construct the filename
filename="${problem_name// /_}_${cleaned_number}.rs"
modname="${problem_name// /_}_${cleaned_number};"

# Create the file
touch "./$filename"

# Echo "mod filename" to lib.rs
echo "mod $modname" >> lib.rs

echo "File created: ./$filename"
echo "mod $filename added to lib.rs"

echo "Open $filename in Vim? (y\n)"
read response
cleaned_response=$(clean_response "$response")

echo "use crate::Solution;" >> "./$filename"

# Check if the response is affirmative
if [[ $cleaned_response == "y" || $cleaned_response == "yes" ]]; then
   nvim $filename # Open file in nvim editor (you can replace nvim with your preferred editor)
fi

