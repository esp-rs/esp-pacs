#[doc = "Register `OTG_CONF` reader"]
pub type R = crate::R<OTG_CONF_SPEC>;
#[doc = "Register `OTG_CONF` writer"]
pub type W = crate::W<OTG_CONF_SPEC>;
#[doc = "Field `SRP_SESSEND_OVERRIDE` reader - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
pub type SRP_SESSEND_OVERRIDE_R = crate::BitReader;
#[doc = "Field `SRP_SESSEND_OVERRIDE` writer - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
pub type SRP_SESSEND_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRP_SESSEND_VALUE` reader - Software over-ride value of srp session end signal."]
pub type SRP_SESSEND_VALUE_R = crate::BitReader;
#[doc = "Field `SRP_SESSEND_VALUE` writer - Software over-ride value of srp session end signal."]
pub type SRP_SESSEND_VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_SEL` reader - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
pub type PHY_SEL_R = crate::BitReader;
#[doc = "Field `PHY_SEL` writer - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
pub type PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFIFO_FORCE_PD` reader - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DFIFO_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DFIFO_FORCE_PD` writer - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DFIFO_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBNCE_FLTR_BYPASS` reader - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
pub type DBNCE_FLTR_BYPASS_R = crate::BitReader;
#[doc = "Field `DBNCE_FLTR_BYPASS` writer - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
pub type DBNCE_FLTR_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCHG_PINS_OVERRIDE` reader - Enable software controlle USB D+ D- exchange"]
pub type EXCHG_PINS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `EXCHG_PINS_OVERRIDE` writer - Enable software controlle USB D+ D- exchange"]
pub type EXCHG_PINS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCHG_PINS` reader - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
pub type EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `EXCHG_PINS` writer - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
pub type EXCHG_PINS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFH` reader - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type VREFH_R = crate::FieldReader;
#[doc = "Field `VREFH` writer - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type VREFH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VREFL` reader - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type VREFL_R = crate::FieldReader;
#[doc = "Field `VREFL` writer - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type VREFL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VREF_OVERRIDE` reader - Enable software controlle input threshold"]
pub type VREF_OVERRIDE_R = crate::BitReader;
#[doc = "Field `VREF_OVERRIDE` writer - Enable software controlle input threshold"]
pub type VREF_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_PULL_OVERRIDE` reader - Enable software controlle USB D+ D- pullup pulldown"]
pub type PAD_PULL_OVERRIDE_R = crate::BitReader;
#[doc = "Field `PAD_PULL_OVERRIDE` writer - Enable software controlle USB D+ D- pullup pulldown"]
pub type PAD_PULL_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLUP` reader - Controlle USB D+ pullup"]
pub type DP_PULLUP_R = crate::BitReader;
#[doc = "Field `DP_PULLUP` writer - Controlle USB D+ pullup"]
pub type DP_PULLUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLDOWN` reader - Controlle USB D+ pulldown"]
pub type DP_PULLDOWN_R = crate::BitReader;
#[doc = "Field `DP_PULLDOWN` writer - Controlle USB D+ pulldown"]
pub type DP_PULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLUP` reader - Controlle USB D+ pullup"]
pub type DM_PULLUP_R = crate::BitReader;
#[doc = "Field `DM_PULLUP` writer - Controlle USB D+ pullup"]
pub type DM_PULLUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLDOWN` reader - Controlle USB D+ pulldown"]
pub type DM_PULLDOWN_R = crate::BitReader;
#[doc = "Field `DM_PULLDOWN` writer - Controlle USB D+ pulldown"]
pub type DM_PULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLUP_VALUE` reader - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
pub type PULLUP_VALUE_R = crate::BitReader;
#[doc = "Field `PULLUP_VALUE` writer - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
pub type PULLUP_VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_PAD_ENABLE` reader - Enable USB pad function"]
pub type USB_PAD_ENABLE_R = crate::BitReader;
#[doc = "Field `USB_PAD_ENABLE` writer - Enable USB pad function"]
pub type USB_PAD_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_CLK_FORCE_ON` reader - Force ahb clock always on"]
pub type AHB_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `AHB_CLK_FORCE_ON` writer - Force ahb clock always on"]
pub type AHB_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CLK_FORCE_ON` reader - Force phy clock always on"]
pub type PHY_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `PHY_CLK_FORCE_ON` writer - Force phy clock always on"]
pub type PHY_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TX_EDGE_SEL` reader - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
pub type PHY_TX_EDGE_SEL_R = crate::BitReader;
#[doc = "Field `PHY_TX_EDGE_SEL` writer - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
pub type PHY_TX_EDGE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFIFO_FORCE_PU` reader - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DFIFO_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DFIFO_FORCE_PU` writer - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DFIFO_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Disable auto clock gating of CSR registers"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Disable auto clock gating of CSR registers"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
    #[inline(always)]
    pub fn srp_sessend_override(&self) -> SRP_SESSEND_OVERRIDE_R {
        SRP_SESSEND_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software over-ride value of srp session end signal."]
    #[inline(always)]
    pub fn srp_sessend_value(&self) -> SRP_SESSEND_VALUE_R {
        SRP_SESSEND_VALUE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
    #[inline(always)]
    pub fn phy_sel(&self) -> PHY_SEL_R {
        PHY_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pd(&self) -> DFIFO_FORCE_PD_R {
        DFIFO_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
    #[inline(always)]
    pub fn dbnce_fltr_bypass(&self) -> DBNCE_FLTR_BYPASS_R {
        DBNCE_FLTR_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable software controlle USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins_override(&self) -> EXCHG_PINS_OVERRIDE_R {
        EXCHG_PINS_OVERRIDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
    #[inline(always)]
    pub fn exchg_pins(&self) -> EXCHG_PINS_R {
        EXCHG_PINS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    pub fn vrefh(&self) -> VREFH_R {
        VREFH_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    pub fn vrefl(&self) -> VREFL_R {
        VREFL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Enable software controlle input threshold"]
    #[inline(always)]
    pub fn vref_override(&self) -> VREF_OVERRIDE_R {
        VREF_OVERRIDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable software controlle USB D+ D- pullup pulldown"]
    #[inline(always)]
    pub fn pad_pull_override(&self) -> PAD_PULL_OVERRIDE_R {
        PAD_PULL_OVERRIDE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Controlle USB D+ pullup"]
    #[inline(always)]
    pub fn dp_pullup(&self) -> DP_PULLUP_R {
        DP_PULLUP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Controlle USB D+ pulldown"]
    #[inline(always)]
    pub fn dp_pulldown(&self) -> DP_PULLDOWN_R {
        DP_PULLDOWN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Controlle USB D+ pullup"]
    #[inline(always)]
    pub fn dm_pullup(&self) -> DM_PULLUP_R {
        DM_PULLUP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Controlle USB D+ pulldown"]
    #[inline(always)]
    pub fn dm_pulldown(&self) -> DM_PULLDOWN_R {
        DM_PULLDOWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
    #[inline(always)]
    pub fn pullup_value(&self) -> PULLUP_VALUE_R {
        PULLUP_VALUE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable USB pad function"]
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> USB_PAD_ENABLE_R {
        USB_PAD_ENABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force ahb clock always on"]
    #[inline(always)]
    pub fn ahb_clk_force_on(&self) -> AHB_CLK_FORCE_ON_R {
        AHB_CLK_FORCE_ON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Force phy clock always on"]
    #[inline(always)]
    pub fn phy_clk_force_on(&self) -> PHY_CLK_FORCE_ON_R {
        PHY_CLK_FORCE_ON_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
    #[inline(always)]
    pub fn phy_tx_edge_sel(&self) -> PHY_TX_EDGE_SEL_R {
        PHY_TX_EDGE_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pu(&self) -> DFIFO_FORCE_PU_R {
        DFIFO_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable auto clock gating of CSR registers"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_CONF")
            .field("srp_sessend_override", &self.srp_sessend_override())
            .field("srp_sessend_value", &self.srp_sessend_value())
            .field("phy_sel", &self.phy_sel())
            .field("dfifo_force_pd", &self.dfifo_force_pd())
            .field("dbnce_fltr_bypass", &self.dbnce_fltr_bypass())
            .field("exchg_pins_override", &self.exchg_pins_override())
            .field("exchg_pins", &self.exchg_pins())
            .field("vrefh", &self.vrefh())
            .field("vrefl", &self.vrefl())
            .field("vref_override", &self.vref_override())
            .field("pad_pull_override", &self.pad_pull_override())
            .field("dp_pullup", &self.dp_pullup())
            .field("dp_pulldown", &self.dp_pulldown())
            .field("dm_pullup", &self.dm_pullup())
            .field("dm_pulldown", &self.dm_pulldown())
            .field("pullup_value", &self.pullup_value())
            .field("usb_pad_enable", &self.usb_pad_enable())
            .field("ahb_clk_force_on", &self.ahb_clk_force_on())
            .field("phy_clk_force_on", &self.phy_clk_force_on())
            .field("phy_tx_edge_sel", &self.phy_tx_edge_sel())
            .field("dfifo_force_pu", &self.dfifo_force_pu())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
    #[inline(always)]
    pub fn srp_sessend_override(&mut self) -> SRP_SESSEND_OVERRIDE_W<'_, OTG_CONF_SPEC> {
        SRP_SESSEND_OVERRIDE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software over-ride value of srp session end signal."]
    #[inline(always)]
    pub fn srp_sessend_value(&mut self) -> SRP_SESSEND_VALUE_W<'_, OTG_CONF_SPEC> {
        SRP_SESSEND_VALUE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
    #[inline(always)]
    pub fn phy_sel(&mut self) -> PHY_SEL_W<'_, OTG_CONF_SPEC> {
        PHY_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pd(&mut self) -> DFIFO_FORCE_PD_W<'_, OTG_CONF_SPEC> {
        DFIFO_FORCE_PD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
    #[inline(always)]
    pub fn dbnce_fltr_bypass(&mut self) -> DBNCE_FLTR_BYPASS_W<'_, OTG_CONF_SPEC> {
        DBNCE_FLTR_BYPASS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable software controlle USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins_override(&mut self) -> EXCHG_PINS_OVERRIDE_W<'_, OTG_CONF_SPEC> {
        EXCHG_PINS_OVERRIDE_W::new(self, 5)
    }
    #[doc = "Bit 6 - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
    #[inline(always)]
    pub fn exchg_pins(&mut self) -> EXCHG_PINS_W<'_, OTG_CONF_SPEC> {
        EXCHG_PINS_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    pub fn vrefh(&mut self) -> VREFH_W<'_, OTG_CONF_SPEC> {
        VREFH_W::new(self, 7)
    }
    #[doc = "Bits 9:10 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    pub fn vrefl(&mut self) -> VREFL_W<'_, OTG_CONF_SPEC> {
        VREFL_W::new(self, 9)
    }
    #[doc = "Bit 11 - Enable software controlle input threshold"]
    #[inline(always)]
    pub fn vref_override(&mut self) -> VREF_OVERRIDE_W<'_, OTG_CONF_SPEC> {
        VREF_OVERRIDE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable software controlle USB D+ D- pullup pulldown"]
    #[inline(always)]
    pub fn pad_pull_override(&mut self) -> PAD_PULL_OVERRIDE_W<'_, OTG_CONF_SPEC> {
        PAD_PULL_OVERRIDE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Controlle USB D+ pullup"]
    #[inline(always)]
    pub fn dp_pullup(&mut self) -> DP_PULLUP_W<'_, OTG_CONF_SPEC> {
        DP_PULLUP_W::new(self, 13)
    }
    #[doc = "Bit 14 - Controlle USB D+ pulldown"]
    #[inline(always)]
    pub fn dp_pulldown(&mut self) -> DP_PULLDOWN_W<'_, OTG_CONF_SPEC> {
        DP_PULLDOWN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Controlle USB D+ pullup"]
    #[inline(always)]
    pub fn dm_pullup(&mut self) -> DM_PULLUP_W<'_, OTG_CONF_SPEC> {
        DM_PULLUP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Controlle USB D+ pulldown"]
    #[inline(always)]
    pub fn dm_pulldown(&mut self) -> DM_PULLDOWN_W<'_, OTG_CONF_SPEC> {
        DM_PULLDOWN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
    #[inline(always)]
    pub fn pullup_value(&mut self) -> PULLUP_VALUE_W<'_, OTG_CONF_SPEC> {
        PULLUP_VALUE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable USB pad function"]
    #[inline(always)]
    pub fn usb_pad_enable(&mut self) -> USB_PAD_ENABLE_W<'_, OTG_CONF_SPEC> {
        USB_PAD_ENABLE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Force ahb clock always on"]
    #[inline(always)]
    pub fn ahb_clk_force_on(&mut self) -> AHB_CLK_FORCE_ON_W<'_, OTG_CONF_SPEC> {
        AHB_CLK_FORCE_ON_W::new(self, 19)
    }
    #[doc = "Bit 20 - Force phy clock always on"]
    #[inline(always)]
    pub fn phy_clk_force_on(&mut self) -> PHY_CLK_FORCE_ON_W<'_, OTG_CONF_SPEC> {
        PHY_CLK_FORCE_ON_W::new(self, 20)
    }
    #[doc = "Bit 21 - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
    #[inline(always)]
    pub fn phy_tx_edge_sel(&mut self) -> PHY_TX_EDGE_SEL_W<'_, OTG_CONF_SPEC> {
        PHY_TX_EDGE_SEL_W::new(self, 21)
    }
    #[doc = "Bit 22 - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pu(&mut self) -> DFIFO_FORCE_PU_W<'_, OTG_CONF_SPEC> {
        DFIFO_FORCE_PU_W::new(self, 22)
    }
    #[doc = "Bit 31 - Disable auto clock gating of CSR registers"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, OTG_CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "USB OTG Wrapper Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_CONF_SPEC;
impl crate::RegisterSpec for OTG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_conf::R`](R) reader structure"]
impl crate::Readable for OTG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_conf::W`](W) writer structure"]
impl crate::Writable for OTG_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_CONF to value 0x001c_0000"]
impl crate::Resettable for OTG_CONF_SPEC {
    const RESET_VALUE: u32 = 0x001c_0000;
}
