///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `VADR_NUM_GT` reader - reg_vadr_num is greater than real interrupt enable.
pub type VADR_NUM_GT_R = crate::BitReader;
///Field `VADR_NUM_GT` writer - reg_vadr_num is greater than real interrupt enable.
pub type VADR_NUM_GT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VADR_NUM_LT` reader - reg_vadr_num is less than real interrupt enable.
pub type VADR_NUM_LT_R = crate::BitReader;
///Field `VADR_NUM_LT` writer - reg_vadr_num is less than real interrupt enable.
pub type VADR_NUM_LT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCARD` reader - an incomplete frame of data was sent interrupt enable.
pub type DISCARD_R = crate::BitReader;
///Field `DISCARD` writer - an incomplete frame of data was sent interrupt enable.
pub type DISCARD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSI_BUF_OVERRUN` reader - buffer overrun interrupt enable.
pub type CSI_BUF_OVERRUN_R = crate::BitReader;
///Field `CSI_BUF_OVERRUN` writer - buffer overrun interrupt enable.
pub type CSI_BUF_OVERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSI_ASYNC_FIFO_OVF` reader - buffer overflow interrupt enable.
pub type CSI_ASYNC_FIFO_OVF_R = crate::BitReader;
///Field `CSI_ASYNC_FIFO_OVF` writer - buffer overflow interrupt enable.
pub type CSI_ASYNC_FIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_CFG_HAS_UPDATED` reader - dma configuration update complete interrupt enable.
pub type DMA_CFG_HAS_UPDATED_R = crate::BitReader;
///Field `DMA_CFG_HAS_UPDATED` writer - dma configuration update complete interrupt enable.
pub type DMA_CFG_HAS_UPDATED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - reg_vadr_num is greater than real interrupt enable.
    #[inline(always)]
    pub fn vadr_num_gt(&self) -> VADR_NUM_GT_R {
        VADR_NUM_GT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - reg_vadr_num is less than real interrupt enable.
    #[inline(always)]
    pub fn vadr_num_lt(&self) -> VADR_NUM_LT_R {
        VADR_NUM_LT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - an incomplete frame of data was sent interrupt enable.
    #[inline(always)]
    pub fn discard(&self) -> DISCARD_R {
        DISCARD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - buffer overrun interrupt enable.
    #[inline(always)]
    pub fn csi_buf_overrun(&self) -> CSI_BUF_OVERRUN_R {
        CSI_BUF_OVERRUN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - buffer overflow interrupt enable.
    #[inline(always)]
    pub fn csi_async_fifo_ovf(&self) -> CSI_ASYNC_FIFO_OVF_R {
        CSI_ASYNC_FIFO_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - dma configuration update complete interrupt enable.
    #[inline(always)]
    pub fn dma_cfg_has_updated(&self) -> DMA_CFG_HAS_UPDATED_R {
        DMA_CFG_HAS_UPDATED_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("vadr_num_gt", &self.vadr_num_gt())
            .field("vadr_num_lt", &self.vadr_num_lt())
            .field("discard", &self.discard())
            .field("csi_buf_overrun", &self.csi_buf_overrun())
            .field("csi_async_fifo_ovf", &self.csi_async_fifo_ovf())
            .field("dma_cfg_has_updated", &self.dma_cfg_has_updated())
            .finish()
    }
}
impl W {
    ///Bit 0 - reg_vadr_num is greater than real interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn vadr_num_gt(&mut self) -> VADR_NUM_GT_W<INT_ENA_SPEC> {
        VADR_NUM_GT_W::new(self, 0)
    }
    ///Bit 1 - reg_vadr_num is less than real interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn vadr_num_lt(&mut self) -> VADR_NUM_LT_W<INT_ENA_SPEC> {
        VADR_NUM_LT_W::new(self, 1)
    }
    ///Bit 2 - an incomplete frame of data was sent interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn discard(&mut self) -> DISCARD_W<INT_ENA_SPEC> {
        DISCARD_W::new(self, 2)
    }
    ///Bit 3 - buffer overrun interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn csi_buf_overrun(&mut self) -> CSI_BUF_OVERRUN_W<INT_ENA_SPEC> {
        CSI_BUF_OVERRUN_W::new(self, 3)
    }
    ///Bit 4 - buffer overflow interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn csi_async_fifo_ovf(&mut self) -> CSI_ASYNC_FIFO_OVF_W<INT_ENA_SPEC> {
        CSI_ASYNC_FIFO_OVF_W::new(self, 4)
    }
    ///Bit 5 - dma configuration update complete interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn dma_cfg_has_updated(&mut self) -> DMA_CFG_HAS_UPDATED_W<INT_ENA_SPEC> {
        DMA_CFG_HAS_UPDATED_W::new(self, 5)
    }
}
/**csi bridge interrupt enable.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
