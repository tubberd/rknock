# rknock

[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

`rknock` is a simple port knocking client written in Rust.  
It is suitable for use with the server-side `knockd` daemon.  
At the moment, it wraps `nmap`, and as such, `nmap` is required  
to run this program.

This is due to constraints of Rusts networking ecosystem, most notably  
the inability to disable TCP retransmissions.  
I plan to eventually port this to use plain unix sockets.