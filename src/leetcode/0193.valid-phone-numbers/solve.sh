#!/bin/bash

#/bin/grep -P '^\d{3}-\d{3}-\d{4}$|^\(\d{3}\) \d{3}-\d{4}$' < file.txt

/bin/grep -E '^[0-9]{3}-[0-9]{3}-[0-9]{4}$|^\([0-9]{3}\) [0-9]{3}-[0-9]{4}$' < file.txt
