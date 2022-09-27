#[doc = "Register `STATUS_NEXT1` reader"]
pub struct R(crate::R<STATUS_NEXT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_NEXT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_NEXT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_NEXT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS1_INTERRUPT_NEXT` reader - Interrupt source signal of GPIO32 ~ 53."]
pub type STATUS1_INTERRUPT_NEXT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:21 - Interrupt source signal of GPIO32 ~ 53."]
    #[inline(always)]
    pub fn status1_interrupt_next(&self) -> STATUS1_INTERRUPT_NEXT_R {
        STATUS1_INTERRUPT_NEXT_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
#[doc = "GPIO32 ~ 53 interrupt source register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_next1](index.html) module"]
pub struct STATUS_NEXT1_SPEC;
impl crate::RegisterSpec for STATUS_NEXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status_next1::R](R) reader structure"]
impl crate::Readable for STATUS_NEXT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS_NEXT1 to value 0"]
impl crate::Resettable for STATUS_NEXT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
