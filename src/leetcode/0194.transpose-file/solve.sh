#!/bin/bash

solution1() {
  names=()
  ages=()
  while read line; do
    name=$(echo "${line}" | grep -oP '\w+\s')
    age=$(echo "${line}" | grep -oP '\s+\w+')
    names+=("$name")
    ages+=("$age")
  done

  for name in ${names[@]}; do
    echo -n "${name} "
  done
  echo ""

  for age in ${ages[@]}; do
    echo -n "${age} "
  done
  echo ""
}

solution2() {
  awk '{print}' ORS=' '
}

solution3() {
  local columns=$(head -n1 "$1" | wc -w)
  for column_num in $(seq 1 "${columns}"); do
    #cat "$1" | cut -d' ' -f "${column_num}" | tr '\n' ' '
    #cat "$1" | cut -d' ' -f "${column_num}" | awk '{print}' ORS=' ' 
    #cat "$1" | cut -d' ' -f "${column_num}" | paste -s -d ' '
    cat "$1" | cut -d' ' -f "${column_num}" | xargs
  done
}

#solution1 < file.txt
#solution2 < file.txt
solution3 file.txt
solution3 file0.txt
