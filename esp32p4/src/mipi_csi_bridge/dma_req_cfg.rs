#[doc = "Register `DMA_REQ_CFG` reader"]
pub type R = crate::R<DMA_REQ_CFG_SPEC>;
#[doc = "Register `DMA_REQ_CFG` writer"]
pub type W = crate::W<DMA_REQ_CFG_SPEC>;
#[doc = "Field `DMA_BURST_LEN` reader - DMA burst length."]
pub type DMA_BURST_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `DMA_BURST_LEN` writer - DMA burst length."]
pub type DMA_BURST_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMA_CFG_UPD_BY_BLK` reader - 1: reg_dma_burst_len &amp; reg_dma_burst_len will be updated by dma block finish. 0: updated by frame."]
pub type DMA_CFG_UPD_BY_BLK_R = crate::BitReader;
#[doc = "Field `DMA_CFG_UPD_BY_BLK` writer - 1: reg_dma_burst_len &amp; reg_dma_burst_len will be updated by dma block finish. 0: updated by frame."]
pub type DMA_CFG_UPD_BY_BLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_FORCE_RD_STATUS` reader - 1: mask dma request when reading frame info. 0: disable mask."]
pub type DMA_FORCE_RD_STATUS_R = crate::BitReader;
#[doc = "Field `DMA_FORCE_RD_STATUS` writer - 1: mask dma request when reading frame info. 0: disable mask."]
pub type DMA_FORCE_RD_STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - DMA burst length."]
    #[inline(always)]
    pub fn dma_burst_len(&self) -> DMA_BURST_LEN_R {
        DMA_BURST_LEN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - 1: reg_dma_burst_len &amp; reg_dma_burst_len will be updated by dma block finish. 0: updated by frame."]
    #[inline(always)]
    pub fn dma_cfg_upd_by_blk(&self) -> DMA_CFG_UPD_BY_BLK_R {
        DMA_CFG_UPD_BY_BLK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: mask dma request when reading frame info. 0: disable mask."]
    #[inline(always)]
    pub fn dma_force_rd_status(&self) -> DMA_FORCE_RD_STATUS_R {
        DMA_FORCE_RD_STATUS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_REQ_CFG")
            .field(
                "dma_burst_len",
                &format_args!("{}", self.dma_burst_len().bits()),
            )
            .field(
                "dma_cfg_upd_by_blk",
                &format_args!("{}", self.dma_cfg_upd_by_blk().bit()),
            )
            .field(
                "dma_force_rd_status",
                &format_args!("{}", self.dma_force_rd_status().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_REQ_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - DMA burst length."]
    #[inline(always)]
    #[must_use]
    pub fn dma_burst_len(&mut self) -> DMA_BURST_LEN_W<DMA_REQ_CFG_SPEC> {
        DMA_BURST_LEN_W::new(self, 0)
    }
    #[doc = "Bit 12 - 1: reg_dma_burst_len &amp; reg_dma_burst_len will be updated by dma block finish. 0: updated by frame."]
    #[inline(always)]
    #[must_use]
    pub fn dma_cfg_upd_by_blk(&mut self) -> DMA_CFG_UPD_BY_BLK_W<DMA_REQ_CFG_SPEC> {
        DMA_CFG_UPD_BY_BLK_W::new(self, 12)
    }
    #[doc = "Bit 16 - 1: mask dma request when reading frame info. 0: disable mask."]
    #[inline(always)]
    #[must_use]
    pub fn dma_force_rd_status(&mut self) -> DMA_FORCE_RD_STATUS_W<DMA_REQ_CFG_SPEC> {
        DMA_FORCE_RD_STATUS_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "dma request configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_req_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_req_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_REQ_CFG_SPEC;
impl crate::RegisterSpec for DMA_REQ_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_req_cfg::R`](R) reader structure"]
impl crate::Readable for DMA_REQ_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_req_cfg::W`](W) writer structure"]
impl crate::Writable for DMA_REQ_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_REQ_CFG to value 0x80"]
impl crate::Resettable for DMA_REQ_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
