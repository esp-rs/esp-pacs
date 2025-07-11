#[doc = "Register `DBUS0_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<DBUS0_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "Field `BUS0_NXTLVL_RD_CNT` reader - The register records the number of times that L1-Cache accesses L2-Cache due to bus0 accessing L1-Cache."]
pub type BUS0_NXTLVL_RD_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L1-Cache accesses L2-Cache due to bus0 accessing L1-Cache."]
    #[inline(always)]
    pub fn bus0_nxtlvl_rd_cnt(&self) -> BUS0_NXTLVL_RD_CNT_R {
        BUS0_NXTLVL_RD_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS0_ACS_NXTLVL_RD_CNT")
            .field("bus0_nxtlvl_rd_cnt", &self.bus0_nxtlvl_rd_cnt())
            .finish()
    }
}
#[doc = "L1-Cache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_acs_nxtlvl_rd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS0_ACS_NXTLVL_RD_CNT_SPEC;
impl crate::RegisterSpec for DBUS0_ACS_NXTLVL_RD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus0_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS0_ACS_NXTLVL_RD_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS0_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for DBUS0_ACS_NXTLVL_RD_CNT_SPEC {}
