[![Apache License](https://img.shields.io/github/license/jonaspleyer/spatial-decomposition?style=flat-square)](https://opensource.org/license/apache-2-0)
[![Test](https://img.shields.io/github/actions/workflow/status/jonaspleyer/spatial-decomposition/test.yml?label=Test&style=flat-square)](https://github.com/jonaspleyer/spatial-decomposition/actions)
[![Crate](https://img.shields.io/crates/v/spatial-decomposition.svg?style=flat-square)](https://crates.io/crates/spatial-decomposition)
![Crates.io Total Downloads](https://img.shields.io/crates/d/spatial-decomposition?style=flat-square)
![docs.rs](https://img.shields.io/docsrs/spatial-decomposition?style=flat-square)

# spatial_decomposition

This crate implements algorithms to decompose spaces into subdomains.
The crated partitions can be reused within numerical solvers such as FETI (finite element tearing
and interconnect) or other methods.
