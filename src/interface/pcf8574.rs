
#[cfg(not(feature="async"))]
use embedded_hal::{delay, i2c};

#[cfg(feature="async")]
use embedded_hal_async::{delay, i2c};

use crate::{
    instructions::{CmdOptions, FnsetDataLen, FnsetFont, FnsetLines}, 
    interface::{InterfaceError, InterfaceTrait}
};



pub struct Pcf8574Interface<I2C, DELAY, ENC>
where 
    I2C: i2c::I2c<i2c::SevenBitAddress>,
    DELAY: delay::DelayNs,
    ENC: Pcf8574EncoderTrait,
{
    i2c: I2C,
    address: i2c::SevenBitAddress,
    delay: DELAY,
    enc: ENC,
    bl: bool,
}

impl<I2C, DELAY, ENC> Pcf8574Interface<I2C, DELAY, ENC>
where
    I2C: i2c::I2c,
    DELAY: delay::DelayNs,
    ENC: Pcf8574EncoderTrait,
{
    pub fn new(i2c: I2C, address: i2c::SevenBitAddress, delay: DELAY, enc: ENC) -> Self {
        Self {
            i2c,
            address,
            delay,
            enc,
            bl: true,
        }
    }
}


#[cfg(not(feature="async"))]
impl<I2C, DELAY, ENC> InterfaceTrait for Pcf8574Interface<I2C, DELAY, ENC>
where 
    I2C: i2c::I2c,
    DELAY: delay::DelayNs,
    ENC: Pcf8574EncoderTrait,
{
    fn init(&mut self, fnset_lines:FnsetLines, fnset_font:FnsetFont) -> Result<(), super::InterfaceError> {
        let payload = self.enc.encode::<false, false>(self.bl, 
            CmdOptions::Fnset as u8 | FnsetDataLen::Bit8 as u8
        );
        
        self.i2c.write(self.address, 
            &payload[..2]
        ).map_err(|_| InterfaceError::Pcf8574I2cError)?;
        self.delay.delay_us(4_100);
        self.i2c.write(self.address, 
            &payload[..2]
        ).map_err(|_| InterfaceError::Pcf8574I2cError)?;
        self.delay.delay_us(100);
        self.i2c.write(self.address, 
            &payload[..2]
        ).map_err(|_| InterfaceError::Pcf8574I2cError)?;

        let payload = self.enc.encode::<false, false>(self.bl,
            CmdOptions::Fnset as u8 | FnsetDataLen::Bit4 as u8 | fnset_lines as u8 | fnset_font as u8
        );
        self.i2c.write(self.address, 
            &payload[..2]
        ).map_err(|_| InterfaceError::Pcf8574I2cError)?;

        // now in 4-bit mode

        self.i2c.write(self.address, 
            &payload
        ).map_err(|_| InterfaceError::Pcf8574I2cError)?;

        Ok(())
    }

    fn send_byte<const RS_VAL:bool>(&mut self, byte: u8) -> Result<(), super::InterfaceError> {
        self.i2c.write(
            self.address,
            &self.enc.encode::<RS_VAL, false>(self.bl, byte)
        ).map_err(|_| InterfaceError::Pcf8574I2cError)?;
        Ok(())
    }

    fn receive_byte<const RS_VAL:bool>(&mut self, byte: &mut u8) -> Result<(), super::InterfaceError> {
        let payload = self.enc.encode::<RS_VAL, true>(self.bl, 0x0f);
        // use payload[1..3] to prime read process
        
        let mut msn: [u8;1] = [0];
        let mut lsn: [u8;1] = [0];

        let mut transactions = [
            i2c::Operation::Write(&payload[2..3]),
            i2c::Operation::Read(&mut msn),
            i2c::Operation::Write(&payload[1..3]),
            i2c::Operation::Read(&mut lsn),
            i2c::Operation::Write(&payload[1..2]),
        ];
        self.i2c.transaction(self.address, &mut transactions).map_err(
            |_| InterfaceError::Pcf8574I2cError
        )?;

        *byte = self.enc.decode_data([msn[0], lsn[0]]);

        Ok(())
    }

    fn backlight(&mut self, bl:bool) -> Result<(), InterfaceError> {
        self.bl = bl;
        self.i2c.write(
            self.address, 
            &self.enc.encode::<false,false>(self.bl, 0x00)[1..2]
        ).map_err(|_| InterfaceError::Pcf8574I2cError)?;
        Ok(())
    }

    fn delay_us(&mut self, us: u32) {
        self.delay.delay_us(us);
    }
}




#[cfg(feature="async")]
impl<I2C, DELAY, ENC> InterfaceTrait for Pcf8574Interface<I2C, DELAY, ENC>
where 
    I2C: i2c::I2c,
    DELAY: delay::DelayNs,
    ENC: Pcf8574EncoderTrait,
{
    async fn init(
        &mut self, 
        fnset_lines:FnsetLines, 
        fnset_font:FnsetFont
    ) -> Result<(), InterfaceError> 
    {
        let payload = self.enc.encode::<false, false>(self.bl, 
            CmdOptions::Fnset as u8 | FnsetDataLen::Bit8 as u8
        );
        
        self.i2c.write(self.address, 
            &payload[..2]
        ).await.map_err(|_| InterfaceError::Pcf8574I2cError)?;
        self.delay.delay_us(4_100).await;
        self.i2c.write(self.address, 
            &payload[..2]
        ).await.map_err(|_| InterfaceError::Pcf8574I2cError)?;
        self.delay.delay_us(100).await;
        self.i2c.write(self.address, 
            &payload[..2]
        ).await.map_err(|_| InterfaceError::Pcf8574I2cError)?;

        let payload = self.enc.encode::<false, false>(self.bl,
            CmdOptions::Fnset as u8 | FnsetDataLen::Bit4 as u8 | fnset_lines as u8 | fnset_font as u8
        );
        self.i2c.write(self.address, 
            &payload[..2]
        ).await.map_err(|_| InterfaceError::Pcf8574I2cError)?;

        // now in 4-bit mode

        self.i2c.write(self.address, 
            &payload
        ).await.map_err(|_| InterfaceError::Pcf8574I2cError)?;

        Ok(())
    }

    async fn send_byte<const RS_VAL:bool>(
        &mut self, 
        byte: u8
    ) -> Result<(), super::InterfaceError> 
    {
        self.i2c.write(
            self.address,
            &self.enc.encode::<RS_VAL, false>(self.bl, byte)
        ).await.map_err(|_| InterfaceError::Pcf8574I2cError)?;
        Ok(())
    }
     
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
    ) -> Result<(), super::InterfaceError> 
    {
        let payload = self.enc.encode::<RS_VAL, true>(self.bl, 0x0f);
        // use payload[1..3] to prime read process
        
        let mut msn: [u8;1] = [0];
        let mut lsn: [u8;1] = [0];

        let mut transactions = [
            i2c::Operation::Write(&payload[2..3]),
            i2c::Operation::Read(&mut msn),
            i2c::Operation::Write(&payload[1..3]),
            i2c::Operation::Read(&mut lsn),
            i2c::Operation::Write(&payload[1..2]),
        ];
        self.i2c.transaction(
            self.address,
             &mut transactions
        ).await.map_err(|_| InterfaceError::Pcf8574I2cError)?;

        *byte = self.enc.decode_data([msn[0], lsn[0]]);

        Ok(())
    }

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

    async fn delay_us(
        &mut self, 
        us: u32
    ) -> ()
    {
        self.delay.delay_us(us).await;
    }

    async fn backlight(
        &mut self, 
        bl:bool
    ) -> Result<(), InterfaceError> 
    {
        self.bl = bl;
        self.i2c.write(
            self.address, 
            &self.enc.encode::<false,false>(self.bl, 0x00)[1..2]
        ).await.map_err(|_| InterfaceError::Pcf8574I2cError)?;
        Ok(())
    }
}


pub trait Pcf8574EncoderTrait {
    fn encode<const RS_VAL:bool, const RNW_VAL:bool>(&self, bl: bool, data:u8) -> [u8; 4];
    fn decode_data(&self, data: [u8;2]) -> u8;
}

pub struct Pcf8574Encoder<
    const RS: u8 = 0x01, 
    const RNW:u8 = 0x02, 
    const EN: u8 = 0x04, 
    const BL: u8 = 0x08, 
    const D4: u8 = 0x10, 
    const D5: u8 = 0x20, 
    const D6: u8 = 0x40, 
    const D7: u8 = 0x80
>;

impl Pcf8574Encoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const RS:u8, const RNW:u8, const EN:u8, const BL:u8, const D4:u8, const D5:u8, const D6:u8, const D7:u8> Pcf8574EncoderTrait 
for Pcf8574Encoder<RS,RNW,EN,BL,D4,D5,D6,D7> {
    fn encode<const RS_VAL:bool, const RNW_VAL:bool>(&self, bl: bool, data:u8) -> [u8; 4] {
        [EN | (RS_VAL as u8)*RS | (RNW_VAL as u8)*RNW | (bl as u8)*BL | ((data&0x10!=0)as u8)*D4 | ((data&0x20!=0)as u8)*D5 | ((data&0x40!=0)as u8)*D6 | ((data&0x80!=0)as u8)*D7, 
              (RS_VAL as u8)*RS | (RNW_VAL as u8)*RNW | (bl as u8)*BL | ((data&0x10!=0)as u8)*D4 | ((data&0x20!=0)as u8)*D5 | ((data&0x40!=0)as u8)*D6 | ((data&0x80!=0)as u8)*D7,
         EN | (RS_VAL as u8)*RS | (RNW_VAL as u8)*RNW | (bl as u8)*BL | ((data&0x01!=0)as u8)*D4 | ((data&0x02!=0)as u8)*D5 | ((data&0x04!=0)as u8)*D6 | ((data&0x08!=0)as u8)*D7, 
              (RS_VAL as u8)*RS | (RNW_VAL as u8)*RNW | (bl as u8)*BL | ((data&0x01!=0)as u8)*D4 | ((data&0x02!=0)as u8)*D5 | ((data&0x04!=0)as u8)*D6 | ((data&0x08!=0)as u8)*D7,]
    }
    fn decode_data(&self, data: [u8;2]) -> u8 {
        (((data[0] & D7) != 0) as u8) * 0x80 |
        (((data[0] & D6) != 0) as u8) * 0x40 |
        (((data[0] & D5) != 0) as u8) * 0x20 |
        (((data[0] & D4) != 0) as u8) * 0x10 |
        (((data[1] & D7) != 0) as u8) * 0x08 |
        (((data[1] & D6) != 0) as u8) * 0x04 |
        (((data[1] & D5) != 0) as u8) * 0x02 |
        (((data[1] & D4) != 0) as u8) * 0x01
    }
}


pub struct Pcf8574EncoderDefault<
    const RS: u8 = 0x01,
    const RNW:u8 = 0x02,
    const EN: u8 = 0x04,
    const BL: u8 = 0x08,
>;

impl Pcf8574EncoderDefault {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const RS:u8, const RNW:u8, const EN:u8, const BL:u8> Pcf8574EncoderTrait 
for Pcf8574EncoderDefault<RS,RNW,EN,BL> {
    fn encode<const RS_VAL:bool, const RNW_VAL:bool>(&self, bl: bool, data:u8) -> [u8; 4] {
        [EN | (RS_VAL as u8)*RS | (RNW_VAL as u8)*RNW | (bl as u8)*BL | (data & 0xf0), 
              (RS_VAL as u8)*RS | (RNW_VAL as u8)*RNW | (bl as u8)*BL | (data & 0xf0),
         EN | (RS_VAL as u8)*RS | (RNW_VAL as u8)*RNW | (bl as u8)*BL | ((data & 0x0f) << 4), 
              (RS_VAL as u8)*RS | (RNW_VAL as u8)*RNW | (bl as u8)*BL | ((data & 0x0f) << 4),]
    }
    fn decode_data(&self, data: [u8;2]) -> u8 {
        (data[0] & 0xf0) | ((data[1] & 0xf0) >> 4)
    }
}

