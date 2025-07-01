#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `VADR_NUM_GT_REAL` writer - reg_vadr_num is greater than real interrupt clr."]
pub type VADR_NUM_GT_REAL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VADR_NUM_LT_REAL` writer - reg_vadr_num is less than real interrupt clr."]
pub type VADR_NUM_LT_REAL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DISCARD` writer - an incomplete frame of data was sent interrupt clr."]
pub type DISCARD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CSI_BUF_OVERRUN` writer - buffer overrun interrupt clr."]
pub type CSI_BUF_OVERRUN_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CSI_ASYNC_FIFO_OVF` writer - buffer overflow interrupt clr."]
pub type CSI_ASYNC_FIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMA_CFG_HAS_UPDATED` writer - dma configuration update complete interrupt clr."]
pub type DMA_CFG_HAS_UPDATED_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt clr."]
    #[inline(always)]
    pub fn vadr_num_gt_real(&mut self) -> VADR_NUM_GT_REAL_W<INT_CLR_SPEC> {
        VADR_NUM_GT_REAL_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt clr."]
    #[inline(always)]
    pub fn vadr_num_lt_real(&mut self) -> VADR_NUM_LT_REAL_W<INT_CLR_SPEC> {
        VADR_NUM_LT_REAL_W::new(self, 1)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt clr."]
    #[inline(always)]
    pub fn discard(&mut self) -> DISCARD_W<INT_CLR_SPEC> {
        DISCARD_W::new(self, 2)
    }
    #[doc = "Bit 3 - buffer overrun interrupt clr."]
    #[inline(always)]
    pub fn csi_buf_overrun(&mut self) -> CSI_BUF_OVERRUN_W<INT_CLR_SPEC> {
        CSI_BUF_OVERRUN_W::new(self, 3)
    }
    #[doc = "Bit 4 - buffer overflow interrupt clr."]
    #[inline(always)]
    pub fn csi_async_fifo_ovf(&mut self) -> CSI_ASYNC_FIFO_OVF_W<INT_CLR_SPEC> {
        CSI_ASYNC_FIFO_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt clr."]
    #[inline(always)]
    pub fn dma_cfg_has_updated(&mut self) -> DMA_CFG_HAS_UPDATED_W<INT_CLR_SPEC> {
        DMA_CFG_HAS_UPDATED_W::new(self, 5)
    }
}
#[doc = "csi bridge interrupt clr.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
