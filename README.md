`jh`: Jump Consistent Hash utility
==================================

A simple command-line utility and HTTP server for computing consistent hashes
using a [Rust implementation](https://github.com/codahale/jumphash) of
[Jump Consistent Hash](http://arxiv.org/abs/1406.2294) by Lamping & Veach.


Usage
-----

### Command-line utility

```bash
jh get "my key string" 1024
# => 731
```

### HTTP server

```bash
jh server -p 3030
```

```bash
curl "http://127.0.0.1:3030/?k=my%20key%20string&n=1024"
# => 731
```


Installation
------------

```bash
cargo build --release
```
