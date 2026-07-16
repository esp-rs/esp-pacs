#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `ZB_WAKE_PROCESS_INT_RAW` reader - need_des"]
pub type ZB_WAKE_PROCESS_INT_RAW_R = crate::BitReader;
#[doc = "Field `ZB_WAKE_PROCESS_INT_RAW` writer - need_des"]
pub type ZB_WAKE_PROCESS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEMPWR_WAKE_PROCESS_INT_RAW` reader - need_des"]
pub type MODEMPWR_WAKE_PROCESS_INT_RAW_R = crate::BitReader;
#[doc = "Field `MODEMPWR_WAKE_PROCESS_INT_RAW` writer - need_des"]
pub type MODEMPWR_WAKE_PROCESS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLE_WAKE_PROCESS_INT_RAW` reader - need_des"]
pub type BLE_WAKE_PROCESS_INT_RAW_R = crate::BitReader;
#[doc = "Field `BLE_WAKE_PROCESS_INT_RAW` writer - need_des"]
pub type BLE_WAKE_PROCESS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_WAKE_PROCESS_INT_RAW` reader - need_des"]
pub type WIFI_WAKE_PROCESS_INT_RAW_R = crate::BitReader;
#[doc = "Field `WIFI_WAKE_PROCESS_INT_RAW` writer - need_des"]
pub type WIFI_WAKE_PROCESS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_CALI_WAKE_PROCESS_INT_RAW` reader - need_des"]
pub type XTAL_CALI_WAKE_PROCESS_INT_RAW_R = crate::BitReader;
#[doc = "Field `XTAL_CALI_WAKE_PROCESS_INT_RAW` writer - need_des"]
pub type XTAL_CALI_WAKE_PROCESS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_WAKE_PROCESS_INT_RAW` reader - need_des"]
pub type TOUCH_WAKE_PROCESS_INT_RAW_R = crate::BitReader;
#[doc = "Field `TOUCH_WAKE_PROCESS_INT_RAW` writer - need_des"]
pub type TOUCH_WAKE_PROCESS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_WAKE_PROCESS_INT_RAW` reader - need_des"]
pub type LPCPU_WAKE_PROCESS_INT_RAW_R = crate::BitReader;
#[doc = "Field `LPCPU_WAKE_PROCESS_INT_RAW` writer - need_des"]
pub type LPCPU_WAKE_PROCESS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPWR_WAKE_PROCESS_INT_RAW` reader - need_des"]
pub type APPWR_WAKE_PROCESS_INT_RAW_R = crate::BitReader;
#[doc = "Field `APPWR_WAKE_PROCESS_INT_RAW` writer - need_des"]
pub type APPWR_WAKE_PROCESS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn zb_wake_process_int_raw(&self) -> ZB_WAKE_PROCESS_INT_RAW_R {
        ZB_WAKE_PROCESS_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn modempwr_wake_process_int_raw(&self) -> MODEMPWR_WAKE_PROCESS_INT_RAW_R {
        MODEMPWR_WAKE_PROCESS_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ble_wake_process_int_raw(&self) -> BLE_WAKE_PROCESS_INT_RAW_R {
        BLE_WAKE_PROCESS_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn wifi_wake_process_int_raw(&self) -> WIFI_WAKE_PROCESS_INT_RAW_R {
        WIFI_WAKE_PROCESS_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xtal_cali_wake_process_int_raw(&self) -> XTAL_CALI_WAKE_PROCESS_INT_RAW_R {
        XTAL_CALI_WAKE_PROCESS_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn touch_wake_process_int_raw(&self) -> TOUCH_WAKE_PROCESS_INT_RAW_R {
        TOUCH_WAKE_PROCESS_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lpcpu_wake_process_int_raw(&self) -> LPCPU_WAKE_PROCESS_INT_RAW_R {
        LPCPU_WAKE_PROCESS_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn appwr_wake_process_int_raw(&self) -> APPWR_WAKE_PROCESS_INT_RAW_R {
        APPWR_WAKE_PROCESS_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("zb_wake_process_int_raw", &self.zb_wake_process_int_raw())
            .field(
                "modempwr_wake_process_int_raw",
                &self.modempwr_wake_process_int_raw(),
            )
            .field("ble_wake_process_int_raw", &self.ble_wake_process_int_raw())
            .field(
                "wifi_wake_process_int_raw",
                &self.wifi_wake_process_int_raw(),
            )
            .field(
                "xtal_cali_wake_process_int_raw",
                &self.xtal_cali_wake_process_int_raw(),
            )
            .field(
                "touch_wake_process_int_raw",
                &self.touch_wake_process_int_raw(),
            )
            .field(
                "lpcpu_wake_process_int_raw",
                &self.lpcpu_wake_process_int_raw(),
            )
            .field(
                "appwr_wake_process_int_raw",
                &self.appwr_wake_process_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn zb_wake_process_int_raw(&mut self) -> ZB_WAKE_PROCESS_INT_RAW_W<'_, INT_RAW_SPEC> {
        ZB_WAKE_PROCESS_INT_RAW_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn modempwr_wake_process_int_raw(
        &mut self,
    ) -> MODEMPWR_WAKE_PROCESS_INT_RAW_W<'_, INT_RAW_SPEC> {
        MODEMPWR_WAKE_PROCESS_INT_RAW_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ble_wake_process_int_raw(&mut self) -> BLE_WAKE_PROCESS_INT_RAW_W<'_, INT_RAW_SPEC> {
        BLE_WAKE_PROCESS_INT_RAW_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn wifi_wake_process_int_raw(&mut self) -> WIFI_WAKE_PROCESS_INT_RAW_W<'_, INT_RAW_SPEC> {
        WIFI_WAKE_PROCESS_INT_RAW_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xtal_cali_wake_process_int_raw(
        &mut self,
    ) -> XTAL_CALI_WAKE_PROCESS_INT_RAW_W<'_, INT_RAW_SPEC> {
        XTAL_CALI_WAKE_PROCESS_INT_RAW_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn touch_wake_process_int_raw(&mut self) -> TOUCH_WAKE_PROCESS_INT_RAW_W<'_, INT_RAW_SPEC> {
        TOUCH_WAKE_PROCESS_INT_RAW_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lpcpu_wake_process_int_raw(&mut self) -> LPCPU_WAKE_PROCESS_INT_RAW_W<'_, INT_RAW_SPEC> {
        LPCPU_WAKE_PROCESS_INT_RAW_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn appwr_wake_process_int_raw(&mut self) -> APPWR_WAKE_PROCESS_INT_RAW_W<'_, INT_RAW_SPEC> {
        APPWR_WAKE_PROCESS_INT_RAW_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
