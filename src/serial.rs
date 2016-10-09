use portio;

pub struct Serial {
    port: portio::Port<u8>,
}

impl Serial {
    pub const fn new(port: u16) -> Serial {
        unsafe {
            Serial {
                port: portio::Port::new(port),
            }
        }
    }

    pub fn init(&mut self) {
        self.port.write_offset(1, 0x00);
        self.port.write_offset(3, 0x80);
        self.port.write_offset(0, 0x03);
        self.port.write_offset(1, 0x00);
        self.port.write_offset(3, 0x03);
        self.port.write_offset(2, 0xC7);
        self.port.write_offset(4, 0x0B);
    }

    fn is_transmit_empty(&mut self) -> bool {
        self.port.read_offset(5) & 0x20 == 0
    }

    pub fn write(&mut self, value: u8) {
        while self.is_transmit_empty() { }

        self.port.write(value);
    }
}

impl ::core::fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write(byte)
        }
        Ok(())
    }
}