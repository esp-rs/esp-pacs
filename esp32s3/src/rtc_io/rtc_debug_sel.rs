#[doc = "Register `RTC_DEBUG_SEL` reader"]
pub struct R(crate::R<RTC_DEBUG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_DEBUG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_DEBUG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_DEBUG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_DEBUG_SEL` writer"]
pub struct W(crate::W<RTC_DEBUG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_DEBUG_SEL_SPEC>;
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
impl From<crate::W<RTC_DEBUG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_DEBUG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_DEBUG_SEL0` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL0_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL0` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL1` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL1_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL1` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL2` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL2_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL2` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL2_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL3` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL3_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL3` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL3_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL4` reader - configure rtc debug"]
pub type RTC_DEBUG_SEL4_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL4` writer - configure rtc debug"]
pub type RTC_DEBUG_SEL4_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` reader - configure rtc debug"]
pub type RTC_DEBUG_12M_NO_GATING_R = crate::BitReader;
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` writer - configure rtc debug"]
pub type RTC_DEBUG_12M_NO_GATING_W<'a, const O: u8> = crate::BitWriter<'a, RTC_DEBUG_SEL_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel0(&self) -> RTC_DEBUG_SEL0_R {
        RTC_DEBUG_SEL0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel1(&self) -> RTC_DEBUG_SEL1_R {
        RTC_DEBUG_SEL1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel2(&self) -> RTC_DEBUG_SEL2_R {
        RTC_DEBUG_SEL2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel3(&self) -> RTC_DEBUG_SEL3_R {
        RTC_DEBUG_SEL3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_sel4(&self) -> RTC_DEBUG_SEL4_R {
        RTC_DEBUG_SEL4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - configure rtc debug"]
    #[inline(always)]
    pub fn rtc_debug_12m_no_gating(&self) -> RTC_DEBUG_12M_NO_GATING_R {
        RTC_DEBUG_12M_NO_GATING_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_DEBUG_SEL")
            .field(
                "rtc_debug_sel0",
                &format_args!("{}", self.rtc_debug_sel0().bits()),
            )
            .field(
                "rtc_debug_sel1",
                &format_args!("{}", self.rtc_debug_sel1().bits()),
            )
            .field(
                "rtc_debug_sel2",
                &format_args!("{}", self.rtc_debug_sel2().bits()),
            )
            .field(
                "rtc_debug_sel3",
                &format_args!("{}", self.rtc_debug_sel3().bits()),
            )
            .field(
                "rtc_debug_sel4",
                &format_args!("{}", self.rtc_debug_sel4().bits()),
            )
            .field(
                "rtc_debug_12m_no_gating",
                &format_args!("{}", self.rtc_debug_12m_no_gating().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_DEBUG_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel0(&mut self) -> RTC_DEBUG_SEL0_W<0> {
        RTC_DEBUG_SEL0_W::new(self)
    }
    #[doc = "Bits 5:9 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel1(&mut self) -> RTC_DEBUG_SEL1_W<5> {
        RTC_DEBUG_SEL1_W::new(self)
    }
    #[doc = "Bits 10:14 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel2(&mut self) -> RTC_DEBUG_SEL2_W<10> {
        RTC_DEBUG_SEL2_W::new(self)
    }
    #[doc = "Bits 15:19 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel3(&mut self) -> RTC_DEBUG_SEL3_W<15> {
        RTC_DEBUG_SEL3_W::new(self)
    }
    #[doc = "Bits 20:24 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel4(&mut self) -> RTC_DEBUG_SEL4_W<20> {
        RTC_DEBUG_SEL4_W::new(self)
    }
    #[doc = "Bit 25 - configure rtc debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_12m_no_gating(&mut self) -> RTC_DEBUG_12M_NO_GATING_W<25> {
        RTC_DEBUG_12M_NO_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure rtc debug\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_debug_sel](index.html) module"]
pub struct RTC_DEBUG_SEL_SPEC;
impl crate::RegisterSpec for RTC_DEBUG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_debug_sel::R](R) reader structure"]
impl crate::Readable for RTC_DEBUG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_debug_sel::W](W) writer structure"]
impl crate::Writable for RTC_DEBUG_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_DEBUG_SEL to value 0"]
impl crate::Resettable for RTC_DEBUG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
