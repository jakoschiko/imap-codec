[![Build & Test](https://github.com/duesee/imap-codec/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/duesee/imap-codec/actions/workflows/build_and_test.yml)
[![Audit](https://github.com/duesee/imap-codec/actions/workflows/audit.yml/badge.svg)](https://github.com/duesee/imap-codec/actions/workflows/audit.yml)
[![Coverage](https://coveralls.io/repos/github/duesee/imap-codec/badge.svg?branch=main)](https://coveralls.io/github/duesee/imap-codec?branch=main)
[![Documentation](https://docs.rs/imap-codec/badge.svg)](https://docs.rs/imap-codec)

# imap-{codec,types}

This workspace contains [`imap-codec`] and [`imap-types`], two [rock-solid] and [well-documented] crates to build [IMAP4rev1] clients and servers.
`imap-codec` provides parsing and serialization, and is based on `imap-types`.
`imap-types` provides misuse-resistant types, constructors, and general support for IMAP implementations.
The crates live here together, but `imap-types` is a perfectly fine standalone crate.

If you are looking for a slightly more high-level client or server, take a look at [`imap-flow`].

Let's talk on [Matrix]!

## Features

* Complete [formal syntax] of IMAP4rev1 is implemented. Furthermore, several IMAP [extensions] are supported.
* Correctness and misuse-resistance are enforced on the type level. It's not possible to construct a message that violates the IMAP specification.
* Messages automatically use the most efficient representation. For example, atoms are preferred over quoted strings, and quoted strings are preferred over literals. It's equally easy to manually choose a representation.
* Parsing works in streaming mode. `Incomplete` is returned when there is insufficient data to make a final decision. No message will be truncated.
* Parsing is zero-copy by default. Allocation is avoided during parsing, but all messages can explicitly be converted into more flexible owned variants.
* Fuzzing and property-based tests exercise the library. The library is fuzz-tested never to produce a message it can't parse itself.

## Usage

```rust
use imap_codec::{decode::Decoder, encode::Encoder, CommandCodec};

fn main() {
    let input = b"ABCD UID FETCH 1,2:* (BODY.PEEK[1.2.3.4.MIME]<42.1337>)\r\n";

    let codec = CommandCodec::new();
    let (remainder, cmd) = codec.decode(input).unwrap();
    println!("# Parsed\n\n{:#?}\n\n", cmd);

    let buffer = codec.encode(&cmd).dump();

    // Note: IMAP4rev1 may produce messages that are not valid UTF-8.
    println!("# Serialized\n\n{:?}", std::str::from_utf8(&buffer));
}
```

## Examples

### Simple parsing

Try one of the `parse_*` examples, e.g., ...

```sh
$ cargo run --example=parse_command
```

... to parse some IMAP messages.

### Tokio demo

You can also start the [demo server] with ...

```sh
$ cargo run -p tokio-server -- <host>:<port>
```

... and connect to it with ...

```sh
$ netcat -C <host> <port>
```

There is also a [demo client] available.

**Note:** All demos are a work-in-progress. Feel free to propose API changes to `imap-codec` (or `imap-types`) to simplify them.

### Parsed and serialized IMAP4rev1 connection

The following output was generated by reading the trace from [RFC 3501 section 8](https://tools.ietf.org/html/rfc3501#section-8), printing the input (first line), `Debug`-printing the parsed object (second line), and printing the serialized output (third line).

```rust,compile_fail
// * OK IMAP4rev1 Service Ready
Status(Ok { tag: None, code: None, text: Text("IMAP4rev1 Service Ready") })
// * OK IMAP4rev1 Service Ready

// a001 login mrc secret
Command { tag: Tag("a001"), body: Login { username: Atom(AtomExt("mrc")), password: /* REDACTED */ } }
// a001 LOGIN mrc secret

// a001 OK LOGIN completed
Status(Ok { tag: Some(Tag("a001")), code: None, text: Text("LOGIN completed") })
// a001 OK LOGIN completed

// a002 select inbox
Command { tag: Tag("a002"), body: Select { mailbox: Inbox } }
// a002 SELECT INBOX

// * 18 EXISTS
Data(Exists(18))
// * 18 EXISTS

// * FLAGS (\Answered \Flagged \Deleted \Seen \Draft)
Data(Flags([Answered, Flagged, Deleted, Seen, Draft]))
// * FLAGS (\Answered \Flagged \Deleted \Seen \Draft)

// * 2 RECENT
Data(Recent(2))
// * 2 RECENT

// * OK [UNSEEN 17] Message 17 is the first unseen message
Status(Ok { tag: None, code: Some(Unseen(17)), text: Text("Message 17 is the first unseen message") })
// * OK [UNSEEN 17] Message 17 is the first unseen message

// * OK [UIDVALIDITY 3857529045] UIDs valid
Status(Ok { tag: None, code: Some(UidValidity(3857529045)), text: Text("UIDs valid") })
// * OK [UIDVALIDITY 3857529045] UIDs valid

// a002 OK [READ-WRITE] SELECT completed
Status(Ok { tag: Some(Tag("a002")), code: Some(ReadWrite), text: Text("SELECT completed") })
// a002 OK [READ-WRITE] SELECT completed

// a003 fetch 12 full
Command { tag: Tag("a003"), body: Fetch { sequence_set: SequenceSet([Single(Value(12))]+), macro_or_item_names: Macro(Full), uid: false } }
// a003 FETCH 12 FULL

// * 12 FETCH (FLAGS (\Seen) INTERNALDATE "17-Jul-1996 02:44:25 -0700" RFC822.SIZE 4286 ENVELOPE ("Wed, 17 Jul 1996 02:23:25 -0700 (PDT)" "IMAP4rev1 WG mtg summary and minutes" (("Terry Gray" NIL "gray" "cac.washington.edu")) (("Terry Gray" NIL "gray" "cac.washington.edu")) (("Terry Gray" NIL "gray" "cac.washington.edu")) ((NIL NIL "imap" "cac.washington.edu")) ((NIL NIL "minutes" "CNRI.Reston.VA.US")("John Klensin" NIL "KLENSIN" "MIT.EDU")) NIL NIL "<B27397-0100000@cac.washington.edu>") BODY ("TEXT" "PLAIN" ("CHARSET" "US-ASCII") NIL NIL "7BIT" 3028 92))
Data(Fetch { seq: 12, items: [Flags([Flag(Seen)]), InternalDate(1996-07-17T02:44:25-07:00), Rfc822Size(4286), Envelope(Envelope { date: NString(Some(Quoted(Quoted("Wed, 17 Jul 1996 02:23:25 -0700 (PDT)")))), subject: NString(Some(Quoted(Quoted("IMAP4rev1 WG mtg summary and minutes")))), from: [Address { name: NString(Some(Quoted(Quoted("Terry Gray")))), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("gray")))), host: NString(Some(Quoted(Quoted("cac.washington.edu")))) }], sender: [Address { name: NString(Some(Quoted(Quoted("Terry Gray")))), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("gray")))), host: NString(Some(Quoted(Quoted("cac.washington.edu")))) }], reply_to: [Address { name: NString(Some(Quoted(Quoted("Terry Gray")))), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("gray")))), host: NString(Some(Quoted(Quoted("cac.washington.edu")))) }], to: [Address { name: NString(None), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("imap")))), host: NString(Some(Quoted(Quoted("cac.washington.edu")))) }], cc: [Address { name: NString(None), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("minutes")))), host: NString(Some(Quoted(Quoted("CNRI.Reston.VA.US")))) }, Address { name: NString(Some(Quoted(Quoted("John Klensin")))), adl: NString(None), mailbox: NString(Some(Quoted(Quoted("KLENSIN")))), host: NString(Some(Quoted(Quoted("MIT.EDU")))) }], bcc: [], in_reply_to: NString(None), message_id: NString(Some(Quoted(Quoted("<B27397-0100000@cac.washington.edu>")))) }), Body(Single { body: Body { basic: BasicFields { parameter_list: [(Quoted(Quoted("CHARSET")), Quoted(Quoted("US-ASCII")))], id: NString(None), description: NString(None), content_transfer_encoding: Quoted(Quoted("7BIT")), size: 3028 }, specific: Text { subtype: Quoted(Quoted("PLAIN")), number_of_lines: 92 } }, extension_data: None })]+ })
// * 12 FETCH (FLAGS (\Seen) INTERNALDATE "17-Jul-1996 02:44:25 -0700" RFC822.SIZE 4286 ENVELOPE ("Wed, 17 Jul 1996 02:23:25 -0700 (PDT)" "IMAP4rev1 WG mtg summary and minutes" (("Terry Gray" NIL "gray" "cac.washington.edu")) (("Terry Gray" NIL "gray" "cac.washington.edu")) (("Terry Gray" NIL "gray" "cac.washington.edu")) ((NIL NIL "imap" "cac.washington.edu")) ((NIL NIL "minutes" "CNRI.Reston.VA.US")("John Klensin" NIL "KLENSIN" "MIT.EDU")) NIL NIL "<B27397-0100000@cac.washington.edu>") BODY ("TEXT" "PLAIN" ("CHARSET" "US-ASCII") NIL NIL "7BIT" 3028 92))

// a003 OK FETCH completed
Status(Ok { tag: Some(Tag("a003")), code: None, text: Text("FETCH completed") })
// a003 OK FETCH completed

// a004 fetch 12 body[header]
Command { tag: Tag("a004"), body: Fetch { sequence_set: SequenceSet([Single(Value(12))]+), macro_or_item_names: MessageDataItemNames([BodyExt { section: Some(Header(None)), partial: None, peek: false }]), uid: false } }
// a004 FETCH 12 BODY[HEADER]

// * 12 FETCH (BODY[HEADER] {342}
// Date: Wed, 17 Jul 1996 02:23:25 -0700 (PDT)
// From: Terry Gray <gray@cac.washington.edu>
// Subject: IMAP4rev1 WG mtg summary and minutes
// To: imap@cac.washington.edu
// cc: minutes@CNRI.Reston.VA.US, John Klensin <KLENSIN@MIT.EDU>
// Message-Id: <B27397-0100000@cac.washington.edu>
// MIME-Version: 1.0
// Content-Type: TEXT/PLAIN; CHARSET=US-ASCII
// 
// )
Data(Fetch { seq: 12, items: [BodyExt { section: Some(Header(None)), origin: None, data: NString(Some(Literal(Literal { data: b"Date: Wed, 17 Jul 1996 02:23:25 -0700 (PDT)\r\nFrom: Terry Gray <gray@cac.washington.edu>\r\nSubject: IMAP4rev1 WG mtg summary and minutes\r\nTo: imap@cac.washington.edu\r\ncc: minutes@CNRI.Reston.VA.US, John Klensin <KLENSIN@MIT.EDU>\r\nMessage-Id: <B27397-0100000@cac.washington.edu>\r\nMIME-Version: 1.0\r\nContent-Type: TEXT/PLAIN; CHARSET=US-ASCII\r\n\r\n" }))) }]+ })
// * 12 FETCH (BODY[HEADER] {342}
// Date: Wed, 17 Jul 1996 02:23:25 -0700 (PDT)
// From: Terry Gray <gray@cac.washington.edu>
// Subject: IMAP4rev1 WG mtg summary and minutes
// To: imap@cac.washington.edu
// cc: minutes@CNRI.Reston.VA.US, John Klensin <KLENSIN@MIT.EDU>
// Message-Id: <B27397-0100000@cac.washington.edu>
// MIME-Version: 1.0
// Content-Type: TEXT/PLAIN; CHARSET=US-ASCII
// 
// )

// a004 OK FETCH completed
Status(Ok { tag: Some(Tag("a004")), code: None, text: Text("FETCH completed") })
// a004 OK FETCH completed

// a005 store 12 +flags \deleted
Command { tag: Tag("a005"), body: Store { sequence_set: SequenceSet([Single(Value(12))]+), kind: Add, response: Answer, flags: [Deleted], uid: false } }
// a005 STORE 12 +FLAGS (\Deleted)

// * 12 FETCH (FLAGS (\Seen \Deleted))
Data(Fetch { seq: 12, items: [Flags([Flag(Seen), Flag(Deleted)])]+ })
// * 12 FETCH (FLAGS (\Seen \Deleted))

// a005 OK +FLAGS completed
Status(Ok { tag: Some(Tag("a005")), code: None, text: Text("+FLAGS completed") })
// a005 OK +FLAGS completed

// a006 logout
Command { tag: Tag("a006"), body: Logout }
// a006 LOGOUT

// * BYE IMAP4rev1 server terminating connection
Status(Bye { code: None, text: Text("IMAP4rev1 server terminating connection") })
// * BYE IMAP4rev1 server terminating connection

// a006 OK LOGOUT completed
Status(Ok { tag: Some(Tag("a006")), code: None, text: Text("LOGOUT completed") })
// a006 OK LOGOUT completed
```

# FAQ

<details>
<summary>How does <code>imap-codec</code> compare to <code>imap-proto</code>?</summary>

`imap-codec` provides low-level parsing and serialization support for IMAP4rev1, similar to [`imap-proto`].
The most significant differences are
server support,
the split into `imap-codec` and `imap-types`,
misuse resistance (affecting API design),
and (real-world) test coverage.

No matter if implementing a client- or a server, you need the full set of IMAP type definitions.
When you send a command with a specific [`Tag`], you expect a command completion response with the same [`Tag`].
Thus, commands and responses must work well together (and are best provided by a single crate).
As far as I know, `imap-proto` doesn't provide types that would be reusable in a generic server implementation.
`imap-types` provides type definitions for client- and server implementations.

As a client developer, you will never parse commands or serialize responses.
As a server developer, you will never serialize commands or parse responses.
Thus, you only need "half of" the set of parsers and serializers.
As far as I know, `imap-proto` provides the "client half" only.
`imap-codec` provides both the "client half" and the "server half".

Separating types and codecs increases cohesion and (hopefully) paves the way for IMAP crates that operate at higher levels.
However, the maintenance cost of two crates, `imap-types` and `imap-codec`, could be higher than for `imap-proto`.

Generally, `imap-codec` has a more extensive API surface than `imap-proto` and could be [more challenging to use].
In return, it guarantees that you always construct valid messages and aims to make IMAP usable even for people with less IMAP experience.
For example, `imap-codec` has [build-in support for IMAP literals] and ensures to always use [a correct representation for strings].

`imap-codec` has a high test coverage and is fuzz-tested to ensure properties such as invertibility, misuse-resistance, etc.
You should be unable to crash the library or generate messages that can't be parsed.
However, "interoperability can not be tested in a vacuum" [^1].
`imap-proto` already succeeded in production as it is (transitively) used in [`imap`], [`async-imap`], and [Delta Chat].
It could solve more real-world quirks, provide more IMAP extensions that matter in practice, or generally have a more mature interoperability story.
</details>

<details>
<summary>Have you considered contributing to <code>imap-proto</code>?</summary>

I created `imap-codec` because I needed [server-side support](https://github.com/Email-Analysis-Toolkit/fake-mail-server).
The intention was to eventually merge `imap-codec` into `imap-proto` as soon as it's "ready".
I even did a bit of [preparation work](https://github.com/djc/tokio-imap/graphs/contributors).
However, the different types (and philosophy, maybe), made merging non-trivial.
Both projects can learn from each other and align on their goals.
Still, joining forces would require a fair amount of work from everyone, and I wonder if we are willing (and have the resources) to start such an endeavor.
</details>

# License

This crate is dual-licensed under Apache 2.0 and MIT terms.

# Thanks

Thanks to the [NLnet Foundation](https://nlnet.nl/) for supporting imap-codec through their [NGI Assure](https://nlnet.nl/assure/) program!

<div align="right">
    <img height="100px" src="https://user-images.githubusercontent.com/8997731/215262095-ab12d43a-ca8a-4d44-b79b-7e99ab91ca01.png"/>
    <img height="100px" src="https://user-images.githubusercontent.com/8997731/221422192-60d28ed4-10bb-441e-957d-93af58166707.png"/>
    <img height="100px" src="https://user-images.githubusercontent.com/8997731/215262235-0db02da9-7c6c-498e-a3d2-7ea7901637bf.png"/>
</div>

[rock-solid]: https://github.com/duesee/imap-codec/tree/main/imap-codec/fuzz
[well-documented]: https://docs.rs/imap-codec/latest/imap_codec/
[Matrix]: https://matrix.to/#/#imap-codec:matrix.org
[IMAP4rev1]: https://tools.ietf.org/html/rfc3501
[formal syntax]: https://tools.ietf.org/html/rfc3501#section-9
[extensions]: https://docs.rs/imap-codec/latest/imap_codec/#features
[cargo fuzz]: https://github.com/rust-fuzz/cargo-fuzz
[demo client]: https://github.com/duesee/imap-codec/tree/main/assets/demos/tokio-client
[demo server]: https://github.com/duesee/imap-codec/tree/main/assets/demos/tokio-server
[parse_command]: https://github.com/duesee/imap-codec/blob/main/examples/parse_command.rs

[`imap-codec`]: imap-codec
[`imap-types`]: imap-types
[`imap-flow`]: https://github.com/duesee/imap-flow
[`imap`]: https://github.com/jonhoo/rust-imap
[`imap-proto`]: https://crates.io/crates/imap-proto
[`async-imap`]: https://github.com/async-email/async-imap
[Delta Chat]: https://delta.chat

[core types]: https://docs.rs/imap-types/latest/imap_types/core/index.html
[`Command`]: https://docs.rs/imap-types/latest/imap_types/command/struct.Command.html
[`Response`]: https://docs.rs/imap-types/latest/imap_types/response/enum.Response.html
[`Tag`]: https://docs.rs/imap-types/latest/imap_types/core/struct.Tag.html
[`BodyStructure`]: https://docs.rs/imap-types/latest/imap_types/body/enum.BodyStructure.html
[more challenging to use]: https://github.com/duesee/imap-codec/tree/main/imap-types#examples
[a correct representation for strings]: https://github.com/duesee/imap-codec/tree/main/imap-types#examples
[build-in support for IMAP literals]: https://docs.rs/imap-codec/latest/imap_codec/codec/struct.Encoded.html
[IMAP servers with imap-codec]: https://github.com/Email-Analysis-Toolkit/fake-mail-server
[^1]: https://datatracker.ietf.org/doc/html/rfc2683
