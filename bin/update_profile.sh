#!/bin/sh

set -eu

profile_path="${1:-}"
sdk_version="${2:-}"

if [ -z "$profile_path" ]; then
    echo "Usage: $0 PROFILE.xlsx [SDK_VERSION]" >&2
    exit 1
fi

if [ -n "$sdk_version" ]; then
    exec cargo run --bin generate-fit-profile -- "$profile_path" --sdk-version "$sdk_version"
fi

exec cargo run --bin generate-fit-profile -- "$profile_path"
