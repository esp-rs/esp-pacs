#[doc = "Register `CLK_CONF_FORCE_ON` reader"]
pub type R = crate::R<CLK_CONF_FORCE_ON_SPEC>;
#[doc = "Register `CLK_CONF_FORCE_ON` writer"]
pub type W = crate::W<CLK_CONF_FORCE_ON_SPEC>;
#[doc = "Field `CLK_WIFIBB_FO` reader - "]
pub type CLK_WIFIBB_FO_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_FO` writer - "]
pub type CLK_WIFIBB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_WIFIMAC_FO` reader - "]
pub type CLK_WIFIMAC_FO_R = crate::BitReader;
#[doc = "Field `CLK_WIFIMAC_FO` writer - "]
pub type CLK_WIFIMAC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_WIFI_APB_FO` reader - "]
pub type CLK_WIFI_APB_FO_R = crate::BitReader;
#[doc = "Field `CLK_WIFI_APB_FO` writer - "]
pub type CLK_WIFI_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_FO` reader - "]
pub type CLK_FE_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_FO` writer - "]
pub type CLK_FE_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_APB_FO` reader - "]
pub type CLK_FE_APB_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_APB_FO` writer - "]
pub type CLK_FE_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BTBB_FO` reader - "]
pub type CLK_BTBB_FO_R = crate::BitReader;
#[doc = "Field `CLK_BTBB_FO` writer - "]
pub type CLK_BTBB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BTMAC_FO` reader - "]
pub type CLK_BTMAC_FO_R = crate::BitReader;
#[doc = "Field `CLK_BTMAC_FO` writer - "]
pub type CLK_BTMAC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_APB_FO` reader - "]
pub type CLK_BT_APB_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_APB_FO` writer - "]
pub type CLK_BT_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ZBMAC_FO` reader - "]
pub type CLK_ZBMAC_FO_R = crate::BitReader;
#[doc = "Field `CLK_ZBMAC_FO` writer - "]
pub type CLK_ZBMAC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ZBMAC_APB_FO` reader - "]
pub type CLK_ZBMAC_APB_FO_R = crate::BitReader;
#[doc = "Field `CLK_ZBMAC_APB_FO` writer - "]
pub type CLK_ZBMAC_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ETM_FO` reader - "]
pub type CLK_ETM_FO_R = crate::BitReader;
#[doc = "Field `CLK_ETM_FO` writer - "]
pub type CLK_ETM_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_FO` reader - "]
pub type CLK_MODEM_SEC_FO_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_FO` writer - "]
pub type CLK_MODEM_SEC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BLE_TIMER_FO` reader - "]
pub type CLK_BLE_TIMER_FO_R = crate::BitReader;
#[doc = "Field `CLK_BLE_TIMER_FO` writer - "]
pub type CLK_BLE_TIMER_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DATA_DUMP_FO` reader - "]
pub type CLK_DATA_DUMP_FO_R = crate::BitReader;
#[doc = "Field `CLK_DATA_DUMP_FO` writer - "]
pub type CLK_DATA_DUMP_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_wifibb_fo(&self) -> CLK_WIFIBB_FO_R {
        CLK_WIFIBB_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_wifimac_fo(&self) -> CLK_WIFIMAC_FO_R {
        CLK_WIFIMAC_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_wifi_apb_fo(&self) -> CLK_WIFI_APB_FO_R {
        CLK_WIFI_APB_FO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_fe_fo(&self) -> CLK_FE_FO_R {
        CLK_FE_FO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_fe_apb_fo(&self) -> CLK_FE_APB_FO_R {
        CLK_FE_APB_FO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_btbb_fo(&self) -> CLK_BTBB_FO_R {
        CLK_BTBB_FO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_btmac_fo(&self) -> CLK_BTMAC_FO_R {
        CLK_BTMAC_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_bt_apb_fo(&self) -> CLK_BT_APB_FO_R {
        CLK_BT_APB_FO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_zbmac_fo(&self) -> CLK_ZBMAC_FO_R {
        CLK_ZBMAC_FO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_zbmac_apb_fo(&self) -> CLK_ZBMAC_APB_FO_R {
        CLK_ZBMAC_APB_FO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_etm_fo(&self) -> CLK_ETM_FO_R {
        CLK_ETM_FO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_modem_sec_fo(&self) -> CLK_MODEM_SEC_FO_R {
        CLK_MODEM_SEC_FO_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_ble_timer_fo(&self) -> CLK_BLE_TIMER_FO_R {
        CLK_BLE_TIMER_FO_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_data_dump_fo(&self) -> CLK_DATA_DUMP_FO_R {
        CLK_DATA_DUMP_FO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF_FORCE_ON")
            .field("clk_wifibb_fo", &self.clk_wifibb_fo())
            .field("clk_wifimac_fo", &self.clk_wifimac_fo())
            .field("clk_wifi_apb_fo", &self.clk_wifi_apb_fo())
            .field("clk_fe_fo", &self.clk_fe_fo())
            .field("clk_fe_apb_fo", &self.clk_fe_apb_fo())
            .field("clk_btbb_fo", &self.clk_btbb_fo())
            .field("clk_btmac_fo", &self.clk_btmac_fo())
            .field("clk_bt_apb_fo", &self.clk_bt_apb_fo())
            .field("clk_zbmac_fo", &self.clk_zbmac_fo())
            .field("clk_zbmac_apb_fo", &self.clk_zbmac_apb_fo())
            .field("clk_etm_fo", &self.clk_etm_fo())
            .field("clk_modem_sec_fo", &self.clk_modem_sec_fo())
            .field("clk_ble_timer_fo", &self.clk_ble_timer_fo())
            .field("clk_data_dump_fo", &self.clk_data_dump_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_wifibb_fo(&mut self) -> CLK_WIFIBB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_WIFIBB_FO_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_wifimac_fo(&mut self) -> CLK_WIFIMAC_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_WIFIMAC_FO_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_wifi_apb_fo(&mut self) -> CLK_WIFI_APB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_WIFI_APB_FO_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_fe_fo(&mut self) -> CLK_FE_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_FO_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_fe_apb_fo(&mut self) -> CLK_FE_APB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_APB_FO_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_btbb_fo(&mut self) -> CLK_BTBB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BTBB_FO_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_btmac_fo(&mut self) -> CLK_BTMAC_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BTMAC_FO_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_bt_apb_fo(&mut self) -> CLK_BT_APB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BT_APB_FO_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_zbmac_fo(&mut self) -> CLK_ZBMAC_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_ZBMAC_FO_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_zbmac_apb_fo(&mut self) -> CLK_ZBMAC_APB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_ZBMAC_APB_FO_W::new(self, 9)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_etm_fo(&mut self) -> CLK_ETM_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_ETM_FO_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_modem_sec_fo(&mut self) -> CLK_MODEM_SEC_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_FO_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_ble_timer_fo(&mut self) -> CLK_BLE_TIMER_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BLE_TIMER_FO_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_data_dump_fo(&mut self) -> CLK_DATA_DUMP_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_DATA_DUMP_FO_W::new(self, 31)
    }
}
#[doc = "CLK_CONF_FORCE_ON\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf_force_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf_force_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLK_CONF_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf_force_on::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf_force_on::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_CONF_FORCE_ON to value 0"]
impl crate::Resettable for CLK_CONF_FORCE_ON_SPEC {}
