set dotenv-load

hide_stderr := if env("DEBUG", "0") == "1" { "" } else { "2>/dev/null" }
# hide_stdout := if env("DEBUG", "0") == "1" { "" } else { "1>/dev/null" }


@default:
	just --list;


@setup:
	git config --local core.hooksPath .git_hooks


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
	fail="false";
	function run_cmd() {
		program=("${@:2}");
		if ! echo "$done" | grep "$1" 1>/dev/null 2>&1; then
			"${program[@]}" || fail="true" echo "$1 FAILED!";
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

	if [[ "$fail" == "true" ]]
	then
		exit 1;
	fi


@coverage export_path="":
	#!/bin/bash
	set -e;

	if [[ -z "{{ export_path }}" ]]; then
		coverage=$(cargo llvm-cov --features coverage --all -- --nocapture --quiet {{ hide_stderr }} \
			| grep "^TOTAL" \
			| awk '{print $10}');

		if [[ -z "$coverage" ]]; then
			echo "Tests failed, run 'just test code' to find out why.";
			exit 1;
		fi

		echo "${coverage/%\%/ }";
	else
		cargo llvm-cov --lcov --features coverage --all > "{{ export_path }}" {{ hide_stderr }};
	fi


@clean:
	cargo clean;
