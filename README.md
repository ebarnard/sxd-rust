# SXD

An XML library in Rust.

## Overview

The project is currently broken into two crates:

1. `document` - Basic DOM manipulation and reading/writing XML from strings.
2. `xpath` - Implementation of XPath 1.0 expressions.

There are also scattered utilities for playing around at the command
line.

In the future, I hope to add support for XSLT 1.0.

## Goals

This project has two goals, one more achievable than the other:

1. Help me learn Rust.
2. Replace [libxml] and [libxslt].

[libxml]: http://xmlsoft.org/
[libxslt]: http://xmlsoft.org/

## Contributing

1. Fork it ( https://github.com/shepmaster/sxd-rust/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Add a failing test.
4. Add code to pass the test.
5. Commit your changes (`git commit -am 'Add some feature'`)
6. Ensure tests pass.
7. Push to the branch (`git push origin my-new-feature`)
8. Create a new Pull Request
