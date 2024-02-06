#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `VADR_NUM_GT_INT_RAW` reader - reg_vadr_num is greater than real interrupt raw."]
pub type VADR_NUM_GT_INT_RAW_R = crate::BitReader;
#[doc = "Field `VADR_NUM_GT_INT_RAW` writer - reg_vadr_num is greater than real interrupt raw."]
pub type VADR_NUM_GT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VADR_NUM_LT_INT_RAW` reader - reg_vadr_num is less than real interrupt raw."]
pub type VADR_NUM_LT_INT_RAW_R = crate::BitReader;
#[doc = "Field `VADR_NUM_LT_INT_RAW` writer - reg_vadr_num is less than real interrupt raw."]
pub type VADR_NUM_LT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCARD_INT_RAW` reader - an incomplete frame of data was sent interrupt raw."]
pub type DISCARD_INT_RAW_R = crate::BitReader;
#[doc = "Field `DISCARD_INT_RAW` writer - an incomplete frame of data was sent interrupt raw."]
pub type DISCARD_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_BUF_OVERRUN_INT_RAW` reader - buffer overrun interrupt raw."]
pub type CSI_BUF_OVERRUN_INT_RAW_R = crate::BitReader;
#[doc = "Field `CSI_BUF_OVERRUN_INT_RAW` writer - buffer overrun interrupt raw."]
pub type CSI_BUF_OVERRUN_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_ASYNC_FIFO_OVF_INT_RAW` reader - buffer overflow interrupt raw."]
pub type CSI_ASYNC_FIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CSI_ASYNC_FIFO_OVF_INT_RAW` writer - buffer overflow interrupt raw."]
pub type CSI_ASYNC_FIFO_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CFG_HAS_UPDATED_INT_RAW` reader - dma configuration update complete interrupt raw."]
pub type DMA_CFG_HAS_UPDATED_INT_RAW_R = crate::BitReader;
#[doc = "Field `DMA_CFG_HAS_UPDATED_INT_RAW` writer - dma configuration update complete interrupt raw."]
pub type DMA_CFG_HAS_UPDATED_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt raw."]
    #[inline(always)]
    pub fn vadr_num_gt_int_raw(&self) -> VADR_NUM_GT_INT_RAW_R {
        VADR_NUM_GT_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt raw."]
    #[inline(always)]
    pub fn vadr_num_lt_int_raw(&self) -> VADR_NUM_LT_INT_RAW_R {
        VADR_NUM_LT_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt raw."]
    #[inline(always)]
    pub fn discard_int_raw(&self) -> DISCARD_INT_RAW_R {
        DISCARD_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - buffer overrun interrupt raw."]
    #[inline(always)]
    pub fn csi_buf_overrun_int_raw(&self) -> CSI_BUF_OVERRUN_INT_RAW_R {
        CSI_BUF_OVERRUN_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer overflow interrupt raw."]
    #[inline(always)]
    pub fn csi_async_fifo_ovf_int_raw(&self) -> CSI_ASYNC_FIFO_OVF_INT_RAW_R {
        CSI_ASYNC_FIFO_OVF_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt raw."]
    #[inline(always)]
    pub fn dma_cfg_has_updated_int_raw(&self) -> DMA_CFG_HAS_UPDATED_INT_RAW_R {
        DMA_CFG_HAS_UPDATED_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "vadr_num_gt_int_raw",
                &format_args!("{}", self.vadr_num_gt_int_raw().bit()),
            )
            .field(
                "vadr_num_lt_int_raw",
                &format_args!("{}", self.vadr_num_lt_int_raw().bit()),
            )
            .field(
                "discard_int_raw",
                &format_args!("{}", self.discard_int_raw().bit()),
            )
            .field(
                "csi_buf_overrun_int_raw",
                &format_args!("{}", self.csi_buf_overrun_int_raw().bit()),
            )
            .field(
                "csi_async_fifo_ovf_int_raw",
                &format_args!("{}", self.csi_async_fifo_ovf_int_raw().bit()),
            )
            .field(
                "dma_cfg_has_updated_int_raw",
                &format_args!("{}", self.dma_cfg_has_updated_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt raw."]
    #[inline(always)]
    #[must_use]
    pub fn vadr_num_gt_int_raw(&mut self) -> VADR_NUM_GT_INT_RAW_W<INT_RAW_SPEC> {
        VADR_NUM_GT_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt raw."]
    #[inline(always)]
    #[must_use]
    pub fn vadr_num_lt_int_raw(&mut self) -> VADR_NUM_LT_INT_RAW_W<INT_RAW_SPEC> {
        VADR_NUM_LT_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt raw."]
    #[inline(always)]
    #[must_use]
    pub fn discard_int_raw(&mut self) -> DISCARD_INT_RAW_W<INT_RAW_SPEC> {
        DISCARD_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - buffer overrun interrupt raw."]
    #[inline(always)]
    #[must_use]
    pub fn csi_buf_overrun_int_raw(&mut self) -> CSI_BUF_OVERRUN_INT_RAW_W<INT_RAW_SPEC> {
        CSI_BUF_OVERRUN_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - buffer overflow interrupt raw."]
    #[inline(always)]
    #[must_use]
    pub fn csi_async_fifo_ovf_int_raw(&mut self) -> CSI_ASYNC_FIFO_OVF_INT_RAW_W<INT_RAW_SPEC> {
        CSI_ASYNC_FIFO_OVF_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt raw."]
    #[inline(always)]
    #[must_use]
    pub fn dma_cfg_has_updated_int_raw(&mut self) -> DMA_CFG_HAS_UPDATED_INT_RAW_W<INT_RAW_SPEC> {
        DMA_CFG_HAS_UPDATED_INT_RAW_W::new(self, 5)
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
#[doc = "csi bridge interrupt raw.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
