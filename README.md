# dw-apb-uart Uart driver in Rust
dw-apb-uart serial driver in Rust.

## Support boards
* RK3588
* T-HEAD (C910) Light Lichee Pi 4A
* BST A1000b FADA
* and more...

- _Todo: 16550 UART supports I/O port-mapped (x86 only) and memory-mapped_

## Quick usage

Enable exactly one board feature in `Cargo.toml`:
> Note: these board features are mutually exclusive. Enable only one.

```toml
[dependencies]
dw_apb_uart = { version = "0.2.0", features = ["board_thead-c910"] }
```

Below is a minimal example for:

* UART initialization
* outputting a string
* reading input bytes into a buffer

```rust

fn uart_init() -> DW8250 {
	let mut uart = DW8250::new(UART_BASE);
	uart.init();
	uart
}

fn uart_puts(uart: &mut DW8250, s: &str) {
	for &b in s.as_bytes() {
		uart.putchar(b);
	}
}

fn uart_get(uart: &mut DW8250, buf: &mut [u8]) -> usize {
	let mut n = 0;
	while n < buf.len() {
		match uart.getchar() {
			Some(b) => {
				buf[n] = b;
				n += 1;
			}
			None => break,
		}
	}
	n
}

// Example:
// let mut uart = uart_init();
// uart_puts(&mut uart, "hello, dw-apb-uart\r\n");
// let mut rx = [0u8; 64];
// let count = uart_get(&mut uart, &mut rx);
```

## Other

* `init()` behavior depends on enabled board feature:
  * `board_bst-a1000b`
  * `board_thead-c910`
  * `board_rk3588`
* `getchar()` returns `Option<u8>`: `None` means no byte is available yet.

* This crate is used in [ArceOS](https://github.com/arceos-org/arceos/blob/cc2679d029e13ecea379a46087b9219765d1a5af/crates/dw_apb_uart/) 

