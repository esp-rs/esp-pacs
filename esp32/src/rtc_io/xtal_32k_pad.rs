#[doc = "Register `XTAL_32K_PAD` reader"]
pub struct R(crate::R<XTAL_32K_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_32K_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_32K_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_32K_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL_32K_PAD` writer"]
pub struct W(crate::W<XTAL_32K_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_32K_PAD_SPEC>;
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
impl From<crate::W<XTAL_32K_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_32K_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBIAS_XTAL_32K` reader - 32K XTAL self-bias reference control."]
pub type DBIAS_XTAL_32K_R = crate::FieldReader;
#[doc = "Field `DBIAS_XTAL_32K` writer - 32K XTAL self-bias reference control."]
pub type DBIAS_XTAL_32K_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL_32K_PAD_SPEC, 2, O>;
#[doc = "Field `DRES_XTAL_32K` reader - 32K XTAL resistor bias control."]
pub type DRES_XTAL_32K_R = crate::FieldReader;
#[doc = "Field `DRES_XTAL_32K` writer - 32K XTAL resistor bias control."]
pub type DRES_XTAL_32K_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL_32K_PAD_SPEC, 2, O>;
#[doc = "Field `X32P_FUN_IE` reader - the input enable of the pad"]
pub type X32P_FUN_IE_R = crate::BitReader;
#[doc = "Field `X32P_FUN_IE` writer - the input enable of the pad"]
pub type X32P_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32P_SLP_OE` reader - the output enable of the pad in sleep status"]
pub type X32P_SLP_OE_R = crate::BitReader;
#[doc = "Field `X32P_SLP_OE` writer - the output enable of the pad in sleep status"]
pub type X32P_SLP_OE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32P_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type X32P_SLP_IE_R = crate::BitReader;
#[doc = "Field `X32P_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type X32P_SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32P_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type X32P_SLP_SEL_R = crate::BitReader;
#[doc = "Field `X32P_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type X32P_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32P_FUN_SEL` reader - the functional selection signal of the pad"]
pub type X32P_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `X32P_FUN_SEL` writer - the functional selection signal of the pad"]
pub type X32P_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL_32K_PAD_SPEC, 2, O>;
#[doc = "Field `X32N_FUN_IE` reader - the input enable of the pad"]
pub type X32N_FUN_IE_R = crate::BitReader;
#[doc = "Field `X32N_FUN_IE` writer - the input enable of the pad"]
pub type X32N_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32N_SLP_OE` reader - the output enable of the pad in sleep status"]
pub type X32N_SLP_OE_R = crate::BitReader;
#[doc = "Field `X32N_SLP_OE` writer - the output enable of the pad in sleep status"]
pub type X32N_SLP_OE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32N_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type X32N_SLP_IE_R = crate::BitReader;
#[doc = "Field `X32N_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type X32N_SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32N_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type X32N_SLP_SEL_R = crate::BitReader;
#[doc = "Field `X32N_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type X32N_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32N_FUN_SEL` reader - the functional selection signal of the pad"]
pub type X32N_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `X32N_FUN_SEL` writer - the functional selection signal of the pad"]
pub type X32N_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL_32K_PAD_SPEC, 2, O>;
#[doc = "Field `X32P_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type X32P_MUX_SEL_R = crate::BitReader;
#[doc = "Field `X32P_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type X32P_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32N_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type X32N_MUX_SEL_R = crate::BitReader;
#[doc = "Field `X32N_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type X32N_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `XPD_XTAL_32K` reader - Power up 32kHz crystal oscillator"]
pub type XPD_XTAL_32K_R = crate::BitReader;
#[doc = "Field `XPD_XTAL_32K` writer - Power up 32kHz crystal oscillator"]
pub type XPD_XTAL_32K_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `DAC_XTAL_32K` reader - 32K XTAL bias current DAC."]
pub type DAC_XTAL_32K_R = crate::FieldReader;
#[doc = "Field `DAC_XTAL_32K` writer - 32K XTAL bias current DAC."]
pub type DAC_XTAL_32K_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL_32K_PAD_SPEC, 2, O>;
#[doc = "Field `X32P_RUE` reader - the pull up enable of the pad"]
pub type X32P_RUE_R = crate::BitReader;
#[doc = "Field `X32P_RUE` writer - the pull up enable of the pad"]
pub type X32P_RUE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32P_RDE` reader - the pull down enable of the pad"]
pub type X32P_RDE_R = crate::BitReader;
#[doc = "Field `X32P_RDE` writer - the pull down enable of the pad"]
pub type X32P_RDE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32P_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type X32P_HOLD_R = crate::BitReader;
#[doc = "Field `X32P_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type X32P_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32P_DRV` reader - the driver strength of the pad"]
pub type X32P_DRV_R = crate::FieldReader;
#[doc = "Field `X32P_DRV` writer - the driver strength of the pad"]
pub type X32P_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL_32K_PAD_SPEC, 2, O>;
#[doc = "Field `X32N_RUE` reader - the pull up enable of the pad"]
pub type X32N_RUE_R = crate::BitReader;
#[doc = "Field `X32N_RUE` writer - the pull up enable of the pad"]
pub type X32N_RUE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32N_RDE` reader - the pull down enable of the pad"]
pub type X32N_RDE_R = crate::BitReader;
#[doc = "Field `X32N_RDE` writer - the pull down enable of the pad"]
pub type X32N_RDE_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32N_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type X32N_HOLD_R = crate::BitReader;
#[doc = "Field `X32N_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type X32N_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, XTAL_32K_PAD_SPEC, O>;
#[doc = "Field `X32N_DRV` reader - the driver strength of the pad"]
pub type X32N_DRV_R = crate::FieldReader;
#[doc = "Field `X32N_DRV` writer - the driver strength of the pad"]
pub type X32N_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL_32K_PAD_SPEC, 2, O>;
impl R {
    #[doc = "Bits 1:2 - 32K XTAL self-bias reference control."]
    #[inline(always)]
    pub fn dbias_xtal_32k(&self) -> DBIAS_XTAL_32K_R {
        DBIAS_XTAL_32K_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 32K XTAL resistor bias control."]
    #[inline(always)]
    pub fn dres_xtal_32k(&self) -> DRES_XTAL_32K_R {
        DRES_XTAL_32K_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - the input enable of the pad"]
    #[inline(always)]
    pub fn x32p_fun_ie(&self) -> X32P_FUN_IE_R {
        X32P_FUN_IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32p_slp_oe(&self) -> X32P_SLP_OE_R {
        X32P_SLP_OE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32p_slp_ie(&self) -> X32P_SLP_IE_R {
        X32P_SLP_IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn x32p_slp_sel(&self) -> X32P_SLP_SEL_R {
        X32P_SLP_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn x32p_fun_sel(&self) -> X32P_FUN_SEL_R {
        X32P_FUN_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn x32n_fun_ie(&self) -> X32N_FUN_IE_R {
        X32N_FUN_IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32n_slp_oe(&self) -> X32N_SLP_OE_R {
        X32N_SLP_OE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32n_slp_ie(&self) -> X32N_SLP_IE_R {
        X32N_SLP_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn x32n_slp_sel(&self) -> X32N_SLP_SEL_R {
        X32N_SLP_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn x32n_fun_sel(&self) -> X32N_FUN_SEL_R {
        X32N_FUN_SEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn x32p_mux_sel(&self) -> X32P_MUX_SEL_R {
        X32P_MUX_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn x32n_mux_sel(&self) -> X32N_MUX_SEL_R {
        X32N_MUX_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Power up 32kHz crystal oscillator"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&self) -> XPD_XTAL_32K_R {
        XPD_XTAL_32K_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - 32K XTAL bias current DAC."]
    #[inline(always)]
    pub fn dac_xtal_32k(&self) -> DAC_XTAL_32K_R {
        DAC_XTAL_32K_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn x32p_rue(&self) -> X32P_RUE_R {
        X32P_RUE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn x32p_rde(&self) -> X32P_RDE_R {
        X32P_RDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn x32p_hold(&self) -> X32P_HOLD_R {
        X32P_HOLD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - the driver strength of the pad"]
    #[inline(always)]
    pub fn x32p_drv(&self) -> X32P_DRV_R {
        X32P_DRV_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn x32n_rue(&self) -> X32N_RUE_R {
        X32N_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn x32n_rde(&self) -> X32N_RDE_R {
        X32N_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn x32n_hold(&self) -> X32N_HOLD_R {
        X32N_HOLD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn x32n_drv(&self) -> X32N_DRV_R {
        X32N_DRV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL_32K_PAD")
            .field(
                "dbias_xtal_32k",
                &format_args!("{}", self.dbias_xtal_32k().bits()),
            )
            .field(
                "dres_xtal_32k",
                &format_args!("{}", self.dres_xtal_32k().bits()),
            )
            .field("x32p_fun_ie", &format_args!("{}", self.x32p_fun_ie().bit()))
            .field("x32p_slp_oe", &format_args!("{}", self.x32p_slp_oe().bit()))
            .field("x32p_slp_ie", &format_args!("{}", self.x32p_slp_ie().bit()))
            .field(
                "x32p_slp_sel",
                &format_args!("{}", self.x32p_slp_sel().bit()),
            )
            .field(
                "x32p_fun_sel",
                &format_args!("{}", self.x32p_fun_sel().bits()),
            )
            .field("x32n_fun_ie", &format_args!("{}", self.x32n_fun_ie().bit()))
            .field("x32n_slp_oe", &format_args!("{}", self.x32n_slp_oe().bit()))
            .field("x32n_slp_ie", &format_args!("{}", self.x32n_slp_ie().bit()))
            .field(
                "x32n_slp_sel",
                &format_args!("{}", self.x32n_slp_sel().bit()),
            )
            .field(
                "x32n_fun_sel",
                &format_args!("{}", self.x32n_fun_sel().bits()),
            )
            .field(
                "x32p_mux_sel",
                &format_args!("{}", self.x32p_mux_sel().bit()),
            )
            .field(
                "x32n_mux_sel",
                &format_args!("{}", self.x32n_mux_sel().bit()),
            )
            .field(
                "xpd_xtal_32k",
                &format_args!("{}", self.xpd_xtal_32k().bit()),
            )
            .field(
                "dac_xtal_32k",
                &format_args!("{}", self.dac_xtal_32k().bits()),
            )
            .field("x32p_rue", &format_args!("{}", self.x32p_rue().bit()))
            .field("x32p_rde", &format_args!("{}", self.x32p_rde().bit()))
            .field("x32p_hold", &format_args!("{}", self.x32p_hold().bit()))
            .field("x32p_drv", &format_args!("{}", self.x32p_drv().bits()))
            .field("x32n_rue", &format_args!("{}", self.x32n_rue().bit()))
            .field("x32n_rde", &format_args!("{}", self.x32n_rde().bit()))
            .field("x32n_hold", &format_args!("{}", self.x32n_hold().bit()))
            .field("x32n_drv", &format_args!("{}", self.x32n_drv().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTAL_32K_PAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 1:2 - 32K XTAL self-bias reference control."]
    #[inline(always)]
    #[must_use]
    pub fn dbias_xtal_32k(&mut self) -> DBIAS_XTAL_32K_W<1> {
        DBIAS_XTAL_32K_W::new(self)
    }
    #[doc = "Bits 3:4 - 32K XTAL resistor bias control."]
    #[inline(always)]
    #[must_use]
    pub fn dres_xtal_32k(&mut self) -> DRES_XTAL_32K_W<3> {
        DRES_XTAL_32K_W::new(self)
    }
    #[doc = "Bit 5 - the input enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_fun_ie(&mut self) -> X32P_FUN_IE_W<5> {
        X32P_FUN_IE_W::new(self)
    }
    #[doc = "Bit 6 - the output enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_slp_oe(&mut self) -> X32P_SLP_OE_W<6> {
        X32P_SLP_OE_W::new(self)
    }
    #[doc = "Bit 7 - the input enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_slp_ie(&mut self) -> X32P_SLP_IE_W<7> {
        X32P_SLP_IE_W::new(self)
    }
    #[doc = "Bit 8 - the sleep status selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_slp_sel(&mut self) -> X32P_SLP_SEL_W<8> {
        X32P_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 9:10 - the functional selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_fun_sel(&mut self) -> X32P_FUN_SEL_W<9> {
        X32P_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_fun_ie(&mut self) -> X32N_FUN_IE_W<11> {
        X32N_FUN_IE_W::new(self)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_slp_oe(&mut self) -> X32N_SLP_OE_W<12> {
        X32N_SLP_OE_W::new(self)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_slp_ie(&mut self) -> X32N_SLP_IE_W<13> {
        X32N_SLP_IE_W::new(self)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_slp_sel(&mut self) -> X32N_SLP_SEL_W<14> {
        X32N_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_fun_sel(&mut self) -> X32N_FUN_SEL_W<15> {
        X32N_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_mux_sel(&mut self) -> X32P_MUX_SEL_W<17> {
        X32P_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 18 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_mux_sel(&mut self) -> X32N_MUX_SEL_W<18> {
        X32N_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 19 - Power up 32kHz crystal oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_xtal_32k(&mut self) -> XPD_XTAL_32K_W<19> {
        XPD_XTAL_32K_W::new(self)
    }
    #[doc = "Bits 20:21 - 32K XTAL bias current DAC."]
    #[inline(always)]
    #[must_use]
    pub fn dac_xtal_32k(&mut self) -> DAC_XTAL_32K_W<20> {
        DAC_XTAL_32K_W::new(self)
    }
    #[doc = "Bit 22 - the pull up enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_rue(&mut self) -> X32P_RUE_W<22> {
        X32P_RUE_W::new(self)
    }
    #[doc = "Bit 23 - the pull down enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_rde(&mut self) -> X32P_RDE_W<23> {
        X32P_RDE_W::new(self)
    }
    #[doc = "Bit 24 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_hold(&mut self) -> X32P_HOLD_W<24> {
        X32P_HOLD_W::new(self)
    }
    #[doc = "Bits 25:26 - the driver strength of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_drv(&mut self) -> X32P_DRV_W<25> {
        X32P_DRV_W::new(self)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_rue(&mut self) -> X32N_RUE_W<27> {
        X32N_RUE_W::new(self)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_rde(&mut self) -> X32N_RDE_W<28> {
        X32N_RDE_W::new(self)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_hold(&mut self) -> X32N_HOLD_W<29> {
        X32N_HOLD_W::new(self)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_drv(&mut self) -> X32N_DRV_W<30> {
        X32N_DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_32k_pad](index.html) module"]
pub struct XTAL_32K_PAD_SPEC;
impl crate::RegisterSpec for XTAL_32K_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_32k_pad::R](R) reader structure"]
impl crate::Readable for XTAL_32K_PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_32k_pad::W](W) writer structure"]
impl crate::Writable for XTAL_32K_PAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTAL_32K_PAD to value 0x8410_0010"]
impl crate::Resettable for XTAL_32K_PAD_SPEC {
    const RESET_VALUE: Self::Ux = 0x8410_0010;
}
