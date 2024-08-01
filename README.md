dcmcatalog
===========

DICOM stands for “Digital Imaging and Communications in Medicine” and is the most common open standard for medical images. 
Each DICOM file contains a set of metadata about the patient, clinic, and acquisition, followed by the pixel data or contents of the acquisition itself.
Hospitals often collect all their DICOM files into a single large, shared drive. The goal of this command-line utility is to catalog the patients in such a drive, and help users identify files related to a specific patient for further analysis. A large clinic may store millions of files with paths such as “drive/1234/001/0001234.dcm”, making it difficult to locate the right files
without a catalog.

The CLI utility should:
  1. Locate DICOM files under a specified path.
  2. Print each file's DICOM patient name and patient ID.

## Building

These are the requirements to compile the project

 - [Rust](https://rustup.rs/) 1.72.0 or newer: `rustup update`
 - [Cross](https://github.com/cross-rs/cross) to build for machines using different platforms than the one you are building onto: `cargo install cross`

## Usage

First, build the command executable:

**Building a release**

```
cargo build --release
```

**Building a release on a different target platform**

```
cross build --target x86_64-pc-windows-gnu --release
```

Once built, it's possible to see the command line arguments. You'll need to prepend the command name with the `./target/release/` (or, if you cross-compiled, `./target/x86_64-pc-windows-gnu/release/`) directory, where the executable is:

**Show usage and options**

```
./target/release/dcmcatalog --help
```

```
Usage: dcmcatalog [OPTIONS] <PATH>

Arguments:
  <PATH>
          Path to scan for DICOM files

Options:
  -i, --patient-id <PATIENT_ID>
          Value to search for in the PatientId tag

  -n, --person-name <PERSON_NAME>
          Value to search for in the PersonName tag

  -m, --max-depth <MAX_DEPTH>
          Depth of search into subdirectories
          
          [default: 20]

  -o, --offset <OFFSET>
          The index of the first result to return
          
          [default: 0]

  -l, --limit <LIMIT>
          The count of results to return

  -d, --disk-index <DISK_INDEX>
          Index to the specified directory, useful for very large dataset that won't otherwise fit into memory

  -r, --regenerate
          Force regeneration of disk index. Ignored if indexing is done in memory

  -s, --search-sensibility <SEARCH_SENSIBILITY>
          The maximum Levenshtein distance (number of different letters) under which the query still matches. Max is 2
          
          [default: 2]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Running

if you want to compile and run the code instead of the executable, you can still use all of the commands above, for example:

```
cargo build -- /tmp/dicom -l 10
```

## Dependencies

dcmcatalog is powered by the following dependencies:

* [rayon](https://github.com/rayon-rs/rayon): A data parallelism library for Rust 
* [fs_extra](https://github.com/webdesus/fs_extra): Expanding opportunities standard library std::fs and std::io 
* [tantivy](https://github.com/quickwit-oss/tantivy): A full-text search engine library inspired by Apache Lucene

## Examples

### Index all files in path to disk 

Indexing on disk is run once, subsequent queries on the same path will reuse the same index. The command below uses a subfolder of `/tmp` to index all files in path.

```
./target/release/dcmcatalog "/run/media/dicom/my storage/" -d /tmp
```
### Paginate number of results
It's possible to provide an offset and a limit to reduce the amount of results returned in a single page. Below, a page of 10 elements starting from the 10th result is requested

```
./target/release/dcmcatalog "/run/media/dicom/my storage/" -d /tmp -o 10 -l 10
```

### Find all files for a given person name or patient id

The `--person-name` (`-n`) argument allows to fuzzy search on the field. If you need an exact search, instead, `--search-sensibility` must be set to 0

```
./target/release/dcmcatalog "/run/media/dicom/my storage/" -d /tmp -n anonymous
```

```
./target/release/dcmcatalog "/run/media/dicom/my storage/" -d /tmp -i koji59900 -s 0
```

### Exploring dataset while indexer is running

It's possible to run two instances of the command from two separate shells: one to long run an indexing operation, and one, on the same disk index, to query the index that is being built.
This way as soon as new data is indexed by the first process, it will appear in the query results of the second, in real time.

## License

Licensed under the The BSD Zero Clause License. See LICENSE file for full license