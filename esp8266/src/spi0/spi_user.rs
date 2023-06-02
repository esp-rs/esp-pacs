#[doc = "Register `SPI_USER` reader"]
pub struct R(crate::R<SPI_USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_USER` writer"]
pub struct W(crate::W<SPI_USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_duplex` reader - set spi in full duplex mode"]
pub type SPI_DUPLEX_R = crate::BitReader;
#[doc = "Field `spi_duplex` writer - set spi in full duplex mode"]
pub type SPI_DUPLEX_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_ahb_user_command_4byte` reader - reserved"]
pub type SPI_AHB_USER_COMMAND_4BYTE_R = crate::BitReader;
#[doc = "Field `spi_ahb_user_command_4byte` writer - reserved"]
pub type SPI_AHB_USER_COMMAND_4BYTE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_flash_mode` reader - "]
pub type SPI_FLASH_MODE_R = crate::BitReader;
#[doc = "Field `spi_flash_mode` writer - "]
pub type SPI_FLASH_MODE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_ahb_user_command` reader - reserved"]
pub type SPI_AHB_USER_COMMAND_R = crate::BitReader;
#[doc = "Field `spi_ahb_user_command` writer - reserved"]
pub type SPI_AHB_USER_COMMAND_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_cs_hold` reader - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type SPI_CS_HOLD_R = crate::BitReader;
#[doc = "Field `spi_cs_hold` writer - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type SPI_CS_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_cs_setup` reader - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type SPI_CS_SETUP_R = crate::BitReader;
#[doc = "Field `spi_cs_setup` writer - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type SPI_CS_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_ck_i_edge` reader - In the slave mode, 1: rising-edge; 0: falling-edge"]
pub type SPI_CK_I_EDGE_R = crate::BitReader;
#[doc = "Field `spi_ck_i_edge` writer - In the slave mode, 1: rising-edge; 0: falling-edge"]
pub type SPI_CK_I_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_ck_o_edge` reader - In the master mode, 1: rising-edge; 0: falling-edge"]
pub type SPI_CK_O_EDGE_R = crate::BitReader;
#[doc = "Field `spi_ck_o_edge` writer - In the master mode, 1: rising-edge; 0: falling-edge"]
pub type SPI_CK_O_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_rd_byte_order` reader - In \"read-data\" (MISO) phase, 1: little-endian; 0: big_endian"]
pub type SPI_RD_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `spi_rd_byte_order` writer - In \"read-data\" (MISO) phase, 1: little-endian; 0: big_endian"]
pub type SPI_RD_BYTE_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_wr_byte_order` reader - In \"command\", \"address\", \"write-data\" (MOSI) phases, 1: little-endian; 0: big_endian"]
pub type SPI_WR_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `spi_wr_byte_order` writer - In \"command\", \"address\", \"write-data\" (MOSI) phases, 1: little-endian; 0: big_endian"]
pub type SPI_WR_BYTE_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_fwrite_dual` reader - In the write operations, \"read-data\" phase apply 2 signals"]
pub type SPI_FWRITE_DUAL_R = crate::BitReader;
#[doc = "Field `spi_fwrite_dual` writer - In the write operations, \"read-data\" phase apply 2 signals"]
pub type SPI_FWRITE_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_fwrite_quad` reader - In the write operations, \"read-data\" phase apply 4 signals"]
pub type SPI_FWRITE_QUAD_R = crate::BitReader;
#[doc = "Field `spi_fwrite_quad` writer - In the write operations, \"read-data\" phase apply 4 signals"]
pub type SPI_FWRITE_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_fwrite_dio` reader - In the write operations, \"address\" phase and \"read-data\" phase apply 2 signals"]
pub type SPI_FWRITE_DIO_R = crate::BitReader;
#[doc = "Field `spi_fwrite_dio` writer - In the write operations, \"address\" phase and \"read-data\" phase apply 2 signals"]
pub type SPI_FWRITE_DIO_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_fwrite_qio` reader - In the write operations, \"address\" phase and \"read-data\" phase apply 4 signals"]
pub type SPI_FWRITE_QIO_R = crate::BitReader;
#[doc = "Field `spi_fwrite_qio` writer - In the write operations, \"address\" phase and \"read-data\" phase apply 4 signals"]
pub type SPI_FWRITE_QIO_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_sio` reader - 1: mosi and miso signals share the same pin"]
pub type SPI_SIO_R = crate::BitReader;
#[doc = "Field `spi_sio` writer - 1: mosi and miso signals share the same pin"]
pub type SPI_SIO_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `reg_usr_miso_highpart` reader - 1: \"read-data\" phase only access to high-part of the buffer spi_w8~spi_w15"]
pub type REG_USR_MISO_HIGHPART_R = crate::BitReader;
#[doc = "Field `reg_usr_miso_highpart` writer - 1: \"read-data\" phase only access to high-part of the buffer spi_w8~spi_w15"]
pub type REG_USR_MISO_HIGHPART_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `reg_usr_mosi_highpart` reader - 1: \"write-data\" phase only access to high-part of the buffer spi_w8~spi_w15"]
pub type REG_USR_MOSI_HIGHPART_R = crate::BitReader;
#[doc = "Field `reg_usr_mosi_highpart` writer - 1: \"write-data\" phase only access to high-part of the buffer spi_w8~spi_w15"]
pub type REG_USR_MOSI_HIGHPART_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_usr_mosi` reader - This bit enable the \"write-data\" phase of an operation."]
pub type SPI_USR_MOSI_R = crate::BitReader;
#[doc = "Field `spi_usr_mosi` writer - This bit enable the \"write-data\" phase of an operation."]
pub type SPI_USR_MOSI_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_usr_miso` reader - This bit enable the \"read-data\" phase of an operation."]
pub type SPI_USR_MISO_R = crate::BitReader;
#[doc = "Field `spi_usr_miso` writer - This bit enable the \"read-data\" phase of an operation."]
pub type SPI_USR_MISO_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_usr_dummy` reader - This bit enable the \"dummy\" phase of an operation."]
pub type SPI_USR_DUMMY_R = crate::BitReader;
#[doc = "Field `spi_usr_dummy` writer - This bit enable the \"dummy\" phase of an operation."]
pub type SPI_USR_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_usr_addr` reader - This bit enable the \"address\" phase of an operation."]
pub type SPI_USR_ADDR_R = crate::BitReader;
#[doc = "Field `spi_usr_addr` writer - This bit enable the \"address\" phase of an operation."]
pub type SPI_USR_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
#[doc = "Field `spi_usr_command` reader - This bit enable the \"command\" phase of an operation."]
pub type SPI_USR_COMMAND_R = crate::BitReader;
#[doc = "Field `spi_usr_command` writer - This bit enable the \"command\" phase of an operation."]
pub type SPI_USR_COMMAND_W<'a, const O: u8> = crate::BitWriter<'a, SPI_USER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - set spi in full duplex mode"]
    #[inline(always)]
    pub fn spi_duplex(&self) -> SPI_DUPLEX_R {
        SPI_DUPLEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn spi_ahb_user_command_4byte(&self) -> SPI_AHB_USER_COMMAND_4BYTE_R {
        SPI_AHB_USER_COMMAND_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_flash_mode(&self) -> SPI_FLASH_MODE_R {
        SPI_FLASH_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn spi_ahb_user_command(&self) -> SPI_AHB_USER_COMMAND_R {
        SPI_AHB_USER_COMMAND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_cs_hold(&self) -> SPI_CS_HOLD_R {
        SPI_CS_HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_cs_setup(&self) -> SPI_CS_SETUP_R {
        SPI_CS_SETUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - In the slave mode, 1: rising-edge; 0: falling-edge"]
    #[inline(always)]
    pub fn spi_ck_i_edge(&self) -> SPI_CK_I_EDGE_R {
        SPI_CK_I_EDGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - In the master mode, 1: rising-edge; 0: falling-edge"]
    #[inline(always)]
    pub fn spi_ck_o_edge(&self) -> SPI_CK_O_EDGE_R {
        SPI_CK_O_EDGE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - In \"read-data\" (MISO) phase, 1: little-endian; 0: big_endian"]
    #[inline(always)]
    pub fn spi_rd_byte_order(&self) -> SPI_RD_BYTE_ORDER_R {
        SPI_RD_BYTE_ORDER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - In \"command\", \"address\", \"write-data\" (MOSI) phases, 1: little-endian; 0: big_endian"]
    #[inline(always)]
    pub fn spi_wr_byte_order(&self) -> SPI_WR_BYTE_ORDER_R {
        SPI_WR_BYTE_ORDER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - In the write operations, \"read-data\" phase apply 2 signals"]
    #[inline(always)]
    pub fn spi_fwrite_dual(&self) -> SPI_FWRITE_DUAL_R {
        SPI_FWRITE_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In the write operations, \"read-data\" phase apply 4 signals"]
    #[inline(always)]
    pub fn spi_fwrite_quad(&self) -> SPI_FWRITE_QUAD_R {
        SPI_FWRITE_QUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the write operations, \"address\" phase and \"read-data\" phase apply 2 signals"]
    #[inline(always)]
    pub fn spi_fwrite_dio(&self) -> SPI_FWRITE_DIO_R {
        SPI_FWRITE_DIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - In the write operations, \"address\" phase and \"read-data\" phase apply 4 signals"]
    #[inline(always)]
    pub fn spi_fwrite_qio(&self) -> SPI_FWRITE_QIO_R {
        SPI_FWRITE_QIO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: mosi and miso signals share the same pin"]
    #[inline(always)]
    pub fn spi_sio(&self) -> SPI_SIO_R {
        SPI_SIO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: \"read-data\" phase only access to high-part of the buffer spi_w8~spi_w15"]
    #[inline(always)]
    pub fn reg_usr_miso_highpart(&self) -> REG_USR_MISO_HIGHPART_R {
        REG_USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: \"write-data\" phase only access to high-part of the buffer spi_w8~spi_w15"]
    #[inline(always)]
    pub fn reg_usr_mosi_highpart(&self) -> REG_USR_MOSI_HIGHPART_R {
        REG_USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit enable the \"write-data\" phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_mosi(&self) -> SPI_USR_MOSI_R {
        SPI_USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the \"read-data\" phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_miso(&self) -> SPI_USR_MISO_R {
        SPI_USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the \"dummy\" phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_dummy(&self) -> SPI_USR_DUMMY_R {
        SPI_USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit enable the \"address\" phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_addr(&self) -> SPI_USR_ADDR_R {
        SPI_USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enable the \"command\" phase of an operation."]
    #[inline(always)]
    pub fn spi_usr_command(&self) -> SPI_USR_COMMAND_R {
        SPI_USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_USER")
            .field(
                "spi_usr_command",
                &format_args!("{}", self.spi_usr_command().bit()),
            )
            .field(
                "spi_usr_addr",
                &format_args!("{}", self.spi_usr_addr().bit()),
            )
            .field(
                "spi_usr_dummy",
                &format_args!("{}", self.spi_usr_dummy().bit()),
            )
            .field(
                "spi_usr_miso",
                &format_args!("{}", self.spi_usr_miso().bit()),
            )
            .field(
                "spi_usr_mosi",
                &format_args!("{}", self.spi_usr_mosi().bit()),
            )
            .field(
                "reg_usr_mosi_highpart",
                &format_args!("{}", self.reg_usr_mosi_highpart().bit()),
            )
            .field(
                "reg_usr_miso_highpart",
                &format_args!("{}", self.reg_usr_miso_highpart().bit()),
            )
            .field("spi_sio", &format_args!("{}", self.spi_sio().bit()))
            .field(
                "spi_fwrite_qio",
                &format_args!("{}", self.spi_fwrite_qio().bit()),
            )
            .field(
                "spi_fwrite_dio",
                &format_args!("{}", self.spi_fwrite_dio().bit()),
            )
            .field(
                "spi_fwrite_quad",
                &format_args!("{}", self.spi_fwrite_quad().bit()),
            )
            .field(
                "spi_fwrite_dual",
                &format_args!("{}", self.spi_fwrite_dual().bit()),
            )
            .field(
                "spi_wr_byte_order",
                &format_args!("{}", self.spi_wr_byte_order().bit()),
            )
            .field(
                "spi_rd_byte_order",
                &format_args!("{}", self.spi_rd_byte_order().bit()),
            )
            .field(
                "spi_ck_i_edge",
                &format_args!("{}", self.spi_ck_i_edge().bit()),
            )
            .field(
                "spi_ck_o_edge",
                &format_args!("{}", self.spi_ck_o_edge().bit()),
            )
            .field(
                "spi_cs_setup",
                &format_args!("{}", self.spi_cs_setup().bit()),
            )
            .field("spi_cs_hold", &format_args!("{}", self.spi_cs_hold().bit()))
            .field(
                "spi_ahb_user_command",
                &format_args!("{}", self.spi_ahb_user_command().bit()),
            )
            .field(
                "spi_flash_mode",
                &format_args!("{}", self.spi_flash_mode().bit()),
            )
            .field(
                "spi_ahb_user_command_4byte",
                &format_args!("{}", self.spi_ahb_user_command_4byte().bit()),
            )
            .field("spi_duplex", &format_args!("{}", self.spi_duplex().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_USER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - set spi in full duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi_duplex(&mut self) -> SPI_DUPLEX_W<0> {
        SPI_DUPLEX_W::new(self)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ahb_user_command_4byte(&mut self) -> SPI_AHB_USER_COMMAND_4BYTE_W<1> {
        SPI_AHB_USER_COMMAND_4BYTE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn spi_flash_mode(&mut self) -> SPI_FLASH_MODE_W<2> {
        SPI_FLASH_MODE_W::new(self)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ahb_user_command(&mut self) -> SPI_AHB_USER_COMMAND_W<3> {
        SPI_AHB_USER_COMMAND_W::new(self)
    }
    #[doc = "Bit 4 - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs_hold(&mut self) -> SPI_CS_HOLD_W<4> {
        SPI_CS_HOLD_W::new(self)
    }
    #[doc = "Bit 5 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs_setup(&mut self) -> SPI_CS_SETUP_W<5> {
        SPI_CS_SETUP_W::new(self)
    }
    #[doc = "Bit 6 - In the slave mode, 1: rising-edge; 0: falling-edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ck_i_edge(&mut self) -> SPI_CK_I_EDGE_W<6> {
        SPI_CK_I_EDGE_W::new(self)
    }
    #[doc = "Bit 7 - In the master mode, 1: rising-edge; 0: falling-edge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ck_o_edge(&mut self) -> SPI_CK_O_EDGE_W<7> {
        SPI_CK_O_EDGE_W::new(self)
    }
    #[doc = "Bit 10 - In \"read-data\" (MISO) phase, 1: little-endian; 0: big_endian"]
    #[inline(always)]
    #[must_use]
    pub fn spi_rd_byte_order(&mut self) -> SPI_RD_BYTE_ORDER_W<10> {
        SPI_RD_BYTE_ORDER_W::new(self)
    }
    #[doc = "Bit 11 - In \"command\", \"address\", \"write-data\" (MOSI) phases, 1: little-endian; 0: big_endian"]
    #[inline(always)]
    #[must_use]
    pub fn spi_wr_byte_order(&mut self) -> SPI_WR_BYTE_ORDER_W<11> {
        SPI_WR_BYTE_ORDER_W::new(self)
    }
    #[doc = "Bit 12 - In the write operations, \"read-data\" phase apply 2 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_fwrite_dual(&mut self) -> SPI_FWRITE_DUAL_W<12> {
        SPI_FWRITE_DUAL_W::new(self)
    }
    #[doc = "Bit 13 - In the write operations, \"read-data\" phase apply 4 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_fwrite_quad(&mut self) -> SPI_FWRITE_QUAD_W<13> {
        SPI_FWRITE_QUAD_W::new(self)
    }
    #[doc = "Bit 14 - In the write operations, \"address\" phase and \"read-data\" phase apply 2 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_fwrite_dio(&mut self) -> SPI_FWRITE_DIO_W<14> {
        SPI_FWRITE_DIO_W::new(self)
    }
    #[doc = "Bit 15 - In the write operations, \"address\" phase and \"read-data\" phase apply 4 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_fwrite_qio(&mut self) -> SPI_FWRITE_QIO_W<15> {
        SPI_FWRITE_QIO_W::new(self)
    }
    #[doc = "Bit 16 - 1: mosi and miso signals share the same pin"]
    #[inline(always)]
    #[must_use]
    pub fn spi_sio(&mut self) -> SPI_SIO_W<16> {
        SPI_SIO_W::new(self)
    }
    #[doc = "Bit 24 - 1: \"read-data\" phase only access to high-part of the buffer spi_w8~spi_w15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_miso_highpart(&mut self) -> REG_USR_MISO_HIGHPART_W<24> {
        REG_USR_MISO_HIGHPART_W::new(self)
    }
    #[doc = "Bit 25 - 1: \"write-data\" phase only access to high-part of the buffer spi_w8~spi_w15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_mosi_highpart(&mut self) -> REG_USR_MOSI_HIGHPART_W<25> {
        REG_USR_MOSI_HIGHPART_W::new(self)
    }
    #[doc = "Bit 27 - This bit enable the \"write-data\" phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_mosi(&mut self) -> SPI_USR_MOSI_W<27> {
        SPI_USR_MOSI_W::new(self)
    }
    #[doc = "Bit 28 - This bit enable the \"read-data\" phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_miso(&mut self) -> SPI_USR_MISO_W<28> {
        SPI_USR_MISO_W::new(self)
    }
    #[doc = "Bit 29 - This bit enable the \"dummy\" phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_dummy(&mut self) -> SPI_USR_DUMMY_W<29> {
        SPI_USR_DUMMY_W::new(self)
    }
    #[doc = "Bit 30 - This bit enable the \"address\" phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_addr(&mut self) -> SPI_USR_ADDR_W<30> {
        SPI_USR_ADDR_W::new(self)
    }
    #[doc = "Bit 31 - This bit enable the \"command\" phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_command(&mut self) -> SPI_USR_COMMAND_W<31> {
        SPI_USR_COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This bit enable the \"command\" phase of an operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_user](index.html) module"]
pub struct SPI_USER_SPEC;
impl crate::RegisterSpec for SPI_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_user::R](R) reader structure"]
impl crate::Readable for SPI_USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_user::W](W) writer structure"]
impl crate::Writable for SPI_USER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_USER to value 0"]
impl crate::Resettable for SPI_USER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
