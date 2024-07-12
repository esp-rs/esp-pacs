#[doc = "Register `SLV_RDBUF_DLEN` reader"]
pub type R = crate::R<SLV_RDBUF_DLEN_SPEC>;
#[doc = "Register `SLV_RDBUF_DLEN` writer"]
pub type W = crate::W<SLV_RDBUF_DLEN_SPEC>;
#[doc = "Field `SLV_DMA_RD_BYTELEN` reader - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
pub type SLV_DMA_RD_BYTELEN_R = crate::FieldReader<u32>;
#[doc = "Field `SLV_DMA_RD_BYTELEN` writer - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
pub type SLV_DMA_RD_BYTELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SLV_RD_BUF_DONE` reader - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_RD_BUF_DONE_R = crate::BitReader;
#[doc = "Field `SLV_RD_BUF_DONE` writer - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_RD_BUF_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEG_MAGIC_ERR` reader - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
pub type SEG_MAGIC_ERR_R = crate::BitReader;
#[doc = "Field `SEG_MAGIC_ERR` writer - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
pub type SEG_MAGIC_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("slv_dma_rd_bytelen", &self.slv_dma_rd_bytelen())
            .field("slv_rd_buf_done", &self.slv_rd_buf_done())
            .field("seg_magic_err", &self.seg_magic_err())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - In the slave mode it is the length in bytes for read operations. The register value shall be byte_num."]
    #[inline(always)]
    #[must_use]
    pub fn slv_dma_rd_bytelen(&mut self) -> SLV_DMA_RD_BYTELEN_W<SLV_RDBUF_DLEN_SPEC> {
        SLV_DMA_RD_BYTELEN_W::new(self, 0)
    }
    #[doc = "Bit 24 - The interrupt raw bit for the completion of read-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_buf_done(&mut self) -> SLV_RD_BUF_DONE_W<SLV_RDBUF_DLEN_SPEC> {
        SLV_RD_BUF_DONE_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: The recent magic value in CONF buffer is not right in master DMA seg-trans mode. 0: others."]
    #[inline(always)]
    #[must_use]
    pub fn seg_magic_err(&mut self) -> SEG_MAGIC_ERR_W<SLV_RDBUF_DLEN_SPEC> {
        SEG_MAGIC_ERR_W::new(self, 25)
    }
}
#[doc = "SPI magic error and slave control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slv_rdbuf_dlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_rdbuf_dlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLV_RDBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_RDBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slv_rdbuf_dlen::R`](R) reader structure"]
impl crate::Readable for SLV_RDBUF_DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slv_rdbuf_dlen::W`](W) writer structure"]
impl crate::Writable for SLV_RDBUF_DLEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLV_RDBUF_DLEN to value 0"]
impl crate::Resettable for SLV_RDBUF_DLEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
