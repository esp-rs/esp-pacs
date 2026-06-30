#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `ZB_WAKE_PROCESS_INT_CLR` writer - need_des"]
pub type ZB_WAKE_PROCESS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEMPWR_WAKE_PROCESS_INT_CLR` writer - need_des"]
pub type MODEMPWR_WAKE_PROCESS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLE_WAKE_PROCESS_INT_CLR` writer - need_des"]
pub type BLE_WAKE_PROCESS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_WAKE_PROCESS_INT_CLR` writer - need_des"]
pub type WIFI_WAKE_PROCESS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_CALI_WAKE_PROCESS_INT_CLR` writer - need_des"]
pub type XTAL_CALI_WAKE_PROCESS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_WAKE_PROCESS_INT_CLR` writer - need_des"]
pub type TOUCH_WAKE_PROCESS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_WAKE_PROCESS_INT_CLR` writer - need_des"]
pub type LPCPU_WAKE_PROCESS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPWR_WAKE_PROCESS_INT_CLR` writer - need_des"]
pub type APPWR_WAKE_PROCESS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn zb_wake_process_int_clr(&mut self) -> ZB_WAKE_PROCESS_INT_CLR_W<'_, INT_CLR_SPEC> {
        ZB_WAKE_PROCESS_INT_CLR_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn modempwr_wake_process_int_clr(
        &mut self,
    ) -> MODEMPWR_WAKE_PROCESS_INT_CLR_W<'_, INT_CLR_SPEC> {
        MODEMPWR_WAKE_PROCESS_INT_CLR_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ble_wake_process_int_clr(&mut self) -> BLE_WAKE_PROCESS_INT_CLR_W<'_, INT_CLR_SPEC> {
        BLE_WAKE_PROCESS_INT_CLR_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn wifi_wake_process_int_clr(&mut self) -> WIFI_WAKE_PROCESS_INT_CLR_W<'_, INT_CLR_SPEC> {
        WIFI_WAKE_PROCESS_INT_CLR_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xtal_cali_wake_process_int_clr(
        &mut self,
    ) -> XTAL_CALI_WAKE_PROCESS_INT_CLR_W<'_, INT_CLR_SPEC> {
        XTAL_CALI_WAKE_PROCESS_INT_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn touch_wake_process_int_clr(&mut self) -> TOUCH_WAKE_PROCESS_INT_CLR_W<'_, INT_CLR_SPEC> {
        TOUCH_WAKE_PROCESS_INT_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lpcpu_wake_process_int_clr(&mut self) -> LPCPU_WAKE_PROCESS_INT_CLR_W<'_, INT_CLR_SPEC> {
        LPCPU_WAKE_PROCESS_INT_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn appwr_wake_process_int_clr(&mut self) -> APPWR_WAKE_PROCESS_INT_CLR_W<'_, INT_CLR_SPEC> {
        APPWR_WAKE_PROCESS_INT_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
