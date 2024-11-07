#[doc = "Register `USER` reader"]
pub type R = crate::R<USER_SPEC>;
#[doc = "Register `USER` writer"]
pub type W = crate::W<USER_SPEC>;
#[doc = "Field `DOUTDIN` reader - Set the bit to enable full duplex communication. 1: enable 0: disable."]
pub type DOUTDIN_R = crate::BitReader;
#[doc = "Field `DOUTDIN` writer - Set the bit to enable full duplex communication. 1: enable 0: disable."]
pub type DOUTDIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_HOLD` reader - spi cs keep low when spi is in ¡°done¡± phase. 1: enable 0: disable."]
pub type CS_HOLD_R = crate::BitReader;
#[doc = "Field `CS_HOLD` writer - spi cs keep low when spi is in ¡°done¡± phase. 1: enable 0: disable."]
pub type CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_SETUP` reader - spi cs is enable when spi is in ¡°prepare¡± phase. 1: enable 0: disable."]
pub type CS_SETUP_R = crate::BitReader;
#[doc = "Field `CS_SETUP` writer - spi cs is enable when spi is in ¡°prepare¡± phase. 1: enable 0: disable."]
pub type CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_I_EDGE` reader - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
pub type CK_I_EDGE_R = crate::BitReader;
#[doc = "Field `CK_I_EDGE` writer - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
pub type CK_I_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_OUT_EDGE` reader - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
pub type CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `CK_OUT_EDGE` writer - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
pub type CK_OUT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_BYTE_ORDER` reader - In read-data (MISO) phase 1: big-endian 0: little_endian"]
pub type RD_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `RD_BYTE_ORDER` writer - In read-data (MISO) phase 1: big-endian 0: little_endian"]
pub type RD_BYTE_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_BYTE_ORDER` reader - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
pub type WR_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `WR_BYTE_ORDER` writer - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
pub type WR_BYTE_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_DUAL` reader - In the write operations read-data phase apply 2 signals"]
pub type FWRITE_DUAL_R = crate::BitReader;
#[doc = "Field `FWRITE_DUAL` writer - In the write operations read-data phase apply 2 signals"]
pub type FWRITE_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_QUAD` reader - In the write operations read-data phase apply 4 signals"]
pub type FWRITE_QUAD_R = crate::BitReader;
#[doc = "Field `FWRITE_QUAD` writer - In the write operations read-data phase apply 4 signals"]
pub type FWRITE_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_DIO` reader - In the write operations address phase and read-data phase apply 2 signals."]
pub type FWRITE_DIO_R = crate::BitReader;
#[doc = "Field `FWRITE_DIO` writer - In the write operations address phase and read-data phase apply 2 signals."]
pub type FWRITE_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_QIO` reader - In the write operations address phase and read-data phase apply 4 signals."]
pub type FWRITE_QIO_R = crate::BitReader;
#[doc = "Field `FWRITE_QIO` writer - In the write operations address phase and read-data phase apply 4 signals."]
pub type FWRITE_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIO` reader - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
pub type SIO_R = crate::BitReader;
#[doc = "Field `SIO` writer - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
pub type SIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_HOLD_POL` reader - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
pub type USR_HOLD_POL_R = crate::BitReader;
#[doc = "Field `USR_HOLD_POL` writer - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
pub type USR_HOLD_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DOUT_HOLD` reader - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
pub type USR_DOUT_HOLD_R = crate::BitReader;
#[doc = "Field `USR_DOUT_HOLD` writer - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
pub type USR_DOUT_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DIN_HOLD` reader - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
pub type USR_DIN_HOLD_R = crate::BitReader;
#[doc = "Field `USR_DIN_HOLD` writer - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
pub type USR_DIN_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DUMMY_HOLD` reader - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
pub type USR_DUMMY_HOLD_R = crate::BitReader;
#[doc = "Field `USR_DUMMY_HOLD` writer - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
pub type USR_DUMMY_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_ADDR_HOLD` reader - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
pub type USR_ADDR_HOLD_R = crate::BitReader;
#[doc = "Field `USR_ADDR_HOLD` writer - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
pub type USR_ADDR_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_CMD_HOLD` reader - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
pub type USR_CMD_HOLD_R = crate::BitReader;
#[doc = "Field `USR_CMD_HOLD` writer - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
pub type USR_CMD_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_PREP_HOLD` reader - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
pub type USR_PREP_HOLD_R = crate::BitReader;
#[doc = "Field `USR_PREP_HOLD` writer - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
pub type USR_PREP_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MISO_HIGHPART` reader - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
pub type USR_MISO_HIGHPART_R = crate::BitReader;
#[doc = "Field `USR_MISO_HIGHPART` writer - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
pub type USR_MISO_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MOSI_HIGHPART` reader - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
pub type USR_MOSI_HIGHPART_R = crate::BitReader;
#[doc = "Field `USR_MOSI_HIGHPART` writer - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
pub type USR_MOSI_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable."]
pub type USR_DUMMY_IDLE_R = crate::BitReader;
#[doc = "Field `USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable."]
pub type USR_DUMMY_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MOSI` reader - This bit enable the write-data phase of an operation."]
pub type USR_MOSI_R = crate::BitReader;
#[doc = "Field `USR_MOSI` writer - This bit enable the write-data phase of an operation."]
pub type USR_MOSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MISO` reader - This bit enable the read-data phase of an operation."]
pub type USR_MISO_R = crate::BitReader;
#[doc = "Field `USR_MISO` writer - This bit enable the read-data phase of an operation."]
pub type USR_MISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DUMMY` reader - This bit enable the dummy phase of an operation."]
pub type USR_DUMMY_R = crate::BitReader;
#[doc = "Field `USR_DUMMY` writer - This bit enable the dummy phase of an operation."]
pub type USR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_ADDR` reader - This bit enable the address phase of an operation."]
pub type USR_ADDR_R = crate::BitReader;
#[doc = "Field `USR_ADDR` writer - This bit enable the address phase of an operation."]
pub type USR_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_COMMAND` reader - This bit enable the command phase of an operation."]
pub type USR_COMMAND_R = crate::BitReader;
#[doc = "Field `USR_COMMAND` writer - This bit enable the command phase of an operation."]
pub type USR_COMMAND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable."]
    #[inline(always)]
    pub fn doutdin(&self) -> DOUTDIN_R {
        DOUTDIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - spi cs keep low when spi is in ¡°done¡± phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_hold(&self) -> CS_HOLD_R {
        CS_HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - spi cs is enable when spi is in ¡°prepare¡± phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_setup(&self) -> CS_SETUP_R {
        CS_SETUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
    #[inline(always)]
    pub fn ck_i_edge(&self) -> CK_I_EDGE_R {
        CK_I_EDGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - In read-data (MISO) phase 1: big-endian 0: little_endian"]
    #[inline(always)]
    pub fn rd_byte_order(&self) -> RD_BYTE_ORDER_R {
        RD_BYTE_ORDER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
    #[inline(always)]
    pub fn wr_byte_order(&self) -> WR_BYTE_ORDER_R {
        WR_BYTE_ORDER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    pub fn fwrite_dual(&self) -> FWRITE_DUAL_R {
        FWRITE_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    pub fn fwrite_quad(&self) -> FWRITE_QUAD_R {
        FWRITE_QUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    pub fn fwrite_dio(&self) -> FWRITE_DIO_R {
        FWRITE_DIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    pub fn fwrite_qio(&self) -> FWRITE_QIO_R {
        FWRITE_QIO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
    #[inline(always)]
    pub fn usr_hold_pol(&self) -> USR_HOLD_POL_R {
        USR_HOLD_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_dout_hold(&self) -> USR_DOUT_HOLD_R {
        USR_DOUT_HOLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_din_hold(&self) -> USR_DIN_HOLD_R {
        USR_DIN_HOLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_dummy_hold(&self) -> USR_DUMMY_HOLD_R {
        USR_DUMMY_HOLD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_addr_hold(&self) -> USR_ADDR_HOLD_R {
        USR_ADDR_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_cmd_hold(&self) -> USR_CMD_HOLD_R {
        USR_CMD_HOLD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_prep_hold(&self) -> USR_PREP_HOLD_R {
        USR_PREP_HOLD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_miso_highpart(&self) -> USR_MISO_HIGHPART_R {
        USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&self) -> USR_MOSI_HIGHPART_R {
        USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    pub fn usr_mosi(&self) -> USR_MOSI_R {
        USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    pub fn usr_miso(&self) -> USR_MISO_R {
        USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    pub fn usr_addr(&self) -> USR_ADDR_R {
        USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    pub fn usr_command(&self) -> USR_COMMAND_R {
        USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER")
            .field("doutdin", &self.doutdin())
            .field("cs_hold", &self.cs_hold())
            .field("cs_setup", &self.cs_setup())
            .field("ck_i_edge", &self.ck_i_edge())
            .field("ck_out_edge", &self.ck_out_edge())
            .field("rd_byte_order", &self.rd_byte_order())
            .field("wr_byte_order", &self.wr_byte_order())
            .field("fwrite_dual", &self.fwrite_dual())
            .field("fwrite_quad", &self.fwrite_quad())
            .field("fwrite_dio", &self.fwrite_dio())
            .field("fwrite_qio", &self.fwrite_qio())
            .field("sio", &self.sio())
            .field("usr_hold_pol", &self.usr_hold_pol())
            .field("usr_dout_hold", &self.usr_dout_hold())
            .field("usr_din_hold", &self.usr_din_hold())
            .field("usr_dummy_hold", &self.usr_dummy_hold())
            .field("usr_addr_hold", &self.usr_addr_hold())
            .field("usr_cmd_hold", &self.usr_cmd_hold())
            .field("usr_prep_hold", &self.usr_prep_hold())
            .field("usr_miso_highpart", &self.usr_miso_highpart())
            .field("usr_mosi_highpart", &self.usr_mosi_highpart())
            .field("usr_dummy_idle", &self.usr_dummy_idle())
            .field("usr_mosi", &self.usr_mosi())
            .field("usr_miso", &self.usr_miso())
            .field("usr_dummy", &self.usr_dummy())
            .field("usr_addr", &self.usr_addr())
            .field("usr_command", &self.usr_command())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable."]
    #[inline(always)]
    pub fn doutdin(&mut self) -> DOUTDIN_W<USER_SPEC> {
        DOUTDIN_W::new(self, 0)
    }
    #[doc = "Bit 4 - spi cs keep low when spi is in ¡°done¡± phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_hold(&mut self) -> CS_HOLD_W<USER_SPEC> {
        CS_HOLD_W::new(self, 4)
    }
    #[doc = "Bit 5 - spi cs is enable when spi is in ¡°prepare¡± phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_setup(&mut self) -> CS_SETUP_W<USER_SPEC> {
        CS_SETUP_W::new(self, 5)
    }
    #[doc = "Bit 6 - In the slave mode the bit is same as spi_ck_out_edge in master mode. It is combined with spi_miso_delay_mode bits."]
    #[inline(always)]
    pub fn ck_i_edge(&mut self) -> CK_I_EDGE_W<USER_SPEC> {
        CK_I_EDGE_W::new(self, 6)
    }
    #[doc = "Bit 7 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W<USER_SPEC> {
        CK_OUT_EDGE_W::new(self, 7)
    }
    #[doc = "Bit 10 - In read-data (MISO) phase 1: big-endian 0: little_endian"]
    #[inline(always)]
    pub fn rd_byte_order(&mut self) -> RD_BYTE_ORDER_W<USER_SPEC> {
        RD_BYTE_ORDER_W::new(self, 10)
    }
    #[doc = "Bit 11 - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian"]
    #[inline(always)]
    pub fn wr_byte_order(&mut self) -> WR_BYTE_ORDER_W<USER_SPEC> {
        WR_BYTE_ORDER_W::new(self, 11)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    pub fn fwrite_dual(&mut self) -> FWRITE_DUAL_W<USER_SPEC> {
        FWRITE_DUAL_W::new(self, 12)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    pub fn fwrite_quad(&mut self) -> FWRITE_QUAD_W<USER_SPEC> {
        FWRITE_QUAD_W::new(self, 13)
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    pub fn fwrite_dio(&mut self) -> FWRITE_DIO_W<USER_SPEC> {
        FWRITE_DIO_W::new(self, 14)
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    pub fn fwrite_qio(&mut self) -> FWRITE_QIO_W<USER_SPEC> {
        FWRITE_QIO_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sio(&mut self) -> SIO_W<USER_SPEC> {
        SIO_W::new(self, 16)
    }
    #[doc = "Bit 17 - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low"]
    #[inline(always)]
    pub fn usr_hold_pol(&mut self) -> USR_HOLD_POL_W<USER_SPEC> {
        USR_HOLD_POL_W::new(self, 17)
    }
    #[doc = "Bit 18 - spi is hold at data out state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_dout_hold(&mut self) -> USR_DOUT_HOLD_W<USER_SPEC> {
        USR_DOUT_HOLD_W::new(self, 18)
    }
    #[doc = "Bit 19 - spi is hold at data in state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_din_hold(&mut self) -> USR_DIN_HOLD_W<USER_SPEC> {
        USR_DIN_HOLD_W::new(self, 19)
    }
    #[doc = "Bit 20 - spi is hold at dummy state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_dummy_hold(&mut self) -> USR_DUMMY_HOLD_W<USER_SPEC> {
        USR_DUMMY_HOLD_W::new(self, 20)
    }
    #[doc = "Bit 21 - spi is hold at address state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_addr_hold(&mut self) -> USR_ADDR_HOLD_W<USER_SPEC> {
        USR_ADDR_HOLD_W::new(self, 21)
    }
    #[doc = "Bit 22 - spi is hold at command state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_cmd_hold(&mut self) -> USR_CMD_HOLD_W<USER_SPEC> {
        USR_CMD_HOLD_W::new(self, 22)
    }
    #[doc = "Bit 23 - spi is hold at prepare state the bit combined with spi_usr_hold_pol bit."]
    #[inline(always)]
    pub fn usr_prep_hold(&mut self) -> USR_PREP_HOLD_W<USER_SPEC> {
        USR_PREP_HOLD_W::new(self, 23)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_miso_highpart(&mut self) -> USR_MISO_HIGHPART_W<USER_SPEC> {
        USR_MISO_HIGHPART_W::new(self, 24)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&mut self) -> USR_MOSI_HIGHPART_W<USER_SPEC> {
        USR_MOSI_HIGHPART_W::new(self, 25)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W<USER_SPEC> {
        USR_DUMMY_IDLE_W::new(self, 26)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    pub fn usr_mosi(&mut self) -> USR_MOSI_W<USER_SPEC> {
        USR_MOSI_W::new(self, 27)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    pub fn usr_miso(&mut self) -> USR_MISO_W<USER_SPEC> {
        USR_MISO_W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W<USER_SPEC> {
        USR_DUMMY_W::new(self, 29)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    pub fn usr_addr(&mut self) -> USR_ADDR_W<USER_SPEC> {
        USR_ADDR_W::new(self, 30)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    pub fn usr_command(&mut self) -> USR_COMMAND_W<USER_SPEC> {
        USR_COMMAND_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user::R`](R) reader structure"]
impl crate::Readable for USER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`user::W`](W) writer structure"]
impl crate::Writable for USER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USER to value 0x8000_0040"]
impl crate::Resettable for USER_SPEC {
    const RESET_VALUE: u32 = 0x8000_0040;
}
