# issn-rs

[![Continuous Integration](https://github.com/deutsche-nationalbibliothek/issn-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/deutsche-nationalbibliothek/issn-rs/actions/workflows/ci.yaml)
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
