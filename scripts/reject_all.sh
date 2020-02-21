#!/bin/bash

echo ""
echo ""

for file in $(find . -type f -name "*.new"); do
  ACTUAL="$file"

  echo "Rejecting: $ACTUAL";
  rm -- "$ACTUAL"
done
echo "All processed"
