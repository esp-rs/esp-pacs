#[doc = "Register `USB_CONF` reader"]
pub struct R(crate::R<USB_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CONF` writer"]
pub struct W(crate::W<USB_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CONF_SPEC>;
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
impl From<crate::W<USB_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VREFH` reader - reg_usb_vrefh"]
pub type USB_VREFH_R = crate::FieldReader;
#[doc = "Field `USB_VREFH` writer - reg_usb_vrefh"]
pub type USB_VREFH_W<'a, const O: u8> = crate::FieldWriter<'a, USB_CONF_SPEC, 2, O>;
#[doc = "Field `USB_VREFL` reader - reg_usb_vrefl"]
pub type USB_VREFL_R = crate::FieldReader;
#[doc = "Field `USB_VREFL` writer - reg_usb_vrefl"]
pub type USB_VREFL_W<'a, const O: u8> = crate::FieldWriter<'a, USB_CONF_SPEC, 2, O>;
#[doc = "Field `USB_VREF_OVERRIDE` reader - reg_usb_vref_override"]
pub type USB_VREF_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_VREF_OVERRIDE` writer - reg_usb_vref_override"]
pub type USB_VREF_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_PAD_PULL_OVERRIDE` reader - reg_usb_pad_pull_override"]
pub type USB_PAD_PULL_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_PAD_PULL_OVERRIDE` writer - reg_usb_pad_pull_override"]
pub type USB_PAD_PULL_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_DP_PULLUP` reader - reg_usb_dp_pullup"]
pub type USB_DP_PULLUP_R = crate::BitReader;
#[doc = "Field `USB_DP_PULLUP` writer - reg_usb_dp_pullup"]
pub type USB_DP_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_DP_PULLDOWN` reader - reg_usb_dp_pulldown"]
pub type USB_DP_PULLDOWN_R = crate::BitReader;
#[doc = "Field `USB_DP_PULLDOWN` writer - reg_usb_dp_pulldown"]
pub type USB_DP_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_DM_PULLUP` reader - reg_usb_dm_pullup"]
pub type USB_DM_PULLUP_R = crate::BitReader;
#[doc = "Field `USB_DM_PULLUP` writer - reg_usb_dm_pullup"]
pub type USB_DM_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_DM_PULLDOWN` reader - reg_usb_dm_pulldown"]
pub type USB_DM_PULLDOWN_R = crate::BitReader;
#[doc = "Field `USB_DM_PULLDOWN` writer - reg_usb_dm_pulldown"]
pub type USB_DM_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_PULLUP_VALUE` reader - reg_usb_pullup_value"]
pub type USB_PULLUP_VALUE_R = crate::BitReader;
#[doc = "Field `USB_PULLUP_VALUE` writer - reg_usb_pullup_value"]
pub type USB_PULLUP_VALUE_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_PAD_ENABLE_OVERRIDE` reader - reg_usb_pad_enable_override"]
pub type USB_PAD_ENABLE_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_PAD_ENABLE_OVERRIDE` writer - reg_usb_pad_enable_override"]
pub type USB_PAD_ENABLE_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_PAD_ENABLE` reader - reg_usb_pad_enable"]
pub type USB_PAD_ENABLE_R = crate::BitReader;
#[doc = "Field `USB_PAD_ENABLE` writer - reg_usb_pad_enable"]
pub type USB_PAD_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_TXM` reader - reg_usb_txm"]
pub type USB_TXM_R = crate::BitReader;
#[doc = "Field `USB_TXM` writer - reg_usb_txm"]
pub type USB_TXM_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_TXP` reader - reg_usb_txp"]
pub type USB_TXP_R = crate::BitReader;
#[doc = "Field `USB_TXP` writer - reg_usb_txp"]
pub type USB_TXP_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_TX_EN` reader - reg_usb_tx_en"]
pub type USB_TX_EN_R = crate::BitReader;
#[doc = "Field `USB_TX_EN` writer - reg_usb_tx_en"]
pub type USB_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_TX_EN_OVERRIDE` reader - reg_usb_tx_en_override"]
pub type USB_TX_EN_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_TX_EN_OVERRIDE` writer - reg_usb_tx_en_override"]
pub type USB_TX_EN_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `USB_RESET_DISABLE` reader - reg_usb_reset_disable"]
pub type USB_RESET_DISABLE_R = crate::BitReader;
#[doc = "Field `USB_RESET_DISABLE` writer - reg_usb_reset_disable"]
pub type USB_RESET_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - reg_io_mux_reset_disable"]
pub type IO_MUX_RESET_DISABLE_R = crate::BitReader;
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - reg_io_mux_reset_disable"]
pub type IO_MUX_RESET_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `SW_USB_PHY_SEL` reader - reg_sw_usb_phy_sel"]
pub type SW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_USB_PHY_SEL` writer - reg_sw_usb_phy_sel"]
pub type SW_USB_PHY_SEL_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
#[doc = "Field `SW_HW_USB_PHY_SEL` reader - reg_sw_hw_usb_phy_sel"]
pub type SW_HW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_HW_USB_PHY_SEL` writer - reg_sw_hw_usb_phy_sel"]
pub type SW_HW_USB_PHY_SEL_W<'a, const O: u8> = crate::BitWriter<'a, USB_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - reg_usb_vrefh"]
    #[inline(always)]
    pub fn usb_vrefh(&self) -> USB_VREFH_R {
        USB_VREFH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reg_usb_vrefl"]
    #[inline(always)]
    pub fn usb_vrefl(&self) -> USB_VREFL_R {
        USB_VREFL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - reg_usb_vref_override"]
    #[inline(always)]
    pub fn usb_vref_override(&self) -> USB_VREF_OVERRIDE_R {
        USB_VREF_OVERRIDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_usb_pad_pull_override"]
    #[inline(always)]
    pub fn usb_pad_pull_override(&self) -> USB_PAD_PULL_OVERRIDE_R {
        USB_PAD_PULL_OVERRIDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_usb_dp_pullup"]
    #[inline(always)]
    pub fn usb_dp_pullup(&self) -> USB_DP_PULLUP_R {
        USB_DP_PULLUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_usb_dp_pulldown"]
    #[inline(always)]
    pub fn usb_dp_pulldown(&self) -> USB_DP_PULLDOWN_R {
        USB_DP_PULLDOWN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_usb_dm_pullup"]
    #[inline(always)]
    pub fn usb_dm_pullup(&self) -> USB_DM_PULLUP_R {
        USB_DM_PULLUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_usb_dm_pulldown"]
    #[inline(always)]
    pub fn usb_dm_pulldown(&self) -> USB_DM_PULLDOWN_R {
        USB_DM_PULLDOWN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_usb_pullup_value"]
    #[inline(always)]
    pub fn usb_pullup_value(&self) -> USB_PULLUP_VALUE_R {
        USB_PULLUP_VALUE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_usb_pad_enable_override"]
    #[inline(always)]
    pub fn usb_pad_enable_override(&self) -> USB_PAD_ENABLE_OVERRIDE_R {
        USB_PAD_ENABLE_OVERRIDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_usb_pad_enable"]
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> USB_PAD_ENABLE_R {
        USB_PAD_ENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_usb_txm"]
    #[inline(always)]
    pub fn usb_txm(&self) -> USB_TXM_R {
        USB_TXM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_usb_txp"]
    #[inline(always)]
    pub fn usb_txp(&self) -> USB_TXP_R {
        USB_TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - reg_usb_tx_en"]
    #[inline(always)]
    pub fn usb_tx_en(&self) -> USB_TX_EN_R {
        USB_TX_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - reg_usb_tx_en_override"]
    #[inline(always)]
    pub fn usb_tx_en_override(&self) -> USB_TX_EN_OVERRIDE_R {
        USB_TX_EN_OVERRIDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reg_usb_reset_disable"]
    #[inline(always)]
    pub fn usb_reset_disable(&self) -> USB_RESET_DISABLE_R {
        USB_RESET_DISABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - reg_io_mux_reset_disable"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reg_sw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&self) -> SW_USB_PHY_SEL_R {
        SW_USB_PHY_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - reg_sw_hw_usb_phy_sel"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&self) -> SW_HW_USB_PHY_SEL_R {
        SW_HW_USB_PHY_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_CONF")
            .field("usb_vrefh", &format_args!("{}", self.usb_vrefh().bits()))
            .field("usb_vrefl", &format_args!("{}", self.usb_vrefl().bits()))
            .field(
                "usb_vref_override",
                &format_args!("{}", self.usb_vref_override().bit()),
            )
            .field(
                "usb_pad_pull_override",
                &format_args!("{}", self.usb_pad_pull_override().bit()),
            )
            .field(
                "usb_dp_pullup",
                &format_args!("{}", self.usb_dp_pullup().bit()),
            )
            .field(
                "usb_dp_pulldown",
                &format_args!("{}", self.usb_dp_pulldown().bit()),
            )
            .field(
                "usb_dm_pullup",
                &format_args!("{}", self.usb_dm_pullup().bit()),
            )
            .field(
                "usb_dm_pulldown",
                &format_args!("{}", self.usb_dm_pulldown().bit()),
            )
            .field(
                "usb_pullup_value",
                &format_args!("{}", self.usb_pullup_value().bit()),
            )
            .field(
                "usb_pad_enable_override",
                &format_args!("{}", self.usb_pad_enable_override().bit()),
            )
            .field(
                "usb_pad_enable",
                &format_args!("{}", self.usb_pad_enable().bit()),
            )
            .field("usb_txm", &format_args!("{}", self.usb_txm().bit()))
            .field("usb_txp", &format_args!("{}", self.usb_txp().bit()))
            .field("usb_tx_en", &format_args!("{}", self.usb_tx_en().bit()))
            .field(
                "usb_tx_en_override",
                &format_args!("{}", self.usb_tx_en_override().bit()),
            )
            .field(
                "usb_reset_disable",
                &format_args!("{}", self.usb_reset_disable().bit()),
            )
            .field(
                "io_mux_reset_disable",
                &format_args!("{}", self.io_mux_reset_disable().bit()),
            )
            .field(
                "sw_usb_phy_sel",
                &format_args!("{}", self.sw_usb_phy_sel().bit()),
            )
            .field(
                "sw_hw_usb_phy_sel",
                &format_args!("{}", self.sw_hw_usb_phy_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USB_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_usb_vrefh"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vrefh(&mut self) -> USB_VREFH_W<0> {
        USB_VREFH_W::new(self)
    }
    #[doc = "Bits 2:3 - reg_usb_vrefl"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vrefl(&mut self) -> USB_VREFL_W<2> {
        USB_VREFL_W::new(self)
    }
    #[doc = "Bit 4 - reg_usb_vref_override"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vref_override(&mut self) -> USB_VREF_OVERRIDE_W<4> {
        USB_VREF_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 5 - reg_usb_pad_pull_override"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pad_pull_override(&mut self) -> USB_PAD_PULL_OVERRIDE_W<5> {
        USB_PAD_PULL_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 6 - reg_usb_dp_pullup"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dp_pullup(&mut self) -> USB_DP_PULLUP_W<6> {
        USB_DP_PULLUP_W::new(self)
    }
    #[doc = "Bit 7 - reg_usb_dp_pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dp_pulldown(&mut self) -> USB_DP_PULLDOWN_W<7> {
        USB_DP_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 8 - reg_usb_dm_pullup"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dm_pullup(&mut self) -> USB_DM_PULLUP_W<8> {
        USB_DM_PULLUP_W::new(self)
    }
    #[doc = "Bit 9 - reg_usb_dm_pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dm_pulldown(&mut self) -> USB_DM_PULLDOWN_W<9> {
        USB_DM_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 10 - reg_usb_pullup_value"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pullup_value(&mut self) -> USB_PULLUP_VALUE_W<10> {
        USB_PULLUP_VALUE_W::new(self)
    }
    #[doc = "Bit 11 - reg_usb_pad_enable_override"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pad_enable_override(&mut self) -> USB_PAD_ENABLE_OVERRIDE_W<11> {
        USB_PAD_ENABLE_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 12 - reg_usb_pad_enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pad_enable(&mut self) -> USB_PAD_ENABLE_W<12> {
        USB_PAD_ENABLE_W::new(self)
    }
    #[doc = "Bit 13 - reg_usb_txm"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txm(&mut self) -> USB_TXM_W<13> {
        USB_TXM_W::new(self)
    }
    #[doc = "Bit 14 - reg_usb_txp"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txp(&mut self) -> USB_TXP_W<14> {
        USB_TXP_W::new(self)
    }
    #[doc = "Bit 15 - reg_usb_tx_en"]
    #[inline(always)]
    #[must_use]
    pub fn usb_tx_en(&mut self) -> USB_TX_EN_W<15> {
        USB_TX_EN_W::new(self)
    }
    #[doc = "Bit 16 - reg_usb_tx_en_override"]
    #[inline(always)]
    #[must_use]
    pub fn usb_tx_en_override(&mut self) -> USB_TX_EN_OVERRIDE_W<16> {
        USB_TX_EN_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 17 - reg_usb_reset_disable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_reset_disable(&mut self) -> USB_RESET_DISABLE_W<17> {
        USB_RESET_DISABLE_W::new(self)
    }
    #[doc = "Bit 18 - reg_io_mux_reset_disable"]
    #[inline(always)]
    #[must_use]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W<18> {
        IO_MUX_RESET_DISABLE_W::new(self)
    }
    #[doc = "Bit 19 - reg_sw_usb_phy_sel"]
    #[inline(always)]
    #[must_use]
    pub fn sw_usb_phy_sel(&mut self) -> SW_USB_PHY_SEL_W<19> {
        SW_USB_PHY_SEL_W::new(self)
    }
    #[doc = "Bit 20 - reg_sw_hw_usb_phy_sel"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hw_usb_phy_sel(&mut self) -> SW_HW_USB_PHY_SEL_W<20> {
        SW_HW_USB_PHY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb configure\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_conf](index.html) module"]
pub struct USB_CONF_SPEC;
impl crate::RegisterSpec for USB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_conf::R](R) reader structure"]
impl crate::Readable for USB_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_conf::W](W) writer structure"]
impl crate::Writable for USB_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_CONF to value 0"]
impl crate::Resettable for USB_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
