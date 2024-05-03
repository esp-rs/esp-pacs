#[doc = "Register `L2_DBUS0_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<L2_DBUS0_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "Field `L2_DBUS0_NXTLVL_RD_CNT` reader - The register records the number of times that L2-Cache accesses external memory due to L1-DCache accessing L2-Cache due to bus0 accessing L1-DCache."]
pub type L2_DBUS0_NXTLVL_RD_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L2-Cache accesses external memory due to L1-DCache accessing L2-Cache due to bus0 accessing L1-DCache."]
    #[inline(always)]
    pub fn l2_dbus0_nxtlvl_rd_cnt(&self) -> L2_DBUS0_NXTLVL_RD_CNT_R {
        L2_DBUS0_NXTLVL_RD_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_DBUS0_ACS_NXTLVL_RD_CNT")
            .field(
                "l2_dbus0_nxtlvl_rd_cnt",
                &self.l2_dbus0_nxtlvl_rd_cnt().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_DBUS0_ACS_NXTLVL_RD_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "L2-Cache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus0_acs_nxtlvl_rd_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_DBUS0_ACS_NXTLVL_RD_CNT_SPEC;
impl crate::RegisterSpec for L2_DBUS0_ACS_NXTLVL_RD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_dbus0_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for L2_DBUS0_ACS_NXTLVL_RD_CNT_SPEC {}
#[doc = "`reset()` method sets L2_DBUS0_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for L2_DBUS0_ACS_NXTLVL_RD_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
