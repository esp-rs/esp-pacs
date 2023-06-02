#[doc = "Register `SLV_WRBUF_DLEN` reader"]
pub struct R(crate::R<SLV_WRBUF_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_WRBUF_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_WRBUF_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_WRBUF_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_WRBUF_DLEN` writer"]
pub struct W(crate::W<SLV_WRBUF_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_WRBUF_DLEN_SPEC>;
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
impl From<crate::W<SLV_WRBUF_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_WRBUF_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_WR_BUF_DONE` reader - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_WR_BUF_DONE_R = crate::BitReader;
#[doc = "Field `SLV_WR_BUF_DONE` writer - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_WR_BUF_DONE_W<'a, const O: u8> = crate::BitWriter<'a, SLV_WRBUF_DLEN_SPEC, O>;
#[doc = "Field `CONF_BASE_BITLEN` reader - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
pub type CONF_BASE_BITLEN_R = crate::FieldReader;
#[doc = "Field `CONF_BASE_BITLEN` writer - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
pub type CONF_BASE_BITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, SLV_WRBUF_DLEN_SPEC, 7, O>;
impl R {
    #[doc = "Bit 24 - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&self) -> SLV_WR_BUF_DONE_R {
        SLV_WR_BUF_DONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
    #[inline(always)]
    pub fn conf_base_bitlen(&self) -> CONF_BASE_BITLEN_R {
        CONF_BASE_BITLEN_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLV_WRBUF_DLEN")
            .field(
                "slv_wr_buf_done",
                &format_args!("{}", self.slv_wr_buf_done().bit()),
            )
            .field(
                "conf_base_bitlen",
                &format_args!("{}", self.conf_base_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLV_WRBUF_DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 24 - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_buf_done(&mut self) -> SLV_WR_BUF_DONE_W<24> {
        SLV_WR_BUF_DONE_W::new(self)
    }
    #[doc = "Bits 25:31 - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn conf_base_bitlen(&mut self) -> CONF_BASE_BITLEN_W<25> {
        CONF_BASE_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI slave Wr_BUF interrupt and CONF control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_wrbuf_dlen](index.html) module"]
pub struct SLV_WRBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_WRBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_wrbuf_dlen::R](R) reader structure"]
impl crate::Readable for SLV_WRBUF_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_wrbuf_dlen::W](W) writer structure"]
impl crate::Writable for SLV_WRBUF_DLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLV_WRBUF_DLEN to value 0xd800_0000"]
impl crate::Resettable for SLV_WRBUF_DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0xd800_0000;
}
