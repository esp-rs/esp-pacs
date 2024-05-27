#[doc = "Register `CAPABILITIES2` reader"]
pub type R = crate::R<CAPABILITIES2_SPEC>;
#[doc = "Field `CAPABLITIES2` reader - NA"]
pub type CAPABLITIES2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn capablities2(&self) -> CAPABLITIES2_R {
        CAPABLITIES2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPABILITIES2")
            .field("capablities2", &self.capablities2())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPABILITIES2_SPEC;
impl crate::RegisterSpec for CAPABILITIES2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities2::R`](R) reader structure"]
impl crate::Readable for CAPABILITIES2_SPEC {}
#[doc = "`reset()` method sets CAPABILITIES2 to value 0x0100"]
impl crate::Resettable for CAPABILITIES2_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
