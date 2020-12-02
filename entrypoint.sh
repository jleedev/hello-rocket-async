#!/usr/bin/env bash
[ -n "$PORT" ] && export ROCKET_PORT="${PORT}"
exec "$@"
