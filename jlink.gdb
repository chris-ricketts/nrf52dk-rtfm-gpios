target extended-remote :2331
monitor flash breakpoints 1
# allow hprints to show up in gdb
monitor semihosting enable
monitor semihosting IOClient 3

monitor reset
load

break main
continue
