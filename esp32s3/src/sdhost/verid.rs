///Register `VERID` reader
pub type R = crate::R<VERID_SPEC>;
///Field `VERSIONID` reader - Hardware version register. Can also be read by fireware.
pub type VERSIONID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Hardware version register. Can also be read by fireware.
    #[inline(always)]
    pub fn versionid(&self) -> VERSIONID_R {
        VERSIONID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERID")
            .field("versionid", &self.versionid())
            .finish()
    }
}
/**Version ID (scratchpad) register

You can [`read`](crate::generic::Reg::read) this register and get [`verid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VERID_SPEC;
impl crate::RegisterSpec for VERID_SPEC {
    type Ux = u32;
}
///`read()` method returns [`verid::R`](R) reader structure
impl crate::Readable for VERID_SPEC {}
///`reset()` method sets VERID to value 0x5432_270a
impl crate::Resettable for VERID_SPEC {
    const RESET_VALUE: u32 = 0x5432_270a;
}
