#!/bin/sh
# Runs the streaming example keeping the stdin file descriptor open.
# Send data to it using
# `cat tests/fixtures/Activity.fit  > /proc/$(pgrep 'streaming$')/fd/0`
(while [ 1 ]; do sleep 1; done) | cargo run --example streaming
