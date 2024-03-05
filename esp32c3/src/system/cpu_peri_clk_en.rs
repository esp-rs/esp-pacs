#[doc = "Register `CPU_PERI_CLK_EN` reader"]
pub type R = crate::R<CPU_PERI_CLK_EN_SPEC>;
#[doc = "Register `CPU_PERI_CLK_EN` writer"]
pub type W = crate::W<CPU_PERI_CLK_EN_SPEC>;
#[doc = "Field `CLK_EN_ASSIST_DEBUG` reader - reg_clk_en_assist_debug"]
pub type CLK_EN_ASSIST_DEBUG_R = crate::BitReader;
#[doc = "Field `CLK_EN_ASSIST_DEBUG` writer - reg_clk_en_assist_debug"]
pub type CLK_EN_ASSIST_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN_DEDICATED_GPIO` reader - reg_clk_en_dedicated_gpio"]
pub type CLK_EN_DEDICATED_GPIO_R = crate::BitReader;
#[doc = "Field `CLK_EN_DEDICATED_GPIO` writer - reg_clk_en_dedicated_gpio"]
pub type CLK_EN_DEDICATED_GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - reg_clk_en_assist_debug"]
    #[inline(always)]
    pub fn clk_en_assist_debug(&self) -> CLK_EN_ASSIST_DEBUG_R {
        CLK_EN_ASSIST_DEBUG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_clk_en_dedicated_gpio"]
    #[inline(always)]
    pub fn clk_en_dedicated_gpio(&self) -> CLK_EN_DEDICATED_GPIO_R {
        CLK_EN_DEDICATED_GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERI_CLK_EN")
            .field(
                "clk_en_assist_debug",
                &format_args!("{}", self.clk_en_assist_debug().bit()),
            )
            .field(
                "clk_en_dedicated_gpio",
                &format_args!("{}", self.clk_en_dedicated_gpio().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_PERI_CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 6 - reg_clk_en_assist_debug"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en_assist_debug(&mut self) -> CLK_EN_ASSIST_DEBUG_W<CPU_PERI_CLK_EN_SPEC> {
        CLK_EN_ASSIST_DEBUG_W::new(self, 6)
    }
    #[doc = "Bit 7 - reg_clk_en_dedicated_gpio"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en_dedicated_gpio(&mut self) -> CLK_EN_DEDICATED_GPIO_W<CPU_PERI_CLK_EN_SPEC> {
        CLK_EN_DEDICATED_GPIO_W::new(self, 7)
    }
}
#[doc = "cpu_peripheral clock gating register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_PERI_CLK_EN_SPEC;
impl crate::RegisterSpec for CPU_PERI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_peri_clk_en::R`](R) reader structure"]
impl crate::Readable for CPU_PERI_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_peri_clk_en::W`](W) writer structure"]
impl crate::Writable for CPU_PERI_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_PERI_CLK_EN to value 0"]
impl crate::Resettable for CPU_PERI_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
