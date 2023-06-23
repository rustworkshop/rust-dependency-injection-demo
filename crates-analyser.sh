#!/bin/bash

# emit very roughly formatted markdown table
# ready to be copy-pasted into the readme

echo "| crate	| last_version	| version	| downloads	| repo	| description	|"
echo "| --	| --	| --	| --	| --	| --	|"

for page in {1..10}
do
  # https://stackoverflow.com/questions/2990414/echo-that-outputs-to-stderr/23550347#23550347
  url="https://crates.io/api/v1/crates?q=dependency+injection&page=$page"
  >&2 echo "Fetching $url"
  crates=`curl --silent "$url" | jq --raw-output ".crates[].name"`

  for crate in $crates
  do
    ./crate-analyser.sh "$crate"
  done
done
