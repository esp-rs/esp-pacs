#[doc = "Register `HP_SLEEP_LP_CK_POWER` reader"]
pub struct R(crate::R<HP_SLEEP_LP_CK_POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_SLEEP_LP_CK_POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_SLEEP_LP_CK_POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_SLEEP_LP_CK_POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_SLEEP_LP_CK_POWER` writer"]
pub struct W(crate::W<HP_SLEEP_LP_CK_POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_SLEEP_LP_CK_POWER_SPEC>;
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
impl From<crate::W<HP_SLEEP_LP_CK_POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_SLEEP_LP_CK_POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_SLEEP_XPD_XTAL32K` reader - need_des"]
pub type HP_SLEEP_XPD_XTAL32K_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_XPD_XTAL32K` writer - need_des"]
pub type HP_SLEEP_XPD_XTAL32K_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_LP_CK_POWER_SPEC, O>;
#[doc = "Field `HP_SLEEP_XPD_RC32K` reader - need_des"]
pub type HP_SLEEP_XPD_RC32K_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_XPD_RC32K` writer - need_des"]
pub type HP_SLEEP_XPD_RC32K_W<'a, const O: u8> = crate::BitWriter<'a, HP_SLEEP_LP_CK_POWER_SPEC, O>;
#[doc = "Field `HP_SLEEP_XPD_FOSC_CLK` reader - need_des"]
pub type HP_SLEEP_XPD_FOSC_CLK_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_XPD_FOSC_CLK` writer - need_des"]
pub type HP_SLEEP_XPD_FOSC_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_LP_CK_POWER_SPEC, O>;
#[doc = "Field `HP_SLEEP_PD_OSC_CLK` reader - need_des"]
pub type HP_SLEEP_PD_OSC_CLK_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_OSC_CLK` writer - need_des"]
pub type HP_SLEEP_PD_OSC_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_LP_CK_POWER_SPEC, O>;
impl R {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_xtal32k(&self) -> HP_SLEEP_XPD_XTAL32K_R {
        HP_SLEEP_XPD_XTAL32K_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_rc32k(&self) -> HP_SLEEP_XPD_RC32K_R {
        HP_SLEEP_XPD_RC32K_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_fosc_clk(&self) -> HP_SLEEP_XPD_FOSC_CLK_R {
        HP_SLEEP_XPD_FOSC_CLK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_osc_clk(&self) -> HP_SLEEP_PD_OSC_CLK_R {
        HP_SLEEP_PD_OSC_CLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_LP_CK_POWER")
            .field(
                "hp_sleep_xpd_xtal32k",
                &format_args!("{}", self.hp_sleep_xpd_xtal32k().bit()),
            )
            .field(
                "hp_sleep_xpd_rc32k",
                &format_args!("{}", self.hp_sleep_xpd_rc32k().bit()),
            )
            .field(
                "hp_sleep_xpd_fosc_clk",
                &format_args!("{}", self.hp_sleep_xpd_fosc_clk().bit()),
            )
            .field(
                "hp_sleep_pd_osc_clk",
                &format_args!("{}", self.hp_sleep_pd_osc_clk().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_SLEEP_LP_CK_POWER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_xpd_xtal32k(&mut self) -> HP_SLEEP_XPD_XTAL32K_W<28> {
        HP_SLEEP_XPD_XTAL32K_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_xpd_rc32k(&mut self) -> HP_SLEEP_XPD_RC32K_W<29> {
        HP_SLEEP_XPD_RC32K_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_xpd_fosc_clk(&mut self) -> HP_SLEEP_XPD_FOSC_CLK_W<30> {
        HP_SLEEP_XPD_FOSC_CLK_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_osc_clk(&mut self) -> HP_SLEEP_PD_OSC_CLK_W<31> {
        HP_SLEEP_PD_OSC_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_sleep_lp_ck_power](index.html) module"]
pub struct HP_SLEEP_LP_CK_POWER_SPEC;
impl crate::RegisterSpec for HP_SLEEP_LP_CK_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_sleep_lp_ck_power::R](R) reader structure"]
impl crate::Readable for HP_SLEEP_LP_CK_POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_sleep_lp_ck_power::W](W) writer structure"]
impl crate::Writable for HP_SLEEP_LP_CK_POWER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_SLEEP_LP_CK_POWER to value 0x4000_0000"]
impl crate::Resettable for HP_SLEEP_LP_CK_POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
