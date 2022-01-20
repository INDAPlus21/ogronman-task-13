cd src/ 
javac -h . main.java
javac *.java 
java main -Djava.library.path="LD_LIBRARY_PATH"
cd ../