#[doc = "Register `TIME1` reader"]
pub struct R(crate::R<TIME1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIME_HI` reader - RTC timer high 16 bits"]
pub type TIME_HI_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn time_hi(&self) -> TIME_HI_R {
        TIME_HI_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time1](index.html) module"]
pub struct TIME1_SPEC;
impl crate::RegisterSpec for TIME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time1::R](R) reader structure"]
impl crate::Readable for TIME1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIME1 to value 0"]
impl crate::Resettable for TIME1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
