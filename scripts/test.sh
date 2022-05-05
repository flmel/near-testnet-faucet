#!/bin/bash

export out=$(echo "done deploying 1234" | grep -i done | awk '{print $3}')
out2=$(pfiles $1 2> /dev/null | grep peername)
echo $out

