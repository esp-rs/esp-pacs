#[doc = "Register `SLV_RDBUF_DLEN` reader"]
pub struct R(crate::R<SLV_RDBUF_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_RDBUF_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_RDBUF_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_RDBUF_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_RDBUF_DLEN` writer"]
pub struct W(crate::W<SLV_RDBUF_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_RDBUF_DLEN_SPEC>;
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
impl From<crate::W<SLV_RDBUF_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_RDBUF_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_DMA_RD_BYTELEN` reader - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
pub type SLV_DMA_RD_BYTELEN_R = crate::FieldReader<u32>;
#[doc = "Field `SLV_DMA_RD_BYTELEN` writer - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
pub type SLV_DMA_RD_BYTELEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, SLV_RDBUF_DLEN_SPEC, 20, O, u32>;
#[doc = "Field `SLV_RD_BUF_DONE` reader - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_RD_BUF_DONE_R = crate::BitReader;
#[doc = "Field `SLV_RD_BUF_DONE` writer - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_RD_BUF_DONE_W<'a, const O: u8> = crate::BitWriter<'a, SLV_RDBUF_DLEN_SPEC, O>;
#[doc = "Field `SEG_MAGIC_ERR` reader - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
pub type SEG_MAGIC_ERR_R = crate::BitReader;
#[doc = "Field `SEG_MAGIC_ERR` writer - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
pub type SEG_MAGIC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, SLV_RDBUF_DLEN_SPEC, O>;
impl R {
    #[doc = "Bits 0:19 - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
    #[inline(always)]
    pub fn slv_dma_rd_bytelen(&self) -> SLV_DMA_RD_BYTELEN_R {
        SLV_DMA_RD_BYTELEN_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 24 - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&self) -> SLV_RD_BUF_DONE_R {
        SLV_RD_BUF_DONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
    #[inline(always)]
    pub fn seg_magic_err(&self) -> SEG_MAGIC_ERR_R {
        SEG_MAGIC_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLV_RDBUF_DLEN")
            .field(
                "slv_dma_rd_bytelen",
                &format_args!("{}", self.slv_dma_rd_bytelen().bits()),
            )
            .field(
                "slv_rd_buf_done",
                &format_args!("{}", self.slv_rd_buf_done().bit()),
            )
            .field(
                "seg_magic_err",
                &format_args!("{}", self.seg_magic_err().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLV_RDBUF_DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
    #[inline(always)]
    #[must_use]
    pub fn slv_dma_rd_bytelen(&mut self) -> SLV_DMA_RD_BYTELEN_W<0> {
        SLV_DMA_RD_BYTELEN_W::new(self)
    }
    #[doc = "Bit 24 - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_buf_done(&mut self) -> SLV_RD_BUF_DONE_W<24> {
        SLV_RD_BUF_DONE_W::new(self)
    }
    #[doc = "Bit 25 - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
    #[inline(always)]
    #[must_use]
    pub fn seg_magic_err(&mut self) -> SEG_MAGIC_ERR_W<25> {
        SEG_MAGIC_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI magic error and slave control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_rdbuf_dlen](index.html) module"]
pub struct SLV_RDBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_RDBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_rdbuf_dlen::R](R) reader structure"]
impl crate::Readable for SLV_RDBUF_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_rdbuf_dlen::W](W) writer structure"]
impl crate::Writable for SLV_RDBUF_DLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLV_RDBUF_DLEN to value 0"]
impl crate::Resettable for SLV_RDBUF_DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
