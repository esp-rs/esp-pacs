#[doc = "Register `INT_ENA_RTC` reader"]
pub struct R(crate::R<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA_RTC` writer"]
pub struct W(crate::W<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_SPEC>;
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
impl From<crate::W<INT_ENA_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUPER_WDT_INT_ENA` reader - need_des"]
pub type SUPER_WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SUPER_WDT_INT_ENA` writer - need_des"]
pub type SUPER_WDT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `WDT_INT_ENA` reader - need_des"]
pub type WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `WDT_INT_ENA` writer - need_des"]
pub type WDT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn super_wdt_int_ena(&self) -> SUPER_WDT_INT_ENA_R {
        SUPER_WDT_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_RTC")
            .field(
                "super_wdt_int_ena",
                &format_args!("{}", self.super_wdt_int_ena().bit()),
            )
            .field("wdt_int_ena", &format_args!("{}", self.wdt_int_ena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn super_wdt_int_ena(&mut self) -> SUPER_WDT_INT_ENA_W<30> {
        SUPER_WDT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W<31> {
        WDT_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc](index.html) module"]
pub struct INT_ENA_RTC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena_rtc::R](R) reader structure"]
impl crate::Readable for INT_ENA_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA_RTC to value 0"]
impl crate::Resettable for INT_ENA_RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
