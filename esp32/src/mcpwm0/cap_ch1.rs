#[doc = "Register `CAP_CH1` reader"]
pub type R = crate::R<CAP_CH1_SPEC>;
#[doc = "Field `CAP1_VALUE` reader - "]
pub type CAP1_VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cap1_value(&self) -> CAP1_VALUE_R {
        CAP1_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_CH1")
            .field("cap1_value", &format_args!("{}", self.cap1_value().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_CH1_SPEC;
impl crate::RegisterSpec for CAP_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_ch1::R`](R) reader structure"]
impl crate::Readable for CAP_CH1_SPEC {}
#[doc = "`reset()` method sets CAP_CH1 to value 0"]
impl crate::Resettable for CAP_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
