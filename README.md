# Rust Based Shellcode Runner
The aim of this project is to write a basic Rust program that will take a block of shellcode and execute it. 
The reason for this project is to self teach how to manage unsafe memory blocks in Rust programs, alongside learning to program more proficiently in Rust.

## Structure
The project consists of a single main.rs file responsible for downloading a payload from a web server and executing this in a local thread.
main.rs is located in src directory and all dependencies are outlines in the cargo file.
Resources directory contains the calc.bin shellcode file which is run by the loader.

## Notes
- Can further adjust this to gracefully exit after the run.
- Could also explore obfuscation and encryption of sensitive IOCs in payload later down the line.
- Possibly explore executing the same shellcode from remote thread in a different processes memory.


## References
- [Rust Based Shellcode Runner](https://docs.redteamleaders.com/offensive-security/defense-evasion/simple-shellcode-runner-in-rust)