#[doc = "Register `COEX_LP_CLK_CONF` reader"]
pub type R = crate::R<COEX_LP_CLK_CONF_SPEC>;
#[doc = "Register `COEX_LP_CLK_CONF` writer"]
pub type W = crate::W<COEX_LP_CLK_CONF_SPEC>;
#[doc = "Field `CLK_COEX_LP_SEL_OSC_SLOW` reader - "]
pub type CLK_COEX_LP_SEL_OSC_SLOW_R = crate::BitReader;
#[doc = "Field `CLK_COEX_LP_SEL_OSC_SLOW` writer - "]
pub type CLK_COEX_LP_SEL_OSC_SLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_COEX_LP_SEL_OSC_FAST` reader - "]
pub type CLK_COEX_LP_SEL_OSC_FAST_R = crate::BitReader;
#[doc = "Field `CLK_COEX_LP_SEL_OSC_FAST` writer - "]
pub type CLK_COEX_LP_SEL_OSC_FAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_COEX_LP_SEL_XTAL` reader - "]
pub type CLK_COEX_LP_SEL_XTAL_R = crate::BitReader;
#[doc = "Field `CLK_COEX_LP_SEL_XTAL` writer - "]
pub type CLK_COEX_LP_SEL_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_COEX_LP_SEL_XTAL32K` reader - "]
pub type CLK_COEX_LP_SEL_XTAL32K_R = crate::BitReader;
#[doc = "Field `CLK_COEX_LP_SEL_XTAL32K` writer - "]
pub type CLK_COEX_LP_SEL_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_COEX_LP_DIV_NUM` reader - "]
pub type CLK_COEX_LP_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `CLK_COEX_LP_DIV_NUM` writer - "]
pub type CLK_COEX_LP_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_coex_lp_sel_osc_slow(&self) -> CLK_COEX_LP_SEL_OSC_SLOW_R {
        CLK_COEX_LP_SEL_OSC_SLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_coex_lp_sel_osc_fast(&self) -> CLK_COEX_LP_SEL_OSC_FAST_R {
        CLK_COEX_LP_SEL_OSC_FAST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_coex_lp_sel_xtal(&self) -> CLK_COEX_LP_SEL_XTAL_R {
        CLK_COEX_LP_SEL_XTAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_coex_lp_sel_xtal32k(&self) -> CLK_COEX_LP_SEL_XTAL32K_R {
        CLK_COEX_LP_SEL_XTAL32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15"]
    #[inline(always)]
    pub fn clk_coex_lp_div_num(&self) -> CLK_COEX_LP_DIV_NUM_R {
        CLK_COEX_LP_DIV_NUM_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COEX_LP_CLK_CONF")
            .field(
                "clk_coex_lp_sel_osc_slow",
                &format_args!("{}", self.clk_coex_lp_sel_osc_slow().bit()),
            )
            .field(
                "clk_coex_lp_sel_osc_fast",
                &format_args!("{}", self.clk_coex_lp_sel_osc_fast().bit()),
            )
            .field(
                "clk_coex_lp_sel_xtal",
                &format_args!("{}", self.clk_coex_lp_sel_xtal().bit()),
            )
            .field(
                "clk_coex_lp_sel_xtal32k",
                &format_args!("{}", self.clk_coex_lp_sel_xtal32k().bit()),
            )
            .field(
                "clk_coex_lp_div_num",
                &format_args!("{}", self.clk_coex_lp_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COEX_LP_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_sel_osc_slow(
        &mut self,
    ) -> CLK_COEX_LP_SEL_OSC_SLOW_W<COEX_LP_CLK_CONF_SPEC> {
        CLK_COEX_LP_SEL_OSC_SLOW_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_sel_osc_fast(
        &mut self,
    ) -> CLK_COEX_LP_SEL_OSC_FAST_W<COEX_LP_CLK_CONF_SPEC> {
        CLK_COEX_LP_SEL_OSC_FAST_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_sel_xtal(&mut self) -> CLK_COEX_LP_SEL_XTAL_W<COEX_LP_CLK_CONF_SPEC> {
        CLK_COEX_LP_SEL_XTAL_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_sel_xtal32k(&mut self) -> CLK_COEX_LP_SEL_XTAL32K_W<COEX_LP_CLK_CONF_SPEC> {
        CLK_COEX_LP_SEL_XTAL32K_W::new(self, 3)
    }
    #[doc = "Bits 4:15"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_div_num(&mut self) -> CLK_COEX_LP_DIV_NUM_W<COEX_LP_CLK_CONF_SPEC> {
        CLK_COEX_LP_DIV_NUM_W::new(self, 4)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`coex_lp_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coex_lp_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COEX_LP_CLK_CONF_SPEC;
impl crate::RegisterSpec for COEX_LP_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_lp_clk_conf::R`](R) reader structure"]
impl crate::Readable for COEX_LP_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`coex_lp_clk_conf::W`](W) writer structure"]
impl crate::Writable for COEX_LP_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEX_LP_CLK_CONF to value 0"]
impl crate::Resettable for COEX_LP_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
