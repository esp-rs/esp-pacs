#[doc = "Register `LC_STATE1` reader"]
pub type R = crate::R<LC_STATE1_SPEC>;
#[doc = "Field `LC_STATE1` reader - "]
pub type LC_STATE1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lc_state1(&self) -> LC_STATE1_R {
        LC_STATE1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_STATE1")
            .field("lc_state1", &format_args!("{}", self.lc_state1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LC_STATE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_state1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_STATE1_SPEC;
impl crate::RegisterSpec for LC_STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_state1::R`](R) reader structure"]
impl crate::Readable for LC_STATE1_SPEC {}
#[doc = "`reset()` method sets LC_STATE1 to value 0"]
impl crate::Resettable for LC_STATE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
