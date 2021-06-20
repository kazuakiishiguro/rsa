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

assert 0 1
assert 1 3
assert 1 187963

echo OK
