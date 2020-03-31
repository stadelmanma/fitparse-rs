# Fitparser

## Overview

Parses FIT formatted files and exports their contents using
[Serde](https://github.com/serde-rs/serde). This code was heavily
inspired by the
[python-fitparse](https://github.com/dtcooper/python-fitparse) module
and uses the specifications defined by the Fit File SDK which is
maintained by [ANT](thisisant.com).

The goal of this crate is to parse valid FIT files according to the
defined FIT profile and export them into a more usable format. To that
end we leverage the Serde framework which allows end users to export the
data in whatever format suits their needs. This library provides a
`fit_to_json` executable for use and to serve as a template for any
other serialization format implemented using Serde.

This library **does not** support writing FIT files at this time.

Features in Progress:
 * Properly handing compressed timestamps
 * Streamed FIT files
 * Parsing developer data fields

## Usage

TODO - Add when library API gets more finalized

## Updating the FIT profile

All FIT files are generated based on a customizable profile. The profile
used here is pulled from ANT's offical SDK which can be accessed
[here](https://www.thisisant.com/developer/resources/downloads/). To
update the FIT profile set the environment variable `FIT_PROFILE` to the
path of the desired Profile.xlsx file and then run `cargo build`. When
this variable is not set cargo will simply emit a warning stating that
the profile was not updated.

A profile file is not required for building the library as the files
generated are committed to the repository. The profile only needs
updated to support custom extensions or when ANT releases an updated
version.
