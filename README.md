# Fitparser
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/fitparser.svg)](https://crates.io/crates/fitparser)
[![Docs.rs](https://docs.rs/fitparser/badge.svg)](https://docs.rs/fitparser)
[![Build Status](https://travis-ci.org/stadelmanma/fitparse-rs.svg?branch=master)](https://travis-ci.org/stadelmanma/fitparse-rs)


## Overview

Parses FIT formatted files and exports their contents using
[Serde](https://github.com/serde-rs/serde). This code was heavily
inspired by the
[python-fitparse](https://github.com/dtcooper/python-fitparse) module
and uses the specifications defined by the Fit File SDK which is
maintained by [ANT](thisisant.com).

The goal of this crate is to parse valid FIT files according to the
defined FIT profile and export their data into a more usable format. To that
end we leverage the Serde framework which allows end users to export the
data in whatever format suits their needs. This library provides a
`fit_to_json` example executable to serve as a template for any
other serialization format implemented using Serde.

Notes:
 * This library **does not** support writing FIT files at this time.
 * Files with Developer Data fields can be parsed but the developer
   fields are dropped.
  * We do not validate the CRC values at this time
  * The FIT SDK is regularly updated by Garmin/Ant this library may not
    be up to date; check the `src/profile/messages.rs` for the packaged version.
    Submit an issue and I will gladly bump it!

## Usage

See library documentation at [docs.rs/fitparser](https://docs.rs/fitparser)
for full usage information. Below is a basic example of calling the parser
on a FIT file.
```rust
use fitparser;
use std::fs::File;
use std::io::prelude::*;

let mut fp = File::open("tests/fixtures/Activity.fit")?;
for data in fitparser::from_reader(&mut fp)? {
    // print the data in FIT file
    println!("{:#?}", data);
}
```


## Updating the FIT profile

All FIT files are generated based on a customizable profile. The profile
used here is pulled from ANT's offical SDK which can be accessed
[here](https://www.thisisant.com/developer/resources/downloads/). The
cargo build command expects the environment variable `FIT_PROFILE` to be set to the
path of the desired Profile.xlsx file and the `FIT_PROFILE_VERSION` variable to
the appropriate version. To make updating simpler a script is provided
`./bin/update_profile.sh` that accepts the profile path the as first argument
and optionally a profile version as the second. The version can be omitted
if the path to the Profile.xlsx file contains `FitSDKRelease_XX.YY.ZZ`.

```sh
./bin/update_profile.sh ~/Downloads/FitSDKRelease_21.40.00/Profile.xlsx
```

A profile file is not required for building the library as the files
generated are committed to the repository. The profile only needs
updated to support custom extensions or when ANT releases an updated
version.
