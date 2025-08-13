# Vennstore (core)

A utility for tagging files. Filenames are assumd to be encoded in UTF-8,
so this utility cannot be used on Windows OS.

## Usage

A file to be inserted must reside on the same filesystem as the file tree's.

## Dependency note

`generic-array` is a transitive dependency of `sha1` and must
therefore match in version. For example, if the latter's version is
`0.10.6`, the former's must be `0.14.7`.

## Build

`make build`


## Doc

`make doc`
