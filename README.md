Rustworking CLI
==============
Rustworking is a project that takes a few networking operations often used by
system administrators and implements them in Rust. This will allow these
operations to be performed in bulk which can allow for quick and easy testing
of multiple hosts in a network.

This is a CLI that works like any other command line tool for Linux systems.

You can find the core of the Rustworking project [here](https://github.com/timmonfette1/rustworking-core)

Pre-requisites
-------------
This tool makes use of the liboping tool. The installation of this is handled for you,
however it needs the `autoreconf` tool to install.  This tool is installed alongside the
`autoconf` package.

Some operating systems may need this installed (mine did) so make sure to install it
before attempting to compile the tool.<br />
For example: `$ sudo apt-get install autoconf`

Proper Cloning
-------------
This project uses a few git submodules. These modules are nested like such:

  - rustworking-cli uses rustworking-core
  - rustworking-core uses rust-oping (my version)
  - rust-oping uses liboping

If you clone the CLI or the rustworking-core crate, you can recusrively install
all of these submodules with `$ git clone --recursive`

Alternatively, you can go into each nested submodule and run:<br />
`$ git submodule init ; git submodule update`<br />
to make sure the submodules are properly cloned.

How to Compile
------------
Download the code and build it:<br />
`$ cargo build --release`

After that, you can find the executable binary in:<br />
`target/release/rustworking-cli`

You can add that to any directory in your PATH to make it easier to execute.

How to Run
------------
You'll most likely need to run the tool with root permissions;
a lot of operating systems don't like giving regular users the ability to create
sockets to various IPs.

You can see a usage for the tool by simply running:<br />
`$ rustworking-cli -h` or `$ rustworking-cli --help`

The tool will only run if you specify which operation you'd like to run:

  - `$ rustworking-cli -t ping`
  - `$ rustworking-cli -t tcp`
  - `$ rustworking-cli --tool udp`

After that, it'll run the desired operation on whatever IP address(es) you specify.

By default, the tool will use "localhost" as the IP and "80" as the port. See the usage
section below for a detailed explanation of the accepted options for the tool.

Usage
-----------
```
Usage:
    ./target/release/rustworking-cli [OPTIONS]

Test connections to a server or set of servers.

optional arguments:
  -h,--help             show this help message and exit
  -V,--version          Show version
  -v,--verbose          Verbose execution
  -t,--tool TOOL        Network tool to use [ping, tcp, udp]
  -i,--ip IP            IP Address of server
  -p,--port PORT        Port to test connection on
  -s,--subnet SUBNET    Subnet of addresses to test on
  -f,--filepath FILEPATH
                        Path to file of IP addresses
```

Whether or not the tool runs on a file, subnet or single IP Addresses depends on what
you provide to it.

A file has the highest priority (will always use this if provided).<br />
A subnet has medium priority (will run if no file was provided).<br />
A single IP Address has low priority (will run if no file or subnet was provided).

As a final note, when building a file of IP Addresses to test on, put ONE IP per line
and include the port in the line.

Example:<br />
```
1.1.1.1:8080
::1:9090
```
