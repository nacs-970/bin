#!/bin/sh
case "$1" in 
	1 | y | on) redshift -P -O 2500;;
	0 | n | off) redshift -P -x;;
esac
