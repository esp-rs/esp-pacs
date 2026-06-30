#[doc = "Register `LP_AONCLKRST_ROOT_CLK_CONF` reader"]
pub type R = crate::R<LP_AONCLKRST_ROOT_CLK_CONF_SPEC>;
#[doc = "Register `LP_AONCLKRST_ROOT_CLK_CONF` writer"]
pub type W = crate::W<LP_AONCLKRST_ROOT_CLK_CONF_SPEC>;
#[doc = "Field `LP_AONCLKRST_SLOW_CLK_SEL` reader - need_des"]
pub type LP_AONCLKRST_SLOW_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_SLOW_CLK_SEL` writer - need_des"]
pub type LP_AONCLKRST_SLOW_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_FAST_CLK_SEL` reader - need_des"]
pub type LP_AONCLKRST_FAST_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_FAST_CLK_SEL` writer - need_des"]
pub type LP_AONCLKRST_FAST_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_AONCLKRST_PLL80M_CLK_FORCE_ON` reader - need_des"]
pub type LP_AONCLKRST_PLL80M_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_PLL80M_CLK_FORCE_ON` writer - need_des"]
pub type LP_AONCLKRST_PLL80M_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_XTAL_CLK_FORCE_ON` reader - need_des"]
pub type LP_AONCLKRST_XTAL_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_XTAL_CLK_FORCE_ON` writer - need_des"]
pub type LP_AONCLKRST_XTAL_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_FOSC_CLK_FORCE_ON` reader - need_des"]
pub type LP_AONCLKRST_FOSC_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_FOSC_CLK_FORCE_ON` writer - need_des"]
pub type LP_AONCLKRST_FOSC_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_ANA_SEL_REF_PLL8M` reader - need_des"]
pub type LP_AONCLKRST_ANA_SEL_REF_PLL8M_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ANA_SEL_REF_PLL8M` writer - need_des"]
pub type LP_AONCLKRST_ANA_SEL_REF_PLL8M_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_slow_clk_sel(&self) -> LP_AONCLKRST_SLOW_CLK_SEL_R {
        LP_AONCLKRST_SLOW_CLK_SEL_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fast_clk_sel(&self) -> LP_AONCLKRST_FAST_CLK_SEL_R {
        LP_AONCLKRST_FAST_CLK_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_pll80m_clk_force_on(&self) -> LP_AONCLKRST_PLL80M_CLK_FORCE_ON_R {
        LP_AONCLKRST_PLL80M_CLK_FORCE_ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_xtal_clk_force_on(&self) -> LP_AONCLKRST_XTAL_CLK_FORCE_ON_R {
        LP_AONCLKRST_XTAL_CLK_FORCE_ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fosc_clk_force_on(&self) -> LP_AONCLKRST_FOSC_CLK_FORCE_ON_R {
        LP_AONCLKRST_FOSC_CLK_FORCE_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_ana_sel_ref_pll8m(&self) -> LP_AONCLKRST_ANA_SEL_REF_PLL8M_R {
        LP_AONCLKRST_ANA_SEL_REF_PLL8M_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_ROOT_CLK_CONF")
            .field(
                "lp_aonclkrst_slow_clk_sel",
                &self.lp_aonclkrst_slow_clk_sel(),
            )
            .field(
                "lp_aonclkrst_fast_clk_sel",
                &self.lp_aonclkrst_fast_clk_sel(),
            )
            .field(
                "lp_aonclkrst_pll80m_clk_force_on",
                &self.lp_aonclkrst_pll80m_clk_force_on(),
            )
            .field(
                "lp_aonclkrst_xtal_clk_force_on",
                &self.lp_aonclkrst_xtal_clk_force_on(),
            )
            .field(
                "lp_aonclkrst_fosc_clk_force_on",
                &self.lp_aonclkrst_fosc_clk_force_on(),
            )
            .field(
                "lp_aonclkrst_ana_sel_ref_pll8m",
                &self.lp_aonclkrst_ana_sel_ref_pll8m(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_slow_clk_sel(
        &mut self,
    ) -> LP_AONCLKRST_SLOW_CLK_SEL_W<'_, LP_AONCLKRST_ROOT_CLK_CONF_SPEC> {
        LP_AONCLKRST_SLOW_CLK_SEL_W::new(self, 23)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fast_clk_sel(
        &mut self,
    ) -> LP_AONCLKRST_FAST_CLK_SEL_W<'_, LP_AONCLKRST_ROOT_CLK_CONF_SPEC> {
        LP_AONCLKRST_FAST_CLK_SEL_W::new(self, 26)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_pll80m_clk_force_on(
        &mut self,
    ) -> LP_AONCLKRST_PLL80M_CLK_FORCE_ON_W<'_, LP_AONCLKRST_ROOT_CLK_CONF_SPEC> {
        LP_AONCLKRST_PLL80M_CLK_FORCE_ON_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_xtal_clk_force_on(
        &mut self,
    ) -> LP_AONCLKRST_XTAL_CLK_FORCE_ON_W<'_, LP_AONCLKRST_ROOT_CLK_CONF_SPEC> {
        LP_AONCLKRST_XTAL_CLK_FORCE_ON_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fosc_clk_force_on(
        &mut self,
    ) -> LP_AONCLKRST_FOSC_CLK_FORCE_ON_W<'_, LP_AONCLKRST_ROOT_CLK_CONF_SPEC> {
        LP_AONCLKRST_FOSC_CLK_FORCE_ON_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_ana_sel_ref_pll8m(
        &mut self,
    ) -> LP_AONCLKRST_ANA_SEL_REF_PLL8M_W<'_, LP_AONCLKRST_ROOT_CLK_CONF_SPEC> {
        LP_AONCLKRST_ANA_SEL_REF_PLL8M_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_root_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_root_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_ROOT_CLK_CONF_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_ROOT_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_root_clk_conf::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_ROOT_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_root_clk_conf::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_ROOT_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_ROOT_CLK_CONF to value 0x0400_0000"]
impl crate::Resettable for LP_AONCLKRST_ROOT_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0400_0000;
}
