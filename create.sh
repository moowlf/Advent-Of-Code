#!/bin/sh

set -e

CookieJar=".cookies"
CurrentYear="2023"

# Function responsible for enforcing some rules related to the repository.
check_git_repository()
{
    # Check repository current branch
    CurrentBranch=$(git branch --show-current)
    if [ $CurrentBranch != "dev" ];then
        echo "Error. Git repository must be in master."
        exit 1;
    fi

    # Check if repository has zero differences
    NumberOfCommitedFiles=$(git diff --numstat | wc -l)
    if [ $NumberOfCommitedFiles -ne 0 ];then
        echo "Error. Git repository has uncommited changes."
        exit 1;
    fi
}

get_input_file()
{
    currentDay="$1"
    outputFolder="$2"
    curl -b "$CookieJar" "https://adventofcode.com/$CurrentYear/day/$currentDay/input" -o "$outputFolder/input.txt" --silent
}

# Function that creates both the folder and the file of the problem
create_folder_file_in_new_branch()
{
    problemIdentifier=$1
    problemIdentifierWithLeadingZeroes=$(printf %02d "$problemIdentifier")

    # Folder without spaces
    FolderName=$CurrentYear/day_$problemIdentifierWithLeadingZeroes

    git checkout -b $FolderName

    # Create folders
    mkdir $FolderName

    InputFolder="$FolderName/inputs"
    mkdir $InputFolder

    get_input_file "$problemIdentifier" "$InputFolder"
    cp templates/python.py $FolderName/main.py
}

# Function calls
check_git_repository
read -p "> What's the problem identifier? " problemIdentifier
create_folder_file_in_new_branch $problemIdentifier
