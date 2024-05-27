///Register `DBUS2_ACS_WB_CNT` reader
pub type R = crate::R<DBUS2_ACS_WB_CNT_SPEC>;
///Field `DBUS2_ACS_WB_CNT` reader - The bits are used to count the number of cache evictions by dbus2 access cache.
pub type DBUS2_ACS_WB_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:19 - The bits are used to count the number of cache evictions by dbus2 access cache.
    #[inline(always)]
    pub fn dbus2_acs_wb_cnt(&self) -> DBUS2_ACS_WB_CNT_R {
        DBUS2_ACS_WB_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS2_ACS_WB_CNT")
            .field("dbus2_acs_wb_cnt", &self.dbus2_acs_wb_cnt())
            .finish()
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`dbus2_acs_wb_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DBUS2_ACS_WB_CNT_SPEC;
impl crate::RegisterSpec for DBUS2_ACS_WB_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dbus2_acs_wb_cnt::R`](R) reader structure
impl crate::Readable for DBUS2_ACS_WB_CNT_SPEC {}
///`reset()` method sets DBUS2_ACS_WB_CNT to value 0
impl crate::Resettable for DBUS2_ACS_WB_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
