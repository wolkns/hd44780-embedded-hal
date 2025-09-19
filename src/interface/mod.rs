pub mod pcf8574;

use crate::instructions::{FnsetLines, FnsetFont};


#[derive(Debug, Clone, Copy)]
pub enum InterfaceError {
    Pcf8574I2cError
}


#[cfg(not(feature="async"))]
pub trait InterfaceTrait
{
    fn init(&mut self, fnset_lines:FnsetLines, fnset_font:FnsetFont) -> Result<(), InterfaceError>;
    
    fn send_byte<const RS_VAL:bool>(&mut self, byte: u8) -> Result<(), InterfaceError>;

    fn send_bytes<const RS_VAL:bool>(&mut self, bytes: &[u8]) -> Result<(), InterfaceError> {
        for &byte in bytes {
            self.send_byte::<RS_VAL>(byte)?;
        }
        Ok(())
    }

    fn receive_byte<const RS_VAL:bool>(&mut self, byte: &mut u8) -> Result<(), InterfaceError>;

    fn receive_bytes<const RS_VAL:bool>(&mut self, bytes: &mut [u8] ) -> Result<(), InterfaceError> {
        for byte in bytes {
            self.receive_byte::<RS_VAL>(byte)?;
        }
        Ok(())
    }

    fn delay_us(&mut self, us:u32);

    fn backlight(&mut self, bl:bool) -> Result<(), InterfaceError>;
}



#[cfg(feature="async")]
pub trait InterfaceTrait
{
    fn init(
        &mut self, 
        fnset_lines:FnsetLines, 
        fnset_font:FnsetFont,
    ) -> impl Future<Output = Result<(), InterfaceError>>;
    
    fn send_byte<const RS_VAL:bool>(
        &mut self, 
        byte: u8
    ) -> impl Future<Output = Result<(), InterfaceError>>;

    fn send_bytes<const RS_VAL:bool>(
        &mut self,
        bytes: &[u8]
    ) -> impl Future<Output = Result<(), InterfaceError>>;

    fn receive_byte<const RS_VAL:bool>(
        &mut self, 
        byte: &mut u8
    ) -> impl Future<Output = Result<(), InterfaceError>>;

    fn receive_bytes<const RS_VAL:bool>(
        &mut self, 
        bytes: &mut [u8] 
    ) -> impl Future<Output = Result<(), InterfaceError>>;

    fn delay_us(
        &mut self, 
        us:u32
    ) -> impl Future<Output = ()>;

    fn backlight(
        &mut self, 
        bl:bool
    ) -> impl Future<Output = Result<(), InterfaceError>>;
}

