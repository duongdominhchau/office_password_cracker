# MS Office Password Cracker

## About

This repo contains a generic bruteforce implementation and a hash function used by MS Office. There is also example code
to crack a specific hash used by MS Excel.

## TODO

- Make it safe, there is no validation right now
- Benchmark
- Parallelize it. Maybe we can return a reference to string instead of a String, that way we can reuse memory space and
    avoid the costly drop()
