#[doc = "Register `DBUS1_ACS_NXTLVL_WR_CNT` reader"]
pub type R = crate::R<DBUS1_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "Field `DBUS1_NXTLVL_WR_CNT` reader - The register records the number of write back when bus1 accesses L1-Cache."]
pub type DBUS1_NXTLVL_WR_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of write back when bus1 accesses L1-Cache."]
    #[inline(always)]
    pub fn dbus1_nxtlvl_wr_cnt(&self) -> DBUS1_NXTLVL_WR_CNT_R {
        DBUS1_NXTLVL_WR_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS1_ACS_NXTLVL_WR_CNT")
            .field("dbus1_nxtlvl_wr_cnt", &self.dbus1_nxtlvl_wr_cnt())
            .finish()
    }
}
#[doc = "L1-DCache bus1 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus1_acs_nxtlvl_wr_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS1_ACS_NXTLVL_WR_CNT_SPEC;
impl crate::RegisterSpec for DBUS1_ACS_NXTLVL_WR_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus1_acs_nxtlvl_wr_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS1_ACS_NXTLVL_WR_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS1_ACS_NXTLVL_WR_CNT to value 0"]
impl crate::Resettable for DBUS1_ACS_NXTLVL_WR_CNT_SPEC {}
