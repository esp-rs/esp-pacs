#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `VADR_NUM_GT_INT_ENA` reader - reg_vadr_num is greater than real interrupt enable."]
pub type VADR_NUM_GT_INT_ENA_R = crate::BitReader;
#[doc = "Field `VADR_NUM_GT_INT_ENA` writer - reg_vadr_num is greater than real interrupt enable."]
pub type VADR_NUM_GT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VADR_NUM_LT_INT_ENA` reader - reg_vadr_num is less than real interrupt enable."]
pub type VADR_NUM_LT_INT_ENA_R = crate::BitReader;
#[doc = "Field `VADR_NUM_LT_INT_ENA` writer - reg_vadr_num is less than real interrupt enable."]
pub type VADR_NUM_LT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCARD_INT_ENA` reader - an incomplete frame of data was sent interrupt enable."]
pub type DISCARD_INT_ENA_R = crate::BitReader;
#[doc = "Field `DISCARD_INT_ENA` writer - an incomplete frame of data was sent interrupt enable."]
pub type DISCARD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_BUF_OVERRUN_INT_ENA` reader - buffer overrun interrupt enable."]
pub type CSI_BUF_OVERRUN_INT_ENA_R = crate::BitReader;
#[doc = "Field `CSI_BUF_OVERRUN_INT_ENA` writer - buffer overrun interrupt enable."]
pub type CSI_BUF_OVERRUN_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_ASYNC_FIFO_OVF_INT_ENA` reader - buffer overflow interrupt enable."]
pub type CSI_ASYNC_FIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `CSI_ASYNC_FIFO_OVF_INT_ENA` writer - buffer overflow interrupt enable."]
pub type CSI_ASYNC_FIFO_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CFG_HAS_UPDATED_INT_ENA` reader - dma configuration update complete interrupt enable."]
pub type DMA_CFG_HAS_UPDATED_INT_ENA_R = crate::BitReader;
#[doc = "Field `DMA_CFG_HAS_UPDATED_INT_ENA` writer - dma configuration update complete interrupt enable."]
pub type DMA_CFG_HAS_UPDATED_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt enable."]
    #[inline(always)]
    pub fn vadr_num_gt_int_ena(&self) -> VADR_NUM_GT_INT_ENA_R {
        VADR_NUM_GT_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt enable."]
    #[inline(always)]
    pub fn vadr_num_lt_int_ena(&self) -> VADR_NUM_LT_INT_ENA_R {
        VADR_NUM_LT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt enable."]
    #[inline(always)]
    pub fn discard_int_ena(&self) -> DISCARD_INT_ENA_R {
        DISCARD_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - buffer overrun interrupt enable."]
    #[inline(always)]
    pub fn csi_buf_overrun_int_ena(&self) -> CSI_BUF_OVERRUN_INT_ENA_R {
        CSI_BUF_OVERRUN_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer overflow interrupt enable."]
    #[inline(always)]
    pub fn csi_async_fifo_ovf_int_ena(&self) -> CSI_ASYNC_FIFO_OVF_INT_ENA_R {
        CSI_ASYNC_FIFO_OVF_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt enable."]
    #[inline(always)]
    pub fn dma_cfg_has_updated_int_ena(&self) -> DMA_CFG_HAS_UPDATED_INT_ENA_R {
        DMA_CFG_HAS_UPDATED_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "vadr_num_gt_int_ena",
                &format_args!("{}", self.vadr_num_gt_int_ena().bit()),
            )
            .field(
                "vadr_num_lt_int_ena",
                &format_args!("{}", self.vadr_num_lt_int_ena().bit()),
            )
            .field(
                "discard_int_ena",
                &format_args!("{}", self.discard_int_ena().bit()),
            )
            .field(
                "csi_buf_overrun_int_ena",
                &format_args!("{}", self.csi_buf_overrun_int_ena().bit()),
            )
            .field(
                "csi_async_fifo_ovf_int_ena",
                &format_args!("{}", self.csi_async_fifo_ovf_int_ena().bit()),
            )
            .field(
                "dma_cfg_has_updated_int_ena",
                &format_args!("{}", self.dma_cfg_has_updated_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn vadr_num_gt_int_ena(&mut self) -> VADR_NUM_GT_INT_ENA_W<INT_ENA_SPEC> {
        VADR_NUM_GT_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn vadr_num_lt_int_ena(&mut self) -> VADR_NUM_LT_INT_ENA_W<INT_ENA_SPEC> {
        VADR_NUM_LT_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn discard_int_ena(&mut self) -> DISCARD_INT_ENA_W<INT_ENA_SPEC> {
        DISCARD_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - buffer overrun interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn csi_buf_overrun_int_ena(&mut self) -> CSI_BUF_OVERRUN_INT_ENA_W<INT_ENA_SPEC> {
        CSI_BUF_OVERRUN_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - buffer overflow interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn csi_async_fifo_ovf_int_ena(&mut self) -> CSI_ASYNC_FIFO_OVF_INT_ENA_W<INT_ENA_SPEC> {
        CSI_ASYNC_FIFO_OVF_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_cfg_has_updated_int_ena(&mut self) -> DMA_CFG_HAS_UPDATED_INT_ENA_W<INT_ENA_SPEC> {
        DMA_CFG_HAS_UPDATED_INT_ENA_W::new(self, 5)
    }
}
#[doc = "csi bridge interrupt enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
