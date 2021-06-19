#!/bin/bash

assert() {
    expected="$1"
    input1="$2"
    input2="$3"

    ./rsa "$input1" "$input2" > tmp.txt || exit
    actual=$(cat ./tmp.txt )

    if [ "$actual" = "$expected" ]; then
	echo "$input => $actual"
    else
	echo "$input => $expected expected, but got $actual"
	exit 1
    fi

    rm tmp.txt
}

assert 2 6 4
assert 1 1 0
assert 2 8 6
assert 6 54 24

echo OK
