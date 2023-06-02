#[doc = "Register `MODEM_RST_CONF` reader"]
pub struct R(crate::R<MODEM_RST_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODEM_RST_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODEM_RST_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODEM_RST_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODEM_RST_CONF` writer"]
pub struct W(crate::W<MODEM_RST_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODEM_RST_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MODEM_RST_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODEM_RST_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST_WIFIBB` reader - "]
pub type RST_WIFIBB_R = crate::BitReader;
#[doc = "Field `RST_WIFIBB` writer - "]
pub type RST_WIFIBB_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_WIFIMAC` reader - "]
pub type RST_WIFIMAC_R = crate::BitReader;
#[doc = "Field `RST_WIFIMAC` writer - "]
pub type RST_WIFIMAC_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_FE` reader - "]
pub type RST_FE_R = crate::BitReader;
#[doc = "Field `RST_FE` writer - "]
pub type RST_FE_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_BTMAC_APB` reader - "]
pub type RST_BTMAC_APB_R = crate::BitReader;
#[doc = "Field `RST_BTMAC_APB` writer - "]
pub type RST_BTMAC_APB_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_BTMAC` reader - "]
pub type RST_BTMAC_R = crate::BitReader;
#[doc = "Field `RST_BTMAC` writer - "]
pub type RST_BTMAC_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_BTBB_APB` reader - "]
pub type RST_BTBB_APB_R = crate::BitReader;
#[doc = "Field `RST_BTBB_APB` writer - "]
pub type RST_BTBB_APB_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_BTBB` reader - "]
pub type RST_BTBB_R = crate::BitReader;
#[doc = "Field `RST_BTBB` writer - "]
pub type RST_BTBB_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_ETM` reader - "]
pub type RST_ETM_R = crate::BitReader;
#[doc = "Field `RST_ETM` writer - "]
pub type RST_ETM_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_ZBMAC` reader - "]
pub type RST_ZBMAC_R = crate::BitReader;
#[doc = "Field `RST_ZBMAC` writer - "]
pub type RST_ZBMAC_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_MODEM_ECB` reader - "]
pub type RST_MODEM_ECB_R = crate::BitReader;
#[doc = "Field `RST_MODEM_ECB` writer - "]
pub type RST_MODEM_ECB_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_MODEM_CCM` reader - "]
pub type RST_MODEM_CCM_R = crate::BitReader;
#[doc = "Field `RST_MODEM_CCM` writer - "]
pub type RST_MODEM_CCM_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_MODEM_BAH` reader - "]
pub type RST_MODEM_BAH_R = crate::BitReader;
#[doc = "Field `RST_MODEM_BAH` writer - "]
pub type RST_MODEM_BAH_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_MODEM_SEC` reader - "]
pub type RST_MODEM_SEC_R = crate::BitReader;
#[doc = "Field `RST_MODEM_SEC` writer - "]
pub type RST_MODEM_SEC_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_BLE_TIMER` reader - "]
pub type RST_BLE_TIMER_R = crate::BitReader;
#[doc = "Field `RST_BLE_TIMER` writer - "]
pub type RST_BLE_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
#[doc = "Field `RST_DATA_DUMP` reader - "]
pub type RST_DATA_DUMP_R = crate::BitReader;
#[doc = "Field `RST_DATA_DUMP` writer - "]
pub type RST_DATA_DUMP_W<'a, const O: u8> = crate::BitWriter<'a, MODEM_RST_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rst_wifibb(&self) -> RST_WIFIBB_R {
        RST_WIFIBB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rst_wifimac(&self) -> RST_WIFIMAC_R {
        RST_WIFIMAC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rst_fe(&self) -> RST_FE_R {
        RST_FE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rst_btmac_apb(&self) -> RST_BTMAC_APB_R {
        RST_BTMAC_APB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rst_btmac(&self) -> RST_BTMAC_R {
        RST_BTMAC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rst_btbb_apb(&self) -> RST_BTBB_APB_R {
        RST_BTBB_APB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rst_btbb(&self) -> RST_BTBB_R {
        RST_BTBB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rst_etm(&self) -> RST_ETM_R {
        RST_ETM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rst_zbmac(&self) -> RST_ZBMAC_R {
        RST_ZBMAC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rst_modem_ecb(&self) -> RST_MODEM_ECB_R {
        RST_MODEM_ECB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rst_modem_ccm(&self) -> RST_MODEM_CCM_R {
        RST_MODEM_CCM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rst_modem_bah(&self) -> RST_MODEM_BAH_R {
        RST_MODEM_BAH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rst_modem_sec(&self) -> RST_MODEM_SEC_R {
        RST_MODEM_SEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rst_ble_timer(&self) -> RST_BLE_TIMER_R {
        RST_BLE_TIMER_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rst_data_dump(&self) -> RST_DATA_DUMP_R {
        RST_DATA_DUMP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_RST_CONF")
            .field("rst_wifibb", &format_args!("{}", self.rst_wifibb().bit()))
            .field("rst_wifimac", &format_args!("{}", self.rst_wifimac().bit()))
            .field("rst_fe", &format_args!("{}", self.rst_fe().bit()))
            .field(
                "rst_btmac_apb",
                &format_args!("{}", self.rst_btmac_apb().bit()),
            )
            .field("rst_btmac", &format_args!("{}", self.rst_btmac().bit()))
            .field(
                "rst_btbb_apb",
                &format_args!("{}", self.rst_btbb_apb().bit()),
            )
            .field("rst_btbb", &format_args!("{}", self.rst_btbb().bit()))
            .field("rst_etm", &format_args!("{}", self.rst_etm().bit()))
            .field("rst_zbmac", &format_args!("{}", self.rst_zbmac().bit()))
            .field(
                "rst_modem_ecb",
                &format_args!("{}", self.rst_modem_ecb().bit()),
            )
            .field(
                "rst_modem_ccm",
                &format_args!("{}", self.rst_modem_ccm().bit()),
            )
            .field(
                "rst_modem_bah",
                &format_args!("{}", self.rst_modem_bah().bit()),
            )
            .field(
                "rst_modem_sec",
                &format_args!("{}", self.rst_modem_sec().bit()),
            )
            .field(
                "rst_ble_timer",
                &format_args!("{}", self.rst_ble_timer().bit()),
            )
            .field(
                "rst_data_dump",
                &format_args!("{}", self.rst_data_dump().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEM_RST_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rst_wifibb(&mut self) -> RST_WIFIBB_W<8> {
        RST_WIFIBB_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rst_wifimac(&mut self) -> RST_WIFIMAC_W<10> {
        RST_WIFIMAC_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn rst_fe(&mut self) -> RST_FE_W<14> {
        RST_FE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rst_btmac_apb(&mut self) -> RST_BTMAC_APB_W<15> {
        RST_BTMAC_APB_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rst_btmac(&mut self) -> RST_BTMAC_W<16> {
        RST_BTMAC_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rst_btbb_apb(&mut self) -> RST_BTBB_APB_W<17> {
        RST_BTBB_APB_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn rst_btbb(&mut self) -> RST_BTBB_W<18> {
        RST_BTBB_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn rst_etm(&mut self) -> RST_ETM_W<22> {
        RST_ETM_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn rst_zbmac(&mut self) -> RST_ZBMAC_W<24> {
        RST_ZBMAC_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rst_modem_ecb(&mut self) -> RST_MODEM_ECB_W<25> {
        RST_MODEM_ECB_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn rst_modem_ccm(&mut self) -> RST_MODEM_CCM_W<26> {
        RST_MODEM_CCM_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn rst_modem_bah(&mut self) -> RST_MODEM_BAH_W<27> {
        RST_MODEM_BAH_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rst_modem_sec(&mut self) -> RST_MODEM_SEC_W<29> {
        RST_MODEM_SEC_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ble_timer(&mut self) -> RST_BLE_TIMER_W<30> {
        RST_BLE_TIMER_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rst_data_dump(&mut self) -> RST_DATA_DUMP_W<31> {
        RST_DATA_DUMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_rst_conf](index.html) module"]
pub struct MODEM_RST_CONF_SPEC;
impl crate::RegisterSpec for MODEM_RST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modem_rst_conf::R](R) reader structure"]
impl crate::Readable for MODEM_RST_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modem_rst_conf::W](W) writer structure"]
impl crate::Writable for MODEM_RST_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEM_RST_CONF to value 0"]
impl crate::Resettable for MODEM_RST_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
