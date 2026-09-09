#[doc = "Register `BTMAC_CLK_CFG` reader"]
pub type R = crate::R<BTMAC_CLK_CFG_SPEC>;
#[doc = "Register `BTMAC_CLK_CFG` writer"]
pub type W = crate::W<BTMAC_CLK_CFG_SPEC>;
#[doc = "Field `CLK_BTMAC_LOW_RATE` reader - "]
pub type CLK_BTMAC_LOW_RATE_R = crate::BitReader;
#[doc = "Field `CLK_BTMAC_LOW_RATE` writer - "]
pub type CLK_BTMAC_LOW_RATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_btmac_low_rate(&self) -> CLK_BTMAC_LOW_RATE_R {
        CLK_BTMAC_LOW_RATE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTMAC_CLK_CFG")
            .field("clk_btmac_low_rate", &self.clk_btmac_low_rate())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_btmac_low_rate(&mut self) -> CLK_BTMAC_LOW_RATE_W<'_, BTMAC_CLK_CFG_SPEC> {
        CLK_BTMAC_LOW_RATE_W::new(self, 0)
    }
}
#[doc = "BTMAC_CLK_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`btmac_clk_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btmac_clk_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTMAC_CLK_CFG_SPEC;
impl crate::RegisterSpec for BTMAC_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btmac_clk_cfg::R`](R) reader structure"]
impl crate::Readable for BTMAC_CLK_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btmac_clk_cfg::W`](W) writer structure"]
impl crate::Writable for BTMAC_CLK_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTMAC_CLK_CFG to value 0"]
impl crate::Resettable for BTMAC_CLK_CFG_SPEC {}
