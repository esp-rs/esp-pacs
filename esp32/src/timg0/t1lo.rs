#[doc = "Register `T1LO` reader"]
pub struct R(crate::R<T1LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LO` reader - Register to store timer 1 time-base counter current value lower 32 bits."]
pub type LO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register to store timer 1 time-base counter current value lower 32 bits."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1lo](index.html) module"]
pub struct T1LO_SPEC;
impl crate::RegisterSpec for T1LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1lo::R](R) reader structure"]
impl crate::Readable for T1LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T1LO to value 0"]
impl crate::Resettable for T1LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
