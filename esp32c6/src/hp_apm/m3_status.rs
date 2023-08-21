#[doc = "Register `M3_STATUS` reader"]
pub type R = crate::R<M3_STATUS_SPEC>;
#[doc = "Field `M3_EXCEPTION_STATUS` reader - Exception status"]
pub type M3_EXCEPTION_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Exception status"]
    #[inline(always)]
    pub fn m3_exception_status(&self) -> M3_EXCEPTION_STATUS_R {
        M3_EXCEPTION_STATUS_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3_STATUS")
            .field(
                "m3_exception_status",
                &format_args!("{}", self.m3_exception_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M3_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "M3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3_STATUS_SPEC;
impl crate::RegisterSpec for M3_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3_status::R`](R) reader structure"]
impl crate::Readable for M3_STATUS_SPEC {}
#[doc = "`reset()` method sets M3_STATUS to value 0"]
impl crate::Resettable for M3_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
