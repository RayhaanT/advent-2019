#!/usr/bin/bash
# Make the .in and .out files for a sample input

index=1
daySig=$1
if ((daySig < 10)); then
    daySig="0$1"
fi
while [ -f "samples/day${daySig}s${index}.in" ]
do
    ((index=index+1))
done

touch "samples/day${daySig}s${index}.in" "samples/day${daySig}s${index}.out"

if [ ! -z "$2" ]; then
    echo $2 >> "samples/day${daySig}s${index}.out"
fi
if [ ! -z "$3" ]; then
    echo $3 >> "samples/day${daySig}s${index}.in"
fi
