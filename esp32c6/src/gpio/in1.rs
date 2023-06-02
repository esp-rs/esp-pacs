#[doc = "Register `IN1` reader"]
pub struct R(crate::R<IN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_NEXT` reader - GPIO input register for GPIO32-34"]
pub type DATA_NEXT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - GPIO input register for GPIO32-34"]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN1")
            .field("data_next", &format_args!("{}", self.data_next().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GPIO input register for GPIO32-34\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in1](index.html) module"]
pub struct IN1_SPEC;
impl crate::RegisterSpec for IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in1::R](R) reader structure"]
impl crate::Readable for IN1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN1 to value 0"]
impl crate::Resettable for IN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
