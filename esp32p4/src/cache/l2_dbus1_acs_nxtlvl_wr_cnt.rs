///Register `L2_DBUS1_ACS_NXTLVL_WR_CNT` reader
pub type R = crate::R<L2_DBUS1_ACS_NXTLVL_WR_CNT_SPEC>;
///Field `L2_DBUS1_NXTLVL_WR_CNT` reader - The register records the number of write back when L1-DCache accesses L2-Cache due to bus1 accessing L1-DCache.
pub type L2_DBUS1_NXTLVL_WR_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The register records the number of write back when L1-DCache accesses L2-Cache due to bus1 accessing L1-DCache.
    #[inline(always)]
    pub fn l2_dbus1_nxtlvl_wr_cnt(&self) -> L2_DBUS1_NXTLVL_WR_CNT_R {
        L2_DBUS1_NXTLVL_WR_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_DBUS1_ACS_NXTLVL_WR_CNT")
            .field("l2_dbus1_nxtlvl_wr_cnt", &self.l2_dbus1_nxtlvl_wr_cnt())
            .finish()
    }
}
/**L2-Cache bus1 WB-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus1_acs_nxtlvl_wr_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_DBUS1_ACS_NXTLVL_WR_CNT_SPEC;
impl crate::RegisterSpec for L2_DBUS1_ACS_NXTLVL_WR_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_dbus1_acs_nxtlvl_wr_cnt::R`](R) reader structure
impl crate::Readable for L2_DBUS1_ACS_NXTLVL_WR_CNT_SPEC {}
///`reset()` method sets L2_DBUS1_ACS_NXTLVL_WR_CNT to value 0
impl crate::Resettable for L2_DBUS1_ACS_NXTLVL_WR_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
