#!/bin/bash


solution1() {
  for line_num in $(seq 1 9); do
    read line
  done
  read line
  echo "${line}"
}

solution2() {
  line_number=0
  while read line; do
    line_number=$(( $line_number + 1))
    if [[ line_number -eq 10 ]]; then
      echo "${line}"
      break
    fi
  done
}

solution3() {
  sed -n '10p' $1
}

#solution1 <file.txt
#solution2 < file.txt
solution3 file.txt
