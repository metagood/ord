#!/bin/bash

mkdir -p 300-inscriptions

for ((i=1; i<=300; i++)); do
    filename="${i}.json"
    filepath="300-inscriptions/${filename}"
    tokenID=$i

    echo '{
   "use_p": 1,
   "params": ["tokenID='$tokenID'"]
}' > "$filepath"

done

echo "Files created!"
