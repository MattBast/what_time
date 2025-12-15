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

## Checks
Run the following commands to check code quality and run tests:
```bash
cargo check
cargo clippy -- -D warnings
cargo test
```

## UX rules

### Blocks of text
- No more than two or three lines of text before adding a newline.
- Prefer bullet points ideally with icons instead of disc bullets.

### Type Scale
- Use the Major Third scale rule for deciding the size of text. Start with default of 16px.
- Use [Typescale](https://typescale.com/) as a shortcut for calculating Text Size

### Spacing between groups of elements
- Rule of 8px. I.e. go up from 8px, to 16px, to 24px etc

### Line Height
- Headers = 1 to 1 ratio or `leading-none`
- Paragraghs = 1 to 1.5 ratio or `leading-normal`

### Letter spacing
- Display font (titles and header) = -1px or `tracking-tight`
- Body text (paragraphs and bullets) = 0px or `tracking-normal`
- CTAS (things to clikc on) = 1px or `tracking-wide`

### Subtle design
Don't distract from the context with flashy graphics. Use the following to add some subtle life to the page:
- Gradients
- Soft textured backgrounds
- Shadows
