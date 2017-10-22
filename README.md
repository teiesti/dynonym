# dynonym

`dynonym` is a minimalistic HTTP server that manages dynamic DNS records. It operates on the edge between the Web and the Domain Name System, taking care of these tasks:

1. Listening for incoming update requests via HTTP.
2. Verifying authentication and authorization.
3. Forwarding the request to a DNS server using [RFC 2136][30].

`dynonym` is written in [Rust][10] and built on top of [Rocket][20]. Its functionality is inspired by [No-IP][40] and [DynDNS][50].

## License

`dynonym` is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.


[10]: https://www.rust-lang.org/
[20]: https://rocket.rs/
[30]: https://tools.ietf.org/html/rfc2136
[40]: https://www.noip.com/
[50]: https://dyn.com/remote-access/
