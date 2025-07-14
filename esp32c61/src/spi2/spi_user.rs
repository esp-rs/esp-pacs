#[doc = "Register `SPI_USER` reader"]
pub type R = crate::R<SPI_USER_SPEC>;
#[doc = "Register `SPI_USER` writer"]
pub type W = crate::W<SPI_USER_SPEC>;
#[doc = "Field `SPI_DOUTDIN` reader - Configures whether or not to enable full-duplex communication. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_DOUTDIN_R = crate::BitReader;
#[doc = "Field `SPI_DOUTDIN` writer - Configures whether or not to enable full-duplex communication. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_DOUTDIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_QPI_MODE` reader - Configures whether or not to enable QPI mode. \\\\ 0: Disable \\\\ 1: Enable \\\\ This configuration is applicable when the SPI controller works as master or slave. Can be configured in CONF state."]
pub type SPI_QPI_MODE_R = crate::BitReader;
#[doc = "Field `SPI_QPI_MODE` writer - Configures whether or not to enable QPI mode. \\\\ 0: Disable \\\\ 1: Enable \\\\ This configuration is applicable when the SPI controller works as master or slave. Can be configured in CONF state."]
pub type SPI_QPI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_OPI_MODE` reader - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
pub type SPI_OPI_MODE_R = crate::BitReader;
#[doc = "Field `SPI_TSCK_I_EDGE` reader - Configures whether or not to change the polarity of TSCK in slave transfer. \\\\ 0: TSCK = SPI_CK_I \\\\ 1: TSCK = !SPI_CK_I \\\\"]
pub type SPI_TSCK_I_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_TSCK_I_EDGE` writer - Configures whether or not to change the polarity of TSCK in slave transfer. \\\\ 0: TSCK = SPI_CK_I \\\\ 1: TSCK = !SPI_CK_I \\\\"]
pub type SPI_TSCK_I_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS_HOLD` reader - Configures whether or not to keep SPI CS low when SPI is in DONE state. \\\\ 0: Not keep low \\\\ 1: Keep low \\\\ Can be configured in CONF state."]
pub type SPI_CS_HOLD_R = crate::BitReader;
#[doc = "Field `SPI_CS_HOLD` writer - Configures whether or not to keep SPI CS low when SPI is in DONE state. \\\\ 0: Not keep low \\\\ 1: Keep low \\\\ Can be configured in CONF state."]
pub type SPI_CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS_SETUP` reader - Configures whether or not to enable SPI CS when SPI is in prepare (PREP) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_CS_SETUP_R = crate::BitReader;
#[doc = "Field `SPI_CS_SETUP` writer - Configures whether or not to enable SPI CS when SPI is in prepare (PREP) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_RSCK_I_EDGE` reader - Configures whether or not to change the polarity of RSCK in slave transfer. \\\\ 0: RSCK = !SPI_CK_I \\\\ 1: RSCK = SPI_CK_I \\\\"]
pub type SPI_RSCK_I_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_RSCK_I_EDGE` writer - Configures whether or not to change the polarity of RSCK in slave transfer. \\\\ 0: RSCK = !SPI_CK_I \\\\ 1: RSCK = SPI_CK_I \\\\"]
pub type SPI_RSCK_I_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CK_OUT_EDGE` reader - Configures SPI clock mode together with SPI_CK_IDLE_EDGE. Can be configured in CONF state. For more information, see Section <a href='GP-SPI2 master clock control'>link</a>."]
pub type SPI_CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_CK_OUT_EDGE` writer - Configures SPI clock mode together with SPI_CK_IDLE_EDGE. Can be configured in CONF state. For more information, see Section <a href='GP-SPI2 master clock control'>link</a>."]
pub type SPI_CK_OUT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FWRITE_DUAL` reader - Configures whether or not to enable the 2-bit mode of read-data phase in write operations.\\\\ 0: Not enable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FWRITE_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_FWRITE_DUAL` writer - Configures whether or not to enable the 2-bit mode of read-data phase in write operations.\\\\ 0: Not enable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FWRITE_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FWRITE_QUAD` reader - Configures whether or not to enable the 4-bit mode of read-data phase in write operations. \\\\ 0: Not enable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FWRITE_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_FWRITE_QUAD` writer - Configures whether or not to enable the 4-bit mode of read-data phase in write operations. \\\\ 0: Not enable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FWRITE_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FWRITE_OCT` reader - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
pub type SPI_FWRITE_OCT_R = crate::BitReader;
#[doc = "Field `SPI_USR_CONF_NXT` reader - Configures whether or not to enable the CONF state for the next transaction (segment) in a configurable segmented transfer. \\\\ 0: this transfer will end after the current transaction (segment) is finished. Or this is not a configurable segmented transfer. \\\\ 1: this configurable segmented transfer will continue its next transaction (segment). \\\\ Can be configured in CONF state."]
pub type SPI_USR_CONF_NXT_R = crate::BitReader;
#[doc = "Field `SPI_USR_CONF_NXT` writer - Configures whether or not to enable the CONF state for the next transaction (segment) in a configurable segmented transfer. \\\\ 0: this transfer will end after the current transaction (segment) is finished. Or this is not a configurable segmented transfer. \\\\ 1: this configurable segmented transfer will continue its next transaction (segment). \\\\ Can be configured in CONF state."]
pub type SPI_USR_CONF_NXT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SIO` reader - Configures whether or not to enable 3-line half-duplex communication, where MOSI and MISO signals share the same pin.\\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_SIO_R = crate::BitReader;
#[doc = "Field `SPI_SIO` writer - Configures whether or not to enable 3-line half-duplex communication, where MOSI and MISO signals share the same pin.\\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_SIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_MISO_HIGHPART` reader - Configures whether or not to enable high part mode, i.e., only access to high part of the buffers: SPI_W8_REG ~ SPI_W15_REG in read-data phase. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_MISO_HIGHPART_R = crate::BitReader;
#[doc = "Field `SPI_USR_MISO_HIGHPART` writer - Configures whether or not to enable high part mode, i.e., only access to high part of the buffers: SPI_W8_REG ~ SPI_W15_REG in read-data phase. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_MISO_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_MOSI_HIGHPART` reader - Configures whether or not to enable high part mode, i.e., only access to high part of the buffers: SPI_W8_REG ~ SPI_W15_REG in write-data phase. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_MOSI_HIGHPART_R = crate::BitReader;
#[doc = "Field `SPI_USR_MOSI_HIGHPART` writer - Configures whether or not to enable high part mode, i.e., only access to high part of the buffers: SPI_W8_REG ~ SPI_W15_REG in write-data phase. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_MOSI_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_DUMMY_IDLE` reader - Configures whether or not to disable SPI clock in DUMMY state. \\\\ 0: Not disable \\\\ 1: Disable \\\\ Can be configured in CONF state."]
pub type SPI_USR_DUMMY_IDLE_R = crate::BitReader;
#[doc = "Field `SPI_USR_DUMMY_IDLE` writer - Configures whether or not to disable SPI clock in DUMMY state. \\\\ 0: Not disable \\\\ 1: Disable \\\\ Can be configured in CONF state."]
pub type SPI_USR_DUMMY_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_MOSI` reader - Configures whether or not to enable the write-data (DOUT) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_MOSI_R = crate::BitReader;
#[doc = "Field `SPI_USR_MOSI` writer - Configures whether or not to enable the write-data (DOUT) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_MOSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_MISO` reader - Configures whether or not to enable the read-data (DIN) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_MISO_R = crate::BitReader;
#[doc = "Field `SPI_USR_MISO` writer - Configures whether or not to enable the read-data (DIN) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_MISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_DUMMY` reader - Configures whether or not to enable the DUMMY state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_DUMMY_R = crate::BitReader;
#[doc = "Field `SPI_USR_DUMMY` writer - Configures whether or not to enable the DUMMY state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_ADDR` reader - Configures whether or not to enable the address (ADDR) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_ADDR_R = crate::BitReader;
#[doc = "Field `SPI_USR_ADDR` writer - Configures whether or not to enable the address (ADDR) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR_COMMAND` reader - Configures whether or not to enable the command (CMD) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_COMMAND_R = crate::BitReader;
#[doc = "Field `SPI_USR_COMMAND` writer - Configures whether or not to enable the command (CMD) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_USR_COMMAND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable full-duplex communication. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_doutdin(&self) -> SPI_DOUTDIN_R {
        SPI_DOUTDIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable QPI mode. \\\\ 0: Disable \\\\ 1: Enable \\\\ This configuration is applicable when the SPI controller works as master or slave. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_qpi_mode(&self) -> SPI_QPI_MODE_R {
        SPI_QPI_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_opi_mode(&self) -> SPI_OPI_MODE_R {
        SPI_OPI_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to change the polarity of TSCK in slave transfer. \\\\ 0: TSCK = SPI_CK_I \\\\ 1: TSCK = !SPI_CK_I \\\\"]
    #[inline(always)]
    pub fn spi_tsck_i_edge(&self) -> SPI_TSCK_I_EDGE_R {
        SPI_TSCK_I_EDGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to keep SPI CS low when SPI is in DONE state. \\\\ 0: Not keep low \\\\ 1: Keep low \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_hold(&self) -> SPI_CS_HOLD_R {
        SPI_CS_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to enable SPI CS when SPI is in prepare (PREP) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_setup(&self) -> SPI_CS_SETUP_R {
        SPI_CS_SETUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to change the polarity of RSCK in slave transfer. \\\\ 0: RSCK = !SPI_CK_I \\\\ 1: RSCK = SPI_CK_I \\\\"]
    #[inline(always)]
    pub fn spi_rsck_i_edge(&self) -> SPI_RSCK_I_EDGE_R {
        SPI_RSCK_I_EDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures SPI clock mode together with SPI_CK_IDLE_EDGE. Can be configured in CONF state. For more information, see Section <a href='GP-SPI2 master clock control'>link</a>."]
    #[inline(always)]
    pub fn spi_ck_out_edge(&self) -> SPI_CK_OUT_EDGE_R {
        SPI_CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether or not to enable the 2-bit mode of read-data phase in write operations.\\\\ 0: Not enable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fwrite_dual(&self) -> SPI_FWRITE_DUAL_R {
        SPI_FWRITE_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to enable the 4-bit mode of read-data phase in write operations. \\\\ 0: Not enable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fwrite_quad(&self) -> SPI_FWRITE_QUAD_R {
        SPI_FWRITE_QUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fwrite_oct(&self) -> SPI_FWRITE_OCT_R {
        SPI_FWRITE_OCT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the CONF state for the next transaction (segment) in a configurable segmented transfer. \\\\ 0: this transfer will end after the current transaction (segment) is finished. Or this is not a configurable segmented transfer. \\\\ 1: this configurable segmented transfer will continue its next transaction (segment). \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_conf_nxt(&self) -> SPI_USR_CONF_NXT_R {
        SPI_USR_CONF_NXT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to enable 3-line half-duplex communication, where MOSI and MISO signals share the same pin.\\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_sio(&self) -> SPI_SIO_R {
        SPI_SIO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Configures whether or not to enable high part mode, i.e., only access to high part of the buffers: SPI_W8_REG ~ SPI_W15_REG in read-data phase. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_miso_highpart(&self) -> SPI_USR_MISO_HIGHPART_R {
        SPI_USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Configures whether or not to enable high part mode, i.e., only access to high part of the buffers: SPI_W8_REG ~ SPI_W15_REG in write-data phase. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_mosi_highpart(&self) -> SPI_USR_MOSI_HIGHPART_R {
        SPI_USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Configures whether or not to disable SPI clock in DUMMY state. \\\\ 0: Not disable \\\\ 1: Disable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_dummy_idle(&self) -> SPI_USR_DUMMY_IDLE_R {
        SPI_USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Configures whether or not to enable the write-data (DOUT) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_mosi(&self) -> SPI_USR_MOSI_R {
        SPI_USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Configures whether or not to enable the read-data (DIN) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_miso(&self) -> SPI_USR_MISO_R {
        SPI_USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures whether or not to enable the DUMMY state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_dummy(&self) -> SPI_USR_DUMMY_R {
        SPI_USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configures whether or not to enable the address (ADDR) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_addr(&self) -> SPI_USR_ADDR_R {
        SPI_USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures whether or not to enable the command (CMD) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_command(&self) -> SPI_USR_COMMAND_R {
        SPI_USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_USER")
            .field("spi_doutdin", &self.spi_doutdin())
            .field("spi_qpi_mode", &self.spi_qpi_mode())
            .field("spi_opi_mode", &self.spi_opi_mode())
            .field("spi_tsck_i_edge", &self.spi_tsck_i_edge())
            .field("spi_cs_hold", &self.spi_cs_hold())
            .field("spi_cs_setup", &self.spi_cs_setup())
            .field("spi_rsck_i_edge", &self.spi_rsck_i_edge())
            .field("spi_ck_out_edge", &self.spi_ck_out_edge())
            .field("spi_fwrite_dual", &self.spi_fwrite_dual())
            .field("spi_fwrite_quad", &self.spi_fwrite_quad())
            .field("spi_fwrite_oct", &self.spi_fwrite_oct())
            .field("spi_usr_conf_nxt", &self.spi_usr_conf_nxt())
            .field("spi_sio", &self.spi_sio())
            .field("spi_usr_miso_highpart", &self.spi_usr_miso_highpart())
            .field("spi_usr_mosi_highpart", &self.spi_usr_mosi_highpart())
            .field("spi_usr_dummy_idle", &self.spi_usr_dummy_idle())
            .field("spi_usr_mosi", &self.spi_usr_mosi())
            .field("spi_usr_miso", &self.spi_usr_miso())
            .field("spi_usr_dummy", &self.spi_usr_dummy())
            .field("spi_usr_addr", &self.spi_usr_addr())
            .field("spi_usr_command", &self.spi_usr_command())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable full-duplex communication. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_doutdin(&mut self) -> SPI_DOUTDIN_W<SPI_USER_SPEC> {
        SPI_DOUTDIN_W::new(self, 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable QPI mode. \\\\ 0: Disable \\\\ 1: Enable \\\\ This configuration is applicable when the SPI controller works as master or slave. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_qpi_mode(&mut self) -> SPI_QPI_MODE_W<SPI_USER_SPEC> {
        SPI_QPI_MODE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Configures whether or not to change the polarity of TSCK in slave transfer. \\\\ 0: TSCK = SPI_CK_I \\\\ 1: TSCK = !SPI_CK_I \\\\"]
    #[inline(always)]
    pub fn spi_tsck_i_edge(&mut self) -> SPI_TSCK_I_EDGE_W<SPI_USER_SPEC> {
        SPI_TSCK_I_EDGE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to keep SPI CS low when SPI is in DONE state. \\\\ 0: Not keep low \\\\ 1: Keep low \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_hold(&mut self) -> SPI_CS_HOLD_W<SPI_USER_SPEC> {
        SPI_CS_HOLD_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable SPI CS when SPI is in prepare (PREP) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_setup(&mut self) -> SPI_CS_SETUP_W<SPI_USER_SPEC> {
        SPI_CS_SETUP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to change the polarity of RSCK in slave transfer. \\\\ 0: RSCK = !SPI_CK_I \\\\ 1: RSCK = SPI_CK_I \\\\"]
    #[inline(always)]
    pub fn spi_rsck_i_edge(&mut self) -> SPI_RSCK_I_EDGE_W<SPI_USER_SPEC> {
        SPI_RSCK_I_EDGE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures SPI clock mode together with SPI_CK_IDLE_EDGE. Can be configured in CONF state. For more information, see Section <a href='GP-SPI2 master clock control'>link</a>."]
    #[inline(always)]
    pub fn spi_ck_out_edge(&mut self) -> SPI_CK_OUT_EDGE_W<SPI_USER_SPEC> {
        SPI_CK_OUT_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 12 - Configures whether or not to enable the 2-bit mode of read-data phase in write operations.\\\\ 0: Not enable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fwrite_dual(&mut self) -> SPI_FWRITE_DUAL_W<SPI_USER_SPEC> {
        SPI_FWRITE_DUAL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable the 4-bit mode of read-data phase in write operations. \\\\ 0: Not enable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fwrite_quad(&mut self) -> SPI_FWRITE_QUAD_W<SPI_USER_SPEC> {
        SPI_FWRITE_QUAD_W::new(self, 13)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the CONF state for the next transaction (segment) in a configurable segmented transfer. \\\\ 0: this transfer will end after the current transaction (segment) is finished. Or this is not a configurable segmented transfer. \\\\ 1: this configurable segmented transfer will continue its next transaction (segment). \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_conf_nxt(&mut self) -> SPI_USR_CONF_NXT_W<SPI_USER_SPEC> {
        SPI_USR_CONF_NXT_W::new(self, 15)
    }
    #[doc = "Bit 17 - Configures whether or not to enable 3-line half-duplex communication, where MOSI and MISO signals share the same pin.\\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_sio(&mut self) -> SPI_SIO_W<SPI_USER_SPEC> {
        SPI_SIO_W::new(self, 17)
    }
    #[doc = "Bit 24 - Configures whether or not to enable high part mode, i.e., only access to high part of the buffers: SPI_W8_REG ~ SPI_W15_REG in read-data phase. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_miso_highpart(&mut self) -> SPI_USR_MISO_HIGHPART_W<SPI_USER_SPEC> {
        SPI_USR_MISO_HIGHPART_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to enable high part mode, i.e., only access to high part of the buffers: SPI_W8_REG ~ SPI_W15_REG in write-data phase. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_mosi_highpart(&mut self) -> SPI_USR_MOSI_HIGHPART_W<SPI_USER_SPEC> {
        SPI_USR_MOSI_HIGHPART_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to disable SPI clock in DUMMY state. \\\\ 0: Not disable \\\\ 1: Disable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_dummy_idle(&mut self) -> SPI_USR_DUMMY_IDLE_W<SPI_USER_SPEC> {
        SPI_USR_DUMMY_IDLE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to enable the write-data (DOUT) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_mosi(&mut self) -> SPI_USR_MOSI_W<SPI_USER_SPEC> {
        SPI_USR_MOSI_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to enable the read-data (DIN) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_miso(&mut self) -> SPI_USR_MISO_W<SPI_USER_SPEC> {
        SPI_USR_MISO_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to enable the DUMMY state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_dummy(&mut self) -> SPI_USR_DUMMY_W<SPI_USER_SPEC> {
        SPI_USR_DUMMY_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to enable the address (ADDR) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_addr(&mut self) -> SPI_USR_ADDR_W<SPI_USER_SPEC> {
        SPI_USR_ADDR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to enable the command (CMD) state of an operation. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_command(&mut self) -> SPI_USR_COMMAND_W<SPI_USER_SPEC> {
        SPI_USR_COMMAND_W::new(self, 31)
    }
}
#[doc = "SPI USER control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_USER_SPEC;
impl crate::RegisterSpec for SPI_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_user::R`](R) reader structure"]
impl crate::Readable for SPI_USER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_user::W`](W) writer structure"]
impl crate::Writable for SPI_USER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_USER to value 0x8000_00c0"]
impl crate::Resettable for SPI_USER_SPEC {
    const RESET_VALUE: u32 = 0x8000_00c0;
}
