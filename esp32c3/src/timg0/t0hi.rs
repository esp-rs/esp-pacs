#[doc = "Register `T0HI` reader"]
pub type R = crate::R<T0HI_SPEC>;
#[doc = "Field `HI` reader - t0_hi"]
pub type HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - t0_hi"]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0HI")
            .field("hi", &format_args!("{}", self.hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TIMG_T0HI_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0HI_SPEC;
impl crate::RegisterSpec for T0HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0hi::R`](R) reader structure"]
impl crate::Readable for T0HI_SPEC {}
#[doc = "`reset()` method sets T0HI to value 0"]
impl crate::Resettable for T0HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
