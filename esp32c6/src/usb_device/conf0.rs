#[doc = "Register `CONF0` reader"]
pub struct R(crate::R<CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0` writer"]
pub struct W(crate::W<CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0_SPEC>;
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
impl From<crate::W<CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_PHY_SEL` reader - Select internal/external PHY"]
pub type USB_SERIAL_JTAG_PHY_SEL_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_PHY_SEL` writer - Select internal/external PHY"]
pub type USB_SERIAL_JTAG_PHY_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE` reader - Enable software control USB D+ D- exchange"]
pub type USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE` writer - Enable software control USB D+ D- exchange"]
pub type USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_EXCHG_PINS` reader - USB D+ D- exchange"]
pub type USB_SERIAL_JTAG_EXCHG_PINS_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_EXCHG_PINS` writer - USB D+ D- exchange"]
pub type USB_SERIAL_JTAG_EXCHG_PINS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_VREFH` reader - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type USB_SERIAL_JTAG_VREFH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SERIAL_JTAG_VREFH` writer - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type USB_SERIAL_JTAG_VREFH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `USB_SERIAL_JTAG_VREFL` reader - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type USB_SERIAL_JTAG_VREFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SERIAL_JTAG_VREFL` writer - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type USB_SERIAL_JTAG_VREFL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `USB_SERIAL_JTAG_VREF_OVERRIDE` reader - Enable software control input threshold"]
pub type USB_SERIAL_JTAG_VREF_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_VREF_OVERRIDE` writer - Enable software control input threshold"]
pub type USB_SERIAL_JTAG_VREF_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_PAD_PULL_OVERRIDE` reader - Enable software control USB D+ D- pullup pulldown"]
pub type USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_PAD_PULL_OVERRIDE` writer - Enable software control USB D+ D- pullup pulldown"]
pub type USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_DP_PULLUP` reader - Control USB D+ pull up."]
pub type USB_SERIAL_JTAG_DP_PULLUP_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_DP_PULLUP` writer - Control USB D+ pull up."]
pub type USB_SERIAL_JTAG_DP_PULLUP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_DP_PULLDOWN` reader - Control USB D+ pull down."]
pub type USB_SERIAL_JTAG_DP_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_DP_PULLDOWN` writer - Control USB D+ pull down."]
pub type USB_SERIAL_JTAG_DP_PULLDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_DM_PULLUP` reader - Control USB D- pull up."]
pub type USB_SERIAL_JTAG_DM_PULLUP_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_DM_PULLUP` writer - Control USB D- pull up."]
pub type USB_SERIAL_JTAG_DM_PULLUP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_DM_PULLDOWN` reader - Control USB D- pull down."]
pub type USB_SERIAL_JTAG_DM_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_DM_PULLDOWN` writer - Control USB D- pull down."]
pub type USB_SERIAL_JTAG_DM_PULLDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_PULLUP_VALUE` reader - Control pull up value."]
pub type USB_SERIAL_JTAG_PULLUP_VALUE_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_PULLUP_VALUE` writer - Control pull up value."]
pub type USB_SERIAL_JTAG_PULLUP_VALUE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_USB_PAD_ENABLE` reader - Enable USB pad function."]
pub type USB_SERIAL_JTAG_USB_PAD_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_USB_PAD_ENABLE` writer - Enable USB pad function."]
pub type USB_SERIAL_JTAG_USB_PAD_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN` reader - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
pub type USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN` writer - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
pub type USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Select internal/external PHY"]
    #[inline(always)]
    pub fn usb_serial_jtag_phy_sel(&self) -> USB_SERIAL_JTAG_PHY_SEL_R {
        USB_SERIAL_JTAG_PHY_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable software control USB D+ D- exchange"]
    #[inline(always)]
    pub fn usb_serial_jtag_exchg_pins_override(&self) -> USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_R {
        USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB D+ D- exchange"]
    #[inline(always)]
    pub fn usb_serial_jtag_exchg_pins(&self) -> USB_SERIAL_JTAG_EXCHG_PINS_R {
        USB_SERIAL_JTAG_EXCHG_PINS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    pub fn usb_serial_jtag_vrefh(&self) -> USB_SERIAL_JTAG_VREFH_R {
        USB_SERIAL_JTAG_VREFH_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    pub fn usb_serial_jtag_vrefl(&self) -> USB_SERIAL_JTAG_VREFL_R {
        USB_SERIAL_JTAG_VREFL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Enable software control input threshold"]
    #[inline(always)]
    pub fn usb_serial_jtag_vref_override(&self) -> USB_SERIAL_JTAG_VREF_OVERRIDE_R {
        USB_SERIAL_JTAG_VREF_OVERRIDE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable software control USB D+ D- pullup pulldown"]
    #[inline(always)]
    pub fn usb_serial_jtag_pad_pull_override(&self) -> USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_R {
        USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Control USB D+ pull up."]
    #[inline(always)]
    pub fn usb_serial_jtag_dp_pullup(&self) -> USB_SERIAL_JTAG_DP_PULLUP_R {
        USB_SERIAL_JTAG_DP_PULLUP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Control USB D+ pull down."]
    #[inline(always)]
    pub fn usb_serial_jtag_dp_pulldown(&self) -> USB_SERIAL_JTAG_DP_PULLDOWN_R {
        USB_SERIAL_JTAG_DP_PULLDOWN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control USB D- pull up."]
    #[inline(always)]
    pub fn usb_serial_jtag_dm_pullup(&self) -> USB_SERIAL_JTAG_DM_PULLUP_R {
        USB_SERIAL_JTAG_DM_PULLUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Control USB D- pull down."]
    #[inline(always)]
    pub fn usb_serial_jtag_dm_pulldown(&self) -> USB_SERIAL_JTAG_DM_PULLDOWN_R {
        USB_SERIAL_JTAG_DM_PULLDOWN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Control pull up value."]
    #[inline(always)]
    pub fn usb_serial_jtag_pullup_value(&self) -> USB_SERIAL_JTAG_PULLUP_VALUE_R {
        USB_SERIAL_JTAG_PULLUP_VALUE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable USB pad function."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_pad_enable(&self) -> USB_SERIAL_JTAG_USB_PAD_ENABLE_R {
        USB_SERIAL_JTAG_USB_PAD_ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_jtag_bridge_en(&self) -> USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_R {
        USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select internal/external PHY"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_phy_sel(&mut self) -> USB_SERIAL_JTAG_PHY_SEL_W<0> {
        USB_SERIAL_JTAG_PHY_SEL_W::new(self)
    }
    #[doc = "Bit 1 - Enable software control USB D+ D- exchange"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_exchg_pins_override(
        &mut self,
    ) -> USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_W<1> {
        USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 2 - USB D+ D- exchange"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_exchg_pins(&mut self) -> USB_SERIAL_JTAG_EXCHG_PINS_W<2> {
        USB_SERIAL_JTAG_EXCHG_PINS_W::new(self)
    }
    #[doc = "Bits 3:4 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_vrefh(&mut self) -> USB_SERIAL_JTAG_VREFH_W<3> {
        USB_SERIAL_JTAG_VREFH_W::new(self)
    }
    #[doc = "Bits 5:6 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_vrefl(&mut self) -> USB_SERIAL_JTAG_VREFL_W<5> {
        USB_SERIAL_JTAG_VREFL_W::new(self)
    }
    #[doc = "Bit 7 - Enable software control input threshold"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_vref_override(&mut self) -> USB_SERIAL_JTAG_VREF_OVERRIDE_W<7> {
        USB_SERIAL_JTAG_VREF_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 8 - Enable software control USB D+ D- pullup pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_pad_pull_override(&mut self) -> USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_W<8> {
        USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 9 - Control USB D+ pull up."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dp_pullup(&mut self) -> USB_SERIAL_JTAG_DP_PULLUP_W<9> {
        USB_SERIAL_JTAG_DP_PULLUP_W::new(self)
    }
    #[doc = "Bit 10 - Control USB D+ pull down."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dp_pulldown(&mut self) -> USB_SERIAL_JTAG_DP_PULLDOWN_W<10> {
        USB_SERIAL_JTAG_DP_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 11 - Control USB D- pull up."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dm_pullup(&mut self) -> USB_SERIAL_JTAG_DM_PULLUP_W<11> {
        USB_SERIAL_JTAG_DM_PULLUP_W::new(self)
    }
    #[doc = "Bit 12 - Control USB D- pull down."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dm_pulldown(&mut self) -> USB_SERIAL_JTAG_DM_PULLDOWN_W<12> {
        USB_SERIAL_JTAG_DM_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 13 - Control pull up value."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_pullup_value(&mut self) -> USB_SERIAL_JTAG_PULLUP_VALUE_W<13> {
        USB_SERIAL_JTAG_PULLUP_VALUE_W::new(self)
    }
    #[doc = "Bit 14 - Enable USB pad function."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_usb_pad_enable(&mut self) -> USB_SERIAL_JTAG_USB_PAD_ENABLE_W<14> {
        USB_SERIAL_JTAG_USB_PAD_ENABLE_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_usb_jtag_bridge_en(
        &mut self,
    ) -> USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_W<15> {
        USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY hardware configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0](index.html) module"]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0::R](R) reader structure"]
impl crate::Readable for CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0::W](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0x4200"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x4200;
}
