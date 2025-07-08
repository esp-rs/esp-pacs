#[doc = "Register `DBUS2_ACS_CONFLICT_CNT` reader"]
pub type R = crate::R<DBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "Field `DBUS2_CONFLICT_CNT` reader - The register records the number of access-conflicts when bus2 accesses L1-DCache."]
pub type DBUS2_CONFLICT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of access-conflicts when bus2 accesses L1-DCache."]
    #[inline(always)]
    pub fn dbus2_conflict_cnt(&self) -> DBUS2_CONFLICT_CNT_R {
        DBUS2_CONFLICT_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS2_ACS_CONFLICT_CNT")
            .field("dbus2_conflict_cnt", &self.dbus2_conflict_cnt())
            .finish()
    }
}
#[doc = "L1-DCache bus2 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_conflict_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS2_ACS_CONFLICT_CNT_SPEC;
impl crate::RegisterSpec for DBUS2_ACS_CONFLICT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus2_acs_conflict_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS2_ACS_CONFLICT_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS2_ACS_CONFLICT_CNT to value 0"]
impl crate::Resettable for DBUS2_ACS_CONFLICT_CNT_SPEC {}
