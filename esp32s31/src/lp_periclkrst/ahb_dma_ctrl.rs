#[doc = "Register `AHB_DMA_CTRL` reader"]
pub type R = crate::R<AHB_DMA_CTRL_SPEC>;
#[doc = "Register `AHB_DMA_CTRL` writer"]
pub type W = crate::W<AHB_DMA_CTRL_SPEC>;
#[doc = "Field `LP_AHB_DMA_CLK_EN` reader - need_des"]
pub type LP_AHB_DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AHB_DMA_CLK_EN` writer - need_des"]
pub type LP_AHB_DMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AHB_DMA_RST_EN` reader - need_des"]
pub type LP_AHB_DMA_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_AHB_DMA_RST_EN` writer - need_des"]
pub type LP_AHB_DMA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_ahb_dma_clk_en(&self) -> LP_AHB_DMA_CLK_EN_R {
        LP_AHB_DMA_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_ahb_dma_rst_en(&self) -> LP_AHB_DMA_RST_EN_R {
        LP_AHB_DMA_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_CTRL")
            .field("lp_ahb_dma_clk_en", &self.lp_ahb_dma_clk_en())
            .field("lp_ahb_dma_rst_en", &self.lp_ahb_dma_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_ahb_dma_clk_en(&mut self) -> LP_AHB_DMA_CLK_EN_W<'_, AHB_DMA_CTRL_SPEC> {
        LP_AHB_DMA_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_ahb_dma_rst_en(&mut self) -> LP_AHB_DMA_RST_EN_W<'_, AHB_DMA_CTRL_SPEC> {
        LP_AHB_DMA_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_CTRL_SPEC;
impl crate::RegisterSpec for AHB_DMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_ctrl::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_ctrl::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_CTRL to value 0"]
impl crate::Resettable for AHB_DMA_CTRL_SPEC {}
