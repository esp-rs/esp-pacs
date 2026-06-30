#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `ZB_WAKE_PROCESS_INT_ST` reader - need_des"]
pub type ZB_WAKE_PROCESS_INT_ST_R = crate::BitReader;
#[doc = "Field `MODEMPWR_WAKE_PROCESS_INT_ST` reader - need_des"]
pub type MODEMPWR_WAKE_PROCESS_INT_ST_R = crate::BitReader;
#[doc = "Field `BLE_WAKE_PROCESS_INT_ST` reader - need_des"]
pub type BLE_WAKE_PROCESS_INT_ST_R = crate::BitReader;
#[doc = "Field `WIFI_WAKE_PROCESS_INT_ST` reader - need_des"]
pub type WIFI_WAKE_PROCESS_INT_ST_R = crate::BitReader;
#[doc = "Field `XTAL_CALI_WAKE_PROCESS_INT_ST` reader - need_des"]
pub type XTAL_CALI_WAKE_PROCESS_INT_ST_R = crate::BitReader;
#[doc = "Field `TOUCH_WAKE_PROCESS_INT_ST` reader - need_des"]
pub type TOUCH_WAKE_PROCESS_INT_ST_R = crate::BitReader;
#[doc = "Field `LPCPU_WAKE_PROCESS_INT_ST` reader - need_des"]
pub type LPCPU_WAKE_PROCESS_INT_ST_R = crate::BitReader;
#[doc = "Field `APPWR_WAKE_PROCESS_INT_ST` reader - need_des"]
pub type APPWR_WAKE_PROCESS_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn zb_wake_process_int_st(&self) -> ZB_WAKE_PROCESS_INT_ST_R {
        ZB_WAKE_PROCESS_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn modempwr_wake_process_int_st(&self) -> MODEMPWR_WAKE_PROCESS_INT_ST_R {
        MODEMPWR_WAKE_PROCESS_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ble_wake_process_int_st(&self) -> BLE_WAKE_PROCESS_INT_ST_R {
        BLE_WAKE_PROCESS_INT_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn wifi_wake_process_int_st(&self) -> WIFI_WAKE_PROCESS_INT_ST_R {
        WIFI_WAKE_PROCESS_INT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xtal_cali_wake_process_int_st(&self) -> XTAL_CALI_WAKE_PROCESS_INT_ST_R {
        XTAL_CALI_WAKE_PROCESS_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn touch_wake_process_int_st(&self) -> TOUCH_WAKE_PROCESS_INT_ST_R {
        TOUCH_WAKE_PROCESS_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lpcpu_wake_process_int_st(&self) -> LPCPU_WAKE_PROCESS_INT_ST_R {
        LPCPU_WAKE_PROCESS_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn appwr_wake_process_int_st(&self) -> APPWR_WAKE_PROCESS_INT_ST_R {
        APPWR_WAKE_PROCESS_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("zb_wake_process_int_st", &self.zb_wake_process_int_st())
            .field(
                "modempwr_wake_process_int_st",
                &self.modempwr_wake_process_int_st(),
            )
            .field("ble_wake_process_int_st", &self.ble_wake_process_int_st())
            .field("wifi_wake_process_int_st", &self.wifi_wake_process_int_st())
            .field(
                "xtal_cali_wake_process_int_st",
                &self.xtal_cali_wake_process_int_st(),
            )
            .field(
                "touch_wake_process_int_st",
                &self.touch_wake_process_int_st(),
            )
            .field(
                "lpcpu_wake_process_int_st",
                &self.lpcpu_wake_process_int_st(),
            )
            .field(
                "appwr_wake_process_int_st",
                &self.appwr_wake_process_int_st(),
            )
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
