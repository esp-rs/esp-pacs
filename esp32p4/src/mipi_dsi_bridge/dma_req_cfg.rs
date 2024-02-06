#[doc = "Register `DMA_REQ_CFG` reader"]
pub type R = crate::R<DMA_REQ_CFG_SPEC>;
#[doc = "Register `DMA_REQ_CFG` writer"]
pub type W = crate::W<DMA_REQ_CFG_SPEC>;
#[doc = "Field `DMA_BURST_LEN` reader - this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller"]
pub type DMA_BURST_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `DMA_BURST_LEN` writer - this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller"]
pub type DMA_BURST_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn dma_burst_len(&self) -> DMA_BURST_LEN_R {
        DMA_BURST_LEN_R::new((self.bits & 0x0fff) as u16)
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
    #[doc = "Bits 0:11 - this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    #[must_use]
    pub fn dma_burst_len(&mut self) -> DMA_BURST_LEN_W<DMA_REQ_CFG_SPEC> {
        DMA_BURST_LEN_W::new(self, 0)
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
#[doc = "dsi bridge dma burst len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_req_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_req_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_REQ_CFG_SPEC;
impl crate::RegisterSpec for DMA_REQ_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_req_cfg::R`](R) reader structure"]
impl crate::Readable for DMA_REQ_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_req_cfg::W`](W) writer structure"]
impl crate::Writable for DMA_REQ_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_REQ_CFG to value 0x80"]
impl crate::Resettable for DMA_REQ_CFG_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
