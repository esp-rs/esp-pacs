#[doc = "Register `HI` reader"]
pub type R = crate::R<HI_SPEC>;
#[doc = "Field `HI` reader - Register to store timer 0 time-base counter current value higher 32 bits."]
pub type HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register to store timer 0 time-base counter current value higher 32 bits."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HI").field("hi", &self.hi()).finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HI_SPEC;
impl crate::RegisterSpec for HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hi::R`](R) reader structure"]
impl crate::Readable for HI_SPEC {}
#[doc = "`reset()` method sets HI to value 0"]
impl crate::Resettable for HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
