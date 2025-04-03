# webhello: web server demo code
Bart Massey 2024

This is demos of how webservers work and can be used.

There are several branches in this repo that correspond to
various ways to do a webserver in Rust.

The main branch is a server written entirely in safe
Rust. It provides a hello page at `/` and a `/favicon.ico`.

Run with `cargo run` and go to `localhost:3000`.

## Branches

* `raw`: Simple server using only `std`.
* `main`, `full-raw`: More "advanced" server using only `std`.
* `threaded-raw`: "advanced" server using only `std` with threads.
* `hyper`: Simple server using `hyper`.
* `warp`: Simple server using `warp`.
* `axum`: Simple server using `axum`.
* `axum-full`: Full server using `axum`.

## Network Stuff

    PHY  - packets on wire: header w/ address, payload
           (eth mac 0a:1b:2c:3d:4e:5f)
    IPv4 - packets, headers, payload; wrapped by PHY
           (IPv4 addr 1.217.3.4) "best effort"
    TCP  - bidi streams, ports, split and wrapped by IPv4
           (16-bit port 3000)
    HTTP - text packets, headers and body, wrapped by TCP
           (textual URL http://1.217.3.4/)
    HTML - "special" text in HTTP body
    TLS  - encryption for TCP streams, protects HTTP
           (used for "https")

    DNS -  map names to IPv4 addresses
           (example.org -> 1.217.3.4)
