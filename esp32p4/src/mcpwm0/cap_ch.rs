#[doc = "Register `CAP_CH%s` reader"]
pub type R = crate::R<CAP_CH_SPEC>;
#[doc = "Field `CAP_VALUE` reader - Represents value of last capture on CAP%s"]
pub type CAP_VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents value of last capture on CAP%s"]
    #[inline(always)]
    pub fn cap_value(&self) -> CAP_VALUE_R {
        CAP_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_CH")
            .field("cap_value", &format_args!("{}", self.cap_value().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "CAP%s capture value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_CH_SPEC;
impl crate::RegisterSpec for CAP_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_ch::R`](R) reader structure"]
impl crate::Readable for CAP_CH_SPEC {}
#[doc = "`reset()` method sets CAP_CH%s to value 0"]
impl crate::Resettable for CAP_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
