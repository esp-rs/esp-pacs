#[doc = "Register `AHB_DMA_MISC_CONF` reader"]
pub type R = crate::R<AHB_DMA_MISC_CONF_SPEC>;
#[doc = "Register `AHB_DMA_MISC_CONF` writer"]
pub type W = crate::W<AHB_DMA_MISC_CONF_SPEC>;
#[doc = "Field `AHB_DMA_AHBM_RST_INTER` reader - Write 1 and then 0 to reset the internal AHB FSM."]
pub type AHB_DMA_AHBM_RST_INTER_R = crate::BitReader;
#[doc = "Field `AHB_DMA_AHBM_RST_INTER` writer - Write 1 and then 0 to reset the internal AHB FSM."]
pub type AHB_DMA_AHBM_RST_INTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_ARB_PRI_DIS` reader - Configures whether to disable the fixed-priority channel arbitration.\\\\0: Enable\\\\1: Disable\\\\"]
pub type AHB_DMA_ARB_PRI_DIS_R = crate::BitReader;
#[doc = "Field `AHB_DMA_ARB_PRI_DIS` writer - Configures whether to disable the fixed-priority channel arbitration.\\\\0: Enable\\\\1: Disable\\\\"]
pub type AHB_DMA_ARB_PRI_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_CLK_EN` reader - Configures clock gating.\\\\0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
pub type AHB_DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `AHB_DMA_CLK_EN` writer - Configures clock gating.\\\\0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
pub type AHB_DMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 and then 0 to reset the internal AHB FSM."]
    #[inline(always)]
    pub fn ahb_dma_ahbm_rst_inter(&self) -> AHB_DMA_AHBM_RST_INTER_R {
        AHB_DMA_AHBM_RST_INTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether to disable the fixed-priority channel arbitration.\\\\0: Enable\\\\1: Disable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_arb_pri_dis(&self) -> AHB_DMA_ARB_PRI_DIS_R {
        AHB_DMA_ARB_PRI_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures clock gating.\\\\0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
    #[inline(always)]
    pub fn ahb_dma_clk_en(&self) -> AHB_DMA_CLK_EN_R {
        AHB_DMA_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_MISC_CONF")
            .field("ahb_dma_ahbm_rst_inter", &self.ahb_dma_ahbm_rst_inter())
            .field("ahb_dma_arb_pri_dis", &self.ahb_dma_arb_pri_dis())
            .field("ahb_dma_clk_en", &self.ahb_dma_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 and then 0 to reset the internal AHB FSM."]
    #[inline(always)]
    pub fn ahb_dma_ahbm_rst_inter(
        &mut self,
    ) -> AHB_DMA_AHBM_RST_INTER_W<'_, AHB_DMA_MISC_CONF_SPEC> {
        AHB_DMA_AHBM_RST_INTER_W::new(self, 0)
    }
    #[doc = "Bit 2 - Configures whether to disable the fixed-priority channel arbitration.\\\\0: Enable\\\\1: Disable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_arb_pri_dis(&mut self) -> AHB_DMA_ARB_PRI_DIS_W<'_, AHB_DMA_MISC_CONF_SPEC> {
        AHB_DMA_ARB_PRI_DIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures clock gating.\\\\0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
    #[inline(always)]
    pub fn ahb_dma_clk_en(&mut self) -> AHB_DMA_CLK_EN_W<'_, AHB_DMA_MISC_CONF_SPEC> {
        AHB_DMA_CLK_EN_W::new(self, 3)
    }
}
#[doc = "Miscellaneous register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_misc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_misc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_MISC_CONF_SPEC;
impl crate::RegisterSpec for AHB_DMA_MISC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_misc_conf::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_MISC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_misc_conf::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_MISC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_MISC_CONF to value 0"]
impl crate::Resettable for AHB_DMA_MISC_CONF_SPEC {}
