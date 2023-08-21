#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `PHY_SEL` reader - Select internal/external PHY"]
pub type PHY_SEL_R = crate::BitReader;
#[doc = "Field `PHY_SEL` writer - Select internal/external PHY"]
pub type PHY_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXCHG_PINS_OVERRIDE` reader - Enable software control USB D+ D- exchange"]
pub type EXCHG_PINS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `EXCHG_PINS_OVERRIDE` writer - Enable software control USB D+ D- exchange"]
pub type EXCHG_PINS_OVERRIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXCHG_PINS` reader - USB D+ D- exchange"]
pub type EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `EXCHG_PINS` writer - USB D+ D- exchange"]
pub type EXCHG_PINS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREFH` reader - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type VREFH_R = crate::FieldReader;
#[doc = "Field `VREFH` writer - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type VREFH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `VREFL` reader - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type VREFL_R = crate::FieldReader;
#[doc = "Field `VREFL` writer - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type VREFL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `VREF_OVERRIDE` reader - Enable software control input threshold"]
pub type VREF_OVERRIDE_R = crate::BitReader;
#[doc = "Field `VREF_OVERRIDE` writer - Enable software control input threshold"]
pub type VREF_OVERRIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAD_PULL_OVERRIDE` reader - Enable software control USB D+ D- pullup pulldown"]
pub type PAD_PULL_OVERRIDE_R = crate::BitReader;
#[doc = "Field `PAD_PULL_OVERRIDE` writer - Enable software control USB D+ D- pullup pulldown"]
pub type PAD_PULL_OVERRIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DP_PULLUP` reader - Control USB D+ pull up."]
pub type DP_PULLUP_R = crate::BitReader;
#[doc = "Field `DP_PULLUP` writer - Control USB D+ pull up."]
pub type DP_PULLUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DP_PULLDOWN` reader - Control USB D+ pull down."]
pub type DP_PULLDOWN_R = crate::BitReader;
#[doc = "Field `DP_PULLDOWN` writer - Control USB D+ pull down."]
pub type DP_PULLDOWN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DM_PULLUP` reader - Control USB D- pull up."]
pub type DM_PULLUP_R = crate::BitReader;
#[doc = "Field `DM_PULLUP` writer - Control USB D- pull up."]
pub type DM_PULLUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DM_PULLDOWN` reader - Control USB D- pull down."]
pub type DM_PULLDOWN_R = crate::BitReader;
#[doc = "Field `DM_PULLDOWN` writer - Control USB D- pull down."]
pub type DM_PULLDOWN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PULLUP_VALUE` reader - Control pull up value."]
pub type PULLUP_VALUE_R = crate::BitReader;
#[doc = "Field `PULLUP_VALUE` writer - Control pull up value."]
pub type PULLUP_VALUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_PAD_ENABLE` reader - Enable USB pad function."]
pub type USB_PAD_ENABLE_R = crate::BitReader;
#[doc = "Field `USB_PAD_ENABLE` writer - Enable USB pad function."]
pub type USB_PAD_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_JTAG_BRIDGE_EN` reader - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
pub type USB_JTAG_BRIDGE_EN_R = crate::BitReader;
#[doc = "Field `USB_JTAG_BRIDGE_EN` writer - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
pub type USB_JTAG_BRIDGE_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Select internal/external PHY"]
    #[inline(always)]
    pub fn phy_sel(&self) -> PHY_SEL_R {
        PHY_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable software control USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins_override(&self) -> EXCHG_PINS_OVERRIDE_R {
        EXCHG_PINS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins(&self) -> EXCHG_PINS_R {
        EXCHG_PINS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    pub fn vrefh(&self) -> VREFH_R {
        VREFH_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    pub fn vrefl(&self) -> VREFL_R {
        VREFL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Enable software control input threshold"]
    #[inline(always)]
    pub fn vref_override(&self) -> VREF_OVERRIDE_R {
        VREF_OVERRIDE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable software control USB D+ D- pullup pulldown"]
    #[inline(always)]
    pub fn pad_pull_override(&self) -> PAD_PULL_OVERRIDE_R {
        PAD_PULL_OVERRIDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Control USB D+ pull up."]
    #[inline(always)]
    pub fn dp_pullup(&self) -> DP_PULLUP_R {
        DP_PULLUP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Control USB D+ pull down."]
    #[inline(always)]
    pub fn dp_pulldown(&self) -> DP_PULLDOWN_R {
        DP_PULLDOWN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control USB D- pull up."]
    #[inline(always)]
    pub fn dm_pullup(&self) -> DM_PULLUP_R {
        DM_PULLUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Control USB D- pull down."]
    #[inline(always)]
    pub fn dm_pulldown(&self) -> DM_PULLDOWN_R {
        DM_PULLDOWN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Control pull up value."]
    #[inline(always)]
    pub fn pullup_value(&self) -> PULLUP_VALUE_R {
        PULLUP_VALUE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable USB pad function."]
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> USB_PAD_ENABLE_R {
        USB_PAD_ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
    #[inline(always)]
    pub fn usb_jtag_bridge_en(&self) -> USB_JTAG_BRIDGE_EN_R {
        USB_JTAG_BRIDGE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("phy_sel", &format_args!("{}", self.phy_sel().bit()))
            .field(
                "exchg_pins_override",
                &format_args!("{}", self.exchg_pins_override().bit()),
            )
            .field("exchg_pins", &format_args!("{}", self.exchg_pins().bit()))
            .field("vrefh", &format_args!("{}", self.vrefh().bits()))
            .field("vrefl", &format_args!("{}", self.vrefl().bits()))
            .field(
                "vref_override",
                &format_args!("{}", self.vref_override().bit()),
            )
            .field(
                "pad_pull_override",
                &format_args!("{}", self.pad_pull_override().bit()),
            )
            .field("dp_pullup", &format_args!("{}", self.dp_pullup().bit()))
            .field("dp_pulldown", &format_args!("{}", self.dp_pulldown().bit()))
            .field("dm_pullup", &format_args!("{}", self.dm_pullup().bit()))
            .field("dm_pulldown", &format_args!("{}", self.dm_pulldown().bit()))
            .field(
                "pullup_value",
                &format_args!("{}", self.pullup_value().bit()),
            )
            .field(
                "usb_pad_enable",
                &format_args!("{}", self.usb_pad_enable().bit()),
            )
            .field(
                "usb_jtag_bridge_en",
                &format_args!("{}", self.usb_jtag_bridge_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Select internal/external PHY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_sel(&mut self) -> PHY_SEL_W<CONF0_SPEC, 0> {
        PHY_SEL_W::new(self)
    }
    #[doc = "Bit 1 - Enable software control USB D+ D- exchange"]
    #[inline(always)]
    #[must_use]
    pub fn exchg_pins_override(&mut self) -> EXCHG_PINS_OVERRIDE_W<CONF0_SPEC, 1> {
        EXCHG_PINS_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 2 - USB D+ D- exchange"]
    #[inline(always)]
    #[must_use]
    pub fn exchg_pins(&mut self) -> EXCHG_PINS_W<CONF0_SPEC, 2> {
        EXCHG_PINS_W::new(self)
    }
    #[doc = "Bits 3:4 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    #[must_use]
    pub fn vrefh(&mut self) -> VREFH_W<CONF0_SPEC, 3> {
        VREFH_W::new(self)
    }
    #[doc = "Bits 5:6 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    #[must_use]
    pub fn vrefl(&mut self) -> VREFL_W<CONF0_SPEC, 5> {
        VREFL_W::new(self)
    }
    #[doc = "Bit 7 - Enable software control input threshold"]
    #[inline(always)]
    #[must_use]
    pub fn vref_override(&mut self) -> VREF_OVERRIDE_W<CONF0_SPEC, 7> {
        VREF_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 8 - Enable software control USB D+ D- pullup pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn pad_pull_override(&mut self) -> PAD_PULL_OVERRIDE_W<CONF0_SPEC, 8> {
        PAD_PULL_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 9 - Control USB D+ pull up."]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup(&mut self) -> DP_PULLUP_W<CONF0_SPEC, 9> {
        DP_PULLUP_W::new(self)
    }
    #[doc = "Bit 10 - Control USB D+ pull down."]
    #[inline(always)]
    #[must_use]
    pub fn dp_pulldown(&mut self) -> DP_PULLDOWN_W<CONF0_SPEC, 10> {
        DP_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 11 - Control USB D- pull up."]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup(&mut self) -> DM_PULLUP_W<CONF0_SPEC, 11> {
        DM_PULLUP_W::new(self)
    }
    #[doc = "Bit 12 - Control USB D- pull down."]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldown(&mut self) -> DM_PULLDOWN_W<CONF0_SPEC, 12> {
        DM_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 13 - Control pull up value."]
    #[inline(always)]
    #[must_use]
    pub fn pullup_value(&mut self) -> PULLUP_VALUE_W<CONF0_SPEC, 13> {
        PULLUP_VALUE_W::new(self)
    }
    #[doc = "Bit 14 - Enable USB pad function."]
    #[inline(always)]
    #[must_use]
    pub fn usb_pad_enable(&mut self) -> USB_PAD_ENABLE_W<CONF0_SPEC, 14> {
        USB_PAD_ENABLE_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
    #[inline(always)]
    #[must_use]
    pub fn usb_jtag_bridge_en(&mut self) -> USB_JTAG_BRIDGE_EN_W<CONF0_SPEC, 15> {
        USB_JTAG_BRIDGE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PHY hardware configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0x4200"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x4200;
}
