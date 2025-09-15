# An alternative library for `base64.js` written in Rust, with better performance

This package aims to mirror the same base64 that [`base64-js`](https://github.com/beatgammit/base64-js`) offers:

- byteLength - Takes a base64 string and returns length of byte array
- toByteArray - Takes a base64 string and returns a byte array
- fromByteArray - Takes a byte array and returns a base64 string

`byteLength` mirrors the origin Javacript code but written in Rust.
`toByteArray` and `fromByteArray` are backed by [`rust-base64`](https://github.com/marshallpierce/rust-base64).

## Benchmark

the `/benchmark/testdata`is the Google snappy test input, which support image, url, plain text, taking this as input,
the [benchmarks](https://github.com/linyihai/base64js-rs/blob/main/bench.result) shows that we gain about 10 performance improvement on `fromByteArray`, about 2~3 improvement on `toByteArray`.
