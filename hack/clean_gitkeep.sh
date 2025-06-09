#!/bin/bash

# Search for .gitkeep files and process them
find . -type f -name ".gitkeep" | while read -r gitkeep_file; do
    dir=$(dirname "$gitkeep_file")
    # Count files in directory excluding .gitkeep
    count=$(find "$dir" -type f ! -name ".gitkeep" | wc -l)
    
    if [ "$count" -gt 0 ]; then
        rm "$gitkeep_file"
        echo "Removed $gitkeep_file: directory $dir has $count other files"
    fi
done
