#!/bin/bash
for i in {0..64}
do
	tr -dc A-Za-z0-9 </dev/urandom | head -c 128 | openssl rsautl -encrypt -pubin -inkey key.pub &> "messages/$i.txt"
done
