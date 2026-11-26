# issn-rs

<!-- [![Rust](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/rust.yaml/badge.svg)](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/rust.yaml) -->
<!-- [![docs.rs](https://img.shields.io/docsrs/marc21?label=Documentation)](https://docs.rs/marc21/latest/marc21/) -->
<!-- [![crates.io](https://img.shields.io/crates/v/marc21)](https://crates.io/crates/marc21) -->
<!-- [![PyPI Version](https://img.shields.io/pypi/v/marc21-learn)](https://pypi.org/project/marc21-learn/#history) -->
<!-- [![PyPI Downloads](https://img.shields.io/pypi/dm/marc21-learn.svg)](https://pypi.org/project/marc21-learn/) -->
[![License](https://img.shields.io/github/license/deutsche-nationalbibliothek/marc21-rs?color=blue)](./LICENSE)

This project provides functions for working with International Standard Serial
Numbers (ISSNs). The `issn` crate focuses on a simple API, high performance,
and minimal dependencies on other libraries.

Please keep in mind that even if an ISSN is syntactically correct, it is
not necessarily associated with a resource and actually assigned by an ISSN
National Center.

This is developed by the Metadata Department of the [German National Library]
(DNB). It is used for data analysis and for automating metadata workflows
(data engineering) as part of automatic content indexing.

## Contributing

All contributors are required to "sign-off" their commits (using `git commit
-s`) to indicate that they have agreed to the [Developer Certificate of
Origin][DCO].

## License

This project is licensed under the [European Union Public License 1.2][EUPL].

[DCO]: https://developercertificate.org
[German National Library]: https://dnb.de/
[EUPL]: ./LICENSE
