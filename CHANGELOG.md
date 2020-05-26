# Change Log

## HEAD
* Rewrote parser logic to more closely resemble how serde deserializers are
typically implemented.
* Added formal error handling so the code should no longer panic anywhere
* Removed the FitFile struct, instead the parser returns a vector of
FitDataRecord objects.
* Moved the `fit_dump` executable to examples and renamed it `fit_to_json`,
this was done to remove the library's dependency on serde_json and structopt.

## v0.1.1
Fixed a parsing error when handling string data fields that caused the
parser to panic.

## v0.1.0
Initial release on crates.io.
