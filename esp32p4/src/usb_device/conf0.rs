#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_PHY_SEL` reader - Select internal/external PHY"]
pub type USB_SERIAL_JTAG_PHY_SEL_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_PHY_SEL` writer - Select internal/external PHY"]
pub type USB_SERIAL_JTAG_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE` reader - Enable software control USB D+ D- exchange"]
pub type USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE` writer - Enable software control USB D+ D- exchange"]
pub type USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_EXCHG_PINS` reader - USB D+ D- exchange"]
pub type USB_SERIAL_JTAG_EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_EXCHG_PINS` writer - USB D+ D- exchange"]
pub type USB_SERIAL_JTAG_EXCHG_PINS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_VREFH` reader - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type USB_SERIAL_JTAG_VREFH_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_VREFH` writer - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type USB_SERIAL_JTAG_VREFH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB_SERIAL_JTAG_VREFL` reader - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type USB_SERIAL_JTAG_VREFL_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_VREFL` writer - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type USB_SERIAL_JTAG_VREFL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB_SERIAL_JTAG_VREF_OVERRIDE` reader - Enable software control input threshold"]
pub type USB_SERIAL_JTAG_VREF_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_VREF_OVERRIDE` writer - Enable software control input threshold"]
pub type USB_SERIAL_JTAG_VREF_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_PAD_PULL_OVERRIDE` reader - Enable software control USB D+ D- pullup pulldown"]
pub type USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_PAD_PULL_OVERRIDE` writer - Enable software control USB D+ D- pullup pulldown"]
pub type USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_DP_PULLUP` reader - Control USB D+ pull up."]
pub type USB_SERIAL_JTAG_DP_PULLUP_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_DP_PULLUP` writer - Control USB D+ pull up."]
pub type USB_SERIAL_JTAG_DP_PULLUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_DP_PULLDOWN` reader - Control USB D+ pull down."]
pub type USB_SERIAL_JTAG_DP_PULLDOWN_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_DP_PULLDOWN` writer - Control USB D+ pull down."]
pub type USB_SERIAL_JTAG_DP_PULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_DM_PULLUP` reader - Control USB D- pull up."]
pub type USB_SERIAL_JTAG_DM_PULLUP_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_DM_PULLUP` writer - Control USB D- pull up."]
pub type USB_SERIAL_JTAG_DM_PULLUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_DM_PULLDOWN` reader - Control USB D- pull down."]
pub type USB_SERIAL_JTAG_DM_PULLDOWN_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_DM_PULLDOWN` writer - Control USB D- pull down."]
pub type USB_SERIAL_JTAG_DM_PULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_PULLUP_VALUE` reader - Control pull up value."]
pub type USB_SERIAL_JTAG_PULLUP_VALUE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_PULLUP_VALUE` writer - Control pull up value."]
pub type USB_SERIAL_JTAG_PULLUP_VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_USB_PAD_ENABLE` reader - Enable USB pad function."]
pub type USB_SERIAL_JTAG_USB_PAD_ENABLE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_USB_PAD_ENABLE` writer - Enable USB pad function."]
pub type USB_SERIAL_JTAG_USB_PAD_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN` reader - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
pub type USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN` writer - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
pub type USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field(
                "usb_serial_jtag_phy_sel",
                &format_args!("{}", self.usb_serial_jtag_phy_sel().bit()),
            )
            .field(
                "usb_serial_jtag_exchg_pins_override",
                &format_args!("{}", self.usb_serial_jtag_exchg_pins_override().bit()),
            )
            .field(
                "usb_serial_jtag_exchg_pins",
                &format_args!("{}", self.usb_serial_jtag_exchg_pins().bit()),
            )
            .field(
                "usb_serial_jtag_vrefh",
                &format_args!("{}", self.usb_serial_jtag_vrefh().bits()),
            )
            .field(
                "usb_serial_jtag_vrefl",
                &format_args!("{}", self.usb_serial_jtag_vrefl().bits()),
            )
            .field(
                "usb_serial_jtag_vref_override",
                &format_args!("{}", self.usb_serial_jtag_vref_override().bit()),
            )
            .field(
                "usb_serial_jtag_pad_pull_override",
                &format_args!("{}", self.usb_serial_jtag_pad_pull_override().bit()),
            )
            .field(
                "usb_serial_jtag_dp_pullup",
                &format_args!("{}", self.usb_serial_jtag_dp_pullup().bit()),
            )
            .field(
                "usb_serial_jtag_dp_pulldown",
                &format_args!("{}", self.usb_serial_jtag_dp_pulldown().bit()),
            )
            .field(
                "usb_serial_jtag_dm_pullup",
                &format_args!("{}", self.usb_serial_jtag_dm_pullup().bit()),
            )
            .field(
                "usb_serial_jtag_dm_pulldown",
                &format_args!("{}", self.usb_serial_jtag_dm_pulldown().bit()),
            )
            .field(
                "usb_serial_jtag_pullup_value",
                &format_args!("{}", self.usb_serial_jtag_pullup_value().bit()),
            )
            .field(
                "usb_serial_jtag_usb_pad_enable",
                &format_args!("{}", self.usb_serial_jtag_usb_pad_enable().bit()),
            )
            .field(
                "usb_serial_jtag_usb_jtag_bridge_en",
                &format_args!("{}", self.usb_serial_jtag_usb_jtag_bridge_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Select internal/external PHY"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_phy_sel(&mut self) -> USB_SERIAL_JTAG_PHY_SEL_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_PHY_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable software control USB D+ D- exchange"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_exchg_pins_override(
        &mut self,
    ) -> USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB D+ D- exchange"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_exchg_pins(&mut self) -> USB_SERIAL_JTAG_EXCHG_PINS_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_EXCHG_PINS_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_vrefh(&mut self) -> USB_SERIAL_JTAG_VREFH_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_VREFH_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_vrefl(&mut self) -> USB_SERIAL_JTAG_VREFL_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_VREFL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Enable software control input threshold"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_vref_override(&mut self) -> USB_SERIAL_JTAG_VREF_OVERRIDE_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_VREF_OVERRIDE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable software control USB D+ D- pullup pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_pad_pull_override(
        &mut self,
    ) -> USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Control USB D+ pull up."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dp_pullup(&mut self) -> USB_SERIAL_JTAG_DP_PULLUP_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_DP_PULLUP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Control USB D+ pull down."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dp_pulldown(&mut self) -> USB_SERIAL_JTAG_DP_PULLDOWN_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_DP_PULLDOWN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Control USB D- pull up."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dm_pullup(&mut self) -> USB_SERIAL_JTAG_DM_PULLUP_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_DM_PULLUP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Control USB D- pull down."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dm_pulldown(&mut self) -> USB_SERIAL_JTAG_DM_PULLDOWN_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_DM_PULLDOWN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Control pull up value."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_pullup_value(&mut self) -> USB_SERIAL_JTAG_PULLUP_VALUE_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_PULLUP_VALUE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable USB pad function."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_usb_pad_enable(
        &mut self,
    ) -> USB_SERIAL_JTAG_USB_PAD_ENABLE_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_USB_PAD_ENABLE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_usb_jtag_bridge_en(
        &mut self,
    ) -> USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_W<CONF0_SPEC> {
        USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
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
