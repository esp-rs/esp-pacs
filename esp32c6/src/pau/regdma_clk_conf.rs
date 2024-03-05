#[doc = "Register `REGDMA_CLK_CONF` reader"]
pub type R = crate::R<REGDMA_CLK_CONF_SPEC>;
#[doc = "Register `REGDMA_CLK_CONF` writer"]
pub type W = crate::W<REGDMA_CLK_CONF_SPEC>;
#[doc = "Field `CLK_EN` reader - clock enable"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - clock enable"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - clock enable"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_CLK_CONF")
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<REGDMA_CLK_CONF_SPEC> {
        CLK_EN_W::new(self, 0)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regdma_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_CLK_CONF_SPEC;
impl crate::RegisterSpec for REGDMA_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_clk_conf::R`](R) reader structure"]
impl crate::Readable for REGDMA_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_clk_conf::W`](W) writer structure"]
impl crate::Writable for REGDMA_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGDMA_CLK_CONF to value 0"]
impl crate::Resettable for REGDMA_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
