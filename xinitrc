#!/bin/sh

sxhkd -c ./sxhkdrc && sleep 2 &
exec ./nrwm
