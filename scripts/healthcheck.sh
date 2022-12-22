#!/bin/bash
set -e
# Read the timestamp from the file
timestamp=$(cat /tmp/last_reading.txt)

# Calculate the difference between the timestamp and the current time
difference=$(( $(date +%s) - $timestamp ))

# Check if the difference is greater than 5 minutes (300 seconds)
if [ $difference -gt 300 ]
then
    # The difference is greater than 5 minutes
    exit 10
else
    # The difference is less than or equal to 5 minutes
    exit 0
fi
