#[doc = "Register `APB_ADC_ARB_CTRL` reader"]
pub struct R(crate::R<APB_ADC_ARB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_ADC_ARB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_ADC_ARB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_ADC_ARB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_ADC_ARB_CTRL` writer"]
pub struct W(crate::W<APB_ADC_ARB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_ADC_ARB_CTRL_SPEC>;
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
impl From<crate::W<APB_ADC_ARB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_ADC_ARB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_ARB_APB_FORCE` reader - adc2 arbiter force to enableapb controller"]
pub type ADC_ARB_APB_FORCE_R = crate::BitReader;
#[doc = "Field `ADC_ARB_APB_FORCE` writer - adc2 arbiter force to enableapb controller"]
pub type ADC_ARB_APB_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, APB_ADC_ARB_CTRL_SPEC, O>;
#[doc = "Field `ADC_ARB_RTC_FORCE` reader - adc2 arbiter force to enable rtc controller"]
pub type ADC_ARB_RTC_FORCE_R = crate::BitReader;
#[doc = "Field `ADC_ARB_RTC_FORCE` writer - adc2 arbiter force to enable rtc controller"]
pub type ADC_ARB_RTC_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, APB_ADC_ARB_CTRL_SPEC, O>;
#[doc = "Field `ADC_ARB_WIFI_FORCE` reader - adc2 arbiter force to enable wifi controller"]
pub type ADC_ARB_WIFI_FORCE_R = crate::BitReader;
#[doc = "Field `ADC_ARB_WIFI_FORCE` writer - adc2 arbiter force to enable wifi controller"]
pub type ADC_ARB_WIFI_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, APB_ADC_ARB_CTRL_SPEC, O>;
#[doc = "Field `ADC_ARB_GRANT_FORCE` reader - adc2 arbiter force grant"]
pub type ADC_ARB_GRANT_FORCE_R = crate::BitReader;
#[doc = "Field `ADC_ARB_GRANT_FORCE` writer - adc2 arbiter force grant"]
pub type ADC_ARB_GRANT_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, APB_ADC_ARB_CTRL_SPEC, O>;
#[doc = "Field `ADC_ARB_APB_PRIORITY` reader - Set adc2 arbiterapb priority"]
pub type ADC_ARB_APB_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ADC_ARB_APB_PRIORITY` writer - Set adc2 arbiterapb priority"]
pub type ADC_ARB_APB_PRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, APB_ADC_ARB_CTRL_SPEC, 2, O>;
#[doc = "Field `ADC_ARB_RTC_PRIORITY` reader - Set adc2 arbiter rtc priority"]
pub type ADC_ARB_RTC_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ADC_ARB_RTC_PRIORITY` writer - Set adc2 arbiter rtc priority"]
pub type ADC_ARB_RTC_PRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, APB_ADC_ARB_CTRL_SPEC, 2, O>;
#[doc = "Field `ADC_ARB_WIFI_PRIORITY` reader - Set adc2 arbiter wifi priority"]
pub type ADC_ARB_WIFI_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ADC_ARB_WIFI_PRIORITY` writer - Set adc2 arbiter wifi priority"]
pub type ADC_ARB_WIFI_PRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, APB_ADC_ARB_CTRL_SPEC, 2, O>;
#[doc = "Field `ADC_ARB_FIX_PRIORITY` reader - adc2 arbiter uses fixed priority"]
pub type ADC_ARB_FIX_PRIORITY_R = crate::BitReader;
#[doc = "Field `ADC_ARB_FIX_PRIORITY` writer - adc2 arbiter uses fixed priority"]
pub type ADC_ARB_FIX_PRIORITY_W<'a, const O: u8> = crate::BitWriter<'a, APB_ADC_ARB_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 2 - adc2 arbiter force to enableapb controller"]
    #[inline(always)]
    pub fn adc_arb_apb_force(&self) -> ADC_ARB_APB_FORCE_R {
        ADC_ARB_APB_FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - adc2 arbiter force to enable rtc controller"]
    #[inline(always)]
    pub fn adc_arb_rtc_force(&self) -> ADC_ARB_RTC_FORCE_R {
        ADC_ARB_RTC_FORCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - adc2 arbiter force to enable wifi controller"]
    #[inline(always)]
    pub fn adc_arb_wifi_force(&self) -> ADC_ARB_WIFI_FORCE_R {
        ADC_ARB_WIFI_FORCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - adc2 arbiter force grant"]
    #[inline(always)]
    pub fn adc_arb_grant_force(&self) -> ADC_ARB_GRANT_FORCE_R {
        ADC_ARB_GRANT_FORCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set adc2 arbiterapb priority"]
    #[inline(always)]
    pub fn adc_arb_apb_priority(&self) -> ADC_ARB_APB_PRIORITY_R {
        ADC_ARB_APB_PRIORITY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Set adc2 arbiter rtc priority"]
    #[inline(always)]
    pub fn adc_arb_rtc_priority(&self) -> ADC_ARB_RTC_PRIORITY_R {
        ADC_ARB_RTC_PRIORITY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Set adc2 arbiter wifi priority"]
    #[inline(always)]
    pub fn adc_arb_wifi_priority(&self) -> ADC_ARB_WIFI_PRIORITY_R {
        ADC_ARB_WIFI_PRIORITY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - adc2 arbiter uses fixed priority"]
    #[inline(always)]
    pub fn adc_arb_fix_priority(&self) -> ADC_ARB_FIX_PRIORITY_R {
        ADC_ARB_FIX_PRIORITY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_ADC_ARB_CTRL")
            .field(
                "adc_arb_apb_force",
                &format_args!("{}", self.adc_arb_apb_force().bit()),
            )
            .field(
                "adc_arb_rtc_force",
                &format_args!("{}", self.adc_arb_rtc_force().bit()),
            )
            .field(
                "adc_arb_wifi_force",
                &format_args!("{}", self.adc_arb_wifi_force().bit()),
            )
            .field(
                "adc_arb_grant_force",
                &format_args!("{}", self.adc_arb_grant_force().bit()),
            )
            .field(
                "adc_arb_apb_priority",
                &format_args!("{}", self.adc_arb_apb_priority().bits()),
            )
            .field(
                "adc_arb_rtc_priority",
                &format_args!("{}", self.adc_arb_rtc_priority().bits()),
            )
            .field(
                "adc_arb_wifi_priority",
                &format_args!("{}", self.adc_arb_wifi_priority().bits()),
            )
            .field(
                "adc_arb_fix_priority",
                &format_args!("{}", self.adc_arb_fix_priority().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_ADC_ARB_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - adc2 arbiter force to enableapb controller"]
    #[inline(always)]
    #[must_use]
    pub fn adc_arb_apb_force(&mut self) -> ADC_ARB_APB_FORCE_W<2> {
        ADC_ARB_APB_FORCE_W::new(self)
    }
    #[doc = "Bit 3 - adc2 arbiter force to enable rtc controller"]
    #[inline(always)]
    #[must_use]
    pub fn adc_arb_rtc_force(&mut self) -> ADC_ARB_RTC_FORCE_W<3> {
        ADC_ARB_RTC_FORCE_W::new(self)
    }
    #[doc = "Bit 4 - adc2 arbiter force to enable wifi controller"]
    #[inline(always)]
    #[must_use]
    pub fn adc_arb_wifi_force(&mut self) -> ADC_ARB_WIFI_FORCE_W<4> {
        ADC_ARB_WIFI_FORCE_W::new(self)
    }
    #[doc = "Bit 5 - adc2 arbiter force grant"]
    #[inline(always)]
    #[must_use]
    pub fn adc_arb_grant_force(&mut self) -> ADC_ARB_GRANT_FORCE_W<5> {
        ADC_ARB_GRANT_FORCE_W::new(self)
    }
    #[doc = "Bits 6:7 - Set adc2 arbiterapb priority"]
    #[inline(always)]
    #[must_use]
    pub fn adc_arb_apb_priority(&mut self) -> ADC_ARB_APB_PRIORITY_W<6> {
        ADC_ARB_APB_PRIORITY_W::new(self)
    }
    #[doc = "Bits 8:9 - Set adc2 arbiter rtc priority"]
    #[inline(always)]
    #[must_use]
    pub fn adc_arb_rtc_priority(&mut self) -> ADC_ARB_RTC_PRIORITY_W<8> {
        ADC_ARB_RTC_PRIORITY_W::new(self)
    }
    #[doc = "Bits 10:11 - Set adc2 arbiter wifi priority"]
    #[inline(always)]
    #[must_use]
    pub fn adc_arb_wifi_priority(&mut self) -> ADC_ARB_WIFI_PRIORITY_W<10> {
        ADC_ARB_WIFI_PRIORITY_W::new(self)
    }
    #[doc = "Bit 12 - adc2 arbiter uses fixed priority"]
    #[inline(always)]
    #[must_use]
    pub fn adc_arb_fix_priority(&mut self) -> ADC_ARB_FIX_PRIORITY_W<12> {
        ADC_ARB_FIX_PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_adc_arb_ctrl](index.html) module"]
pub struct APB_ADC_ARB_CTRL_SPEC;
impl crate::RegisterSpec for APB_ADC_ARB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_adc_arb_ctrl::R](R) reader structure"]
impl crate::Readable for APB_ADC_ARB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_adc_arb_ctrl::W](W) writer structure"]
impl crate::Writable for APB_ADC_ARB_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_ADC_ARB_CTRL to value 0x0900"]
impl crate::Resettable for APB_ADC_ARB_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0900;
}
