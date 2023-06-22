#!/bin/bash

file="$1"  # Path to the addresses file

# Check if the file exists
if [ ! -f "$file" ]; then
  echo "Addresses file not found: $file"
  exit 1
fi

dir="12-inscriptions-testnet"
mkdir -p $dir

inscriptions_dir="$dir/inscriptions"
mkdir -p $inscriptions_dir

addresses_dir="$dir/addresses"
mkdir -p $addresses_dir

# Read the file line by line
while IFS= read -r line; do
  ((count++))

  filename="${count}.address"
  filepath="${addresses_dir}/${filename}"
  echo "$line" > "$filepath"
done < "$file"

parentInscriptionId="3995befab6b08427416bf9442d6877f6057780f31cdca37eb600a12bcf5e9345i0"
offsetInscriptionId="f4429c67523c9437f3db69fe0521f10dec4ae2b3bb64e98a1fd97c53c59803cai0"

for ((i=1; i<=12; i++)); do
    filename="${i}.html"
    filepath="${inscriptions_dir}/${filename}"
    tokenID=$i

    echo '<body/><script>j='$tokenID';d=document;b=d.body
class URLSearchParams{get=k=>k=="tokenID"?j:0}(async()=>{q="\n"
j=9520+(+await(await fetch("/content/'$offsetInscriptionId'")).text()+j)%300
if(isNaN(j))return;
h=(await(await fetch("/content/'$parentInscriptionId'")).text()).split(q)
b.innerHTML=h[2];
z=document.createElement("script")
z.innerHTML=h.slice(4,18).join(q)
b.appendChild(z)})()</script>' > "$filepath"

done

echo "Test files created!"
