#!/bin/bash
mkdir r1
cd r1
mkdir -p a b c d p
touch a/a.txt
echo "pipo" > a/a.txt
touch b/b.txt
touch d/m.txt
#-----------------------
cd ..
mkdir r2
cd r2
mkdir -p a b c d q
touch a/a.txt
echo "pipo" > a/a.txt
touch c/c.txt
touch d/n.txt