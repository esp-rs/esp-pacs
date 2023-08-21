#[doc = "Register `CAP_CH0` reader"]
pub type R = crate::R<CAP_CH0_SPEC>;
#[doc = "Field `CAP0_VALUE` reader - Value of last capture on channel 0"]
pub type CAP0_VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Value of last capture on channel 0"]
    #[inline(always)]
    pub fn cap0_value(&self) -> CAP0_VALUE_R {
        CAP0_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_CH0")
            .field("cap0_value", &format_args!("{}", self.cap0_value().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Value of last capture on channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_CH0_SPEC;
impl crate::RegisterSpec for CAP_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_ch0::R`](R) reader structure"]
impl crate::Readable for CAP_CH0_SPEC {}
#[doc = "`reset()` method sets CAP_CH0 to value 0"]
impl crate::Resettable for CAP_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
