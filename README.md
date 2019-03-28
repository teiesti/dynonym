# dynonym

[![build]][travis]
[![release]][github]
[![crate]][crates.io]
[![doc]][docs.rs]

[build]: https://travis-ci.org/teiesti/dynonym.svg?branch=master
[travis]: https://travis-ci.org/teiesti/dynonym

[release]: https://img.shields.io/github/release/teiesti/dynonym.svg
[github]: https://github.com/teiesti/dynonym/releases

[crate]: https://img.shields.io/crates/v/dynonym.svg
[crates.io]: https://crates.io/crates/dynonym

[doc]: https://docs.rs/dynonym/badge.svg
[docs.rs]: https://docs.rs/dynonym

`dynonym` is a minimalistic HTTP server that manages dynamic DNS records. It operates on the edge
between the Web and the Domain Name System, taking care of the following tasks:

1. Listening for incoming update requests via HTTP.
2. Verifying authentication and authorization.
3. Forwarding the request to a DNS server using [RFC 2136][10].

`dynonym` is written in [Rust][20] and built on top of [warp][30]. Its functionality is inspired by
[No-IP][40] and [DynDNS][50].

While `dynonym` is mainly used as an application, it is nevertheless possible to integrate it in
other projects as a library. Check out the [documentation][55] for details! (This README does
solely care for the application!)

## WARNING

`dynonym` is currently under development. At the moment, there is no stable version! I highly
recommend you to wait until I finish my work!

## Prerequisites

Using `dynonym` makes almost no sense without a DNS server that allows dynamic updates with
[RFC 2136][30]. Although the server starts, the very basic update functionality will fail as soon as
the first update request comes in. Therefore, it is recommended to setup your DNS server first.
Doing this goes beyond the scope of this explanation! Look at the documentation of your DNS server!

If you want to compile `dynonym` from source, you will need the latest stable version of the Rust
compiler and the Cargo package manager. Consider using [`rustup`][60]!

## Installation

There are several ways to install `dynonym`:

1. **Binaries**  
   Binaries are available for download [here][70].

2. **From crates.io**  
   Once you have Cargo installed, you can download, compile and install the latest version from
   [crates.io][80] with `cargo install dynonym`.

3. **From source**  
   This method is not recommended unless you want to make a contribution.
   ```
   git clone https://github.com/teiesti/dynonym.git
   cd dynonym
   cargo install
   ```

## Configuration

`dynonym` uses a TOML-encoded configuration file. The file has three sections. Each section controls
a different part of `dynonym`. Here is a complete list of all the settings:

- `[http]`: HTTP server settings
  - `socket`: the socket address (IP address and port) the HTTP server should listen on

- `[dns]`: DNS client settings
  - `socket`: the socket address (IP address and port) the DNS update client should connect to
  - `ttl`: the resource records' time to live (TTL)

- `[users]`: A list of users

  Each user is defined in subsection `[users.<user>]` where `<user>` is the username. A user can
  have settings:
  - `pw`: a bcrypt hash representing the user's password
  - `domains`: a list of domain names the user is authorized for

### Example
```
[http]
socket = "0.0.0.0:8053"

[dns]
socket = "127.0.0.1:53"
ttl = 60

[users]

[users.maja]
pw = "$2y$12$WmMKjrv4TgDhzXeLirAhLu4ZNfesMhu6kIAwklq9DzNbuKCYoXeEy"
domains = ["cerana.apis", "mellifera.apis"]

[users.willi]
pw = "$2y$12$vZey0./3YcFFOO720/PlyeURFUSRmGs5hVei8yHBIYV/QsSERXWku"
domains = ["mellifera.apis"]
```

## Running the server

Type `dynonym`!

This command will start the HTTP server and listen for incoming update requests. You can stop the
server with `Ctrl+C`.

The server creates a lock file that is auto-removed when the server stops. The lock file makes sure
that only one instance is running at a time. It contains the process ID. Under Linux, you may use
`kill -s SIGINT $(< dynonym.lock)` to stop a server instance running in the background. You can
specify the lock file with `--lock <FILE>`.

## Routes

When a server instance is running, some routes are available via HTTP. A client may call these
routes to interact with the server in order to gather necessary information or trigger a dynamic
update. These are all currently available routes:

- `http://<user>:<pw>@<host>/dns/update?name=<name>&a=<a>&aaaa=<aaaa>`

  Updates the A and AAAA resource records for the given domain name.

  Parameters
    - `<user>`: the user
    - `<pw>`: her password
    - `<host>`: the server's hostname (and port), as set in the configuration file
    - `<name>`: the domain name to update
    - `<a>`: the RDATA for the A record (aka IPv4 address)
    - `<aaaa`: the RDATA for the AAAA record (aka IPv6 address)

  Returns
    - `200 OK` if the update was successful
    - `400 Bad Request` if any parameter (domain or IP address) has an invalid form
    - `401 Unauthorized` if the given credentials are wrong
    - `403 Forbidden` if the user is not authorized to change the given domain
    - `500 Internal Server Error` if the update failed for any other reason

- `http://<host>/ip`

  Returns the client's IP address.

- `http://<host>/port`

  Return the client's port number.

- `http://<host>/socket`

  Return the client's socket address (IP address and port).

## Contributing

I love to include contributions! Please feel free to open an issue or submit a pull request!

## License

`dynonym` is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details!


[10]: https://tools.ietf.org/html/rfc2136
[20]: https://www.rust-lang.org/
[30]: https://github.com/seanmonstar/warp
[40]: https://www.noip.com/
[50]: https://dyn.com/remote-access/
[55]: https://docs.rs/dynonym
[60]: https://www.rustup.rs/
[70]: https://github.com/teiesti/dynonym/releases
[80]: https://crates.io/
