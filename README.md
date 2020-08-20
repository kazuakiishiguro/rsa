# Rsa

This repo is my personal practice of implementing RSA encryption / decryption scheme written in Rust.

## Public and Private key generation
1. Randomly generate two primes <img src="https://latex.codecogs.com/svg.latex?p" title="p" />, <img src="https://latex.codecogs.com/svg.latex?q" title="q" />.
2. Calculate <img src="https://latex.codecogs.com/svg.latex?n&space;=&space;p&space;*&space;q" title="n = p * q" />.
3. Pick the integer <img src="https://latex.codecogs.com/svg.latex?d" title="d" /> which satisfies: <img src="https://latex.codecogs.com/svg.latex?gcd(d,&space;(p-1)(q-1))" title="gcd(d, (p-1)(q-1))" />.
4. The integer <img src="https://latex.codecogs.com/svg.latex?e" title="e" /> is finally computed from <img src="https://latex.codecogs.com/svg.latex?p" title="p" />, <img src="https://latex.codecogs.com/svg.latex?q" title="q" /> and <img src="https://latex.codecogs.com/svg.latex?d" title="d" /> to be the "multiplicative inverse" of <img src="https://latex.codecogs.com/svg.latex?d" title="d" />, modulo <img src="https://latex.codecogs.com/svg.latex?(p-1)(q-1)" title="(p-1)(q-1)" />. Thus we will have <img src="https://latex.codecogs.com/svg.latex?e*d&space;\equiv&space;1&space;(mod&space;(p-1)(q-1))" title="e*d \equiv 1 (mod (p-1)(q-1))" />.
5. Finally we will get the key pair of public key <img src="https://latex.codecogs.com/svg.latex?(e,n)" title="(e,n)" />, and private key <img src="https://latex.codecogs.com/svg.latex?(d,n)" title="(d,n)" />.
