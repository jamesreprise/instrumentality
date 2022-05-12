![Instrumentality](<https://instrumentality.berserksystems.com/header.png>)
---
Instrumentality facilitates the aggregation of data from any source into a single database under
a common set of schemas.

## Thesis
Data should belong to people and those they choose to share it with. The order in which posts are presented should be changed from reverse chronological order (latest first) only when the user expressly wishes to do so.

## Building from Source
```
git clone https://github.com/berserksystems/instrumentality.git
cd instrumentality/
cargo build --release
./target/release/instrumentality.exe
```

## Download
See <https://github.com/berserksystems/instrumentality/releases/>.

## Installation
- Move `InstrumentalityExample.toml` to `Instrumentality.toml` and configure appropriately.
- Generate PEM TLS files `cert.pem` and `privkey.pem` and place in /tls.
- Run with `cargo run --release` or by executing the downloaded binary.

## Documentation
See <https://instrumentality.berserksystems.com/docs/>.

## Architecture
This is an Axum web server, the core logic of which can be seen in [server.rs](src/server.rs). All information is written to a MongoDB cluster, seen in [mdb.rs](src/mdb.rs). 

Instrumentality is interacted with solely through routes, which are stored in [/routes](/src/routes/). 
```
Content Routes:
    >> (frontpage) GET /
    >> (add) POST /add
    >> (view) GET /view?<subjects>
    >> (types) GET /types
    >> (queue) GET /queue?<platforms>
User Management Routes:
    >> (login) GET /login
    >> (invite) GET /invite
    >> (register) POST /register
Subject/Group Management Routes:
    >> (create) POST /create
    >> (delete) POST /delete
    >> (update) POST /update
```
All POST requests only accept JSON. Every route except types requires an API key in the headers as `X-API-KEY`. 
See the documentation for each route for examples on how to use them.

From the top down, [groups](src/group.rs) contain [subjects](src/subject.rs) contain platform / ID pairs. Data also contains platform / ID pairs, linking data to subjects. A platform / ID pair is any discrete source of information. There are some very basic notions of [authentication](src/key.rs) and [users](src/user.rs). 

This server is a backend and intentionally only accepts and returns JSON.

## Features
- Abstraction over common data: content, presence, metadata.
- Abstraction over people and organisations: group and subjects.
- Full TLS support.
- Basic authentication through API keys.
- Registration through referral.
- Basic data verification.
- Queue system for prioritising jobs.

### Future
#### Minor
- [ ] Hot and cold `/queue`
- [ ] Live config reloading.
- [ ] `/leaderboard`.
- [ ] Basic analytics & dashboard on `/`.
- [ ] Channels & webhooks.

#### Major
- [ ] Sharded database.
- [ ] Admin tooling.
- [ ] Example front end.
- [ ] CDN caching media.
- [ ] GraphQL for `/view`.
- [ ] Handling discrepencies/byzantine platforms through consensus.