#!/bin/sh

dbmate wait

dbmate up

echo "Done"

exec "$@"