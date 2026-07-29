#[doc = "Register `LP_CLK_CTRL0` reader"]
pub type R = crate::R<LP_CLK_CTRL0_SPEC>;
#[doc = "Register `LP_CLK_CTRL0` writer"]
pub type W = crate::W<LP_CLK_CTRL0_SPEC>;
#[doc = "Field `FOSC_CLK_FORCE_ON` reader - need_des"]
pub type FOSC_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `FOSC_CLK_FORCE_ON` writer - need_des"]
pub type FOSC_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_CLK_FORCE_ON` reader - need_des"]
pub type XTAL_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `XTAL_CLK_FORCE_ON` writer - need_des"]
pub type XTAL_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL80M_CLK_FORCE_ON` reader - need_des"]
pub type PLL80M_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `PLL80M_CLK_FORCE_ON` writer - need_des"]
pub type PLL80M_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn fosc_clk_force_on(&self) -> FOSC_CLK_FORCE_ON_R {
        FOSC_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn xtal_clk_force_on(&self) -> XTAL_CLK_FORCE_ON_R {
        XTAL_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn pll80m_clk_force_on(&self) -> PLL80M_CLK_FORCE_ON_R {
        PLL80M_CLK_FORCE_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CLK_CTRL0")
            .field("fosc_clk_force_on", &self.fosc_clk_force_on())
            .field("xtal_clk_force_on", &self.xtal_clk_force_on())
            .field("pll80m_clk_force_on", &self.pll80m_clk_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn fosc_clk_force_on(&mut self) -> FOSC_CLK_FORCE_ON_W<'_, LP_CLK_CTRL0_SPEC> {
        FOSC_CLK_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn xtal_clk_force_on(&mut self) -> XTAL_CLK_FORCE_ON_W<'_, LP_CLK_CTRL0_SPEC> {
        XTAL_CLK_FORCE_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn pll80m_clk_force_on(&mut self) -> PLL80M_CLK_FORCE_ON_W<'_, LP_CLK_CTRL0_SPEC> {
        PLL80M_CLK_FORCE_ON_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CLK_CTRL0_SPEC;
impl crate::RegisterSpec for LP_CLK_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_clk_ctrl0::R`](R) reader structure"]
impl crate::Readable for LP_CLK_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_clk_ctrl0::W`](W) writer structure"]
impl crate::Writable for LP_CLK_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_CLK_CTRL0 to value 0"]
impl crate::Resettable for LP_CLK_CTRL0_SPEC {}
