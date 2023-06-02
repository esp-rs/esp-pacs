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
#[doc = "Field `DEBUG_SEL0` reader - "]
pub type DEBUG_SEL0_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL0` writer - "]
pub type DEBUG_SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `DEBUG_SEL1` reader - "]
pub type DEBUG_SEL1_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL1` writer - "]
pub type DEBUG_SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `DEBUG_SEL2` reader - "]
pub type DEBUG_SEL2_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL2` writer - "]
pub type DEBUG_SEL2_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `DEBUG_SEL3` reader - "]
pub type DEBUG_SEL3_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL3` writer - "]
pub type DEBUG_SEL3_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `DEBUG_SEL4` reader - "]
pub type DEBUG_SEL4_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL4` writer - "]
pub type DEBUG_SEL4_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_DEBUG_SEL_SPEC, 5, O>;
#[doc = "Field `DEBUG_12M_NO_GATING` reader - "]
pub type DEBUG_12M_NO_GATING_R = crate::BitReader;
#[doc = "Field `DEBUG_12M_NO_GATING` writer - "]
pub type DEBUG_12M_NO_GATING_W<'a, const O: u8> = crate::BitWriter<'a, RTC_DEBUG_SEL_SPEC, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn debug_sel0(&self) -> DEBUG_SEL0_R {
        DEBUG_SEL0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn debug_sel1(&self) -> DEBUG_SEL1_R {
        DEBUG_SEL1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn debug_sel2(&self) -> DEBUG_SEL2_R {
        DEBUG_SEL2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn debug_sel3(&self) -> DEBUG_SEL3_R {
        DEBUG_SEL3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn debug_sel4(&self) -> DEBUG_SEL4_R {
        DEBUG_SEL4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn debug_12m_no_gating(&self) -> DEBUG_12M_NO_GATING_R {
        DEBUG_12M_NO_GATING_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_DEBUG_SEL")
            .field("debug_sel0", &format_args!("{}", self.debug_sel0().bits()))
            .field("debug_sel1", &format_args!("{}", self.debug_sel1().bits()))
            .field("debug_sel2", &format_args!("{}", self.debug_sel2().bits()))
            .field("debug_sel3", &format_args!("{}", self.debug_sel3().bits()))
            .field("debug_sel4", &format_args!("{}", self.debug_sel4().bits()))
            .field(
                "debug_12m_no_gating",
                &format_args!("{}", self.debug_12m_no_gating().bit()),
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
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel0(&mut self) -> DEBUG_SEL0_W<0> {
        DEBUG_SEL0_W::new(self)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel1(&mut self) -> DEBUG_SEL1_W<5> {
        DEBUG_SEL1_W::new(self)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel2(&mut self) -> DEBUG_SEL2_W<10> {
        DEBUG_SEL2_W::new(self)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel3(&mut self) -> DEBUG_SEL3_W<15> {
        DEBUG_SEL3_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel4(&mut self) -> DEBUG_SEL4_W<20> {
        DEBUG_SEL4_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn debug_12m_no_gating(&mut self) -> DEBUG_12M_NO_GATING_W<25> {
        DEBUG_12M_NO_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_debug_sel](index.html) module"]
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
