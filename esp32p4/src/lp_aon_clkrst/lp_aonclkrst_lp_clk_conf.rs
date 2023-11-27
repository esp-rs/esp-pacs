#[doc = "Register `LP_AONCLKRST_LP_CLK_CONF` reader"]
pub type R = crate::R<LP_AONCLKRST_LP_CLK_CONF_SPEC>;
#[doc = "Register `LP_AONCLKRST_LP_CLK_CONF` writer"]
pub type W = crate::W<LP_AONCLKRST_LP_CLK_CONF_SPEC>;
#[doc = "Field `LP_AONCLKRST_SLOW_CLK_SEL` reader - need_des"]
pub type LP_AONCLKRST_SLOW_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_SLOW_CLK_SEL` writer - need_des"]
pub type LP_AONCLKRST_SLOW_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_AONCLKRST_FAST_CLK_SEL` reader - need_des"]
pub type LP_AONCLKRST_FAST_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_FAST_CLK_SEL` writer - need_des"]
pub type LP_AONCLKRST_FAST_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_AONCLKRST_LP_PERI_DIV_NUM` reader - need_des"]
pub type LP_AONCLKRST_LP_PERI_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_LP_PERI_DIV_NUM` writer - need_des"]
pub type LP_AONCLKRST_LP_PERI_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LP_AONCLKRST_ANA_SEL_REF_PLL8M` reader - need_des"]
pub type LP_AONCLKRST_ANA_SEL_REF_PLL8M_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ANA_SEL_REF_PLL8M` writer - need_des"]
pub type LP_AONCLKRST_ANA_SEL_REF_PLL8M_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_slow_clk_sel(&self) -> LP_AONCLKRST_SLOW_CLK_SEL_R {
        LP_AONCLKRST_SLOW_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fast_clk_sel(&self) -> LP_AONCLKRST_FAST_CLK_SEL_R {
        LP_AONCLKRST_FAST_CLK_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:9 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_peri_div_num(&self) -> LP_AONCLKRST_LP_PERI_DIV_NUM_R {
        LP_AONCLKRST_LP_PERI_DIV_NUM_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_ana_sel_ref_pll8m(&self) -> LP_AONCLKRST_ANA_SEL_REF_PLL8M_R {
        LP_AONCLKRST_ANA_SEL_REF_PLL8M_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_LP_CLK_CONF")
            .field(
                "lp_aonclkrst_slow_clk_sel",
                &format_args!("{}", self.lp_aonclkrst_slow_clk_sel().bits()),
            )
            .field(
                "lp_aonclkrst_fast_clk_sel",
                &format_args!("{}", self.lp_aonclkrst_fast_clk_sel().bits()),
            )
            .field(
                "lp_aonclkrst_lp_peri_div_num",
                &format_args!("{}", self.lp_aonclkrst_lp_peri_div_num().bits()),
            )
            .field(
                "lp_aonclkrst_ana_sel_ref_pll8m",
                &format_args!("{}", self.lp_aonclkrst_ana_sel_ref_pll8m().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_AONCLKRST_LP_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_slow_clk_sel(
        &mut self,
    ) -> LP_AONCLKRST_SLOW_CLK_SEL_W<LP_AONCLKRST_LP_CLK_CONF_SPEC> {
        LP_AONCLKRST_SLOW_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_fast_clk_sel(
        &mut self,
    ) -> LP_AONCLKRST_FAST_CLK_SEL_W<LP_AONCLKRST_LP_CLK_CONF_SPEC> {
        LP_AONCLKRST_FAST_CLK_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:9 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_lp_peri_div_num(
        &mut self,
    ) -> LP_AONCLKRST_LP_PERI_DIV_NUM_W<LP_AONCLKRST_LP_CLK_CONF_SPEC> {
        LP_AONCLKRST_LP_PERI_DIV_NUM_W::new(self, 4)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_ana_sel_ref_pll8m(
        &mut self,
    ) -> LP_AONCLKRST_ANA_SEL_REF_PLL8M_W<LP_AONCLKRST_LP_CLK_CONF_SPEC> {
        LP_AONCLKRST_ANA_SEL_REF_PLL8M_W::new(self, 10)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_lp_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_LP_CLK_CONF_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_LP_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_lp_clk_conf::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_LP_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_lp_clk_conf::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_LP_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_LP_CLK_CONF to value 0x04"]
impl crate::Resettable for LP_AONCLKRST_LP_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
