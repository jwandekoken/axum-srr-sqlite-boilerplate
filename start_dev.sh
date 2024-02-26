#!/bin/bash

# This script executes the project in dev mode using cargo-watch

cargo watch -i "target" -i ".git" -x run
