#[doc = "Register `AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH%s` reader"]
pub type R = crate::R<AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "Register `AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH%s` writer"]
pub type W = crate::W<AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "Field `AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH` reader - reserved"]
pub type AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_R = crate::BitReader;
#[doc = "Field `AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH` writer - reserved"]
pub type AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn ahb_dma_tx_arb_weigh_opt_dir_ch(&self) -> AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_R {
        AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH")
            .field(
                "ahb_dma_tx_arb_weigh_opt_dir_ch",
                &self.ahb_dma_tx_arb_weigh_opt_dir_ch(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn ahb_dma_tx_arb_weigh_opt_dir_ch(
        &mut self,
    ) -> AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_W<'_, AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_SPEC> {
        AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_W::new(self, 0)
    }
}
#[doc = "TX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_tx_arb_weigh_opt_dir_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_tx_arb_weigh_opt_dir_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_tx_arb_weigh_opt_dir_ch::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_tx_arb_weigh_opt_dir_ch::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH%s to value 0"]
impl crate::Resettable for AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_SPEC {}
