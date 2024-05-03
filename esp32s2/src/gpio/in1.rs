#[doc = "Register `IN1` reader"]
pub type R = crate::R<IN1_SPEC>;
#[doc = "Field `IN_DATA1_NEXT` reader - GPIO32 ~ 53 input value. Each bit represents a pad input value."]
pub type IN_DATA1_NEXT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 input value. Each bit represents a pad input value."]
    #[inline(always)]
    pub fn in_data1_next(&self) -> IN_DATA1_NEXT_R {
        IN_DATA1_NEXT_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN1")
            .field("in_data1_next", &self.in_data1_next().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "GPIO32 ~ 53 input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN1_SPEC;
impl crate::RegisterSpec for IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in1::R`](R) reader structure"]
impl crate::Readable for IN1_SPEC {}
#[doc = "`reset()` method sets IN1 to value 0"]
impl crate::Resettable for IN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
