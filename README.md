# Instrumentality 
# Real-time data aggregation platform.
Instrumentality allows for the aggregation of data from any source into a single database under
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

## Documentation
See <https://instrumentality.berserksystems.com/docs/>.

## Architecture
This is a Rocket web server, the core logic of which can be seen in [main.rs](src/main.rs). All information is written to a MongoDB cluster, seen in [mdb.rs](src/mdb.rs). 

Instrumentality is interacted with solely through routes, which are stored in [/routes](/src/routes/). 
```
Content Routes:
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

From the top down, [groups](src/group.rs) contain [subjects](src/subject.rs) contain profile IDs. A profile is any discrete source of information. There are some very basic notions of [authentication](src/key.rs) and [users](src/user.rs). 

This server is a backend and intentionally only returns JSON.

## Features
### MVP
- [x] Content and Presence.
- [x] API keys.
- [x] TLS.
- [x] Errors.
- [x] Favicon/static file serving.
- [x] Registration through referral.
- [x] Data verification.
- [x] Subject and profile management.
- [x] Profile metadata.
- [x] /create, /update, /delete
- [x] /view.
- [x] Automatic deploying of MongoDB indexes.
- [x] /queue.

### Future
#### Minor
- [ ] Better /queue.
- [ ] Live config reloading.
- [ ] /leaderboard.
- [ ] Channels & webhooks

#### Major
- [ ] Handling discrepencies/byzantine platforms through consensus.
- [ ] Sharded database.
- [ ] Admin tooling.
- [ ] Example front end.
- [ ] Object storage.
- [ ] GraphQL for /view.