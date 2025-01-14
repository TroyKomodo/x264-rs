#!/bin/bash

./configure --disable-asm --disable-avs --disable-swscale --disable-lavf --disable-ffms --disable-gpac --disable-lsmash --bit-depth=8

bear -- make -j $(nproc)
