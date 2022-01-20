#!/bin/bash

cargo build --release
export LD_LIBRARY_PATH="$(pwd)/target/release/"
echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
cd src/ 
javac -h . main.java
javac *.java 
java main -Djava.library.path="LD_LIBRARY_PATH"
cd ../