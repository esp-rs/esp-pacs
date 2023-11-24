#[doc = "Register `ACPU_INT` reader"]
pub type R = crate::R<ACPU_INT_SPEC>;
#[doc = "Field `APPCPU_INT` reader - GPIO0~31 APP CPU interrupt status"]
pub type APPCPU_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 APP CPU interrupt status"]
    #[inline(always)]
    pub fn appcpu_int(&self) -> APPCPU_INT_R {
        APPCPU_INT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACPU_INT")
            .field("appcpu_int", &format_args!("{}", self.appcpu_int().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ACPU_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_int::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACPU_INT_SPEC;
impl crate::RegisterSpec for ACPU_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acpu_int::R`](R) reader structure"]
impl crate::Readable for ACPU_INT_SPEC {}
#[doc = "`reset()` method sets ACPU_INT to value 0"]
impl crate::Resettable for ACPU_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
