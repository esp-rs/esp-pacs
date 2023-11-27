#[doc = "Register `CH4_SSTAT0` reader"]
pub type R = crate::R<CH4_SSTAT0_SPEC>;
#[doc = "Field `CH4_SSTAT` reader - NA"]
pub type CH4_SSTAT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch4_sstat(&self) -> CH4_SSTAT_R {
        CH4_SSTAT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4_SSTAT0")
            .field("ch4_sstat", &format_args!("{}", self.ch4_sstat().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_SSTAT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_sstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_SSTAT0_SPEC;
impl crate::RegisterSpec for CH4_SSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_sstat0::R`](R) reader structure"]
impl crate::Readable for CH4_SSTAT0_SPEC {}
#[doc = "`reset()` method sets CH4_SSTAT0 to value 0"]
impl crate::Resettable for CH4_SSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
