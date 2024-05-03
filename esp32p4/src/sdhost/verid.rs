#[doc = "Register `VERID` reader"]
pub type R = crate::R<VERID_SPEC>;
#[doc = "Field `VERSIONID` reader - Hardware version register. Can also be read by fireware."]
pub type VERSIONID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hardware version register. Can also be read by fireware."]
    #[inline(always)]
    pub fn versionid(&self) -> VERSIONID_R {
        VERSIONID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERID")
            .field("versionid", &self.versionid().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VERID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Version ID (scratchpad) register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERID_SPEC;
impl crate::RegisterSpec for VERID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verid::R`](R) reader structure"]
impl crate::Readable for VERID_SPEC {}
#[doc = "`reset()` method sets VERID to value 0x5432_270a"]
impl crate::Resettable for VERID_SPEC {
    const RESET_VALUE: u32 = 0x5432_270a;
}
