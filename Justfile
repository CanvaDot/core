set dotenv-load

export RUST_LOG := 'info'

@default:
	just --list;


@dev:
	#!/bin/bash
	set -e;

	if
		[[ -f /.dockerenv ]] ||
		grep -qE '(docker|containerd|kubepods)' /proc/1/cgroup 2>/dev/null;
	then
		cargo run -p backend 2>&1 & :;
		trunk serve --config Trunk.toml 2>&1 & :;
		wait;
	else
		docker compose -f ./docker/dev.docker-compose.yml up --no-deps --build;
	fi


@test *targets:
	#!/bin/bash
	set -e;

	done="";
	function run_cmd() {
		program=("${@:2}");
		if ! echo "$done" | grep "$1" 1>/dev/null 2>&1; then
			"${program[@]}" || echo "$1 FAILED!";
			done="$done $1";
		else
			echo "Skipping '$1', already ran!";
		fi
	}

	for target in {{ targets }}; do
		case "$target" in
			"tests") run_cmd "tests" cargo +nightly test --all ;;

			"fmt") run_cmd "fmt" cargo +nightly fmt --check --all ;;

			"clippy") run_cmd "clippy" cargo +nightly clippy --all ;;

			*) echo "Unknown command '$target'." ;;
		esac
	done


@clean:
	cargo clean;
