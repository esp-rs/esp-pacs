#[doc = "Register `USB_SERIAL_JTAG_CONF0` reader"]
pub type R = crate::R<USB_SERIAL_JTAG_CONF0_SPEC>;
#[doc = "Register `USB_SERIAL_JTAG_CONF0` writer"]
pub type W = crate::W<USB_SERIAL_JTAG_CONF0_SPEC>;
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
#[doc = "Field `USB_SERIAL_JTAG_USB_PHY_TX_EDGE_SEL` reader - Control at which clock edge the dp and dm are sent to USB PHY, 0: tx output at clock negative edge. 1: tx output at clock positive edge."]
pub type USB_SERIAL_JTAG_USB_PHY_TX_EDGE_SEL_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_USB_PHY_TX_EDGE_SEL` writer - Control at which clock edge the dp and dm are sent to USB PHY, 0: tx output at clock negative edge. 1: tx output at clock positive edge."]
pub type USB_SERIAL_JTAG_USB_PHY_TX_EDGE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16 - Control at which clock edge the dp and dm are sent to USB PHY, 0: tx output at clock negative edge. 1: tx output at clock positive edge."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_phy_tx_edge_sel(&self) -> USB_SERIAL_JTAG_USB_PHY_TX_EDGE_SEL_R {
        USB_SERIAL_JTAG_USB_PHY_TX_EDGE_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_SERIAL_JTAG_CONF0")
            .field("usb_serial_jtag_phy_sel", &self.usb_serial_jtag_phy_sel())
            .field(
                "usb_serial_jtag_exchg_pins_override",
                &self.usb_serial_jtag_exchg_pins_override(),
            )
            .field(
                "usb_serial_jtag_exchg_pins",
                &self.usb_serial_jtag_exchg_pins(),
            )
            .field("usb_serial_jtag_vrefh", &self.usb_serial_jtag_vrefh())
            .field("usb_serial_jtag_vrefl", &self.usb_serial_jtag_vrefl())
            .field(
                "usb_serial_jtag_vref_override",
                &self.usb_serial_jtag_vref_override(),
            )
            .field(
                "usb_serial_jtag_pad_pull_override",
                &self.usb_serial_jtag_pad_pull_override(),
            )
            .field(
                "usb_serial_jtag_dp_pullup",
                &self.usb_serial_jtag_dp_pullup(),
            )
            .field(
                "usb_serial_jtag_dp_pulldown",
                &self.usb_serial_jtag_dp_pulldown(),
            )
            .field(
                "usb_serial_jtag_dm_pullup",
                &self.usb_serial_jtag_dm_pullup(),
            )
            .field(
                "usb_serial_jtag_dm_pulldown",
                &self.usb_serial_jtag_dm_pulldown(),
            )
            .field(
                "usb_serial_jtag_pullup_value",
                &self.usb_serial_jtag_pullup_value(),
            )
            .field(
                "usb_serial_jtag_usb_pad_enable",
                &self.usb_serial_jtag_usb_pad_enable(),
            )
            .field(
                "usb_serial_jtag_usb_jtag_bridge_en",
                &self.usb_serial_jtag_usb_jtag_bridge_en(),
            )
            .field(
                "usb_serial_jtag_usb_phy_tx_edge_sel",
                &self.usb_serial_jtag_usb_phy_tx_edge_sel(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Select internal/external PHY"]
    #[inline(always)]
    pub fn usb_serial_jtag_phy_sel(
        &mut self,
    ) -> USB_SERIAL_JTAG_PHY_SEL_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_PHY_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable software control USB D+ D- exchange"]
    #[inline(always)]
    pub fn usb_serial_jtag_exchg_pins_override(
        &mut self,
    ) -> USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_EXCHG_PINS_OVERRIDE_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB D+ D- exchange"]
    #[inline(always)]
    pub fn usb_serial_jtag_exchg_pins(
        &mut self,
    ) -> USB_SERIAL_JTAG_EXCHG_PINS_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_EXCHG_PINS_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    pub fn usb_serial_jtag_vrefh(
        &mut self,
    ) -> USB_SERIAL_JTAG_VREFH_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_VREFH_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    pub fn usb_serial_jtag_vrefl(
        &mut self,
    ) -> USB_SERIAL_JTAG_VREFL_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_VREFL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Enable software control input threshold"]
    #[inline(always)]
    pub fn usb_serial_jtag_vref_override(
        &mut self,
    ) -> USB_SERIAL_JTAG_VREF_OVERRIDE_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_VREF_OVERRIDE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable software control USB D+ D- pullup pulldown"]
    #[inline(always)]
    pub fn usb_serial_jtag_pad_pull_override(
        &mut self,
    ) -> USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_PAD_PULL_OVERRIDE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Control USB D+ pull up."]
    #[inline(always)]
    pub fn usb_serial_jtag_dp_pullup(
        &mut self,
    ) -> USB_SERIAL_JTAG_DP_PULLUP_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_DP_PULLUP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Control USB D+ pull down."]
    #[inline(always)]
    pub fn usb_serial_jtag_dp_pulldown(
        &mut self,
    ) -> USB_SERIAL_JTAG_DP_PULLDOWN_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_DP_PULLDOWN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Control USB D- pull up."]
    #[inline(always)]
    pub fn usb_serial_jtag_dm_pullup(
        &mut self,
    ) -> USB_SERIAL_JTAG_DM_PULLUP_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_DM_PULLUP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Control USB D- pull down."]
    #[inline(always)]
    pub fn usb_serial_jtag_dm_pulldown(
        &mut self,
    ) -> USB_SERIAL_JTAG_DM_PULLDOWN_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_DM_PULLDOWN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Control pull up value."]
    #[inline(always)]
    pub fn usb_serial_jtag_pullup_value(
        &mut self,
    ) -> USB_SERIAL_JTAG_PULLUP_VALUE_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_PULLUP_VALUE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable USB pad function."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_pad_enable(
        &mut self,
    ) -> USB_SERIAL_JTAG_USB_PAD_ENABLE_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_USB_PAD_ENABLE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit usb_jtag, the connection between usb_jtag and internal JTAG is disconnected, and MTMS, MTDI, MTCK are output through GPIO Matrix, MTDO is input through GPIO Matrix."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_jtag_bridge_en(
        &mut self,
    ) -> USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_USB_JTAG_BRIDGE_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Control at which clock edge the dp and dm are sent to USB PHY, 0: tx output at clock negative edge. 1: tx output at clock positive edge."]
    #[inline(always)]
    pub fn usb_serial_jtag_usb_phy_tx_edge_sel(
        &mut self,
    ) -> USB_SERIAL_JTAG_USB_PHY_TX_EDGE_SEL_W<'_, USB_SERIAL_JTAG_CONF0_SPEC> {
        USB_SERIAL_JTAG_USB_PHY_TX_EDGE_SEL_W::new(self, 16)
    }
}
#[doc = "PHY hardware configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_SERIAL_JTAG_CONF0_SPEC;
impl crate::RegisterSpec for USB_SERIAL_JTAG_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_serial_jtag_conf0::R`](R) reader structure"]
impl crate::Readable for USB_SERIAL_JTAG_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_serial_jtag_conf0::W`](W) writer structure"]
impl crate::Writable for USB_SERIAL_JTAG_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_SERIAL_JTAG_CONF0 to value 0x4200"]
impl crate::Resettable for USB_SERIAL_JTAG_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x4200;
}
