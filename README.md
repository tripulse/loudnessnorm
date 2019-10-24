# loudnessnorm
[![](https://travis-ci.org/nullvideo/loudnessnorm.svg?branch=master)](https://travis-ci.org/nullvideo/loudnessnorm)
[![](https://ci.appveyor.com/api/projects/status/2jk2t1t23d72516f?svg=true)](https://ci.appveyor.com/project/nullvideo/loudnessnorm)

An implementation of **RMS** based audio normalization *in Rust*. Currently, it's only able to decode WAVE files
with 32-bit floating point samples (because it's easier to work with).

The loudness of the samples are normalized by calculating the **RMS** then the gain is changed to
bring the average amplitude of the source signal to a target level (by default `0 dBFS`).

## Installation
For Windows NT based systems just download the executable from *AppVeyor*. Navigate to one of the
option listed there then head to the ARTIFACTS tab and
download the executable listed here.

A package on crates.io is also present. If the toolchain for Rust is already present in the system then run this command (it's better because it natively
compiles in your system).
```
cargo install loudnessnorm
```

---

- #### Why 32-bit floating point?
  The intent was to keep the source files the same specs. But, the WAVE library I'm using for I/O doesn't allow this kind of
  behaviour (conversion of sample formats is not allowed). The final descision is to keep it 32-bit floating point because
  sample conversion would add a overhead to the program.
