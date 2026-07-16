#[doc = "Register `MODEM_INTR_WAKEUP_CONF` reader"]
pub type R = crate::R<MODEM_INTR_WAKEUP_CONF_SPEC>;
#[doc = "Register `MODEM_INTR_WAKEUP_CONF` writer"]
pub type W = crate::W<MODEM_INTR_WAKEUP_CONF_SPEC>;
#[doc = "Field `WIFI_MAC_INTR_WAKEUP_EN` reader - "]
pub type WIFI_MAC_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `WIFI_MAC_INTR_WAKEUP_EN` writer - "]
pub type WIFI_MAC_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_MAC_NMI_WAKEUP_EN` reader - "]
pub type WIFI_MAC_NMI_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `WIFI_MAC_NMI_WAKEUP_EN` writer - "]
pub type WIFI_MAC_NMI_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_PWR_INTR_WAKEUP_EN` reader - "]
pub type WIFI_PWR_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `WIFI_PWR_INTR_WAKEUP_EN` writer - "]
pub type WIFI_PWR_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_BB_INTR_WAKEUP_EN` reader - "]
pub type WIFI_BB_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `WIFI_BB_INTR_WAKEUP_EN` writer - "]
pub type WIFI_BB_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_MAC_INTR_WAKEUP_EN` reader - "]
pub type BT_MAC_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `BT_MAC_INTR_WAKEUP_EN` writer - "]
pub type BT_MAC_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_BB_INTR_WAKEUP_EN` reader - "]
pub type BT_BB_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `BT_BB_INTR_WAKEUP_EN` writer - "]
pub type BT_BB_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_BB_NMI_WAKEUP_EN` reader - "]
pub type BT_BB_NMI_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `BT_BB_NMI_WAKEUP_EN` writer - "]
pub type BT_BB_NMI_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TIMER_INTR_WAKEUP_EN` reader - "]
pub type LP_TIMER_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `LP_TIMER_INTR_WAKEUP_EN` writer - "]
pub type LP_TIMER_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COEX_INTR_WAKEUP_EN` reader - "]
pub type COEX_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `COEX_INTR_WAKEUP_EN` writer - "]
pub type COEX_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLE_TIMER_INTR_WAKEUP_EN` reader - "]
pub type BLE_TIMER_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `BLE_TIMER_INTR_WAKEUP_EN` writer - "]
pub type BLE_TIMER_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLE_SEC_INTR_WAKEUP_EN` reader - "]
pub type BLE_SEC_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `BLE_SEC_INTR_WAKEUP_EN` writer - "]
pub type BLE_SEC_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_MST_INTR_WAKEUP_EN` reader - "]
pub type I2C_MST_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `I2C_MST_INTR_WAKEUP_EN` writer - "]
pub type I2C_MST_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZB_MAC_INTR_WAKEUP_EN` reader - "]
pub type ZB_MAC_INTR_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `ZB_MAC_INTR_WAKEUP_EN` writer - "]
pub type ZB_MAC_INTR_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_MAC_INT1_WAKEUP_EN` reader - "]
pub type BT_MAC_INT1_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `BT_MAC_INT1_WAKEUP_EN` writer - "]
pub type BT_MAC_INT1_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifi_mac_intr_wakeup_en(&self) -> WIFI_MAC_INTR_WAKEUP_EN_R {
        WIFI_MAC_INTR_WAKEUP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifi_mac_nmi_wakeup_en(&self) -> WIFI_MAC_NMI_WAKEUP_EN_R {
        WIFI_MAC_NMI_WAKEUP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifi_pwr_intr_wakeup_en(&self) -> WIFI_PWR_INTR_WAKEUP_EN_R {
        WIFI_PWR_INTR_WAKEUP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifi_bb_intr_wakeup_en(&self) -> WIFI_BB_INTR_WAKEUP_EN_R {
        WIFI_BB_INTR_WAKEUP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bt_mac_intr_wakeup_en(&self) -> BT_MAC_INTR_WAKEUP_EN_R {
        BT_MAC_INTR_WAKEUP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bt_bb_intr_wakeup_en(&self) -> BT_BB_INTR_WAKEUP_EN_R {
        BT_BB_INTR_WAKEUP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bt_bb_nmi_wakeup_en(&self) -> BT_BB_NMI_WAKEUP_EN_R {
        BT_BB_NMI_WAKEUP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lp_timer_intr_wakeup_en(&self) -> LP_TIMER_INTR_WAKEUP_EN_R {
        LP_TIMER_INTR_WAKEUP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn coex_intr_wakeup_en(&self) -> COEX_INTR_WAKEUP_EN_R {
        COEX_INTR_WAKEUP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ble_timer_intr_wakeup_en(&self) -> BLE_TIMER_INTR_WAKEUP_EN_R {
        BLE_TIMER_INTR_WAKEUP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ble_sec_intr_wakeup_en(&self) -> BLE_SEC_INTR_WAKEUP_EN_R {
        BLE_SEC_INTR_WAKEUP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c_mst_intr_wakeup_en(&self) -> I2C_MST_INTR_WAKEUP_EN_R {
        I2C_MST_INTR_WAKEUP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn zb_mac_intr_wakeup_en(&self) -> ZB_MAC_INTR_WAKEUP_EN_R {
        ZB_MAC_INTR_WAKEUP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bt_mac_int1_wakeup_en(&self) -> BT_MAC_INT1_WAKEUP_EN_R {
        BT_MAC_INT1_WAKEUP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_INTR_WAKEUP_CONF")
            .field("wifi_mac_intr_wakeup_en", &self.wifi_mac_intr_wakeup_en())
            .field("wifi_mac_nmi_wakeup_en", &self.wifi_mac_nmi_wakeup_en())
            .field("wifi_pwr_intr_wakeup_en", &self.wifi_pwr_intr_wakeup_en())
            .field("wifi_bb_intr_wakeup_en", &self.wifi_bb_intr_wakeup_en())
            .field("bt_mac_intr_wakeup_en", &self.bt_mac_intr_wakeup_en())
            .field("bt_bb_intr_wakeup_en", &self.bt_bb_intr_wakeup_en())
            .field("bt_bb_nmi_wakeup_en", &self.bt_bb_nmi_wakeup_en())
            .field("lp_timer_intr_wakeup_en", &self.lp_timer_intr_wakeup_en())
            .field("coex_intr_wakeup_en", &self.coex_intr_wakeup_en())
            .field("ble_timer_intr_wakeup_en", &self.ble_timer_intr_wakeup_en())
            .field("ble_sec_intr_wakeup_en", &self.ble_sec_intr_wakeup_en())
            .field("i2c_mst_intr_wakeup_en", &self.i2c_mst_intr_wakeup_en())
            .field("zb_mac_intr_wakeup_en", &self.zb_mac_intr_wakeup_en())
            .field("bt_mac_int1_wakeup_en", &self.bt_mac_int1_wakeup_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifi_mac_intr_wakeup_en(
        &mut self,
    ) -> WIFI_MAC_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        WIFI_MAC_INTR_WAKEUP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifi_mac_nmi_wakeup_en(
        &mut self,
    ) -> WIFI_MAC_NMI_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        WIFI_MAC_NMI_WAKEUP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifi_pwr_intr_wakeup_en(
        &mut self,
    ) -> WIFI_PWR_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        WIFI_PWR_INTR_WAKEUP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifi_bb_intr_wakeup_en(
        &mut self,
    ) -> WIFI_BB_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        WIFI_BB_INTR_WAKEUP_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bt_mac_intr_wakeup_en(
        &mut self,
    ) -> BT_MAC_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        BT_MAC_INTR_WAKEUP_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bt_bb_intr_wakeup_en(
        &mut self,
    ) -> BT_BB_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        BT_BB_INTR_WAKEUP_EN_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bt_bb_nmi_wakeup_en(
        &mut self,
    ) -> BT_BB_NMI_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        BT_BB_NMI_WAKEUP_EN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lp_timer_intr_wakeup_en(
        &mut self,
    ) -> LP_TIMER_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        LP_TIMER_INTR_WAKEUP_EN_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn coex_intr_wakeup_en(
        &mut self,
    ) -> COEX_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        COEX_INTR_WAKEUP_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ble_timer_intr_wakeup_en(
        &mut self,
    ) -> BLE_TIMER_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        BLE_TIMER_INTR_WAKEUP_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ble_sec_intr_wakeup_en(
        &mut self,
    ) -> BLE_SEC_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        BLE_SEC_INTR_WAKEUP_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c_mst_intr_wakeup_en(
        &mut self,
    ) -> I2C_MST_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        I2C_MST_INTR_WAKEUP_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn zb_mac_intr_wakeup_en(
        &mut self,
    ) -> ZB_MAC_INTR_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        ZB_MAC_INTR_WAKEUP_EN_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bt_mac_int1_wakeup_en(
        &mut self,
    ) -> BT_MAC_INT1_WAKEUP_EN_W<'_, MODEM_INTR_WAKEUP_CONF_SPEC> {
        BT_MAC_INT1_WAKEUP_EN_W::new(self, 13)
    }
}
#[doc = "MODEM_INTR_WAKEUP_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_intr_wakeup_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_intr_wakeup_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_INTR_WAKEUP_CONF_SPEC;
impl crate::RegisterSpec for MODEM_INTR_WAKEUP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_intr_wakeup_conf::R`](R) reader structure"]
impl crate::Readable for MODEM_INTR_WAKEUP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_intr_wakeup_conf::W`](W) writer structure"]
impl crate::Writable for MODEM_INTR_WAKEUP_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_INTR_WAKEUP_CONF to value 0x3fff"]
impl crate::Resettable for MODEM_INTR_WAKEUP_CONF_SPEC {
    const RESET_VALUE: u32 = 0x3fff;
}
