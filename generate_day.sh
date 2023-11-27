#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 {day}"
    exit 1
fi

day=$1
mod_file=day$day.rs
lib_file=src/lib.rs

if [ -f "src/$mod_file" ]; then
    echo "File already exists"
    exit 2
fi

echo "Copying template"
cp template.rs src/$mod_file

sed -i -e "s,{day},$day,g" src/$mod_file

has_mod=$(grep -c "pub mod day${day};" $lib_file)

if [ $has_mod -eq 0 ]; then
    echo "Updating $lib_file"
    echo "pub mod day${day};" >> $lib_file
fi

exit 0