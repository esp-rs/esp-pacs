#[doc = "Register `CAPABILITIES` reader"]
pub type R = crate::R<CAPABILITIES_SPEC>;
#[doc = "Field `CAPABLITIES` reader - NA"]
pub type CAPABLITIES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn capablities(&self) -> CAPABLITIES_R {
        CAPABLITIES_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPABILITIES")
            .field("capablities", &self.capablities())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPABILITIES_SPEC;
impl crate::RegisterSpec for CAPABILITIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities::R`](R) reader structure"]
impl crate::Readable for CAPABILITIES_SPEC {}
#[doc = "`reset()` method sets CAPABILITIES to value 0x7c13_fc1c"]
impl crate::Resettable for CAPABILITIES_SPEC {
    const RESET_VALUE: u32 = 0x7c13_fc1c;
}
