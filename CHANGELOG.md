# Change Log

## HEAD
* Improve parser error handling so that the user gets the byte position
where parsing failed intead of the number of bytes remaining

## v0.3.0
* Refactor the internal de-serialization and decoding logic.
* Return better errors when data buffer is incomplete
* Add the FIT Profile version to the profile module as a constant
* Add a script to regenerate the profile module from a new SDK
* Bump packaged FIT SDK version to 21.40.00

## v0.2.0
* Improve the API to allow manipulating the data with less cloning
* Expose the actual MesgNum enum value to the user in the `kind` field
of each FitDataRecord instead of it's name.
* Upated FitDataRecord to store fields as a Vec, sorted by definition
number. Each field contains the name, number, value and units (if defined).
* Rewrote parser logic to more closely resemble how serde deserializers are
typically implemented.
* Added formal error handling so the code should no longer panic anywhere
* Removed the FitFile struct, instead the parser returns a vector of
FitDataRecord objects.
* Moved the `fit_dump` executable to examples and renamed it `fit_to_json`,
this was done to remove the library's dependency on serde_json and structopt.
* Reimplement the `parse` top level function as `from_reader` using the new
return type.

## v0.1.1
Fixed a parsing error when handling string data fields that caused the
parser to panic.

## v0.1.0
Initial release on crates.io.
