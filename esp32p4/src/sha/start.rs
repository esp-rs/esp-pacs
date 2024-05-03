#[doc = "Register `START` reader"]
pub type R = crate::R<START_SPEC>;
#[doc = "Field `START` reader - Reserved."]
pub type START_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 1:31 - Reserved."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("START")
            .field("start", &self.start().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Typical SHA configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::Readable for START_SPEC {}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {
    const RESET_VALUE: u32 = 0;
}
