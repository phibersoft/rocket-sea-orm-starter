# Rocket.rs + SeaOrm Starter Template

🚀 A starter template for building web applications with [Rocket.rs](https://rocket.rs/)
and [SeaOrm](https://www.sea-ql.org/SeaORM/), leveraging the power of Rust for blazing fast and safe web development.

## Features

- **Rocket.rs**: A web framework for Rust that makes it simple to write fast, secure web applications without
  sacrificing flexibility, usability, or type safety.
- **SeaOrm**: An async & dynamic ORM for Rust. Safe, efficient, and designed to work seamlessly with Rocket.rs.
- **Modular Architecture**: Organized in a workspace layout to separate concerns and manage dependencies efficiently.

## Getting Started

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/phibersoft/rocket-sea-orm-starter.git
cd rocket-sea-orm-starter
```

For development,

```bash
cargo run
```

For production,

```bash
cargo build --release
./target/release/rocket-sea-orm-starter
```

For testing,

```bash
cargo test
```

## TODO

- [ ] Add example for Relationship
- [ ] DockerFile for deployment