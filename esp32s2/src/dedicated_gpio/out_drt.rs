#[doc = "Register `OUT_DRT` writer"]
pub type W = crate::W<OUT_DRT_SPEC>;
#[doc = "Field `VALUE` writer - This register is used to configure directive output value of 8-channel dedicated GPIO."]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_DRT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure directive output value of 8-channel dedicated GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<OUT_DRT_SPEC> {
        VALUE_W::new(self, 0)
    }
}
#[doc = "Dedicated GPIO directive output register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_drt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_DRT_SPEC;
impl crate::RegisterSpec for OUT_DRT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_drt::W`](W) writer structure"]
impl crate::Writable for OUT_DRT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_DRT to value 0"]
impl crate::Resettable for OUT_DRT_SPEC {
    const RESET_VALUE: u32 = 0;
}
