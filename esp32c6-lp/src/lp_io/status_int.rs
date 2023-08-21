#[doc = "Register `STATUS_INT` reader"]
pub type R = crate::R<STATUS_INT_SPEC>;
#[doc = "Field `STATUS_INTERRUPT_NEXT` reader - need des"]
pub type STATUS_INTERRUPT_NEXT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - need des"]
    #[inline(always)]
    pub fn status_interrupt_next(&self) -> STATUS_INTERRUPT_NEXT_R {
        STATUS_INTERRUPT_NEXT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_INT")
            .field(
                "status_interrupt_next",
                &format_args!("{}", self.status_interrupt_next().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_int::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_INT_SPEC;
impl crate::RegisterSpec for STATUS_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_int::R`](R) reader structure"]
impl crate::Readable for STATUS_INT_SPEC {}
#[doc = "`reset()` method sets STATUS_INT to value 0"]
impl crate::Resettable for STATUS_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
