#[doc = "Register `L1_DBUS3_ACS_NXTLVL_WR_CNT` reader"]
pub type R = crate::R<L1_DBUS3_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "Field `L1_DBUS3_NXTLVL_WR_CNT` reader - The register records the number of write back when bus0 accesses L1-DCache."]
pub type L1_DBUS3_NXTLVL_WR_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of write back when bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus3_nxtlvl_wr_cnt(&self) -> L1_DBUS3_NXTLVL_WR_CNT_R {
        L1_DBUS3_NXTLVL_WR_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_DBUS3_ACS_NXTLVL_WR_CNT")
            .field(
                "l1_dbus3_nxtlvl_wr_cnt",
                &format_args!("{}", self.l1_dbus3_nxtlvl_wr_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_DBUS3_ACS_NXTLVL_WR_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "L1-DCache bus3 WB-Access Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus3_acs_nxtlvl_wr_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_DBUS3_ACS_NXTLVL_WR_CNT_SPEC;
impl crate::RegisterSpec for L1_DBUS3_ACS_NXTLVL_WR_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dbus3_acs_nxtlvl_wr_cnt::R`](R) reader structure"]
impl crate::Readable for L1_DBUS3_ACS_NXTLVL_WR_CNT_SPEC {}
#[doc = "`reset()` method sets L1_DBUS3_ACS_NXTLVL_WR_CNT to value 0"]
impl crate::Resettable for L1_DBUS3_ACS_NXTLVL_WR_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
