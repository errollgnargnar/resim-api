#!/bin/bash



newaccnt() {
  batch=$(echo $(resim new-account | grep "account\|key" | awk -F": " '{print $2}' | awk 'NF'))
  echo $batch
}

publish() {
  batch=$(resim publish "$1");
  package_id=$(echo "$batch: " | awk -F ": " '{print $2}')
  package_name="$2"
  package=($package_id,"$package_name")
  echo $package
}

reset() {
  resim reset > /dev/null
  batch=$(echo $(resim new-account | grep "account\|key" | awk -F": " '{print $2,$3}' | awk 'NF'))
  echo $batch 
}

showledger() {
  resim show-ledger
}

"$@" # this allows for functions to be called from the command line when running the script. 
