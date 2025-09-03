set dotenv-load

export RUST_LOG := 'info'

@default:
	just --list


@dev:
	#!/bin/bash
	set -e

	if
		[[ -f /.dockerenv ]] ||
		grep -qE '(docker|containerd|kubepods)' /proc/1/cgroup 2>/dev/null;
	then
		cargo run -p backend 2>&1 &
		trunk serve --config Trunk.toml 2>&1 &
		wait
	else
		docker compose -f ./docker/dev.docker-compose.yml up --no-deps --build;
	fi
