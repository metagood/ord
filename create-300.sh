#!/bin/bash

mkdir -p 300-inscriptions

parentInscriptionId="2dbdf9ebbec6be793fd16ae9b797c7cf968ab2427166aaf390b90b71778266abi0"
offsetInscriptionId="90e7d9261f3812eb02519de31fddc43ba33b0308c143f893bc79fa6d6dcaa8bfi0"

for ((i=1; i<=300; i++)); do
    filename="${i}.html"
    filepath="300-inscriptions/${filename}"
    tokenID=$i

    echo '<body/>
<script>j='$tokenID';d=document;b=d.body
class URLSearchParams{get=(k)=>k==="tokenID"?j:0}(async()=>{q="\n"
j=9520+(+await(await fetch("/content/'$offsetInscriptionId'")).text()+j)%300
h=(await(await fetch("/content/'$parentInscriptionId'")).text()).split(q)
b.innerHTML=h.slice(0,27).join(q)
z=d.createElement("script")
z.innerHTML=`${h[28]};${h.slice(32,36).join(q)}`
b.appendChild(z)
init()})()</script>' > "$filepath"

done

echo "Files created!"
