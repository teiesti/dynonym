#!/bin/bash

FOLDER="."
FILES='.+\.\(md\|rs\|\sh\|toml\|txt\|yml\)'
COLS=100

# Exit script on first error
set -o errexit -o nounset

# Explain what we do
echo -n ">>> Seaching for lines exceeding the length limit..."

# Check any text file
FOUND=0
for FILE in $(find $FOLDER -regex $FILES); do
    # Ignore files that are ignored by git
    git check-ignore -q $FILE && continue

    # Seach for lines that are too long
    if [ $(wc -L $FILE | cut -d" " -f1) -gt $COLS ]; then
        [ $FOUND == 0 ] && echo -e "\tError."
        echo -e "Found: $FILE"
        FOUND=1
    fi
done
test $FOUND == 0
echo -e "\tDone."
