#[doc = "Register `OTG_CONF` reader"]
pub struct R(crate::R<OTG_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_CONF` writer"]
pub struct W(crate::W<OTG_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_CONF_SPEC>;
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
impl From<crate::W<OTG_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRP_SESSEND_OVERRIDE` reader - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
pub type SRP_SESSEND_OVERRIDE_R = crate::BitReader;
#[doc = "Field `SRP_SESSEND_OVERRIDE` writer - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
pub type SRP_SESSEND_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `SRP_SESSEND_VALUE` reader - Software over-ride value of srp session end signal."]
pub type SRP_SESSEND_VALUE_R = crate::BitReader;
#[doc = "Field `SRP_SESSEND_VALUE` writer - Software over-ride value of srp session end signal."]
pub type SRP_SESSEND_VALUE_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `PHY_SEL` reader - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
pub type PHY_SEL_R = crate::BitReader;
#[doc = "Field `PHY_SEL` writer - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
pub type PHY_SEL_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `DFIFO_FORCE_PD` reader - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DFIFO_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DFIFO_FORCE_PD` writer - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DFIFO_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `DBNCE_FLTR_BYPASS` reader - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
pub type DBNCE_FLTR_BYPASS_R = crate::BitReader;
#[doc = "Field `DBNCE_FLTR_BYPASS` writer - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
pub type DBNCE_FLTR_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `EXCHG_PINS_OVERRIDE` reader - Enable software controlle USB D+ D- exchange"]
pub type EXCHG_PINS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `EXCHG_PINS_OVERRIDE` writer - Enable software controlle USB D+ D- exchange"]
pub type EXCHG_PINS_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `EXCHG_PINS` reader - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
pub type EXCHG_PINS_R = crate::BitReader;
#[doc = "Field `EXCHG_PINS` writer - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
pub type EXCHG_PINS_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `VREFH` reader - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type VREFH_R = crate::FieldReader;
#[doc = "Field `VREFH` writer - Control single-end input high threshold,1.76V to 2V, step 80mV"]
pub type VREFH_W<'a, const O: u8> = crate::FieldWriter<'a, OTG_CONF_SPEC, 2, O>;
#[doc = "Field `VREFL` reader - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type VREFL_R = crate::FieldReader;
#[doc = "Field `VREFL` writer - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
pub type VREFL_W<'a, const O: u8> = crate::FieldWriter<'a, OTG_CONF_SPEC, 2, O>;
#[doc = "Field `VREF_OVERRIDE` reader - Enable software controlle input threshold"]
pub type VREF_OVERRIDE_R = crate::BitReader;
#[doc = "Field `VREF_OVERRIDE` writer - Enable software controlle input threshold"]
pub type VREF_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `PAD_PULL_OVERRIDE` reader - Enable software controlle USB D+ D- pullup pulldown"]
pub type PAD_PULL_OVERRIDE_R = crate::BitReader;
#[doc = "Field `PAD_PULL_OVERRIDE` writer - Enable software controlle USB D+ D- pullup pulldown"]
pub type PAD_PULL_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `DP_PULLUP` reader - Controlle USB D+ pullup"]
pub type DP_PULLUP_R = crate::BitReader;
#[doc = "Field `DP_PULLUP` writer - Controlle USB D+ pullup"]
pub type DP_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `DP_PULLDOWN` reader - Controlle USB D+ pulldown"]
pub type DP_PULLDOWN_R = crate::BitReader;
#[doc = "Field `DP_PULLDOWN` writer - Controlle USB D+ pulldown"]
pub type DP_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `DM_PULLUP` reader - Controlle USB D+ pullup"]
pub type DM_PULLUP_R = crate::BitReader;
#[doc = "Field `DM_PULLUP` writer - Controlle USB D+ pullup"]
pub type DM_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `DM_PULLDOWN` reader - Controlle USB D+ pulldown"]
pub type DM_PULLDOWN_R = crate::BitReader;
#[doc = "Field `DM_PULLDOWN` writer - Controlle USB D+ pulldown"]
pub type DM_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `PULLUP_VALUE` reader - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
pub type PULLUP_VALUE_R = crate::BitReader;
#[doc = "Field `PULLUP_VALUE` writer - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
pub type PULLUP_VALUE_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `USB_PAD_ENABLE` reader - Enable USB pad function"]
pub type USB_PAD_ENABLE_R = crate::BitReader;
#[doc = "Field `USB_PAD_ENABLE` writer - Enable USB pad function"]
pub type USB_PAD_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `AHB_CLK_FORCE_ON` reader - Force ahb clock always on"]
pub type AHB_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `AHB_CLK_FORCE_ON` writer - Force ahb clock always on"]
pub type AHB_CLK_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `PHY_CLK_FORCE_ON` reader - Force phy clock always on"]
pub type PHY_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `PHY_CLK_FORCE_ON` writer - Force phy clock always on"]
pub type PHY_CLK_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `PHY_TX_EDGE_SEL` reader - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
pub type PHY_TX_EDGE_SEL_R = crate::BitReader;
#[doc = "Field `PHY_TX_EDGE_SEL` writer - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
pub type PHY_TX_EDGE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `DFIFO_FORCE_PU` reader - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DFIFO_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DFIFO_FORCE_PU` writer - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DFIFO_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
#[doc = "Field `CLK_EN` reader - Disable auto clock gating of CSR registers"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Disable auto clock gating of CSR registers"]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, OTG_CONF_SPEC, O>;
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
            .field(
                "srp_sessend_override",
                &format_args!("{}", self.srp_sessend_override().bit()),
            )
            .field(
                "srp_sessend_value",
                &format_args!("{}", self.srp_sessend_value().bit()),
            )
            .field("phy_sel", &format_args!("{}", self.phy_sel().bit()))
            .field(
                "dfifo_force_pd",
                &format_args!("{}", self.dfifo_force_pd().bit()),
            )
            .field(
                "dbnce_fltr_bypass",
                &format_args!("{}", self.dbnce_fltr_bypass().bit()),
            )
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
                "ahb_clk_force_on",
                &format_args!("{}", self.ahb_clk_force_on().bit()),
            )
            .field(
                "phy_clk_force_on",
                &format_args!("{}", self.phy_clk_force_on().bit()),
            )
            .field(
                "phy_tx_edge_sel",
                &format_args!("{}", self.phy_tx_edge_sel().bit()),
            )
            .field(
                "dfifo_force_pu",
                &format_args!("{}", self.dfifo_force_pu().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OTG_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input. 1'b1: the signal is controlled by the software."]
    #[inline(always)]
    #[must_use]
    pub fn srp_sessend_override(&mut self) -> SRP_SESSEND_OVERRIDE_W<0> {
        SRP_SESSEND_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 1 - Software over-ride value of srp session end signal."]
    #[inline(always)]
    #[must_use]
    pub fn srp_sessend_value(&mut self) -> SRP_SESSEND_VALUE_W<1> {
        SRP_SESSEND_VALUE_W::new(self)
    }
    #[doc = "Bit 2 - Select internal external PHY. 1'b0: Select internal PHY. 1'b1: Select external PHY."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sel(&mut self) -> PHY_SEL_W<2> {
        PHY_SEL_W::new(self)
    }
    #[doc = "Bit 3 - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    #[must_use]
    pub fn dfifo_force_pd(&mut self) -> DFIFO_FORCE_PD_W<3> {
        DFIFO_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 4 - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
    #[inline(always)]
    #[must_use]
    pub fn dbnce_fltr_bypass(&mut self) -> DBNCE_FLTR_BYPASS_W<4> {
        DBNCE_FLTR_BYPASS_W::new(self)
    }
    #[doc = "Bit 5 - Enable software controlle USB D+ D- exchange"]
    #[inline(always)]
    #[must_use]
    pub fn exchg_pins_override(&mut self) -> EXCHG_PINS_OVERRIDE_W<5> {
        EXCHG_PINS_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 6 - USB D+ D- exchange. 1'b0: don't change. 1'b1: exchange D+ D-"]
    #[inline(always)]
    #[must_use]
    pub fn exchg_pins(&mut self) -> EXCHG_PINS_W<6> {
        EXCHG_PINS_W::new(self)
    }
    #[doc = "Bits 7:8 - Control single-end input high threshold,1.76V to 2V, step 80mV"]
    #[inline(always)]
    #[must_use]
    pub fn vrefh(&mut self) -> VREFH_W<7> {
        VREFH_W::new(self)
    }
    #[doc = "Bits 9:10 - Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    #[inline(always)]
    #[must_use]
    pub fn vrefl(&mut self) -> VREFL_W<9> {
        VREFL_W::new(self)
    }
    #[doc = "Bit 11 - Enable software controlle input threshold"]
    #[inline(always)]
    #[must_use]
    pub fn vref_override(&mut self) -> VREF_OVERRIDE_W<11> {
        VREF_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 12 - Enable software controlle USB D+ D- pullup pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn pad_pull_override(&mut self) -> PAD_PULL_OVERRIDE_W<12> {
        PAD_PULL_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 13 - Controlle USB D+ pullup"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup(&mut self) -> DP_PULLUP_W<13> {
        DP_PULLUP_W::new(self)
    }
    #[doc = "Bit 14 - Controlle USB D+ pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pulldown(&mut self) -> DP_PULLDOWN_W<14> {
        DP_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 15 - Controlle USB D+ pullup"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup(&mut self) -> DM_PULLUP_W<15> {
        DM_PULLUP_W::new(self)
    }
    #[doc = "Bit 16 - Controlle USB D+ pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldown(&mut self) -> DM_PULLDOWN_W<16> {
        DM_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 17 - Controlle pullup value. 1'b0: typical value is 2.4K. 1'b1: typical value is 1.2K."]
    #[inline(always)]
    #[must_use]
    pub fn pullup_value(&mut self) -> PULLUP_VALUE_W<17> {
        PULLUP_VALUE_W::new(self)
    }
    #[doc = "Bit 18 - Enable USB pad function"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pad_enable(&mut self) -> USB_PAD_ENABLE_W<18> {
        USB_PAD_ENABLE_W::new(self)
    }
    #[doc = "Bit 19 - Force ahb clock always on"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_clk_force_on(&mut self) -> AHB_CLK_FORCE_ON_W<19> {
        AHB_CLK_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 20 - Force phy clock always on"]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_force_on(&mut self) -> PHY_CLK_FORCE_ON_W<20> {
        PHY_CLK_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 21 - Select phy tx signal output clock edge. 1'b0: negedge. 1'b1: posedge."]
    #[inline(always)]
    #[must_use]
    pub fn phy_tx_edge_sel(&mut self) -> PHY_TX_EDGE_SEL_W<21> {
        PHY_TX_EDGE_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    #[must_use]
    pub fn dfifo_force_pu(&mut self) -> DFIFO_FORCE_PU_W<22> {
        DFIFO_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 31 - Disable auto clock gating of CSR registers"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB OTG Wrapper Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_conf](index.html) module"]
pub struct OTG_CONF_SPEC;
impl crate::RegisterSpec for OTG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_conf::R](R) reader structure"]
impl crate::Readable for OTG_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_conf::W](W) writer structure"]
impl crate::Writable for OTG_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_CONF to value 0x001c_0000"]
impl crate::Resettable for OTG_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x001c_0000;
}
