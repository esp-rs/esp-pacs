#[doc = "Register `DMA_RX_I_2` reader"]
pub type R = crate::R<DMA_RX_I_2_SPEC>;
#[doc = "Register `DMA_RX_I_2` writer"]
pub type W = crate::W<DMA_RX_I_2_SPEC>;
#[doc = "Field `DMA_RX_I_ILG_CLR` reader - The clear signal for RX Copy DMA access interrupt."]
pub type DMA_RX_I_ILG_CLR_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_ILG_CLR` writer - The clear signal for RX Copy DMA access interrupt."]
pub type DMA_RX_I_ILG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RX_I_ILG_EN` reader - The enable signal for RX Copy DMA access interrupt."]
pub type DMA_RX_I_ILG_EN_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_ILG_EN` writer - The enable signal for RX Copy DMA access interrupt."]
pub type DMA_RX_I_ILG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RX_I_ILG_INTR` reader - RX Copy DMA access interrupt signal."]
pub type DMA_RX_I_ILG_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The clear signal for RX Copy DMA access interrupt."]
    #[inline(always)]
    pub fn dma_rx_i_ilg_clr(&self) -> DMA_RX_I_ILG_CLR_R {
        DMA_RX_I_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for RX Copy DMA access interrupt."]
    #[inline(always)]
    pub fn dma_rx_i_ilg_en(&self) -> DMA_RX_I_ILG_EN_R {
        DMA_RX_I_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Copy DMA access interrupt signal."]
    #[inline(always)]
    pub fn dma_rx_i_ilg_intr(&self) -> DMA_RX_I_ILG_INTR_R {
        DMA_RX_I_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_RX_I_2")
            .field("dma_rx_i_ilg_clr", &self.dma_rx_i_ilg_clr())
            .field("dma_rx_i_ilg_en", &self.dma_rx_i_ilg_en())
            .field("dma_rx_i_ilg_intr", &self.dma_rx_i_ilg_intr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for RX Copy DMA access interrupt."]
    #[inline(always)]
    pub fn dma_rx_i_ilg_clr(&mut self) -> DMA_RX_I_ILG_CLR_W<DMA_RX_I_2_SPEC> {
        DMA_RX_I_ILG_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable signal for RX Copy DMA access interrupt."]
    #[inline(always)]
    pub fn dma_rx_i_ilg_en(&mut self) -> DMA_RX_I_ILG_EN_W<DMA_RX_I_2_SPEC> {
        DMA_RX_I_ILG_EN_W::new(self, 1)
    }
}
#[doc = "RX Copy DMA permission control register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rx_i_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_i_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_RX_I_2_SPEC;
impl crate::RegisterSpec for DMA_RX_I_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rx_i_2::R`](R) reader structure"]
impl crate::Readable for DMA_RX_I_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_rx_i_2::W`](W) writer structure"]
impl crate::Writable for DMA_RX_I_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_RX_I_2 to value 0"]
impl crate::Resettable for DMA_RX_I_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
