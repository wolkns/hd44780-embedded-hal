pub struct Gpio4BitModeInterface<PINOUT, PINBIDIR, DELAY>
where
    PINOUT : OutputPin + Sized,
    PINBIDIR : OutputPin + InputPin + Sized,
    DELAY : delay::DelayNs,
{
    pin_rs  : PINOUT,
    pin_rnw : PINOUT,
    pin_en  : PINOUT,
    pin_bl  : PINOUT,
    pin_d4  : PINBIDIR,
    pin_d5  : PINBIDIR,
    pin_d6  : PINBIDIR,
    pin_d7  : PINBIDIR,
    delay : DELAY,
}

impl<PINOUT, PINBIDIR, DELAY> Gpio4BitModeInterface<PINOUT, PINBIDIR, DELAY>
where 
    PINOUT : OutputPin + Sized,
    PINBIDIR : OutputPin + InputPin + Sized,
    DELAY : delay::DelayNs,
{
    #[inline]
    pub fn new(
        pin_rs : PINOUT,
        pin_rnw : PINOUT,
        pin_en : PINOUT,
        pin_bl  : PINOUT,
        pin_d4  : PINBIDIR,
        pin_d5  : PINBIDIR,
        pin_d6  : PINBIDIR,
        pin_d7  : PINBIDIR,
        delay : DELAY
    ) -> Self {
        Self {
            pin_rs,
            pin_rnw,
            pin_en,
            pin_bl,
            pin_d4,
            pin_d5,
            pin_d6,
            pin_d7,
            delay,
        }
    }

    #[inline]
    fn enable(
        &mut self
    ) -> Result<(), InterfaceError>
    {
        self.pin_en.set_high().map_err(|_| InterfaceError::GpioError)
    }

    #[inline]
    fn disable(
        &mut self
    ) -> Result<(), InterfaceError>
    {
        self.pin_en.set_low().map_err(|_| InterfaceError::GpioError)
    }

    #[inline]
    fn set_mode_read(
        &mut self
    ) -> Result<(), InterfaceError>
    {
        self.pin_rnw.set_high().map_err(|_| InterfaceError::GpioError)
    }
    
    #[inline]
    fn set_mode_write(
        &mut self
    ) -> Result<(), InterfaceError>
    {
        self.pin_rnw.set_low().map_err(|_| InterfaceError::GpioError)
    }
    
    #[inline]
    fn set_nibble(
        &mut self, 
        data:u8
    ) -> Result<(), InterfaceError>
    {
        self.pin_d7.set_state((data & 0x08 != 0).into())
            .map_err(|_| InterfaceError::GpioError)?;
        self.pin_d6.set_state((data & 0x04 != 0).into())
            .map_err(|_| InterfaceError::GpioError)?;
        self.pin_d5.set_state((data & 0x02 != 0).into())
            .map_err(|_| InterfaceError::GpioError)?;
        self.pin_d4.set_state((data & 0x01 != 0).into())
            .map_err(|_| InterfaceError::GpioError)
    }
    
    #[inline]
    fn reset_nibble(
        &mut self
    ) -> Result<(), InterfaceError>
    {
        self.pin_d7.set_high().map_err(|_| InterfaceError::GpioError)?;
        self.pin_d6.set_high().map_err(|_| InterfaceError::GpioError)?;
        self.pin_d5.set_high().map_err(|_| InterfaceError::GpioError)?;
        self.pin_d4.set_high().map_err(|_| InterfaceError::GpioError)
    }

    #[inline]
    fn reset_pins(
        &mut self
    ) -> Result<(), InterfaceError>
    {
        self.pin_en.set_low().map_err(|_| InterfaceError::GpioError)?;

        self.pin_rs.set_low().map_err(|_| InterfaceError::GpioError)?;
        self.pin_rnw.set_low().map_err(|_| InterfaceError::GpioError)?;

        self.reset_nibble()
    }
}


#[cfg(not(feature="async"))]
impl<PINOUT, PINBIDIR, DELAY> Gpio4BitModeInterface<PINOUT, PINBIDIR, DELAY>
where
    PINOUT : OutputPin + Sized,
    PINBIDIR : OutputPin + InputPin + Sized,
    DELAY : delay::DelayNs,
{
    fn send_nibble<const RS_VAL:bool>(
        &mut self, 
        data:u8
    ) -> Result<(), InterfaceError>
    {
        self.delay.delay_us(9*10);

        self.pin_rs.set_state(RS_VAL.into())
            .map_err(|_| InterfaceError::GpioError)?;
        self.set_nibble(data)?;
        self.enable()?;

        self.delay.delay_us(9*10);

        self.disable()
    }
    
    fn receive_nibble<const RS_VAL:bool>(
        &mut self
    ) -> Result<u8, InterfaceError>
    {
        self.delay.delay_us(9*10);

        self.reset_nibble()?;
        self.pin_rs.set_state(RS_VAL.into())
            .map_err(|_| InterfaceError::GpioError)?;
        self.enable()?;

        self.delay.delay_us(9*10);

        let data : u8 = 
            (self.pin_d7.is_high()
                .map_err(|_| InterfaceError::GpioError)? as u8) << 3 |
            (self.pin_d6.is_high()
                .map_err(|_| InterfaceError::GpioError)? as u8) << 2 |
            (self.pin_d5.is_high()
                .map_err(|_| InterfaceError::GpioError)? as u8) << 1 |
            (self.pin_d4.is_high()
                .map_err(|_| InterfaceError::GpioError)? as u8);
        
        self.disable()?;

        Ok(data)
    }
}

#[cfg(not(feature="async"))]
impl<PINOUT, PINBIDIR, DELAY> InterfaceTrait for Gpio4BitModeInterface<PINOUT, PINBIDIR, DELAY>
where 
    PINOUT : OutputPin + Sized,
    PINBIDIR : OutputPin + InputPin + Sized,
    DELAY : delay::DelayNs,
{
    fn init(
        &mut self, 
        fnset_lines:FnsetLines, 
        fnset_font:FnsetFont
    ) -> Result<(), InterfaceError> 
    {
        self.reset_pins()?;
        self.delay.delay_us(9*10);

        self.send_nibble::<false>(
            (CmdOptions::Fnset as u8 | FnsetDataLen::Bit8 as u8) >> 4
        )?;

        self.delay.delay_us(4_100);

        self.send_nibble::<false>(
            (CmdOptions::Fnset as u8 | FnsetDataLen::Bit8 as u8) >> 4
        )?;
        
        self.delay.delay_us(100);

        self.send_nibble::<false>(
            (CmdOptions::Fnset as u8 | FnsetDataLen::Bit8 as u8) >> 4
        )?;

        self.send_nibble::<false>(
            (CmdOptions::Fnset as u8 | FnsetDataLen::Bit4 as u8) >> 4
        )?;

        // now in 4-bit mode

        self.send_byte::<false>(
            CmdOptions::Fnset as u8 | FnsetDataLen::Bit4 as u8 | fnset_lines as u8 | fnset_font as u8
        )
    }
    
    fn send_byte<const RS_VAL:bool>(
        &mut self, 
        byte: u8
    ) -> Result<(), InterfaceError> 
    {
        self.set_mode_write()?;
        // most significant nibble
        self.send_nibble::<RS_VAL>(byte >> 4)?;
        // least significant nibble
        self.send_nibble::<RS_VAL>(byte)
    }

    fn receive_byte<const RS_VAL:bool>(
        &mut self, 
        byte: &mut u8
    ) -> Result<(), InterfaceError> 
    {
        self.set_mode_read()?;
        let data : u8 =
            (self.receive_nibble::<RS_VAL>()? << 4) |
             self.receive_nibble::<RS_VAL>()?;
        *byte = data;

        Ok(())
    }

    fn backlight(
        &mut self, 
        bl:bool
    ) -> Result<(), InterfaceError>
    {
        self.pin_bl.set_state(bl.into())
            .map_err(|_| InterfaceError::GpioError)
    }

    fn delay_us(
        &mut self, 
        us:u32
    ) {
        self.delay.delay_us(us);
    }
}

#[cfg(feature="async")]
impl<PINOUT, PINBIDIR, DELAY> Gpio4BitModeInterface<PINOUT, PINBIDIR, DELAY>
where
    PINOUT : OutputPin + Sized,
    PINBIDIR : OutputPin + InputPin + Sized,
    DELAY : delay::DelayNs,
{
    async fn send_nibble<const RS_VAL:bool>(
        &mut self, 
        data:u8
    ) -> Result<(), InterfaceError>
    {
        self.delay.delay_us(9*10).await;

        self.pin_rs.set_state(RS_VAL.into())
            .map_err(|_| InterfaceError::GpioError)?;
        self.set_nibble(data)?;
        self.enable()?;

        self.delay.delay_us(9*10).await;

        self.disable()
    }
    
    async fn receive_nibble<const RS_VAL:bool>(
        &mut self
    ) -> Result<u8, InterfaceError>
    {
        self.delay.delay_us(9*10).await;

        self.reset_nibble()?;
        self.pin_rs.set_state(RS_VAL.into())
            .map_err(|_| InterfaceError::GpioError)?;
        self.enable()?;

        self.delay.delay_us(9*10).await;

        let data : u8 = 
            (self.pin_d7.is_high()
                .map_err(|_| InterfaceError::GpioError)? as u8) << 3 |
            (self.pin_d6.is_high()
                .map_err(|_| InterfaceError::GpioError)? as u8) << 2 |
            (self.pin_d5.is_high()
                .map_err(|_| InterfaceError::GpioError)? as u8) << 1 |
            (self.pin_d4.is_high()
                .map_err(|_| InterfaceError::GpioError)? as u8);
        
        self.disable()?;

        Ok(data)
    }
}

#[cfg(feature="async")]
impl<PINOUT, PINBIDIR, DELAY> InterfaceTrait for Gpio4BitModeInterface<PINOUT, PINBIDIR, DELAY>
where 
    PINOUT : OutputPin + Sized,
    PINBIDIR : OutputPin + InputPin + Sized,
    DELAY : delay::DelayNs,
{
    async fn init(
        &mut self, 
        fnset_lines : FnsetLines, 
        fnset_font : FnsetFont,
    ) -> Result<(), InterfaceError>
    {
        self.reset_pins()?;
        self.delay.delay_us(9*10).await;

        self.send_nibble::<false>(
            (CmdOptions::Fnset as u8 | FnsetDataLen::Bit8 as u8) >> 4
        ).await?;

        self.delay.delay_us(4_100).await;

        self.send_nibble::<false>(
            (CmdOptions::Fnset as u8 | FnsetDataLen::Bit8 as u8) >> 4
        ).await?;

        self.delay.delay_us(100).await;

        self.send_nibble::<false>(
            (CmdOptions::Fnset as u8 | FnsetDataLen::Bit8 as u8) >> 4
        ).await?;

        self.send_nibble::<false>(
            (CmdOptions::Fnset as u8 | FnsetDataLen::Bit4 as u8) >> 4
        ).await?;

        // now in 4-bit mode

        self.send_byte::<false>(
            CmdOptions::Fnset as u8 | FnsetDataLen::Bit4 as u8 | fnset_lines as u8 | fnset_font as u8
        ).await
    }

    async fn send_byte<const RS_VAL:bool>(
            &mut self, 
            byte: u8
    ) -> Result<(), InterfaceError> {
        self.set_mode_write()?;

        // most significant nibble
        self.send_nibble::<RS_VAL>(byte >> 4).await?;
        // least significant nibble
        self.send_nibble::<RS_VAL>(byte).await
    }

    async fn receive_byte<const RS_VAL:bool>(
            &mut self, 
            byte: &mut u8
    ) -> Result<(), InterfaceError> {
        self.set_mode_read()?;

        let data : u8 = 
            (self.receive_nibble::<RS_VAL>().await? << 4) |
             self.receive_nibble::<RS_VAL>().await?;
        *byte = data;

        Ok(())
    }

    async fn backlight(
        &mut self, 
        bl:bool
    ) -> Result<(), InterfaceError> {
        self.pin_bl.set_state(bl.into())
            .map_err(|_| InterfaceError::GpioError)
    }

    async fn delay_us(
        &mut self, 
        us:u32
    ) -> () {
        self.delay.delay_us(us).await;
    }
}
