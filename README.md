# issn-rs

[![Continuous Integration](https://github.com/deutsche-nationalbibliothek/issn-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/deutsche-nationalbibliothek/issn-rs/actions/workflows/ci.yaml)
[![License](https://img.shields.io/github/license/deutsche-nationalbibliothek/marc21-rs?color=blue)](./LICENSE)

The goal of the project is to keep simple things simple: The core component of
the `issn` crate is the parsing of an [International Standard Serial Number]
(ISSN), which includes its validation (checksum verification). The error
reporting provides details about the underlying error if an ISSN cannot be
parsed. Finally, the ISSN can be converted back into a string.

The library is used by the following projects:

* [pica] (CLI) — to find invalid ISSNs in PICA+ records using the [check][check2] command
* [marc21] (CLI) — to find invalid ISSNs in MARC21 records using the [check] command
* [marc21-learn] (Python) — provides [utility functions] for working with ISSNs
* [datashed] (CLI) - to [extract ISSNs] from fulltext documents

This project is developed by the Metadata Department of the [German National
Library] (DNB). It is used for data analysis and for automating metadata
workflows (data engineering) as part of automatic subject indexing.

## Example

```rust
use issn::Issn;

let issn = Issn::new("0720-6763")?;
println!("issn = {issn}");
```

## Performance

The software library delivers quite good performance metrics. In the following
benchmark, 4,510,133 ISSNs, sourced from the [bibliographic records] of German
National Library, were validated. The [test program] reads the file line by
line and validates each value as it is read. If a record is invalid, a counter
is incremented. The test data was compiled using the [marc21] tool as follows:

```console
$ marc21 select -ps '02[23].a' dnb_all_dnbmarc.*.mrc.gz -o issn.csv
40,100,473 records, 0 invalid | 93,842 records/s, elapsed: 00:07:28

$ wc -l issn.csv
4510133 issn.csv
```

The following benchmark results were measured using [hyperfine]:

```console
$ hyperfine -S --warmup 10 --min-runs 100 './validate ~/tmp/issn.csv'
Benchmark 1: ./validate ~/tmp/issn.csv (100 runs)
                mean     ±       σ          min     …     max
  Wall Time    301.3 ms  ±    10.9 ms     289.7 ms  …   361.5 ms
  Memory         2.2 MiB ±     0.0 MiB      2.2 MiB …     2.2 MiB
```

## Contributing

All contributors are required to "sign-off" their commits (using `git commit
-s`) to indicate that they have agreed to the [Developer Certificate of
Origin][DCO].

## License

This project is licensed under the [European Union Public License 1.2][EUPL].

[bibliographic records]: https://data.dnb.de/DNB/
[check2]: https://deutsche-nationalbibliothek.github.io/pica-rs/commands/check.html#issn
[check]: https://deutsche-nationalbibliothek.github.io/marc21-rs/marc21-cli/reference/commands/marc21-check.html
[datashed]: https://deutsche-nationalbibliothek.github.io/dataset-tools/index.html#datashed
[DCO]: https://developercertificate.org
[EUPL]: ./LICENSE
[extract ISSNs]: https://deutsche-nationalbibliothek.github.io/dataset-tools/getting-started/datashed.html#bibliographic-references
[German National Library]: https://dnb.de/
[hyperfine]: https://github.com/sharkdp/hyperfine
[International Standard Serial Number]: https://en.wikipedia.org/wiki/ISSN
[marc21]: https://deutsche-nationalbibliothek.github.io/marc21-rs/marc21-cli/
[marc21-learn]: https://pypi.org/project/marc21-learn/
[pica]: https://deutsche-nationalbibliothek.github.io/pica-rs/
[test program]: ./examples/validate.rs
[utility functions]: https://deutsche-nationalbibliothek.github.io/marc21-rs/marc21-learn/reference/pid.html#international-standard-serial-number-issn
