#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `ZB_WAKE_PROCESS_INT_ENA` reader - need_des"]
pub type ZB_WAKE_PROCESS_INT_ENA_R = crate::BitReader;
#[doc = "Field `ZB_WAKE_PROCESS_INT_ENA` writer - need_des"]
pub type ZB_WAKE_PROCESS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEMPWR_WAKE_PROCESS_INT_ENA` reader - need_des"]
pub type MODEMPWR_WAKE_PROCESS_INT_ENA_R = crate::BitReader;
#[doc = "Field `MODEMPWR_WAKE_PROCESS_INT_ENA` writer - need_des"]
pub type MODEMPWR_WAKE_PROCESS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLE_WAKE_PROCESS_INT_ENA` reader - need_des"]
pub type BLE_WAKE_PROCESS_INT_ENA_R = crate::BitReader;
#[doc = "Field `BLE_WAKE_PROCESS_INT_ENA` writer - need_des"]
pub type BLE_WAKE_PROCESS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_WAKE_PROCESS_INT_ENA` reader - need_des"]
pub type WIFI_WAKE_PROCESS_INT_ENA_R = crate::BitReader;
#[doc = "Field `WIFI_WAKE_PROCESS_INT_ENA` writer - need_des"]
pub type WIFI_WAKE_PROCESS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_CALI_WAKE_PROCESS_INT_ENA` reader - need_des"]
pub type XTAL_CALI_WAKE_PROCESS_INT_ENA_R = crate::BitReader;
#[doc = "Field `XTAL_CALI_WAKE_PROCESS_INT_ENA` writer - need_des"]
pub type XTAL_CALI_WAKE_PROCESS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_WAKE_PROCESS_INT_ENA` reader - need_des"]
pub type TOUCH_WAKE_PROCESS_INT_ENA_R = crate::BitReader;
#[doc = "Field `TOUCH_WAKE_PROCESS_INT_ENA` writer - need_des"]
pub type TOUCH_WAKE_PROCESS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_WAKE_PROCESS_INT_ENA` reader - need_des"]
pub type LPCPU_WAKE_PROCESS_INT_ENA_R = crate::BitReader;
#[doc = "Field `LPCPU_WAKE_PROCESS_INT_ENA` writer - need_des"]
pub type LPCPU_WAKE_PROCESS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPWR_WAKE_PROCESS_INT_ENA` reader - need_des"]
pub type APPWR_WAKE_PROCESS_INT_ENA_R = crate::BitReader;
#[doc = "Field `APPWR_WAKE_PROCESS_INT_ENA` writer - need_des"]
pub type APPWR_WAKE_PROCESS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn zb_wake_process_int_ena(&self) -> ZB_WAKE_PROCESS_INT_ENA_R {
        ZB_WAKE_PROCESS_INT_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn modempwr_wake_process_int_ena(&self) -> MODEMPWR_WAKE_PROCESS_INT_ENA_R {
        MODEMPWR_WAKE_PROCESS_INT_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ble_wake_process_int_ena(&self) -> BLE_WAKE_PROCESS_INT_ENA_R {
        BLE_WAKE_PROCESS_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn wifi_wake_process_int_ena(&self) -> WIFI_WAKE_PROCESS_INT_ENA_R {
        WIFI_WAKE_PROCESS_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xtal_cali_wake_process_int_ena(&self) -> XTAL_CALI_WAKE_PROCESS_INT_ENA_R {
        XTAL_CALI_WAKE_PROCESS_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn touch_wake_process_int_ena(&self) -> TOUCH_WAKE_PROCESS_INT_ENA_R {
        TOUCH_WAKE_PROCESS_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lpcpu_wake_process_int_ena(&self) -> LPCPU_WAKE_PROCESS_INT_ENA_R {
        LPCPU_WAKE_PROCESS_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn appwr_wake_process_int_ena(&self) -> APPWR_WAKE_PROCESS_INT_ENA_R {
        APPWR_WAKE_PROCESS_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("zb_wake_process_int_ena", &self.zb_wake_process_int_ena())
            .field(
                "modempwr_wake_process_int_ena",
                &self.modempwr_wake_process_int_ena(),
            )
            .field("ble_wake_process_int_ena", &self.ble_wake_process_int_ena())
            .field(
                "wifi_wake_process_int_ena",
                &self.wifi_wake_process_int_ena(),
            )
            .field(
                "xtal_cali_wake_process_int_ena",
                &self.xtal_cali_wake_process_int_ena(),
            )
            .field(
                "touch_wake_process_int_ena",
                &self.touch_wake_process_int_ena(),
            )
            .field(
                "lpcpu_wake_process_int_ena",
                &self.lpcpu_wake_process_int_ena(),
            )
            .field(
                "appwr_wake_process_int_ena",
                &self.appwr_wake_process_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn zb_wake_process_int_ena(&mut self) -> ZB_WAKE_PROCESS_INT_ENA_W<'_, INT_ENA_SPEC> {
        ZB_WAKE_PROCESS_INT_ENA_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn modempwr_wake_process_int_ena(
        &mut self,
    ) -> MODEMPWR_WAKE_PROCESS_INT_ENA_W<'_, INT_ENA_SPEC> {
        MODEMPWR_WAKE_PROCESS_INT_ENA_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ble_wake_process_int_ena(&mut self) -> BLE_WAKE_PROCESS_INT_ENA_W<'_, INT_ENA_SPEC> {
        BLE_WAKE_PROCESS_INT_ENA_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn wifi_wake_process_int_ena(&mut self) -> WIFI_WAKE_PROCESS_INT_ENA_W<'_, INT_ENA_SPEC> {
        WIFI_WAKE_PROCESS_INT_ENA_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xtal_cali_wake_process_int_ena(
        &mut self,
    ) -> XTAL_CALI_WAKE_PROCESS_INT_ENA_W<'_, INT_ENA_SPEC> {
        XTAL_CALI_WAKE_PROCESS_INT_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn touch_wake_process_int_ena(&mut self) -> TOUCH_WAKE_PROCESS_INT_ENA_W<'_, INT_ENA_SPEC> {
        TOUCH_WAKE_PROCESS_INT_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lpcpu_wake_process_int_ena(&mut self) -> LPCPU_WAKE_PROCESS_INT_ENA_W<'_, INT_ENA_SPEC> {
        LPCPU_WAKE_PROCESS_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn appwr_wake_process_int_ena(&mut self) -> APPWR_WAKE_PROCESS_INT_ENA_W<'_, INT_ENA_SPEC> {
        APPWR_WAKE_PROCESS_INT_ENA_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
