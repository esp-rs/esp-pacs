#[doc = "Register `AHB_DMA_IN_CONF1_CH%s` reader"]
pub type R = crate::R<AHB_DMA_IN_CONF1_CH_SPEC>;
#[doc = "Register `AHB_DMA_IN_CONF1_CH%s` writer"]
pub type W = crate::W<AHB_DMA_IN_CONF1_CH_SPEC>;
#[doc = "Field `AHB_DMA_IN_CHECK_OWNER_CH` reader - Configures whether to enable owner bit check for RX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_IN_CHECK_OWNER_CH_R = crate::BitReader;
#[doc = "Field `AHB_DMA_IN_CHECK_OWNER_CH` writer - Configures whether to enable owner bit check for RX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_IN_CHECK_OWNER_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Configures whether to enable owner bit check for RX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_in_check_owner_ch(&self) -> AHB_DMA_IN_CHECK_OWNER_CH_R {
        AHB_DMA_IN_CHECK_OWNER_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_IN_CONF1_CH")
            .field(
                "ahb_dma_in_check_owner_ch",
                &self.ahb_dma_in_check_owner_ch(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Configures whether to enable owner bit check for RX channel %s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_in_check_owner_ch(
        &mut self,
    ) -> AHB_DMA_IN_CHECK_OWNER_CH_W<'_, AHB_DMA_IN_CONF1_CH_SPEC> {
        AHB_DMA_IN_CHECK_OWNER_CH_W::new(self, 12)
    }
}
#[doc = "Configuration register 1 of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_conf1_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_conf1_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_IN_CONF1_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_IN_CONF1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_in_conf1_ch::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_IN_CONF1_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_in_conf1_ch::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_IN_CONF1_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_IN_CONF1_CH%s to value 0"]
impl crate::Resettable for AHB_DMA_IN_CONF1_CH_SPEC {}
