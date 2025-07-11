#[doc = "Register `DBUS2_ACS_HIT_CNT` reader"]
pub type R = crate::R<DBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "Field `DBUS2_HIT_CNT` reader - The register records the number of hits when bus2 accesses L1-DCache."]
pub type DBUS2_HIT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of hits when bus2 accesses L1-DCache."]
    #[inline(always)]
    pub fn dbus2_hit_cnt(&self) -> DBUS2_HIT_CNT_R {
        DBUS2_HIT_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS2_ACS_HIT_CNT")
            .field("dbus2_hit_cnt", &self.dbus2_hit_cnt())
            .finish()
    }
}
#[doc = "L1-DCache bus2 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_hit_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS2_ACS_HIT_CNT_SPEC;
impl crate::RegisterSpec for DBUS2_ACS_HIT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus2_acs_hit_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS2_ACS_HIT_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS2_ACS_HIT_CNT to value 0"]
impl crate::Resettable for DBUS2_ACS_HIT_CNT_SPEC {}
