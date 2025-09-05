set dotenv-load


@default:
	just --list;


@dev:
	#!/bin/bash
	set -e;

	if
		[[ -f /.dockerenv ]] ||
		grep -qE '(docker|containerd|kubepods)' /proc/1/cgroup 2>/dev/null;
	then
		cargo watch -x 'run -p backend' 2>&1 & :;
		trunk serve --config Trunk.toml 2>&1 & :;
		wait;
	else
		if !command -v docker >/dev/null 2>&1; then
			echo "Please, install a linux compatible version of docker before continuing.";
			exit 1;
		fi

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
			"code") run_cmd "code" cargo +nightly test --all ;;

			"fmt") run_cmd "fmt" cargo +nightly fmt --check --all ;;

			"clippy") run_cmd "clippy" cargo +nightly clippy --all ;;

			*) echo "Unknown command '$target'." ;;
		esac
	done


@coverage export_path="":
	#!/bin/bash
	set -e;

	if [[ -z "{{ export_path }}" ]]; then
		coverage=$(cargo llvm-cov --features no_coverage --all -- --nocapture --quiet 2>/dev/null \
			| grep "^TOTAL" \
			| awk '{print $10}');

		if [[ -z "$coverage" ]]; then
			echo "Tests failed, run 'just test code' to find out why.";
			exit 1;
		fi

		echo "${coverage/%\%/ }";
	else
		cargo llvm-cov --lcov --features no_coverage --all > "{{ export_path }}" 2>/dev/null;
	fi


@clean:
	cargo clean;
