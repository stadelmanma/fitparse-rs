# FIT SDK 21.195.0 Update Notes

This note explains why `generate-fit-profile` needed parsing changes for the
`21.195.0` profile workbook and documents the FIT SDK release-process change
Garmin announced in Q1 2026.

## Summary

The `21.195.0` `Profile.xlsx` exposed assumptions in the existing parser about
how Excel cells would be typed when read through `calamine`.

The fix was to normalize workbook input more consistently:

- treat empty-string cells like missing values for optional text fields
- trim string cells before deciding whether a value is present
- accept numeric values that arrive as strings instead of numeric cells

## What Changed In `Profile.xlsx`

Compared with `FitSDKRelease_21.171.00/Profile.xlsx`, the `21.195.0` workbook
showed a few concrete differences when parsed:

1. Optional comment cells were sometimes represented as explicit empty strings.
   In practice this meant `Some("")` instead of `None`.
2. Some numeric-looking cells were read as strings, for example field
   definition numbers like `"0"` on the `Messages` sheet.
3. Some enum values on the `Types` sheet were stringified decimals with leading
   whitespace instead of numeric cells or `0x..` strings.
4. Blank-looking cells in optional columns such as component metadata were
   sometimes present as empty strings instead of truly empty cells.

## Why This Broke Generation

The old parser assumed that:

- absent optional text cells would be returned as `None`
- numeric cells would be returned as numeric types
- blank optional columns would not produce empty CSV fragments

Those assumptions caused a few regressions with `21.195.0`:

- message docs like `"software message definition"` disappeared because
  an empty-string comment suppressed the fallback message text
- blank field comments produced empty doc lines like `* type:`
- some subfield and component rows were misclassified because empty-string
  cells were treated as populated
- numeric parsing failed when values arrived as strings

## Parsing Changes

The parser now normalizes a few cases more consistently:

- trim string values before use
- treat trimmed-empty strings as absent for optional text fields
- parse numeric values from either typed numeric cells or numeric strings

## FIT SDK Release Process Change In Q1 2026

Garmin announced this change in a forum post published on **January 9, 2026**:

- Source: <https://forums.garmin.com/developer/fit-sdk/b/news-announcements/posts/the-fit-sdk-zip-file-is-being-deprecated-and-will-go-away-in-1q26>

Key points from that announcement:

- starting with the next FIT SDK release, the legacy zip download would no
  longer be available from the developer site
- FIT SDK source would move to GitHub repositories
- when possible, SDKs would be distributed through package managers
- `FitCSVTool.jar`, `Profile.xlsx`, `ActivityRepairTool.jar`, and example files
  would move to a new FIT SDK Tools repository
- Garmin gave an ETA of **February or March 2026**

This likely explains why the workbook and distribution flow now look different
from the older zip-based releases.

## Maintenance Guidance

If a future FIT SDK profile update breaks generation again, first check for:

- empty-string cells in optional text columns
- numeric values exported as strings
- leading/trailing whitespace in identifier or value cells
- blank-looking rows that still contain zero-length strings

Prefer checking workbook cell normalization first before assuming a structural
sheet change.
