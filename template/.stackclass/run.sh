#!/bin/sh
#
# This script is used to run your program on StackClass
#
# This runs after .stackclass/compile.sh
#
# Learn more: https://docs.stackclass.dev/challenges/program-interface

set -e # Exit on failure

exec target/release/stackclass-solana-swap-program "$@"
