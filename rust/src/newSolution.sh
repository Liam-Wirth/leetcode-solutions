#!/bin/bash
# Ask user for the problem name
echo "Tell me the problem name:"
read problem_name

# Remove whitespace and replace spaces with underscores
problem_name=$(echo "$problem_name" | tr -d '[:space:]' | tr ' ' '_')

# Ask user for the problem number
echo "Tell me the problem number:"
read problem_number

# Construct the filename
filename="${problem_name}_${problem_number}.rs"

# Create the file
touch "./$filename"

# Echo "mod filename" to lib.rs
echo "mod $filename" >> lib.rs

echo "File created: ./$filename"
echo "mod $filename added to lib.rs"

