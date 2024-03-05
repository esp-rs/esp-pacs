#[doc = "Register `SPI_USER` reader"]
pub type R = crate::R<SPI_USER_SPEC>;
#[doc = "Register `SPI_USER` writer"]
pub type W = crate::W<SPI_USER_SPEC>;
#[doc = "Field `SPI_DOUTDIN` reader - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_DOUTDIN_R = crate::BitReader;
#[doc = "Field `SPI_DOUTDIN` writer - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_DOUTDIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_QPI_MODE` reader - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
pub type SPI_QPI_MODE_R = crate::BitReader;
#[doc = "Field `SPI_QPI_MODE` writer - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
pub type SPI_QPI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_OPI_MODE` reader - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
pub type SPI_OPI_MODE_R = crate::BitReader;
#[doc = "Field `SPI_OPI_MODE` writer - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
pub type SPI_OPI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_TSCK_I_EDGE` reader - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
pub type SPI_TSCK_I_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_TSCK_I_EDGE` writer - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
pub type SPI_TSCK_I_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS_HOLD` reader - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_CS_HOLD_R = crate::BitReader;
#[doc = "Field `SPI_CS_HOLD` writer - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS_SETUP` reader - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_CS_SETUP_R = crate::BitReader;
#[doc = "Field `SPI_CS_SETUP` writer - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_RSCK_I_EDGE` reader - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
pub type SPI_RSCK_I_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_RSCK_I_EDGE` writer - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
pub type SPI_RSCK_I_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CK_OUT_EDGE` reader - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
pub type SPI_CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_CK_OUT_EDGE` writer - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
pub type SPI_CK_OUT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FWRITE_DUAL` reader - In the write operations read-data phase apply 2 signals. Can be configured in CONF state."]
pub type SPI_FWRITE_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_FWRITE_DUAL` writer - In the write operations read-data phase apply 2 signals. Can be configured in CONF state."]
pub type SPI_FWRITE_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FWRITE_QUAD` reader - In the write operations read-data phase apply 4 signals. Can be configured in CONF state."]
pub type SPI_FWRITE_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_FWRITE_QUAD` writer - In the write operations read-data phase apply 4 signals. Can be configured in CONF state."]
pub type SPI_FWRITE_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FWRITE_OCT` reader - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
pub type SPI_FWRITE_OCT_R = crate::BitReader;
#[doc = "Field `SPI_FWRITE_OCT` writer - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
pub type SPI_FWRITE_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_CONF_NXT` reader - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
pub type SPI_USR_CONF_NXT_R = crate::BitReader;
#[doc = "Field `SPI_USR_CONF_NXT` writer - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
pub type SPI_USR_CONF_NXT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SIO` reader - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_SIO_R = crate::BitReader;
#[doc = "Field `SPI_SIO` writer - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_SIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_MISO_HIGHPART` reader - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_USR_MISO_HIGHPART_R = crate::BitReader;
#[doc = "Field `SPI_USR_MISO_HIGHPART` writer - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_USR_MISO_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_MOSI_HIGHPART` reader - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_USR_MOSI_HIGHPART_R = crate::BitReader;
#[doc = "Field `SPI_USR_MOSI_HIGHPART` writer - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_USR_MOSI_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
pub type SPI_USR_DUMMY_IDLE_R = crate::BitReader;
#[doc = "Field `SPI_USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
pub type SPI_USR_DUMMY_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_MOSI` reader - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_MOSI_R = crate::BitReader;
#[doc = "Field `SPI_USR_MOSI` writer - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_MOSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_MISO` reader - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_MISO_R = crate::BitReader;
#[doc = "Field `SPI_USR_MISO` writer - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_MISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_DUMMY` reader - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_DUMMY_R = crate::BitReader;
#[doc = "Field `SPI_USR_DUMMY` writer - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_ADDR` reader - This bit enable the address phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_ADDR_R = crate::BitReader;
#[doc = "Field `SPI_USR_ADDR` writer - This bit enable the address phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_COMMAND` reader - This bit enable the command phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_COMMAND_R = crate::BitReader;
#[doc = "Field `SPI_USR_COMMAND` writer - This bit enable the command phase of an operation. Can be configured in CONF state."]
pub type SPI_USR_COMMAND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_doutdin(&self) -> SPI_DOUTDIN_R {
        SPI_DOUTDIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_qpi_mode(&self) -> SPI_QPI_MODE_R {
        SPI_QPI_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_opi_mode(&self) -> SPI_OPI_MODE_R {
        SPI_OPI_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
    #[inline(always)]
    pub fn spi_tsck_i_edge(&self) -> SPI_TSCK_I_EDGE_R {
        SPI_TSCK_I_EDGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_hold(&self) -> SPI_CS_HOLD_R {
        SPI_CS_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_setup(&self) -> SPI_CS_SETUP_R {
        SPI_CS_SETUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
    #[inline(always)]
    pub fn spi_rsck_i_edge(&self) -> SPI_RSCK_I_EDGE_R {
        SPI_RSCK_I_EDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_out_edge(&self) -> SPI_CK_OUT_EDGE_R {
        SPI_CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fwrite_dual(&self) -> SPI_FWRITE_DUAL_R {
        SPI_FWRITE_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fwrite_quad(&self) -> SPI_FWRITE_QUAD_R {
        SPI_FWRITE_QUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fwrite_oct(&self) -> SPI_FWRITE_OCT_R {
        SPI_FWRITE_OCT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_conf_nxt(&self) -> SPI_USR_CONF_NXT_R {
        SPI_USR_CONF_NXT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_sio(&self) -> SPI_SIO_R {
        SPI_SIO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_miso_highpart(&self) -> SPI_USR_MISO_HIGHPART_R {
        SPI_USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_mosi_highpart(&self) -> SPI_USR_MOSI_HIGHPART_R {
        SPI_USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_dummy_idle(&self) -> SPI_USR_DUMMY_IDLE_R {
        SPI_USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_mosi(&self) -> SPI_USR_MOSI_R {
        SPI_USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_miso(&self) -> SPI_USR_MISO_R {
        SPI_USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_dummy(&self) -> SPI_USR_DUMMY_R {
        SPI_USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_addr(&self) -> SPI_USR_ADDR_R {
        SPI_USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_command(&self) -> SPI_USR_COMMAND_R {
        SPI_USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_USER")
            .field("spi_doutdin", &format_args!("{}", self.spi_doutdin().bit()))
            .field(
                "spi_qpi_mode",
                &format_args!("{}", self.spi_qpi_mode().bit()),
            )
            .field(
                "spi_opi_mode",
                &format_args!("{}", self.spi_opi_mode().bit()),
            )
            .field(
                "spi_tsck_i_edge",
                &format_args!("{}", self.spi_tsck_i_edge().bit()),
            )
            .field("spi_cs_hold", &format_args!("{}", self.spi_cs_hold().bit()))
            .field(
                "spi_cs_setup",
                &format_args!("{}", self.spi_cs_setup().bit()),
            )
            .field(
                "spi_rsck_i_edge",
                &format_args!("{}", self.spi_rsck_i_edge().bit()),
            )
            .field(
                "spi_ck_out_edge",
                &format_args!("{}", self.spi_ck_out_edge().bit()),
            )
            .field(
                "spi_fwrite_dual",
                &format_args!("{}", self.spi_fwrite_dual().bit()),
            )
            .field(
                "spi_fwrite_quad",
                &format_args!("{}", self.spi_fwrite_quad().bit()),
            )
            .field(
                "spi_fwrite_oct",
                &format_args!("{}", self.spi_fwrite_oct().bit()),
            )
            .field(
                "spi_usr_conf_nxt",
                &format_args!("{}", self.spi_usr_conf_nxt().bit()),
            )
            .field("spi_sio", &format_args!("{}", self.spi_sio().bit()))
            .field(
                "spi_usr_miso_highpart",
                &format_args!("{}", self.spi_usr_miso_highpart().bit()),
            )
            .field(
                "spi_usr_mosi_highpart",
                &format_args!("{}", self.spi_usr_mosi_highpart().bit()),
            )
            .field(
                "spi_usr_dummy_idle",
                &format_args!("{}", self.spi_usr_dummy_idle().bit()),
            )
            .field(
                "spi_usr_mosi",
                &format_args!("{}", self.spi_usr_mosi().bit()),
            )
            .field(
                "spi_usr_miso",
                &format_args!("{}", self.spi_usr_miso().bit()),
            )
            .field(
                "spi_usr_dummy",
                &format_args!("{}", self.spi_usr_dummy().bit()),
            )
            .field(
                "spi_usr_addr",
                &format_args!("{}", self.spi_usr_addr().bit()),
            )
            .field(
                "spi_usr_command",
                &format_args!("{}", self.spi_usr_command().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_USER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_doutdin(&mut self) -> SPI_DOUTDIN_W<SPI_USER_SPEC> {
        SPI_DOUTDIN_W::new(self, 0)
    }
    #[doc = "Bit 3 - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_qpi_mode(&mut self) -> SPI_QPI_MODE_W<SPI_USER_SPEC> {
        SPI_QPI_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_opi_mode(&mut self) -> SPI_OPI_MODE_W<SPI_USER_SPEC> {
        SPI_OPI_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
    #[inline(always)]
    #[must_use]
    pub fn spi_tsck_i_edge(&mut self) -> SPI_TSCK_I_EDGE_W<SPI_USER_SPEC> {
        SPI_TSCK_I_EDGE_W::new(self, 5)
    }
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs_hold(&mut self) -> SPI_CS_HOLD_W<SPI_USER_SPEC> {
        SPI_CS_HOLD_W::new(self, 6)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs_setup(&mut self) -> SPI_CS_SETUP_W<SPI_USER_SPEC> {
        SPI_CS_SETUP_W::new(self, 7)
    }
    #[doc = "Bit 8 - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
    #[inline(always)]
    #[must_use]
    pub fn spi_rsck_i_edge(&mut self) -> SPI_RSCK_I_EDGE_W<SPI_USER_SPEC> {
        SPI_RSCK_I_EDGE_W::new(self, 8)
    }
    #[doc = "Bit 9 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ck_out_edge(&mut self) -> SPI_CK_OUT_EDGE_W<SPI_USER_SPEC> {
        SPI_CK_OUT_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fwrite_dual(&mut self) -> SPI_FWRITE_DUAL_W<SPI_USER_SPEC> {
        SPI_FWRITE_DUAL_W::new(self, 12)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fwrite_quad(&mut self) -> SPI_FWRITE_QUAD_W<SPI_USER_SPEC> {
        SPI_FWRITE_QUAD_W::new(self, 13)
    }
    #[doc = "Bit 14 - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fwrite_oct(&mut self) -> SPI_FWRITE_OCT_W<SPI_USER_SPEC> {
        SPI_FWRITE_OCT_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_conf_nxt(&mut self) -> SPI_USR_CONF_NXT_W<SPI_USER_SPEC> {
        SPI_USR_CONF_NXT_W::new(self, 15)
    }
    #[doc = "Bit 17 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_sio(&mut self) -> SPI_SIO_W<SPI_USER_SPEC> {
        SPI_SIO_W::new(self, 17)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_miso_highpart(&mut self) -> SPI_USR_MISO_HIGHPART_W<SPI_USER_SPEC> {
        SPI_USR_MISO_HIGHPART_W::new(self, 24)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_mosi_highpart(&mut self) -> SPI_USR_MOSI_HIGHPART_W<SPI_USER_SPEC> {
        SPI_USR_MOSI_HIGHPART_W::new(self, 25)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_dummy_idle(&mut self) -> SPI_USR_DUMMY_IDLE_W<SPI_USER_SPEC> {
        SPI_USR_DUMMY_IDLE_W::new(self, 26)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_mosi(&mut self) -> SPI_USR_MOSI_W<SPI_USER_SPEC> {
        SPI_USR_MOSI_W::new(self, 27)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_miso(&mut self) -> SPI_USR_MISO_W<SPI_USER_SPEC> {
        SPI_USR_MISO_W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_dummy(&mut self) -> SPI_USR_DUMMY_W<SPI_USER_SPEC> {
        SPI_USR_DUMMY_W::new(self, 29)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_addr(&mut self) -> SPI_USR_ADDR_W<SPI_USER_SPEC> {
        SPI_USR_ADDR_W::new(self, 30)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_command(&mut self) -> SPI_USR_COMMAND_W<SPI_USER_SPEC> {
        SPI_USR_COMMAND_W::new(self, 31)
    }
}
#[doc = "SPI USER control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_USER_SPEC;
impl crate::RegisterSpec for SPI_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_user::R`](R) reader structure"]
impl crate::Readable for SPI_USER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_user::W`](W) writer structure"]
impl crate::Writable for SPI_USER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_USER to value 0x8000_00c0"]
impl crate::Resettable for SPI_USER_SPEC {
    const RESET_VALUE: u32 = 0x8000_00c0;
}
