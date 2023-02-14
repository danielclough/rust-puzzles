#!/bin/bash

cd user-data

echo $(cargo run -- "$1")