# ESC Terminal

The central computer in the escape room that the players will interact with

## Requirements

- `udiskie` needs to be installed as this program runs it in the background
- `Rust and Cargo`

## Setup

1. Clone the repository.
2. Get yourself a USB drive.
3. Copy `secret.hack` from the root of the repository to the root of the USB drive.
4. Set a environmental variable `ESC_USB_PATH` to the mount point of your USB drive.
5. Inside the repository run `cargo run`.

## Tips

- If you want to bypass the whole USB and hacking minigame part, just press the `HOME` button on your keyboard.