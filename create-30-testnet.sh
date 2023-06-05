#!/bin/bash

mkdir -p 30-inscriptions-testnet

parentInscriptionId="3995befab6b08427416bf9442d6877f6057780f31cdca37eb600a12bcf5e9345i0"
offsetInscriptionId="f4429c67523c9437f3db69fe0521f10dec4ae2b3bb64e98a1fd97c53c59803cai0"

for ((i=1; i<=30; i++)); do
    filename="${i}.html"
    filepath="30-inscriptions-testnet/${filename}"
    tokenID=$i

    echo '<body/>
<script>j=1;d=document;b=d.body
class URLSearchParams{get=(k)=>k=="tokenID"?j:0}(async()=>{q="\n"
j=9520+(+await(await fetch("/content/f4429c67523c9437f3db69fe0521f10dec4ae2b3bb64e98a1fd97c53c59803cai0")).text()+j)%300
if(isNaN(j))return;
h=(await(await fetch("/content/3995befab6b08427416bf9442d6877f6057780f31cdca37eb600a12bcf5e9345i0")).text()).split(q)
b.innerHTML=h[2];
z=document.createElement("script")
z.innerHTML=h.slice(4,18).join(q)
b.appendChild(z)})()</script>' > "$filepath"

done

echo "Test files created!"
