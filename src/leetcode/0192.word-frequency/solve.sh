#!/bin/bash

#set -xe

solution1() {
  declare -A dict
  for word in $(cat $1 | xargs); do
    if [[ x${!dict["$word"]} == x"" ]]; then
      dict["$word"]=0
    fi
    dict["${word}"]=$((1 + dict["${word}"]))
  done
  for word in "${!dict[@]}"; do
    echo "${word} ${dict["${word}"]}"
  done
}

solution2() {
  cat words.txt | tr -s ' ' '\n' | sort | uniq -c | sort -nr | awk '{print $2, $1}'
}

#solution1 words.txt
solution2
