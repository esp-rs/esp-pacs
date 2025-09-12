#[doc = "Register `DMA_TX_I_0` reader"]
pub type R = crate::R<DMA_TX_I_0_SPEC>;
#[doc = "Register `DMA_TX_I_0` writer"]
pub type W = crate::W<DMA_TX_I_0_SPEC>;
#[doc = "Field `DMA_TX_I_LOCK` reader - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
pub type DMA_TX_I_LOCK_R = crate::BitReader;
#[doc = "Field `DMA_TX_I_LOCK` writer - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
pub type DMA_TX_I_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
    #[inline(always)]
    pub fn dma_tx_i_lock(&self) -> DMA_TX_I_LOCK_R {
        DMA_TX_I_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TX_I_0")
            .field("dma_tx_i_lock", &self.dma_tx_i_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
    #[inline(always)]
    pub fn dma_tx_i_lock(&mut self) -> DMA_TX_I_LOCK_W<'_, DMA_TX_I_0_SPEC> {
        DMA_TX_I_LOCK_W::new(self, 0)
    }
}
#[doc = "TX Copy DMA permission control register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tx_i_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tx_i_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_TX_I_0_SPEC;
impl crate::RegisterSpec for DMA_TX_I_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_tx_i_0::R`](R) reader structure"]
impl crate::Readable for DMA_TX_I_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_tx_i_0::W`](W) writer structure"]
impl crate::Writable for DMA_TX_I_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_TX_I_0 to value 0"]
impl crate::Resettable for DMA_TX_I_0_SPEC {}
