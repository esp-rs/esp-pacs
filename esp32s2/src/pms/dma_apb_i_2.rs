#[doc = "Register `DMA_APB_I_2` reader"]
pub type R = crate::R<DMA_APB_I_2_SPEC>;
#[doc = "Register `DMA_APB_I_2` writer"]
pub type W = crate::W<DMA_APB_I_2_SPEC>;
#[doc = "Field `DMA_APB_I_ILG_CLR` reader - The clear signal for internal DMA access interrupt."]
pub type DMA_APB_I_ILG_CLR_R = crate::BitReader;
#[doc = "Field `DMA_APB_I_ILG_CLR` writer - The clear signal for internal DMA access interrupt."]
pub type DMA_APB_I_ILG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_APB_I_ILG_EN` reader - The enable signal for internal DMA access interrupt."]
pub type DMA_APB_I_ILG_EN_R = crate::BitReader;
#[doc = "Field `DMA_APB_I_ILG_EN` writer - The enable signal for internal DMA access interrupt."]
pub type DMA_APB_I_ILG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_APB_I_ILG_INTR` reader - Internal DMA access interrupt signal."]
pub type DMA_APB_I_ILG_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The clear signal for internal DMA access interrupt."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_clr(&self) -> DMA_APB_I_ILG_CLR_R {
        DMA_APB_I_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for internal DMA access interrupt."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_en(&self) -> DMA_APB_I_ILG_EN_R {
        DMA_APB_I_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal DMA access interrupt signal."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_intr(&self) -> DMA_APB_I_ILG_INTR_R {
        DMA_APB_I_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APB_I_2")
            .field("dma_apb_i_ilg_clr", &self.dma_apb_i_ilg_clr())
            .field("dma_apb_i_ilg_en", &self.dma_apb_i_ilg_en())
            .field("dma_apb_i_ilg_intr", &self.dma_apb_i_ilg_intr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for internal DMA access interrupt."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_clr(&mut self) -> DMA_APB_I_ILG_CLR_W<DMA_APB_I_2_SPEC> {
        DMA_APB_I_ILG_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable signal for internal DMA access interrupt."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_en(&mut self) -> DMA_APB_I_ILG_EN_W<DMA_APB_I_2_SPEC> {
        DMA_APB_I_ILG_EN_W::new(self, 1)
    }
}
#[doc = "Internal DMA permission control register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_apb_i_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_apb_i_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_APB_I_2_SPEC;
impl crate::RegisterSpec for DMA_APB_I_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_apb_i_2::R`](R) reader structure"]
impl crate::Readable for DMA_APB_I_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_apb_i_2::W`](W) writer structure"]
impl crate::Writable for DMA_APB_I_2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_APB_I_2 to value 0"]
impl crate::Resettable for DMA_APB_I_2_SPEC {}
