# What Time

Run this command to open the website in dev mode and listen for changes in the codebase. The page will reload whenever a change is detected:
```bash
trunk serve --open
```

May also need to install these if the above fails:
```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
cargo install cargo-generate
```

If there's an error relating to tailwinds, try this:
```bash
export TRUNK_TOOLS_TAILWINDCSS="4.1.0"
```

Sometimes port 3000 gets reserved after running leptos commands a gew times.
So if the "Address already in use" error appears, try killing the process
occupying the port. Start by find the process PID:
```bash
lsof -i :3000
```
