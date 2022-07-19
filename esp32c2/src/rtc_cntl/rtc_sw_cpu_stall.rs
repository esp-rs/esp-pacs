#[doc = "Register `RTC_SW_CPU_STALL` reader"]
pub struct R(crate::R<RTC_SW_CPU_STALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SW_CPU_STALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SW_CPU_STALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SW_CPU_STALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SW_CPU_STALL` writer"]
pub struct W(crate::W<RTC_SW_CPU_STALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SW_CPU_STALL_SPEC>;
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
impl From<crate::W<RTC_SW_CPU_STALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SW_CPU_STALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_STALL_PROCPU_C1` reader - Need add desc"]
pub type SW_STALL_PROCPU_C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_STALL_PROCPU_C1` writer - Need add desc"]
pub type SW_STALL_PROCPU_C1_W<'a> =
    crate::FieldWriter<'a, u32, RTC_SW_CPU_STALL_SPEC, u8, u8, 6, 26>;
impl R {
    #[doc = "Bits 26:31 - Need add desc"]
    #[inline(always)]
    pub fn sw_stall_procpu_c1(&self) -> SW_STALL_PROCPU_C1_R {
        SW_STALL_PROCPU_C1_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - Need add desc"]
    #[inline(always)]
    pub fn sw_stall_procpu_c1(&mut self) -> SW_STALL_PROCPU_C1_W {
        SW_STALL_PROCPU_C1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sw_cpu_stall](index.html) module"]
pub struct RTC_SW_CPU_STALL_SPEC;
impl crate::RegisterSpec for RTC_SW_CPU_STALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_sw_cpu_stall::R](R) reader structure"]
impl crate::Readable for RTC_SW_CPU_STALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_sw_cpu_stall::W](W) writer structure"]
impl crate::Writable for RTC_SW_CPU_STALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SW_CPU_STALL to value 0"]
impl crate::Resettable for RTC_SW_CPU_STALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
