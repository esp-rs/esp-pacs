#[doc = "Register `AHB_DMA_MODULE_CLK_EN` reader"]
pub type R = crate::R<AHB_DMA_MODULE_CLK_EN_SPEC>;
#[doc = "Register `AHB_DMA_MODULE_CLK_EN` writer"]
pub type W = crate::W<AHB_DMA_MODULE_CLK_EN_SPEC>;
#[doc = "Field `AHB_DMA_AHB_APB_SYNC_CLK_EN` reader - Configures whether to force on ahb_apb_sync 1~0 module clock. For bit n:\\\\0 : Not force on ahb_apb_sync n clock \\\\1 : Force on ahb_apb_sync n clock\\\\"]
pub type AHB_DMA_AHB_APB_SYNC_CLK_EN_R = crate::FieldReader;
#[doc = "Field `AHB_DMA_AHB_APB_SYNC_CLK_EN` writer - Configures whether to force on ahb_apb_sync 1~0 module clock. For bit n:\\\\0 : Not force on ahb_apb_sync n clock \\\\1 : Force on ahb_apb_sync n clock\\\\"]
pub type AHB_DMA_AHB_APB_SYNC_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AHB_DMA_OUT_DSCR_CLK_EN` reader - Configures whether to force on out_dscr 1~0 module clock. For bit n:\\\\0 : Not force on out_dscr n clock \\\\1 : Force on out_dscr n clock\\\\"]
pub type AHB_DMA_OUT_DSCR_CLK_EN_R = crate::FieldReader;
#[doc = "Field `AHB_DMA_OUT_DSCR_CLK_EN` writer - Configures whether to force on out_dscr 1~0 module clock. For bit n:\\\\0 : Not force on out_dscr n clock \\\\1 : Force on out_dscr n clock\\\\"]
pub type AHB_DMA_OUT_DSCR_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AHB_DMA_OUT_CTRL_CLK_EN` reader - Configures whether to force on out_ctrl 1~0 module clock. For bit n:\\\\0 : Not force on out_ctrl n clock \\\\1 : Force on out_ctrl n clock\\\\"]
pub type AHB_DMA_OUT_CTRL_CLK_EN_R = crate::FieldReader;
#[doc = "Field `AHB_DMA_OUT_CTRL_CLK_EN` writer - Configures whether to force on out_ctrl 1~0 module clock. For bit n:\\\\0 : Not force on out_ctrl n clock \\\\1 : Force on out_ctrl n clock\\\\"]
pub type AHB_DMA_OUT_CTRL_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AHB_DMA_IN_DSCR_CLK_EN` reader - Configures whether to force on in_dscr 1~0 module clock. For bit n:\\\\0 : Not force on in_dscr n clock \\\\1 : Force on in_dscr n clock\\\\"]
pub type AHB_DMA_IN_DSCR_CLK_EN_R = crate::FieldReader;
#[doc = "Field `AHB_DMA_IN_DSCR_CLK_EN` writer - Configures whether to force on in_dscr 1~0 module clock. For bit n:\\\\0 : Not force on in_dscr n clock \\\\1 : Force on in_dscr n clock\\\\"]
pub type AHB_DMA_IN_DSCR_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AHB_DMA_IN_CTRL_CLK_EN` reader - Configures whether to force on in_ctrl 1~0 module clock. For bit n:\\\\0 : Not force on in_ctrl n clock \\\\1 : Force on in_ctrl n clock\\\\"]
pub type AHB_DMA_IN_CTRL_CLK_EN_R = crate::FieldReader;
#[doc = "Field `AHB_DMA_IN_CTRL_CLK_EN` writer - Configures whether to force on in_ctrl 1~0 module clock. For bit n:\\\\0 : Not force on in_ctrl n clock \\\\1 : Force on in_ctrl n clock\\\\"]
pub type AHB_DMA_IN_CTRL_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AHB_DMA_CMD_ARB_CLK_EN` reader - Configures whether to force on cmd_arb module clock. \\\\0 : Not force on cmd_arb clock \\\\1 : Force on cmd_arb clock\\\\"]
pub type AHB_DMA_CMD_ARB_CLK_EN_R = crate::BitReader;
#[doc = "Field `AHB_DMA_CMD_ARB_CLK_EN` writer - Configures whether to force on cmd_arb module clock. \\\\0 : Not force on cmd_arb clock \\\\1 : Force on cmd_arb clock\\\\"]
pub type AHB_DMA_CMD_ARB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_AHBINF_CLK_EN` reader - Configures whether to force on ahbinf module clock. \\\\0 : Not force on ahbinf clock \\\\1 : Force on ahbinf clock\\\\"]
pub type AHB_DMA_AHBINF_CLK_EN_R = crate::BitReader;
#[doc = "Field `AHB_DMA_AHBINF_CLK_EN` writer - Configures whether to force on ahbinf module clock. \\\\0 : Not force on ahbinf clock \\\\1 : Force on ahbinf clock\\\\"]
pub type AHB_DMA_AHBINF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures whether to force on ahb_apb_sync 1~0 module clock. For bit n:\\\\0 : Not force on ahb_apb_sync n clock \\\\1 : Force on ahb_apb_sync n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_ahb_apb_sync_clk_en(&self) -> AHB_DMA_AHB_APB_SYNC_CLK_EN_R {
        AHB_DMA_AHB_APB_SYNC_CLK_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 5:6 - Configures whether to force on out_dscr 1~0 module clock. For bit n:\\\\0 : Not force on out_dscr n clock \\\\1 : Force on out_dscr n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_dscr_clk_en(&self) -> AHB_DMA_OUT_DSCR_CLK_EN_R {
        AHB_DMA_OUT_DSCR_CLK_EN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Configures whether to force on out_ctrl 1~0 module clock. For bit n:\\\\0 : Not force on out_ctrl n clock \\\\1 : Force on out_ctrl n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_ctrl_clk_en(&self) -> AHB_DMA_OUT_CTRL_CLK_EN_R {
        AHB_DMA_OUT_CTRL_CLK_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 15:16 - Configures whether to force on in_dscr 1~0 module clock. For bit n:\\\\0 : Not force on in_dscr n clock \\\\1 : Force on in_dscr n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_in_dscr_clk_en(&self) -> AHB_DMA_IN_DSCR_CLK_EN_R {
        AHB_DMA_IN_DSCR_CLK_EN_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Configures whether to force on in_ctrl 1~0 module clock. For bit n:\\\\0 : Not force on in_ctrl n clock \\\\1 : Force on in_ctrl n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_in_ctrl_clk_en(&self) -> AHB_DMA_IN_CTRL_CLK_EN_R {
        AHB_DMA_IN_CTRL_CLK_EN_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 27 - Configures whether to force on cmd_arb module clock. \\\\0 : Not force on cmd_arb clock \\\\1 : Force on cmd_arb clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_cmd_arb_clk_en(&self) -> AHB_DMA_CMD_ARB_CLK_EN_R {
        AHB_DMA_CMD_ARB_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Configures whether to force on ahbinf module clock. \\\\0 : Not force on ahbinf clock \\\\1 : Force on ahbinf clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_ahbinf_clk_en(&self) -> AHB_DMA_AHBINF_CLK_EN_R {
        AHB_DMA_AHBINF_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_MODULE_CLK_EN")
            .field(
                "ahb_dma_ahb_apb_sync_clk_en",
                &self.ahb_dma_ahb_apb_sync_clk_en(),
            )
            .field("ahb_dma_out_dscr_clk_en", &self.ahb_dma_out_dscr_clk_en())
            .field("ahb_dma_out_ctrl_clk_en", &self.ahb_dma_out_ctrl_clk_en())
            .field("ahb_dma_in_dscr_clk_en", &self.ahb_dma_in_dscr_clk_en())
            .field("ahb_dma_in_ctrl_clk_en", &self.ahb_dma_in_ctrl_clk_en())
            .field("ahb_dma_cmd_arb_clk_en", &self.ahb_dma_cmd_arb_clk_en())
            .field("ahb_dma_ahbinf_clk_en", &self.ahb_dma_ahbinf_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures whether to force on ahb_apb_sync 1~0 module clock. For bit n:\\\\0 : Not force on ahb_apb_sync n clock \\\\1 : Force on ahb_apb_sync n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_ahb_apb_sync_clk_en(
        &mut self,
    ) -> AHB_DMA_AHB_APB_SYNC_CLK_EN_W<'_, AHB_DMA_MODULE_CLK_EN_SPEC> {
        AHB_DMA_AHB_APB_SYNC_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bits 5:6 - Configures whether to force on out_dscr 1~0 module clock. For bit n:\\\\0 : Not force on out_dscr n clock \\\\1 : Force on out_dscr n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_dscr_clk_en(
        &mut self,
    ) -> AHB_DMA_OUT_DSCR_CLK_EN_W<'_, AHB_DMA_MODULE_CLK_EN_SPEC> {
        AHB_DMA_OUT_DSCR_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bits 10:11 - Configures whether to force on out_ctrl 1~0 module clock. For bit n:\\\\0 : Not force on out_ctrl n clock \\\\1 : Force on out_ctrl n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_out_ctrl_clk_en(
        &mut self,
    ) -> AHB_DMA_OUT_CTRL_CLK_EN_W<'_, AHB_DMA_MODULE_CLK_EN_SPEC> {
        AHB_DMA_OUT_CTRL_CLK_EN_W::new(self, 10)
    }
    #[doc = "Bits 15:16 - Configures whether to force on in_dscr 1~0 module clock. For bit n:\\\\0 : Not force on in_dscr n clock \\\\1 : Force on in_dscr n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_in_dscr_clk_en(
        &mut self,
    ) -> AHB_DMA_IN_DSCR_CLK_EN_W<'_, AHB_DMA_MODULE_CLK_EN_SPEC> {
        AHB_DMA_IN_DSCR_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bits 20:21 - Configures whether to force on in_ctrl 1~0 module clock. For bit n:\\\\0 : Not force on in_ctrl n clock \\\\1 : Force on in_ctrl n clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_in_ctrl_clk_en(
        &mut self,
    ) -> AHB_DMA_IN_CTRL_CLK_EN_W<'_, AHB_DMA_MODULE_CLK_EN_SPEC> {
        AHB_DMA_IN_CTRL_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 27 - Configures whether to force on cmd_arb module clock. \\\\0 : Not force on cmd_arb clock \\\\1 : Force on cmd_arb clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_cmd_arb_clk_en(
        &mut self,
    ) -> AHB_DMA_CMD_ARB_CLK_EN_W<'_, AHB_DMA_MODULE_CLK_EN_SPEC> {
        AHB_DMA_CMD_ARB_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether to force on ahbinf module clock. \\\\0 : Not force on ahbinf clock \\\\1 : Force on ahbinf clock\\\\"]
    #[inline(always)]
    pub fn ahb_dma_ahbinf_clk_en(
        &mut self,
    ) -> AHB_DMA_AHBINF_CLK_EN_W<'_, AHB_DMA_MODULE_CLK_EN_SPEC> {
        AHB_DMA_AHBINF_CLK_EN_W::new(self, 28)
    }
}
#[doc = "Module clock force on register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_module_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_module_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_MODULE_CLK_EN_SPEC;
impl crate::RegisterSpec for AHB_DMA_MODULE_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_module_clk_en::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_MODULE_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_module_clk_en::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_MODULE_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_MODULE_CLK_EN to value 0x1831_8c63"]
impl crate::Resettable for AHB_DMA_MODULE_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0x1831_8c63;
}
