#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `VADR_NUM_GT_REAL_INT_CLR` writer - reg_vadr_num is greater than real interrupt clr."]
pub type VADR_NUM_GT_REAL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VADR_NUM_LT_REAL_INT_CLR` writer - reg_vadr_num is less than real interrupt clr."]
pub type VADR_NUM_LT_REAL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCARD_INT_CLR` writer - an incomplete frame of data was sent interrupt clr."]
pub type DISCARD_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_BUF_OVERRUN_INT_CLR` writer - buffer overrun interrupt clr."]
pub type CSI_BUF_OVERRUN_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_ASYNC_FIFO_OVF_INT_CLR` writer - buffer overflow interrupt clr."]
pub type CSI_ASYNC_FIFO_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CFG_HAS_UPDATED_INT_CLR` writer - dma configuration update complete interrupt clr."]
pub type DMA_CFG_HAS_UPDATED_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt clr."]
    #[inline(always)]
    #[must_use]
    pub fn vadr_num_gt_real_int_clr(&mut self) -> VADR_NUM_GT_REAL_INT_CLR_W<INT_CLR_SPEC> {
        VADR_NUM_GT_REAL_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt clr."]
    #[inline(always)]
    #[must_use]
    pub fn vadr_num_lt_real_int_clr(&mut self) -> VADR_NUM_LT_REAL_INT_CLR_W<INT_CLR_SPEC> {
        VADR_NUM_LT_REAL_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt clr."]
    #[inline(always)]
    #[must_use]
    pub fn discard_int_clr(&mut self) -> DISCARD_INT_CLR_W<INT_CLR_SPEC> {
        DISCARD_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - buffer overrun interrupt clr."]
    #[inline(always)]
    #[must_use]
    pub fn csi_buf_overrun_int_clr(&mut self) -> CSI_BUF_OVERRUN_INT_CLR_W<INT_CLR_SPEC> {
        CSI_BUF_OVERRUN_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - buffer overflow interrupt clr."]
    #[inline(always)]
    #[must_use]
    pub fn csi_async_fifo_ovf_int_clr(&mut self) -> CSI_ASYNC_FIFO_OVF_INT_CLR_W<INT_CLR_SPEC> {
        CSI_ASYNC_FIFO_OVF_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt clr."]
    #[inline(always)]
    #[must_use]
    pub fn dma_cfg_has_updated_int_clr(&mut self) -> DMA_CFG_HAS_UPDATED_INT_CLR_W<INT_CLR_SPEC> {
        DMA_CFG_HAS_UPDATED_INT_CLR_W::new(self, 5)
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
#[doc = "csi bridge interrupt clr.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
