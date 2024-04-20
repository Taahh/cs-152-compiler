#!/bin/bash
#don't look at this

cargo build
if [ $? == 0 ];
	then
	cargo run if.tt
	else
	echo "Build failed."
fi


