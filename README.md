# Nagios plugin for memory

Simple nagios plugin to check memory usage in percent.

## Build 
```
$ cargo build --release
```

## Usage 
```
$ ./check_local_memory_usage -w 80 -c 80
```
