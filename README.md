# search-in-sight (sis)

A simple terminal-based fuzzy finder inspired by fzf, built with Rust and Ratatui.

![image](https://github.com/user-attachments/assets/efd1cd0b-d0e0-4219-a706-4a6d735b7764)

## Features

- Interactive fuzzy search through piped input
- Real-time filtering as you type
- Keyboard navigation (Up/Down arrows)
- Clean terminal UI with inline viewport

## Installation

### Building from source

```bash
git clone https://github.com/0l3d/search-in-sight
cd search-in-sight
cargo build --release
```

The binary will be available at `target/release/sis`

## Usage

Pipe any input to `sis` and start searching:

```bash
# Search through files
ls | sis

# Search through command history
history | sis

# Search through processes
ps aux | sis

# Search through any text
cat file.txt | sis
```

### Controls

- **Type**: Filter items in real-time
- **Up/Down**: Navigate through results
- **Enter**: Select current item and print to stdout
- **Esc**: Exit without selection
- **Backspace**: Delete character
- **Left/Right**: Move cursor

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [color-eyre](https://github.com/yaahc/color-eyre) - Error handling
- [matchr](https://github.com/0l3d/matchr) - Fuzzy matching algorithm

## License

GPL-3.0

## Author

[0l3d](https://github.com/0l3d)
