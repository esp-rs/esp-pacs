#[doc = "Register `SLAVE1` reader"]
pub struct R(crate::R<SLAVE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE1` writer"]
pub struct W(crate::W<SLAVE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE1_SPEC>;
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
impl From<crate::W<SLAVE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_ADDR_ERR_CLR` reader - 1: Clear SPI_SLV_ADDR_ERR. 0: not valid. Can be changed by CONF_buf."]
pub type SLV_ADDR_ERR_CLR_R = crate::BitReader;
#[doc = "Field `SLV_ADDR_ERR_CLR` writer - 1: Clear SPI_SLV_ADDR_ERR. 0: not valid. Can be changed by CONF_buf."]
pub type SLV_ADDR_ERR_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_CMD_ERR_CLR` reader - 1: Clear SPI_SLV_CMD_ERR. 0: not valid. Can be changed by CONF_buf."]
pub type SLV_CMD_ERR_CLR_R = crate::BitReader;
#[doc = "Field `SLV_CMD_ERR_CLR` writer - 1: Clear SPI_SLV_CMD_ERR. 0: not valid. Can be changed by CONF_buf."]
pub type SLV_CMD_ERR_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_NO_QPI_EN` reader - 1: spi slave QPI mode is not supported. 0: spi slave QPI mode is supported."]
pub type SLV_NO_QPI_EN_R = crate::BitReader;
#[doc = "Field `SLV_NO_QPI_EN` writer - 1: spi slave QPI mode is not supported. 0: spi slave QPI mode is supported."]
pub type SLV_NO_QPI_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_ADDR_ERR` reader - 1: The address value of the last SPI transfer is not supported by SPI slave. 0: The address value is supported or no address value is received."]
pub type SLV_ADDR_ERR_R = crate::BitReader;
#[doc = "Field `SLV_CMD_ERR` reader - 1: The command value of the last SPI transfer is not supported by SPI slave. 0: The command value is supported or no command value is received."]
pub type SLV_CMD_ERR_R = crate::BitReader;
#[doc = "Field `SLV_WR_DMA_DONE` reader - The interrupt raw bit for the completion of dma write operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_WR_DMA_DONE_R = crate::BitReader;
#[doc = "Field `SLV_WR_DMA_DONE` writer - The interrupt raw bit for the completion of dma write operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_WR_DMA_DONE_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_LAST_COMMAND` reader - In the slave mode it is the value of command."]
pub type SLV_LAST_COMMAND_R = crate::FieldReader;
#[doc = "Field `SLV_LAST_COMMAND` writer - In the slave mode it is the value of command."]
pub type SLV_LAST_COMMAND_W<'a, const O: u8> = crate::FieldWriter<'a, SLAVE1_SPEC, 8, O>;
#[doc = "Field `SLV_LAST_ADDR` reader - In the slave mode it is the value of address."]
pub type SLV_LAST_ADDR_R = crate::FieldReader;
#[doc = "Field `SLV_LAST_ADDR` writer - In the slave mode it is the value of address."]
pub type SLV_LAST_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, SLAVE1_SPEC, 8, O>;
impl R {
    #[doc = "Bit 10 - 1: Clear SPI_SLV_ADDR_ERR. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_addr_err_clr(&self) -> SLV_ADDR_ERR_CLR_R {
        SLV_ADDR_ERR_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Clear SPI_SLV_CMD_ERR. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_cmd_err_clr(&self) -> SLV_CMD_ERR_CLR_R {
        SLV_CMD_ERR_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: spi slave QPI mode is not supported. 0: spi slave QPI mode is supported."]
    #[inline(always)]
    pub fn slv_no_qpi_en(&self) -> SLV_NO_QPI_EN_R {
        SLV_NO_QPI_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: The address value of the last SPI transfer is not supported by SPI slave. 0: The address value is supported or no address value is received."]
    #[inline(always)]
    pub fn slv_addr_err(&self) -> SLV_ADDR_ERR_R {
        SLV_ADDR_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: The command value of the last SPI transfer is not supported by SPI slave. 0: The command value is supported or no command value is received."]
    #[inline(always)]
    pub fn slv_cmd_err(&self) -> SLV_CMD_ERR_R {
        SLV_CMD_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for the completion of dma write operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_wr_dma_done(&self) -> SLV_WR_DMA_DONE_R {
        SLV_WR_DMA_DONE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&self) -> SLV_LAST_COMMAND_R {
        SLV_LAST_COMMAND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    pub fn slv_last_addr(&self) -> SLV_LAST_ADDR_R {
        SLV_LAST_ADDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE1")
            .field(
                "slv_addr_err_clr",
                &format_args!("{}", self.slv_addr_err_clr().bit()),
            )
            .field(
                "slv_cmd_err_clr",
                &format_args!("{}", self.slv_cmd_err_clr().bit()),
            )
            .field(
                "slv_no_qpi_en",
                &format_args!("{}", self.slv_no_qpi_en().bit()),
            )
            .field(
                "slv_addr_err",
                &format_args!("{}", self.slv_addr_err().bit()),
            )
            .field("slv_cmd_err", &format_args!("{}", self.slv_cmd_err().bit()))
            .field(
                "slv_wr_dma_done",
                &format_args!("{}", self.slv_wr_dma_done().bit()),
            )
            .field(
                "slv_last_command",
                &format_args!("{}", self.slv_last_command().bits()),
            )
            .field(
                "slv_last_addr",
                &format_args!("{}", self.slv_last_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLAVE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 10 - 1: Clear SPI_SLV_ADDR_ERR. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    #[must_use]
    pub fn slv_addr_err_clr(&mut self) -> SLV_ADDR_ERR_CLR_W<10> {
        SLV_ADDR_ERR_CLR_W::new(self)
    }
    #[doc = "Bit 11 - 1: Clear SPI_SLV_CMD_ERR. 0: not valid. Can be changed by CONF_buf."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd_err_clr(&mut self) -> SLV_CMD_ERR_CLR_W<11> {
        SLV_CMD_ERR_CLR_W::new(self)
    }
    #[doc = "Bit 12 - 1: spi slave QPI mode is not supported. 0: spi slave QPI mode is supported."]
    #[inline(always)]
    #[must_use]
    pub fn slv_no_qpi_en(&mut self) -> SLV_NO_QPI_EN_W<12> {
        SLV_NO_QPI_EN_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt raw bit for the completion of dma write operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_dma_done(&mut self) -> SLV_WR_DMA_DONE_W<15> {
        SLV_WR_DMA_DONE_W::new(self)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of command."]
    #[inline(always)]
    #[must_use]
    pub fn slv_last_command(&mut self) -> SLV_LAST_COMMAND_W<16> {
        SLV_LAST_COMMAND_W::new(self)
    }
    #[doc = "Bits 24:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    #[must_use]
    pub fn slv_last_addr(&mut self) -> SLV_LAST_ADDR_W<24> {
        SLV_LAST_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI slave control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave1](index.html) module"]
pub struct SLAVE1_SPEC;
impl crate::RegisterSpec for SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave1::R](R) reader structure"]
impl crate::Readable for SLAVE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave1::W](W) writer structure"]
impl crate::Writable for SLAVE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLAVE1 to value 0"]
impl crate::Resettable for SLAVE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
