#!/bin/sh
#
# This script is used to compile your program on StackClass
#
# This runs before .stackclass/run.sh
#
# Learn more: https://docs.stackclass.dev/challenges/program-interface

set -e # Exit on failure

cargo build --release