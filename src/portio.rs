use core::marker::PhantomData;

unsafe fn inb(port: u16) -> u8 {
	let result: u8;
	asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
	return result;
}

unsafe fn outb(value: u8, port: u16) {
	asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) ::: "volatile");
}

unsafe fn inw(port: u16) -> u16 {
	let result: u16;
	asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
	return result;
}

unsafe fn outw(value: u16, port: u16) {
	asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) ::: "volatile");
}

unsafe fn inl(port: u16) -> u32 {
	let result: u32;
	asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
	return result;
}

unsafe fn outl(value: u32, port: u16) {
	asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) ::: "volatile");
}

pub struct Port<T: PortInOut> {
	port: u16,
	phantom: PhantomData<T>,
}

impl<T: PortInOut> Port<T> {
	pub const unsafe fn new(port: u16) -> Port<T> {
		Port { port: port, phantom: PhantomData }
	}

	pub fn read(&mut self) -> T {
		unsafe { T::port_in(self.port) }
	}

	pub fn read_offset(&mut self, offset: u16) -> T {
		unsafe { T::port_in(self.port + offset) }
	}

	pub fn write(&mut self, value: T) {
		unsafe { T::port_out(value, self.port); }
	}

	pub fn write_offset(&mut self, offset: u16, value: T) {
		unsafe { T::port_out(value, self.port + offset); }
	}
}

pub trait PortInOut {
	unsafe fn port_in(port: u16) -> Self;
	unsafe fn port_out(value: Self, port: u16);
}
impl PortInOut for u8 {
	unsafe fn port_in(port: u16) -> u8 { inb(port) }
	unsafe fn port_out(value: u8, port: u16) { outb(value, port) }
}
impl PortInOut for u16 {
	unsafe fn port_in(port: u16) -> u16 { inw(port) }
	unsafe fn port_out(value: u16, port: u16) { outw(value, port) }
}
impl PortInOut for u32 {
	unsafe fn port_in(port: u16) -> u32 { inl(port) }
	unsafe fn port_out(value: u32, port: u16) { outl(value, port) }
}
