#[doc = "Register `DBUS3_ACS_HIT_CNT` reader"]
pub type R = crate::R<DBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "Field `DBUS3_HIT_CNT` reader - The register records the number of hits when bus3 accesses L1-DCache."]
pub type DBUS3_HIT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of hits when bus3 accesses L1-DCache."]
    #[inline(always)]
    pub fn dbus3_hit_cnt(&self) -> DBUS3_HIT_CNT_R {
        DBUS3_HIT_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS3_ACS_HIT_CNT")
            .field("dbus3_hit_cnt", &self.dbus3_hit_cnt())
            .finish()
    }
}
#[doc = "L1-DCache bus3 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus3_acs_hit_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS3_ACS_HIT_CNT_SPEC;
impl crate::RegisterSpec for DBUS3_ACS_HIT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus3_acs_hit_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS3_ACS_HIT_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS3_ACS_HIT_CNT to value 0"]
impl crate::Resettable for DBUS3_ACS_HIT_CNT_SPEC {}
