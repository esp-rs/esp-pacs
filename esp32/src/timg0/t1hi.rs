#[doc = "Register `T1HI` reader"]
pub type R = crate::R<T1HI_SPEC>;
#[doc = "Field `HI` reader - Register to store timer 1 time-base counter current value higher 32 bits."]
pub type HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register to store timer 1 time-base counter current value higher 32 bits."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1HI")
            .field("hi", &format_args!("{}", self.hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T1HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1HI_SPEC;
impl crate::RegisterSpec for T1HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1hi::R`](R) reader structure"]
impl crate::Readable for T1HI_SPEC {}
#[doc = "`reset()` method sets T1HI to value 0"]
impl crate::Resettable for T1HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
