#!/usr/bin/env bash

original_paranoid=`cat /proc/sys/kernel/perf_event_paranoid`
echo Original value in /proc/sys/kernel/perf_event_paranoid=$original_paranoid
echo Changing it to -1 to allow profiling
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid > /dev/null
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph
echo Returning original value of $original_paranoid to /proc/sys/kernel/perf_event_paranoid
echo $original_paranoid | sudo tee /proc/sys/kernel/perf_event_paranoid > /dev/null
sudo -k
echo This terminal no longer has sudo privileges