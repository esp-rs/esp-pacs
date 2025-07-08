#[doc = "Register `DBUS2_ACS_NXTLVL_WR_CNT` reader"]
pub type R = crate::R<DBUS2_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "Field `DBUS2_NXTLVL_WR_CNT` reader - The register records the number of write back when bus2 accesses L1-Cache."]
pub type DBUS2_NXTLVL_WR_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of write back when bus2 accesses L1-Cache."]
    #[inline(always)]
    pub fn dbus2_nxtlvl_wr_cnt(&self) -> DBUS2_NXTLVL_WR_CNT_R {
        DBUS2_NXTLVL_WR_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS2_ACS_NXTLVL_WR_CNT")
            .field("dbus2_nxtlvl_wr_cnt", &self.dbus2_nxtlvl_wr_cnt())
            .finish()
    }
}
#[doc = "L1-DCache bus2 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_nxtlvl_wr_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS2_ACS_NXTLVL_WR_CNT_SPEC;
impl crate::RegisterSpec for DBUS2_ACS_NXTLVL_WR_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus2_acs_nxtlvl_wr_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS2_ACS_NXTLVL_WR_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS2_ACS_NXTLVL_WR_CNT to value 0"]
impl crate::Resettable for DBUS2_ACS_NXTLVL_WR_CNT_SPEC {}
