#[doc = "Register `STATUS_NEXT` reader"]
pub struct R(crate::R<STATUS_NEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_NEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_NEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_NEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS_INTERRUPT_NEXT` reader - GPIO interrupt source register for GPIO0-31"]
pub type STATUS_INTERRUPT_NEXT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt source register for GPIO0-31"]
    #[inline(always)]
    pub fn status_interrupt_next(&self) -> STATUS_INTERRUPT_NEXT_R {
        STATUS_INTERRUPT_NEXT_R::new(self.bits)
    }
}
#[doc = "GPIO interrupt source register for GPIO0-31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_next](index.html) module"]
pub struct STATUS_NEXT_SPEC;
impl crate::RegisterSpec for STATUS_NEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status_next::R](R) reader structure"]
impl crate::Readable for STATUS_NEXT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS_NEXT to value 0"]
impl crate::Resettable for STATUS_NEXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
