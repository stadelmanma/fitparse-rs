#!/bin/sh
set -e

vers_pat='s/.*FitSDKRelease_([0-9]{2}\.[0-9]{2}\.[0-9]{2}).*/\1/gp'
help="usage: $(basename $0)  path/to/Profile.xlsx  [FIT profile version]"

FIT_PROFILE="$1"
if [ -z "$FIT_PROFILE" ]; then
    echo "error no Profile.xlsx file provided."
    echo $help
    exit 1
elif [ "$FIT_PROFILE" = "-h" ] || [ "$FIT_PROFILE" = "--help" ]; then
    echo $help
    exit 1
fi
if [ -z "$2" ]; then
    FIT_PROFILE_VERSION="$(echo $FIT_PROFILE | sed -rn "$vers_pat")"
    echo "extracted FIT profile version from path: $FIT_PROFILE_VERSION"
fi
if [ -z "$FIT_PROFILE_VERSION" ]; then
    echo "error no profile version provided and c"
    echo $help
    exit 1
fi

FIT_PROFILE="$FIT_PROFILE" FIT_PROFILE_VERSION="$FIT_PROFILE_VERSION" cargo build
