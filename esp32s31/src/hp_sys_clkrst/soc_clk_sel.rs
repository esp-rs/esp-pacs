#[doc = "Register `SOC_CLK_SEL` reader"]
pub type R = crate::R<SOC_CLK_SEL_SPEC>;
#[doc = "Register `SOC_CLK_SEL` writer"]
pub type W = crate::W<SOC_CLK_SEL_SPEC>;
#[doc = "Field `REG_SOC_CLK_SEL` reader - Configures to select the clock source of HP_ROOT_CLK.\\\\ 0 (default): XTAL_CLK\\\\ 1: PLL_F300M_CLK\\\\ 2: RC_FAST_CLK\\\\"]
pub type REG_SOC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `REG_SOC_CLK_SEL` writer - Configures to select the clock source of HP_ROOT_CLK.\\\\ 0 (default): XTAL_CLK\\\\ 1: PLL_F300M_CLK\\\\ 2: RC_FAST_CLK\\\\"]
pub type REG_SOC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Configures to select the clock source of HP_ROOT_CLK.\\\\ 0 (default): XTAL_CLK\\\\ 1: PLL_F300M_CLK\\\\ 2: RC_FAST_CLK\\\\"]
    #[inline(always)]
    pub fn reg_soc_clk_sel(&self) -> REG_SOC_CLK_SEL_R {
        REG_SOC_CLK_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOC_CLK_SEL")
            .field("reg_soc_clk_sel", &self.reg_soc_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures to select the clock source of HP_ROOT_CLK.\\\\ 0 (default): XTAL_CLK\\\\ 1: PLL_F300M_CLK\\\\ 2: RC_FAST_CLK\\\\"]
    #[inline(always)]
    pub fn reg_soc_clk_sel(&mut self) -> REG_SOC_CLK_SEL_W<'_, SOC_CLK_SEL_SPEC> {
        REG_SOC_CLK_SEL_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_clk_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_clk_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOC_CLK_SEL_SPEC;
impl crate::RegisterSpec for SOC_CLK_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_clk_sel::R`](R) reader structure"]
impl crate::Readable for SOC_CLK_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soc_clk_sel::W`](W) writer structure"]
impl crate::Writable for SOC_CLK_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOC_CLK_SEL to value 0"]
impl crate::Resettable for SOC_CLK_SEL_SPEC {}
