#[doc = "Register `MODEM_LP_TIMER_CONF` reader"]
pub struct R(crate::R<MODEM_LP_TIMER_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODEM_LP_TIMER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODEM_LP_TIMER_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODEM_LP_TIMER_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODEM_LP_TIMER_CONF` writer"]
pub struct W(crate::W<MODEM_LP_TIMER_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODEM_LP_TIMER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MODEM_LP_TIMER_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODEM_LP_TIMER_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_TIMER_SEL_RTC_SLOW` reader - ."]
pub type LP_TIMER_SEL_RTC_SLOW_R = crate::BitReader;
#[doc = "Field `LP_TIMER_SEL_RTC_SLOW` writer - ."]
pub type LP_TIMER_SEL_RTC_SLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, MODEM_LP_TIMER_CONF_SPEC, O>;
#[doc = "Field `LP_TIMER_SEL_8M` reader - ."]
pub type LP_TIMER_SEL_8M_R = crate::BitReader;
#[doc = "Field `LP_TIMER_SEL_8M` writer - ."]
pub type LP_TIMER_SEL_8M_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_LP_TIMER_CONF_SPEC, O>;
#[doc = "Field `LP_TIMER_SEL_XTAL` reader - ."]
pub type LP_TIMER_SEL_XTAL_R = crate::BitReader;
#[doc = "Field `LP_TIMER_SEL_XTAL` writer - ."]
pub type LP_TIMER_SEL_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_LP_TIMER_CONF_SPEC, O>;
#[doc = "Field `LP_TIMER_SEL_XTAL32K` reader - ."]
pub type LP_TIMER_SEL_XTAL32K_R = crate::BitReader;
#[doc = "Field `LP_TIMER_SEL_XTAL32K` writer - ."]
pub type LP_TIMER_SEL_XTAL32K_W<'a, const O: u8> =
    crate::BitWriter<'a, MODEM_LP_TIMER_CONF_SPEC, O>;
#[doc = "Field `LP_TIMER_CLK_DIV_NUM` reader - ."]
pub type LP_TIMER_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_TIMER_CLK_DIV_NUM` writer - ."]
pub type LP_TIMER_CLK_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, MODEM_LP_TIMER_CONF_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_rtc_slow(&self) -> LP_TIMER_SEL_RTC_SLOW_R {
        LP_TIMER_SEL_RTC_SLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_8m(&self) -> LP_TIMER_SEL_8M_R {
        LP_TIMER_SEL_8M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_xtal(&self) -> LP_TIMER_SEL_XTAL_R {
        LP_TIMER_SEL_XTAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_xtal32k(&self) -> LP_TIMER_SEL_XTAL32K_R {
        LP_TIMER_SEL_XTAL32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - ."]
    #[inline(always)]
    pub fn lp_timer_clk_div_num(&self) -> LP_TIMER_CLK_DIV_NUM_R {
        LP_TIMER_CLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_LP_TIMER_CONF")
            .field(
                "lp_timer_sel_rtc_slow",
                &format_args!("{}", self.lp_timer_sel_rtc_slow().bit()),
            )
            .field(
                "lp_timer_sel_8m",
                &format_args!("{}", self.lp_timer_sel_8m().bit()),
            )
            .field(
                "lp_timer_sel_xtal",
                &format_args!("{}", self.lp_timer_sel_xtal().bit()),
            )
            .field(
                "lp_timer_sel_xtal32k",
                &format_args!("{}", self.lp_timer_sel_xtal32k().bit()),
            )
            .field(
                "lp_timer_clk_div_num",
                &format_args!("{}", self.lp_timer_clk_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEM_LP_TIMER_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    #[must_use]
    pub fn lp_timer_sel_rtc_slow(&mut self) -> LP_TIMER_SEL_RTC_SLOW_W<0> {
        LP_TIMER_SEL_RTC_SLOW_W::new(self)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    #[must_use]
    pub fn lp_timer_sel_8m(&mut self) -> LP_TIMER_SEL_8M_W<1> {
        LP_TIMER_SEL_8M_W::new(self)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    #[must_use]
    pub fn lp_timer_sel_xtal(&mut self) -> LP_TIMER_SEL_XTAL_W<2> {
        LP_TIMER_SEL_XTAL_W::new(self)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    #[must_use]
    pub fn lp_timer_sel_xtal32k(&mut self) -> LP_TIMER_SEL_XTAL32K_W<3> {
        LP_TIMER_SEL_XTAL32K_W::new(self)
    }
    #[doc = "Bits 4:11 - ."]
    #[inline(always)]
    #[must_use]
    pub fn lp_timer_clk_div_num(&mut self) -> LP_TIMER_CLK_DIV_NUM_W<4> {
        LP_TIMER_CLK_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_lp_timer_conf](index.html) module"]
pub struct MODEM_LP_TIMER_CONF_SPEC;
impl crate::RegisterSpec for MODEM_LP_TIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modem_lp_timer_conf::R](R) reader structure"]
impl crate::Readable for MODEM_LP_TIMER_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modem_lp_timer_conf::W](W) writer structure"]
impl crate::Writable for MODEM_LP_TIMER_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEM_LP_TIMER_CONF to value 0"]
impl crate::Resettable for MODEM_LP_TIMER_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
