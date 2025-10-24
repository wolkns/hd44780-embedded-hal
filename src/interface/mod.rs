pub mod pcf8574;


pub mod gpio {
    use embedded_hal::digital::{OutputPin, InputPin};

    #[cfg(not(feature="async"))]
    use embedded_hal::delay;

    #[cfg(feature="async")]
    use embedded_hal_async::delay;

    use crate::{
        instructions::{CmdOptions, FnsetLines, FnsetFont, FnsetDataLen},
        interface::{InterfaceTrait, InterfaceError}
    };
    
    include!("gpio4bit.rs");
    include!("gpio8bit.rs");
}

use crate::instructions::{FnsetLines, FnsetFont};


#[derive(Debug, Clone, Copy)]
pub enum InterfaceError {
    Pcf8574I2cError,
    GpioError,
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

    fn backlight(&mut self, bl:bool) -> Result<(), InterfaceError>;

    fn delay_us(&mut self, us:u32);
}



#[cfg(feature="async")]
pub trait InterfaceTrait
{
    async fn init(
        &mut self, 
        fnset_lines:FnsetLines, 
        fnset_font:FnsetFont,
    ) -> Result<(), InterfaceError>;
    
    async fn send_byte<const RS_VAL:bool>(
        &mut self, 
        byte: u8
    ) -> Result<(), InterfaceError>;

    async fn send_bytes<const RS_VAL:bool>(
        &mut self,
        bytes: &[u8]
    ) -> Result<(), InterfaceError>
    {
        for &byte in bytes {
            self.send_byte::<RS_VAL>(byte).await?;
        }
        Ok(())
    }

    async fn receive_byte<const RS_VAL:bool>(
        &mut self, 
        byte: &mut u8
    ) -> Result<(), InterfaceError>;

    async fn receive_bytes<const RS_VAL:bool>(
        &mut self, 
        bytes: &mut [u8] 
    ) -> Result<(), InterfaceError>
    {
        for byte in bytes {
            self.receive_byte::<RS_VAL>(byte).await?;
        }
        Ok(())
    }

    async fn backlight(
        &mut self, 
        bl:bool
    ) -> Result<(), InterfaceError>;

    async fn delay_us(
        &mut self, 
        us:u32
    ) -> ();
}

