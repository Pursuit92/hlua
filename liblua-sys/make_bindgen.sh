#!/bin/bash

cat <(echo '#![allow(non_camel_case_types,non_snake_case)]') \
	<(bindgen -builtins $1) \
	> $2
