#[doc = "Register `HCDMAB7` reader"]
pub type R = crate::R<HCDMAB7_SPEC>;
#[doc = "Field `H_HCDMAB7` reader - "]
pub type H_HCDMAB7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab7(&self) -> H_HCDMAB7_R {
        H_HCDMAB7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB7")
            .field("h_hcdmab7", &format_args!("{}", self.h_hcdmab7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMAB7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMAB7_SPEC;
impl crate::RegisterSpec for HCDMAB7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdmab7::R`](R) reader structure"]
impl crate::Readable for HCDMAB7_SPEC {}
#[doc = "`reset()` method sets HCDMAB7 to value 0"]
impl crate::Resettable for HCDMAB7_SPEC {
    const RESET_VALUE: u32 = 0;
}
