# Runix

This project is a port of Unix v6 to the STM32 in Rust. I aims to replicate the exact behavior of Unix such that and Unix v6 apps can be compiled to the cortex m4 architecture and run.

Ideally, the "computer" will have a simple text screen to use as a terminal, a keyboard, and file storage (SD card). Maybe after this incorporating a network card to use tools like rsh/telnet/rlogin (predecessors to ssh) to connect to the computer remotely.

## Dev

The project uses cargo embed + probe-rs to easily flash images to the stm32. Run `cargo embed` with the device connected and it should just work.