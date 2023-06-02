#[doc = "Register `CLK_CONF1` reader"]
pub struct R(crate::R<CLK_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF1` writer"]
pub struct W(crate::W<CLK_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF1_SPEC>;
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
impl From<crate::W<CLK_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_WIFIBB_22M_EN` reader - "]
pub type CLK_WIFIBB_22M_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_22M_EN` writer - "]
pub type CLK_WIFIBB_22M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIBB_40M_EN` reader - "]
pub type CLK_WIFIBB_40M_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_40M_EN` writer - "]
pub type CLK_WIFIBB_40M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIBB_44M_EN` reader - "]
pub type CLK_WIFIBB_44M_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_44M_EN` writer - "]
pub type CLK_WIFIBB_44M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIBB_80M_EN` reader - "]
pub type CLK_WIFIBB_80M_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_80M_EN` writer - "]
pub type CLK_WIFIBB_80M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIBB_40X_EN` reader - "]
pub type CLK_WIFIBB_40X_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_40X_EN` writer - "]
pub type CLK_WIFIBB_40X_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIBB_80X_EN` reader - "]
pub type CLK_WIFIBB_80X_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_80X_EN` writer - "]
pub type CLK_WIFIBB_80X_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIBB_40X1_EN` reader - "]
pub type CLK_WIFIBB_40X1_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_40X1_EN` writer - "]
pub type CLK_WIFIBB_40X1_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIBB_80X1_EN` reader - "]
pub type CLK_WIFIBB_80X1_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_80X1_EN` writer - "]
pub type CLK_WIFIBB_80X1_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIBB_160X1_EN` reader - "]
pub type CLK_WIFIBB_160X1_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_160X1_EN` writer - "]
pub type CLK_WIFIBB_160X1_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIMAC_EN` reader - "]
pub type CLK_WIFIMAC_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIMAC_EN` writer - "]
pub type CLK_WIFIMAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFI_APB_EN` reader - "]
pub type CLK_WIFI_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFI_APB_EN` writer - "]
pub type CLK_WIFI_APB_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_20M_EN` reader - "]
pub type CLK_FE_20M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_20M_EN` writer - "]
pub type CLK_FE_20M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_40M_EN` reader - "]
pub type CLK_FE_40M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_40M_EN` writer - "]
pub type CLK_FE_40M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_80M_EN` reader - "]
pub type CLK_FE_80M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_80M_EN` writer - "]
pub type CLK_FE_80M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_160M_EN` reader - "]
pub type CLK_FE_160M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_160M_EN` writer - "]
pub type CLK_FE_160M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_CAL_160M_EN` reader - "]
pub type CLK_FE_CAL_160M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_CAL_160M_EN` writer - "]
pub type CLK_FE_CAL_160M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_APB_EN` reader - "]
pub type CLK_FE_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_APB_EN` writer - "]
pub type CLK_FE_APB_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_BT_APB_EN` reader - "]
pub type CLK_BT_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_BT_APB_EN` writer - "]
pub type CLK_BT_APB_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_BT_EN` reader - "]
pub type CLK_BT_EN_R = crate::BitReader;
#[doc = "Field `CLK_BT_EN` writer - "]
pub type CLK_BT_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_WIFIBB_480M_EN` reader - "]
pub type CLK_WIFIBB_480M_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIBB_480M_EN` writer - "]
pub type CLK_WIFIBB_480M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_480M_EN` reader - "]
pub type CLK_FE_480M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_480M_EN` writer - "]
pub type CLK_FE_480M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_ANAMODE_40M_EN` reader - "]
pub type CLK_FE_ANAMODE_40M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_ANAMODE_40M_EN` writer - "]
pub type CLK_FE_ANAMODE_40M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_ANAMODE_80M_EN` reader - "]
pub type CLK_FE_ANAMODE_80M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_ANAMODE_80M_EN` writer - "]
pub type CLK_FE_ANAMODE_80M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_ANAMODE_160M_EN` reader - "]
pub type CLK_FE_ANAMODE_160M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_ANAMODE_160M_EN` writer - "]
pub type CLK_FE_ANAMODE_160M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_wifibb_22m_en(&self) -> CLK_WIFIBB_22M_EN_R {
        CLK_WIFIBB_22M_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_wifibb_40m_en(&self) -> CLK_WIFIBB_40M_EN_R {
        CLK_WIFIBB_40M_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_wifibb_44m_en(&self) -> CLK_WIFIBB_44M_EN_R {
        CLK_WIFIBB_44M_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_wifibb_80m_en(&self) -> CLK_WIFIBB_80M_EN_R {
        CLK_WIFIBB_80M_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_wifibb_40x_en(&self) -> CLK_WIFIBB_40X_EN_R {
        CLK_WIFIBB_40X_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_wifibb_80x_en(&self) -> CLK_WIFIBB_80X_EN_R {
        CLK_WIFIBB_80X_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_wifibb_40x1_en(&self) -> CLK_WIFIBB_40X1_EN_R {
        CLK_WIFIBB_40X1_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_wifibb_80x1_en(&self) -> CLK_WIFIBB_80X1_EN_R {
        CLK_WIFIBB_80X1_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_wifibb_160x1_en(&self) -> CLK_WIFIBB_160X1_EN_R {
        CLK_WIFIBB_160X1_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_wifimac_en(&self) -> CLK_WIFIMAC_EN_R {
        CLK_WIFIMAC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_wifi_apb_en(&self) -> CLK_WIFI_APB_EN_R {
        CLK_WIFI_APB_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_fe_20m_en(&self) -> CLK_FE_20M_EN_R {
        CLK_FE_20M_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_fe_40m_en(&self) -> CLK_FE_40M_EN_R {
        CLK_FE_40M_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_fe_80m_en(&self) -> CLK_FE_80M_EN_R {
        CLK_FE_80M_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_fe_160m_en(&self) -> CLK_FE_160M_EN_R {
        CLK_FE_160M_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_fe_cal_160m_en(&self) -> CLK_FE_CAL_160M_EN_R {
        CLK_FE_CAL_160M_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_fe_apb_en(&self) -> CLK_FE_APB_EN_R {
        CLK_FE_APB_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_bt_apb_en(&self) -> CLK_BT_APB_EN_R {
        CLK_BT_APB_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_bt_en(&self) -> CLK_BT_EN_R {
        CLK_BT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clk_wifibb_480m_en(&self) -> CLK_WIFIBB_480M_EN_R {
        CLK_WIFIBB_480M_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_fe_480m_en(&self) -> CLK_FE_480M_EN_R {
        CLK_FE_480M_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_fe_anamode_40m_en(&self) -> CLK_FE_ANAMODE_40M_EN_R {
        CLK_FE_ANAMODE_40M_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_fe_anamode_80m_en(&self) -> CLK_FE_ANAMODE_80M_EN_R {
        CLK_FE_ANAMODE_80M_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_fe_anamode_160m_en(&self) -> CLK_FE_ANAMODE_160M_EN_R {
        CLK_FE_ANAMODE_160M_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF1")
            .field(
                "clk_wifibb_22m_en",
                &format_args!("{}", self.clk_wifibb_22m_en().bit()),
            )
            .field(
                "clk_wifibb_40m_en",
                &format_args!("{}", self.clk_wifibb_40m_en().bit()),
            )
            .field(
                "clk_wifibb_44m_en",
                &format_args!("{}", self.clk_wifibb_44m_en().bit()),
            )
            .field(
                "clk_wifibb_80m_en",
                &format_args!("{}", self.clk_wifibb_80m_en().bit()),
            )
            .field(
                "clk_wifibb_40x_en",
                &format_args!("{}", self.clk_wifibb_40x_en().bit()),
            )
            .field(
                "clk_wifibb_80x_en",
                &format_args!("{}", self.clk_wifibb_80x_en().bit()),
            )
            .field(
                "clk_wifibb_40x1_en",
                &format_args!("{}", self.clk_wifibb_40x1_en().bit()),
            )
            .field(
                "clk_wifibb_80x1_en",
                &format_args!("{}", self.clk_wifibb_80x1_en().bit()),
            )
            .field(
                "clk_wifibb_160x1_en",
                &format_args!("{}", self.clk_wifibb_160x1_en().bit()),
            )
            .field(
                "clk_wifimac_en",
                &format_args!("{}", self.clk_wifimac_en().bit()),
            )
            .field(
                "clk_wifi_apb_en",
                &format_args!("{}", self.clk_wifi_apb_en().bit()),
            )
            .field(
                "clk_fe_20m_en",
                &format_args!("{}", self.clk_fe_20m_en().bit()),
            )
            .field(
                "clk_fe_40m_en",
                &format_args!("{}", self.clk_fe_40m_en().bit()),
            )
            .field(
                "clk_fe_80m_en",
                &format_args!("{}", self.clk_fe_80m_en().bit()),
            )
            .field(
                "clk_fe_160m_en",
                &format_args!("{}", self.clk_fe_160m_en().bit()),
            )
            .field(
                "clk_fe_cal_160m_en",
                &format_args!("{}", self.clk_fe_cal_160m_en().bit()),
            )
            .field(
                "clk_fe_apb_en",
                &format_args!("{}", self.clk_fe_apb_en().bit()),
            )
            .field(
                "clk_bt_apb_en",
                &format_args!("{}", self.clk_bt_apb_en().bit()),
            )
            .field("clk_bt_en", &format_args!("{}", self.clk_bt_en().bit()))
            .field(
                "clk_wifibb_480m_en",
                &format_args!("{}", self.clk_wifibb_480m_en().bit()),
            )
            .field(
                "clk_fe_480m_en",
                &format_args!("{}", self.clk_fe_480m_en().bit()),
            )
            .field(
                "clk_fe_anamode_40m_en",
                &format_args!("{}", self.clk_fe_anamode_40m_en().bit()),
            )
            .field(
                "clk_fe_anamode_80m_en",
                &format_args!("{}", self.clk_fe_anamode_80m_en().bit()),
            )
            .field(
                "clk_fe_anamode_160m_en",
                &format_args!("{}", self.clk_fe_anamode_160m_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_22m_en(&mut self) -> CLK_WIFIBB_22M_EN_W<0> {
        CLK_WIFIBB_22M_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_40m_en(&mut self) -> CLK_WIFIBB_40M_EN_W<1> {
        CLK_WIFIBB_40M_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_44m_en(&mut self) -> CLK_WIFIBB_44M_EN_W<2> {
        CLK_WIFIBB_44M_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_80m_en(&mut self) -> CLK_WIFIBB_80M_EN_W<3> {
        CLK_WIFIBB_80M_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_40x_en(&mut self) -> CLK_WIFIBB_40X_EN_W<4> {
        CLK_WIFIBB_40X_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_80x_en(&mut self) -> CLK_WIFIBB_80X_EN_W<5> {
        CLK_WIFIBB_80X_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_40x1_en(&mut self) -> CLK_WIFIBB_40X1_EN_W<6> {
        CLK_WIFIBB_40X1_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_80x1_en(&mut self) -> CLK_WIFIBB_80X1_EN_W<7> {
        CLK_WIFIBB_80X1_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_160x1_en(&mut self) -> CLK_WIFIBB_160X1_EN_W<8> {
        CLK_WIFIBB_160X1_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifimac_en(&mut self) -> CLK_WIFIMAC_EN_W<9> {
        CLK_WIFIMAC_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifi_apb_en(&mut self) -> CLK_WIFI_APB_EN_W<10> {
        CLK_WIFI_APB_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_20m_en(&mut self) -> CLK_FE_20M_EN_W<11> {
        CLK_FE_20M_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_40m_en(&mut self) -> CLK_FE_40M_EN_W<12> {
        CLK_FE_40M_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_80m_en(&mut self) -> CLK_FE_80M_EN_W<13> {
        CLK_FE_80M_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_160m_en(&mut self) -> CLK_FE_160M_EN_W<14> {
        CLK_FE_160M_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_cal_160m_en(&mut self) -> CLK_FE_CAL_160M_EN_W<15> {
        CLK_FE_CAL_160M_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_apb_en(&mut self) -> CLK_FE_APB_EN_W<16> {
        CLK_FE_APB_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn clk_bt_apb_en(&mut self) -> CLK_BT_APB_EN_W<17> {
        CLK_BT_APB_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn clk_bt_en(&mut self) -> CLK_BT_EN_W<18> {
        CLK_BT_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifibb_480m_en(&mut self) -> CLK_WIFIBB_480M_EN_W<19> {
        CLK_WIFIBB_480M_EN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_480m_en(&mut self) -> CLK_FE_480M_EN_W<20> {
        CLK_FE_480M_EN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_anamode_40m_en(&mut self) -> CLK_FE_ANAMODE_40M_EN_W<21> {
        CLK_FE_ANAMODE_40M_EN_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_anamode_80m_en(&mut self) -> CLK_FE_ANAMODE_80M_EN_W<22> {
        CLK_FE_ANAMODE_80M_EN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_anamode_160m_en(&mut self) -> CLK_FE_ANAMODE_160M_EN_W<23> {
        CLK_FE_ANAMODE_160M_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf1](index.html) module"]
pub struct CLK_CONF1_SPEC;
impl crate::RegisterSpec for CLK_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf1::R](R) reader structure"]
impl crate::Readable for CLK_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf1::W](W) writer structure"]
impl crate::Writable for CLK_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF1 to value 0"]
impl crate::Resettable for CLK_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
