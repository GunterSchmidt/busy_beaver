# BB_Challenge Library support application

This application code is to test and run stuff in the [[bb_challenge library](https://github.com/GunterSchmidt/bb_challenge)].

It contains a bunch of test code, most of it can be disregarded, but may help to identify how the library is used.

Note: Code in test_run_deciders is deprecated and should not be used.

It might be helpful to have both in a workspace with a cargo.toml looking like this:

[workspace]  
resolver = "2"  
members = ["bb_challenge", "busy_beaver"]  
default-members = ["busy_beaver"]  


