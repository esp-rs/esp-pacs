#[doc = "Register `CH3_SSTAT0` reader"]
pub type R = crate::R<CH3_SSTAT0_SPEC>;
#[doc = "Field `CH3_SSTAT` reader - NA"]
pub type CH3_SSTAT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch3_sstat(&self) -> CH3_SSTAT_R {
        CH3_SSTAT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH3_SSTAT0")
            .field("ch3_sstat", &format_args!("{}", self.ch3_sstat().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH3_SSTAT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_sstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3_SSTAT0_SPEC;
impl crate::RegisterSpec for CH3_SSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_sstat0::R`](R) reader structure"]
impl crate::Readable for CH3_SSTAT0_SPEC {}
#[doc = "`reset()` method sets CH3_SSTAT0 to value 0"]
impl crate::Resettable for CH3_SSTAT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
