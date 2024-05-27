///Register `CHN_STATUS` reader
pub type R = crate::R<CHN_STATUS_SPEC>;
///Field `PAD_ACTIVE` reader - need_des
pub type PAD_ACTIVE_R = crate::FieldReader<u16>;
///Field `MEAS_DONE` reader - need_des
pub type MEAS_DONE_R = crate::BitReader;
///Field `SCAN_CURR` reader - need_des
pub type SCAN_CURR_R = crate::FieldReader;
impl R {
    ///Bits 0:14 - need_des
    #[inline(always)]
    pub fn pad_active(&self) -> PAD_ACTIVE_R {
        PAD_ACTIVE_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bit 15 - need_des
    #[inline(always)]
    pub fn meas_done(&self) -> MEAS_DONE_R {
        MEAS_DONE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - need_des
    #[inline(always)]
    pub fn scan_curr(&self) -> SCAN_CURR_R {
        SCAN_CURR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHN_STATUS")
            .field("pad_active", &self.pad_active())
            .field("meas_done", &self.meas_done())
            .field("scan_curr", &self.scan_curr())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`chn_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CHN_STATUS_SPEC;
impl crate::RegisterSpec for CHN_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`chn_status::R`](R) reader structure
impl crate::Readable for CHN_STATUS_SPEC {}
///`reset()` method sets CHN_STATUS to value 0
impl crate::Resettable for CHN_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
