#!/bin/bash

##### contract stuff below #####

new_treasury() {
  resim call-function "$1" Treasury new | awk -F "Component: " '{print $2}' | awk 'NF'
}

"$@" # this allows for functions to be called from the command line when running the script. 
