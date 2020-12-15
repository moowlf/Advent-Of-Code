#!/bin/sh

set -e

CookieJar=".cookies"
CurrentYear="2020"

# Function responsible for enforcing some rules related to the repository.
check_git_repository()
{
    # Check repository current branch
    CurrentBranch=$(git branch --show-current)
    if [ $CurrentBranch != "main" ];then
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

    # Folder without spaces
    FolderName=$CurrentYear/$(printf %02d "$problemIdentifier")
    
    git checkout -b $FolderName

    # Create folders
    InputFolder="$FolderName/inputs"
    mkdir -p $FolderName $InputFolder

    get_input_file "$problemIdentifier" "$InputFolder"
    
    # Create file
    FileContent="
    \n
    // Cpp Includes\n
    \n
    int main(){\n
    }\n
    "
    echo $FileContent > "$FolderName/main.cpp"
}

# Function calls
check_git_repository
read -p "> What's the problem identifier? " problemIdentifier
create_folder_file_in_new_branch $problemIdentifier