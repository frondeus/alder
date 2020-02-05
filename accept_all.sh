#!/bin/bash

echo ""
echo ""

for file in $(find . -type f -name "*.new"); do
  ACTUAL="$file"
  EXPECTED="${file%%.new}"

  echo "Accepting: $ACTUAL";
  echo "-----"
  diff -y -N "$EXPECTED" "$ACTUAL" | colordiff
  echo ""
  echo ""
  echo "-----"
  read -p "[A]ccept or [R]reject" -n 1 -r
  echo    # (optional) move to a new line
  if [[ $REPLY =~ ^[Aa]$ ]]
  then
    mv -- "$ACTUAL" "$EXPECTED"
  fi
done
echo "All processed"
