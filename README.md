# Adventurers

The GRPC server for the Adventure game.

## Development

### Requirements

- [Rust 1.91.0+](https://rustup.rs)
- [PostgreSQL](https://www.postgresql.org/)
- [Direnv](https://direnv.net/)
- [SQLX CLI](https://crates.io/crates/sqlx-cli)

### Database

#### Git submodule

The database migrations come from a shared submodule: [migrations](https://github.com/mnstrapp/migrations).

To ensure it is installed, run the following in your terminal:

```
git submodule init
git submodule update
```

This will create the **migrations** directory in your root directory, so that we have access to the migrations for running.

In order to get any updates to your migrations, run:

```
git submodule update
```

#### Instructions

Instructions for the database setup are part of the git submodule. Make sure that you have that installed first, then [click here](./migrations/README.md) to continue.

### Protobufs

In order to make the GRPC code work, the Protobufs must be compiled.

#### Git submodule

The protobufs come from a shared adventure submodule: [adventure_proto](https://github.com/mnstrapp/adventure_proto).

To ensure it is installed, run the following in your terminal:

```
git submodule init
git submodule update
```

This will create the **adventure_proto** directory in your root directory, so that we have access to the protobufs for compiling.

In order to get any updates to your protobufs, run:

```
git submodule update
```

### Build

There is a **build.rs** already configured, so all protobufs will be generated before compile time. All we need to do is build the project via: `cargo build`.