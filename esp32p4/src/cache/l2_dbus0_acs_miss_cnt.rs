#[doc = "Register `L2_DBUS0_ACS_MISS_CNT` reader"]
pub type R = crate::R<L2_DBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "Field `L2_DBUS0_MISS_CNT` reader - The register records the number of missing when L1-DCache accesses L2-Cache due to bus0 accessing L1-DCache."]
pub type L2_DBUS0_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when L1-DCache accesses L2-Cache due to bus0 accessing L1-DCache."]
    #[inline(always)]
    pub fn l2_dbus0_miss_cnt(&self) -> L2_DBUS0_MISS_CNT_R {
        L2_DBUS0_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_DBUS0_ACS_MISS_CNT")
            .field("l2_dbus0_miss_cnt", &self.l2_dbus0_miss_cnt())
            .finish()
    }
}
#[doc = "L2-Cache bus0 Miss-Access Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus0_acs_miss_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_DBUS0_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for L2_DBUS0_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_dbus0_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for L2_DBUS0_ACS_MISS_CNT_SPEC {}
#[doc = "`reset()` method sets L2_DBUS0_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L2_DBUS0_ACS_MISS_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
