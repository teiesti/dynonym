# dynonym

`dynonym` is a minimalistic HTTP server that manages dynamic DNS records. It operates on the edge
between the Web and the Domain Name System, taking care of the following tasks:

1. Listening for incoming update requests via HTTP.
2. Verifying authentication and authorization.
3. Forwarding the request to a DNS server using [RFC 2136][10].

`dynonym` is written in [Rust][20] and built on top of [Rocket][30]. Its functionality is inspired
by [No-IP][40] and [DynDNS][50].

While `dynonym` is mainly used as an application, it is nevertheless possible to integrate it in
other projects as a library. Check out the [documentation][55] for details! (This README does
solely care for the application!)

## WARNING

`dynonym` is currently under development. At the moment, there is no stable version! Since I prepare
myself for the final release, even some parts of this README are flawed. I would highly recommend
you to wait until I finish my work!

## Prerequisites

Using `dynonym` makes almost no sense without a DNS server that allows for dynamic updates using
[RFC 2136][30]: While most commands will still work, the very basic update functionality will fail
as soon as the first update request comes in. Therefore, it is recommended to setup your DNS server
first. Doing this goes beyond the scope of this explanation! Look at the documentation of your DNS
server!

If you want to compile `dynonym`, you will need a nightly version of the Rust compiler and the Cargo
package manager. Consider using [`rustup`][60]!

## Installation

There are several ways to install `dynonym`:

1. **Binaries**  
   Binaries are available for download [here][70].

2. **From crates.io**  
   Once you have Cargo installed, you can download, compile and install the latest version from [crates.io][80] with `cargo install dynonym`.

3. **From source**  
   This method not recommended unless you want to make a contribution.
   ```
   git clone https://github.com/teiesti/dynonym.git
   cd dynonym
   cargo install
   ```

## Configuration

`dynonym` uses a TOML-encoded configuration file. While it is possible to write it by hand, it is
easier to use the command line tool. The most important commands are listed below. In case you are
searching for a more advanced feature not listed here, consider using `--help` in any context!

- `dynonym configure default`

  Creates a default configuration. You may want to specify the configuration file using
  `--config <FILE>`.

- `dynonym configure dns --socket <ADDR>`

  Specifies the socket address (IP address and port) of your DNS server. You will need to adjust
  these settings in order to get `dynonym` working with your DNS server.

- `dynonym configure users add <USER>`

  Adds a user. Since there is no default user within the default configuration, you will need to add
  one. (Note: This is not a bug but a security feature!)

- `dynonym configure users auth <USER> <DOMAIN>`

  Authorizes a user to update a domain.

## Running the server

Type `dynonym serve`!

This command will start the HTTP server and listen for incoming update requests. You can stop the
server with `Ctrl+C`.

The server creates a lock file that is auto-removed when the server stops. The lock file makes sure
that only one instance is running at a time. It contains the process ID. Under Linux, you may use
`kill -s SIGINT $(< dynonym.lock)` to stop a server instance running in the background. You can
specify the lock file using `--lock <FILE>`.

## Routes

When a server instance is running, some routes are available via HTTP. A client may use these routes
to interact with the server in order to gather necessary information or trigger a dynamic update.
These are all currently available routes:

- `http://<user>:<pw>@<url>/dns/update?domain=<domain>&ipv4=<ipv4>&ipv6=<ipv6>`

  Updates the given domain. Returns
    - `200 OK` if the update was successful
    - `400 Bad Request` if any parameter (domain or IP address) has an invalid form
    - `401 Unauthorized` if the given credentials are wrong
    - `403 Forbidden` if the user is not authorized to change the given domain
    - `500 Internal Server Error` if the update failed for any other reason


- `http://<url>/ip`

  Returns the client's IP address.

- `http://<url>/port`

  Return the client's port number.

- `http://<url>/socket`

  Return the client's socket address (IP address and port).

## Contributing

I love to include contributions! Please feel free to open an issue or submit a pull request!

## License

`dynonym` is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details!


[10]: https://tools.ietf.org/html/rfc2136
[20]: https://www.rust-lang.org/
[30]: https://rocket.rs/
[40]: https://www.noip.com/
[50]: https://dyn.com/remote-access/
[55]: https://docs.rs/dynonym/
[60]: https://www.rustup.rs/
[70]: https://github.com/teiesti/dynonym/releases
[80]: https://crates.io/
