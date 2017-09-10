rust-fftw3
===========
[![Build Status](https://travis-ci.org/termoshtt/rust-fftw3.svg?branch=master)](https://travis-ci.org/termoshtt/rust-fftw3)

FFTW binding for Rust

This repository includes three crates:

- [![Crate](http://meritbadge.herokuapp.com/fftw)](https://crates.io/crates/fftw)
  [![docs](https://img.shields.io/badge/docs-gh--pages-blue.svg)](https://termoshtt.github.io/rust-fftw3/fftw)
  fftw: safe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-sys)](https://crates.io/crates/fftw-sys)
  [![docs](https://img.shields.io/badge/docs-gh--pages-blue.svg)](https://termoshtt.github.io/rust-fftw3/fftw_sys)
  fftw-sys: unsafe wrapper in Rust
- [![Crate](http://meritbadge.herokuapp.com/fftw-src)](https://crates.io/crates/fftw-src)
  [![docs](https://img.shields.io/badge/docs-gh--pages-blue.svg)](https://termoshtt.github.io/rust-fftw3/fftw_src)
  fftw-src: source of FFTW


Feature flags
--------------

- `source`: download and complie FFTW (defualt)
- `system`: use system's libfftw3 (experimental)
- `intel-mkl` use Intel MKL backend through [intel-mkl-src](https://github.com/termoshtt/rust-intel-mkl) (experimental)

LICENSE
--------

The codes in this crate are licensed by MIT-License (see [LICENSE](LICENSE)),
and the backends are redistributed under following licenses:

- [FFTW](http://www.fftw.org/) is free software and distributed under GPLv2 ([License and Copyright](http://www.fftw.org/fftw3_doc/License-and-Copyright.html))
- [Intel MKL](https://software.intel.com/en-us/mkl) is distributed under the [Intel Simplified Software License for Intel(R) Math Kernel Library](https://github.com/termoshtt/rust-intel-mkl/blob/master/mkl_lib/license.txt)
