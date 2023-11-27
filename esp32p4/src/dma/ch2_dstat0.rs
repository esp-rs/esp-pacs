#[doc = "Register `CH2_DSTAT0` reader"]
pub type R = crate::R<CH2_DSTAT0_SPEC>;
#[doc = "Field `CH2_DSTAT` reader - NA"]
pub type CH2_DSTAT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch2_dstat(&self) -> CH2_DSTAT_R {
        CH2_DSTAT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_DSTAT0")
            .field("ch2_dstat", &format_args!("{}", self.ch2_dstat().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2_DSTAT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_DSTAT0_SPEC;
impl crate::RegisterSpec for CH2_DSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_dstat0::R`](R) reader structure"]
impl crate::Readable for CH2_DSTAT0_SPEC {}
#[doc = "`reset()` method sets CH2_DSTAT0 to value 0"]
impl crate::Resettable for CH2_DSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
