#[doc = "Register `HCDMAB4` reader"]
pub type R = crate::R<HCDMAB4_SPEC>;
#[doc = "Field `H_HCDMAB4` reader - "]
pub type H_HCDMAB4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab4(&self) -> H_HCDMAB4_R {
        H_HCDMAB4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB4")
            .field("h_hcdmab4", &format_args!("{}", self.h_hcdmab4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMAB4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMAB4_SPEC;
impl crate::RegisterSpec for HCDMAB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdmab4::R`](R) reader structure"]
impl crate::Readable for HCDMAB4_SPEC {}
#[doc = "`reset()` method sets HCDMAB4 to value 0"]
impl crate::Resettable for HCDMAB4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
