#!/bin/sh
crate="$1"
crate_json_file=".crate_info.json"
#echo "https://crates.io/api/v1/crates/$crate" # clickable link for poking around api data

curl --silent "https://crates.io/api/v1/crates/$crate" > "$crate_json_file"

repo=`jq --raw-output '.crate.repository' "$crate_json_file" | cut --characters 20-`
version=`jq --raw-output '.crate.max_stable_version' "$crate_json_file"`
downloads=`jq --raw-output '.crate.downloads' "$crate_json_file"`
last_version=`jq --raw-output '.crate.downloads' "$crate_json_file"`
description=`jq --raw-output '.crate.description' "$crate_json_file"`

echo "| [$crate](https://crates.io/crates/$crate)	| $last_version	| $version	| $downloads	| [$repo](https://github.com/$repo)	| $description	|"

# cleanup
rm "$crate_json_file"
