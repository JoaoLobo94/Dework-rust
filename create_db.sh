#!/bin/sh

cargo watch -x 'run --bin dework'

diesel setup

diesel migration run

echo "All tables migrated"