# Instrumentality - An extensible real-time data aggregation platform.
Instrumentality allows for the aggregation of data from any source into a single database under
a common set of schemas.

## Thesis
Data should belong to people and those they choose to share it with.

## Documentation
See <https://instrumentality.berserksystems.com/docs/>.

## Download
See <https://github.com/berserksystems/instrumentality/releases/>.

## Building from Source
```
git clone https://github.com/berserksystems/instrumentality.git
cd instrumentality/
cargo build --release
./target/release/instrumentality.exe
```

## Architecture
`Subject`s are `/create`d and `/update`d to include `Profile`s, `Data` about which is `/add`ed.
`User`s of Instrumentality are `/register`ed by referral and `/add` and `/view` `Data`.
`Data` cannot be deleted. `Subject`'s `Profile`s can be modified to reflect changes.

## Features
### MVP
[x] Content and Presence.
[x] API keys.
[x] TLS.
[x] Errors.
[x] Favicon/static file serving.
[x] Registration through referral.
[x] Data verification.
[x] Subject and profile management.
[x] Profile metadata.
[x] /create, /update, /delete
[x] /view.

### Future
#### Minor
[ ] Automatic deploying of MongoDB indexes.
[ ] Live config reloading.
[ ] /leaderboard.
[ ] Discord integration & webhooks.
[ ] /queue.

#### Major
[ ] Handling discrepencies/byzantine platforms through consensus.
[ ] Sharded database.
[ ] Admin tooling.
[ ] Example front end.
[ ] Object storage.
[ ] GraphQL for /view.