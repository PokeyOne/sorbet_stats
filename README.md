# Sorbet Stats

This is a small utility built in Rust to help you track your progress in Sorbet.
It takes in the JSON output from Sorbet metrics and converts it to csv files,
and in the future can be used to generate graphs and charts.

## Usage

The first thing you need to do is generate a JSON file from Sorbet. To do this,
see the [Sorbet documentation](https://sorbet.org/docs/metrics).

Once you have this file, simply run the following command:

    sorbet_stats <path-to-json-file> --csv <path-to-csv-file>

This will generate a csv file at the path you specified.

Currently, all this has in it are the number of files with each sigil.

## Installation

To install this utility, run the following command:

    cargo install sorbet_stats

And this will install the utility to your local bin directory.

*you will need cargo installed to use this utility*. Cargo comes from installing
Rust, and is how Rust crates are installed.

## License

Sorbet Stats is licensed under the MIT license. See the [LICENSE](LICENSE) file
for more information.