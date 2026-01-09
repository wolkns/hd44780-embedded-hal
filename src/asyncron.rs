
use crate::{
    Hd44780, Hd44780Error, instructions::*, interface, lcd_write, types
};
use crate::buffer::UnsafeBuffer;


const FMT_BUFFER_SIZE:usize = 64;
const PING_DDRAM_ADDR : u8     = 0x13;
const PING_DDRAM_ADDR_ALT : u8 = 0x0e;

pub trait Hd44780Trait<DPTYPE>
where DPTYPE: types::DisplayTypeTrait,
{
    async fn init(&mut self) -> Result<(), Hd44780Error>;

    async fn create_char(
        &mut self,
        charcode: DPTYPE::Font,
        charmap: DPTYPE::CharMap
    ) -> Result<(), Hd44780Error>;

    async fn clear(&mut self) -> Result<(), Hd44780Error>;
    async fn home(&mut self) -> Result<(), Hd44780Error>;
    async fn entry(&mut self, dir:EntryDir, ads:EntryAds) -> Result<(), Hd44780Error>;
    async fn display(&mut self, state:DpState, cursor:DpCursor, blink:DpBlink) -> Result<(), Hd44780Error>;
    async fn shift(&mut self, dp_type:ShiftType, dir:ShiftDir) -> Result<(), Hd44780Error>;
    async fn position(&mut self, row:u8, col:u8) -> Result<(), Hd44780Error>;

    async fn print_bytes(&mut self, bytes:&[u8]) -> Result<(), Hd44780Error>;
    async fn print_str(&mut self, string:&str) -> Result<(), Hd44780Error> {
        self.print_bytes(string.as_bytes()).await?;
        Ok(())
    }
    async fn print_fmt(&mut self, args:core::fmt::Arguments<'_>) -> Result<(), Hd44780Error> {
        let mut data:[u8;FMT_BUFFER_SIZE] = [0;FMT_BUFFER_SIZE];
        let mut buf = UnsafeBuffer::new(&mut data);

        match core::fmt::write(&mut buf, args)
        {
            Err(e) => Err(Hd44780Error::FmtError(e)),
            Ok(_) => self.print_str(buf.as_str()).await
        }
    }

    async fn backlight(&mut self, bl:bool) -> Result<(), Hd44780Error>;

    async fn read_data(&mut self, buffer:&mut [u8]) -> Result<(), Hd44780Error>;
    async fn read_address_counter(&mut self) -> Result<u8, Hd44780Error>;
    async fn is_busy(&mut self) -> Result<bool, Hd44780Error>;

    async fn ping(&mut self) -> Result<bool, Hd44780Error>;
}


impl<INTERFACE, DPTYPE> Hd44780Trait<DPTYPE> 
    for Hd44780<INTERFACE, DPTYPE>
where
    INTERFACE: interface::InterfaceTrait,
    DPTYPE: types::DisplayTypeTrait,
{
    async fn init(&mut self) -> Result<(), Hd44780Error> {
        self.interface.init(
            self.dp_type.lines(), 
            self.dp_type.font()
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;

        self.display(DpState::On, DpCursor::Off, DpBlink::Off).await?;
        self.clear().await
    }

    async fn create_char(
        &mut self,
        charcode: DPTYPE::Font,
        charmap: DPTYPE::CharMap
    ) -> Result<(), Hd44780Error> 
    {
        // Set EntryMode to increment and turn off Accompanies display shift.
        self.entry(EntryDir::Inc, EntryAds::Off).await?;

        self.interface.send_byte::<false>(
            CmdOptions::SetCg as u8 | DPTYPE::into_cgram(charcode)
        ).await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        self.interface.send_bytes::<true>(charmap.as_ref()).await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )
    }

    async fn clear(&mut self) -> Result<(), Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Clear as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;
        self.interface.delay_us(1_520).await;
        Ok(())
    }

    async fn home(&mut self) -> Result<(), Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Home as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;
        self.interface.delay_us(1_520).await;
        Ok(())
    }

    async fn entry(&mut self, dir:EntryDir, ads:EntryAds) -> Result<(), Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Entry as u8 | dir as u8 | ads as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))
    }

    async fn display(&mut self, state:DpState, cursor:DpCursor, blink:DpBlink) -> Result<(), Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Dp as u8 | state as u8 | cursor as u8 | blink as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))
    }

    async fn shift(&mut self, dp_type:ShiftType, dir:ShiftDir) -> Result<(), Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Shift as u8 | dp_type as u8 | dir as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))
    }

    async fn position(&mut self, row:u8, col:u8) -> Result<(), Hd44780Error> {
        if row >= self.dp_type.rows() || col >= self.dp_type.cols() {
            return Err(Hd44780Error::RowColOutOfRange);
        }

        let dd: u8 = match self.dp_type.lines() {
            FnsetLines::One => self.dp_type.cols() * row + col,
            FnsetLines::Two => 0x40 * (row % 2) + self.dp_type.cols() * (row / 2) + col,
        };
        self.interface.send_byte::<false>(
            CmdOptions::SetDd as u8 | dd
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))
    }

    async fn print_bytes(&mut self, bytes:&[u8]) -> Result<(), Hd44780Error> {
        self.interface.send_bytes::<true>(
            bytes
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))
    }

    async fn backlight(&mut self, bl:bool) -> Result<(), Hd44780Error> {
        self.interface.backlight(bl)
        .await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )
    }

    async fn read_data(&mut self, buffer:&mut [u8]) -> Result<(), Hd44780Error> {
        self.interface.receive_bytes::<true>(buffer)
        .await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )
    }

    async fn read_address_counter(&mut self) -> Result<u8, Hd44780Error> {
        let mut ac: u8 = 0;
        self.interface.receive_byte::<false>(&mut ac)
        .await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(ac & 0x7f)
    }

    async fn is_busy(&mut self) -> Result<bool, Hd44780Error> {
        let mut ac: u8 = 0;
        self.interface.receive_byte::<false>(&mut ac)
        .await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok((ac & 0x80) != 0)
    }
    
    /// Pings the lcd controller by: 
    ///  -> reading ac value
    ///  -> changeing ac value
    ///  -> reading ac value
    ///  -> compare if the change happend
    /// If successfull it writes the initialy read adress counter
    /// back as DDRAM address.

    /// WARNING!!!!: If you had set the CGRAM before, 
    /// you may encounter unwanted behaviour.
    /// CGRAM is set create_char().
    async fn ping(&mut self) -> Result<bool, Hd44780Error> {
        // init read address counter
        let previous_ac: u8 = self.read_address_counter().await?;
        
        // change ac
        let new_ac = match previous_ac == PING_DDRAM_ADDR {
            true => PING_DDRAM_ADDR_ALT,
            false => PING_DDRAM_ADDR
        };
        self.interface.send_byte::<false>(
            CmdOptions::SetDd as u8 | new_ac
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;

        let read_new_ac = self.read_address_counter().await?;

        // read again and compare
        if new_ac != read_new_ac {
            return Ok(false);
        }

        // restore previous ac as DDRAM address
        self.interface.send_byte::<false>(
            CmdOptions::SetDd as u8 | previous_ac
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;

        Ok(true)
    }
}

