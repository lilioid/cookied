# cookied - An RFC 865 compliant Quote of the Day Server

*cookied* is a small server program designed to supply a user with quotes of the day based on RFC 865.
A quote of the day service is specified as a useful debugging and measurement tool which simply sends a short message without regard to its input.

This software is named *cookied* because the accompanying DHCP-Option (with the option-code 8) is called the *Cookie Server Option
* which is how I even found the RFC.
I also just really like cookies so the name stuck.

## How to run

The server binary can be built using either `cargo build` or `nix build .` and then run.
Note that the server expects file descriptors to be passed to it (also known as socket-activation) like it is done with [systemd socket units](https://www.man7.org/linux/man-pages/man5/systemd.socket.5.html) or a manual wrapper such as [systemfd](https://github.com/mitsuhiko/systemfd) or [systemd-socket-activate](https://www.man7.org/linux/man-pages/man1/systemd-socket-activate.1.html).

```bash
systemd-socket-activate -l 127.0.0.1:17 cookied
# or
systemfd -s tcp::127.0.0.1:17 -s udp::127.0.0.1:17 -- ./target/debug/cookied
```

The server accepts the following commandline arguments:

```text
An RFC865 quote-of-the-day server

Note that the server can only be started using socket activation. This could be achieved using e.g. systemd socket activation or a wrapper program like systemfd.

Usage: cookied [OPTIONS]

Options:
      --alg <ALG>
          Which algorithm to use for response generation
          
          [default: time-and-place]

          Possible values:
          - pattern:        Respond with the hex value 0x55 to be easily recognizable
          - time-and-place: Respond with the current time and the remote address of the client
          - text:           Respond with the text given via --text

      --text <TEXT>
          Use this text as a response for --alg=text
          
          [default: "Hello World"]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## How to use the Service

Once the server is running, it responds on the given ports with a small quote.
This can be viewed, for example, with netcat:

```bash
nc 127.0.0.1 17
```

