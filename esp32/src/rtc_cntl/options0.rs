#[doc = "Register `OPTIONS0` reader"]
pub struct R(crate::R<OPTIONS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTIONS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTIONS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTIONS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTIONS0` writer"]
pub struct W(crate::W<OPTIONS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTIONS0_SPEC>;
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
impl From<crate::W<OPTIONS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTIONS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_STALL_APPCPU_C0` reader - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
pub type SW_STALL_APPCPU_C0_R = crate::FieldReader;
#[doc = "Field `SW_STALL_APPCPU_C0` writer - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
pub type SW_STALL_APPCPU_C0_W<'a, const O: u8> = crate::FieldWriter<'a, OPTIONS0_SPEC, 2, O>;
#[doc = "Field `SW_STALL_PROCPU_C0` reader - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
pub type SW_STALL_PROCPU_C0_R = crate::FieldReader;
#[doc = "Field `SW_STALL_PROCPU_C0` writer - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
pub type SW_STALL_PROCPU_C0_W<'a, const O: u8> = crate::FieldWriter<'a, OPTIONS0_SPEC, 2, O>;
#[doc = "Field `SW_APPCPU_RST` writer - APP CPU SW reset"]
pub type SW_APPCPU_RST_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `SW_PROCPU_RST` writer - PRO CPU SW reset"]
pub type SW_PROCPU_RST_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BB_I2C_FORCE_PD` reader - BB_I2C force power down"]
pub type BB_I2C_FORCE_PD_R = crate::BitReader;
#[doc = "Field `BB_I2C_FORCE_PD` writer - BB_I2C force power down"]
pub type BB_I2C_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BB_I2C_FORCE_PU` reader - BB_I2C force power up"]
pub type BB_I2C_FORCE_PU_R = crate::BitReader;
#[doc = "Field `BB_I2C_FORCE_PU` writer - BB_I2C force power up"]
pub type BB_I2C_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BBPLL_I2C_FORCE_PD` reader - BB_PLL _I2C force power down"]
pub type BBPLL_I2C_FORCE_PD_R = crate::BitReader;
#[doc = "Field `BBPLL_I2C_FORCE_PD` writer - BB_PLL _I2C force power down"]
pub type BBPLL_I2C_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BBPLL_I2C_FORCE_PU` reader - BB_PLL_I2C force power up"]
pub type BBPLL_I2C_FORCE_PU_R = crate::BitReader;
#[doc = "Field `BBPLL_I2C_FORCE_PU` writer - BB_PLL_I2C force power up"]
pub type BBPLL_I2C_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BBPLL_FORCE_PD` reader - BB_PLL force power down"]
pub type BBPLL_FORCE_PD_R = crate::BitReader;
#[doc = "Field `BBPLL_FORCE_PD` writer - BB_PLL force power down"]
pub type BBPLL_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BBPLL_FORCE_PU` reader - BB_PLL force power up"]
pub type BBPLL_FORCE_PU_R = crate::BitReader;
#[doc = "Field `BBPLL_FORCE_PU` writer - BB_PLL force power up"]
pub type BBPLL_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `XTL_FORCE_PD` reader - crystall force power down"]
pub type XTL_FORCE_PD_R = crate::BitReader;
#[doc = "Field `XTL_FORCE_PD` writer - crystall force power down"]
pub type XTL_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `XTL_FORCE_PU` reader - crystall force power up"]
pub type XTL_FORCE_PU_R = crate::BitReader;
#[doc = "Field `XTL_FORCE_PU` writer - crystall force power up"]
pub type XTL_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BIAS_SLEEP_FOLW_8M` reader - BIAS_SLEEP follow CK8M"]
pub type BIAS_SLEEP_FOLW_8M_R = crate::BitReader;
#[doc = "Field `BIAS_SLEEP_FOLW_8M` writer - BIAS_SLEEP follow CK8M"]
pub type BIAS_SLEEP_FOLW_8M_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BIAS_FORCE_SLEEP` reader - BIAS_SLEEP force sleep"]
pub type BIAS_FORCE_SLEEP_R = crate::BitReader;
#[doc = "Field `BIAS_FORCE_SLEEP` writer - BIAS_SLEEP force sleep"]
pub type BIAS_FORCE_SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BIAS_FORCE_NOSLEEP` reader - BIAS_SLEEP force no sleep"]
pub type BIAS_FORCE_NOSLEEP_R = crate::BitReader;
#[doc = "Field `BIAS_FORCE_NOSLEEP` writer - BIAS_SLEEP force no sleep"]
pub type BIAS_FORCE_NOSLEEP_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BIAS_I2C_FOLW_8M` reader - BIAS_I2C follow CK8M"]
pub type BIAS_I2C_FOLW_8M_R = crate::BitReader;
#[doc = "Field `BIAS_I2C_FOLW_8M` writer - BIAS_I2C follow CK8M"]
pub type BIAS_I2C_FOLW_8M_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BIAS_I2C_FORCE_PD` reader - BIAS_I2C force power down"]
pub type BIAS_I2C_FORCE_PD_R = crate::BitReader;
#[doc = "Field `BIAS_I2C_FORCE_PD` writer - BIAS_I2C force power down"]
pub type BIAS_I2C_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BIAS_I2C_FORCE_PU` reader - BIAS_I2C force power up"]
pub type BIAS_I2C_FORCE_PU_R = crate::BitReader;
#[doc = "Field `BIAS_I2C_FORCE_PU` writer - BIAS_I2C force power up"]
pub type BIAS_I2C_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BIAS_CORE_FOLW_8M` reader - BIAS_CORE follow CK8M"]
pub type BIAS_CORE_FOLW_8M_R = crate::BitReader;
#[doc = "Field `BIAS_CORE_FOLW_8M` writer - BIAS_CORE follow CK8M"]
pub type BIAS_CORE_FOLW_8M_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BIAS_CORE_FORCE_PD` reader - BIAS_CORE force power down"]
pub type BIAS_CORE_FORCE_PD_R = crate::BitReader;
#[doc = "Field `BIAS_CORE_FORCE_PD` writer - BIAS_CORE force power down"]
pub type BIAS_CORE_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `BIAS_CORE_FORCE_PU` reader - BIAS_CORE force power up"]
pub type BIAS_CORE_FORCE_PU_R = crate::BitReader;
#[doc = "Field `BIAS_CORE_FORCE_PU` writer - BIAS_CORE force power up"]
pub type BIAS_CORE_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `XTL_FORCE_ISO` reader - "]
pub type XTL_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `XTL_FORCE_ISO` writer - "]
pub type XTL_FORCE_ISO_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `PLL_FORCE_ISO` reader - "]
pub type PLL_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `PLL_FORCE_ISO` writer - "]
pub type PLL_FORCE_ISO_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `ANALOG_FORCE_ISO` reader - "]
pub type ANALOG_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `ANALOG_FORCE_ISO` writer - "]
pub type ANALOG_FORCE_ISO_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `XTL_FORCE_NOISO` reader - "]
pub type XTL_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `XTL_FORCE_NOISO` writer - "]
pub type XTL_FORCE_NOISO_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `PLL_FORCE_NOISO` reader - "]
pub type PLL_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `PLL_FORCE_NOISO` writer - "]
pub type PLL_FORCE_NOISO_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `ANALOG_FORCE_NOISO` reader - "]
pub type ANALOG_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `ANALOG_FORCE_NOISO` writer - "]
pub type ANALOG_FORCE_NOISO_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `DG_WRAP_FORCE_RST` reader - digital wrap force reset in deep sleep"]
pub type DG_WRAP_FORCE_RST_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_RST` writer - digital wrap force reset in deep sleep"]
pub type DG_WRAP_FORCE_RST_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `DG_WRAP_FORCE_NORST` reader - digital core force no reset in deep sleep"]
pub type DG_WRAP_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_NORST` writer - digital core force no reset in deep sleep"]
pub type DG_WRAP_FORCE_NORST_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
#[doc = "Field `SW_SYS_RST` writer - SW system reset"]
pub type SW_SYS_RST_W<'a, const O: u8> = crate::BitWriter<'a, OPTIONS0_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c0(&self) -> SW_STALL_APPCPU_C0_R {
        SW_STALL_APPCPU_C0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&self) -> SW_STALL_PROCPU_C0_R {
        SW_STALL_PROCPU_C0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&self) -> BB_I2C_FORCE_PD_R {
        BB_I2C_FORCE_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&self) -> BB_I2C_FORCE_PU_R {
        BB_I2C_FORCE_PU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&self) -> BBPLL_I2C_FORCE_PD_R {
        BBPLL_I2C_FORCE_PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&self) -> BBPLL_I2C_FORCE_PU_R {
        BBPLL_I2C_FORCE_PU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn bbpll_force_pd(&self) -> BBPLL_FORCE_PD_R {
        BBPLL_FORCE_PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn bbpll_force_pu(&self) -> BBPLL_FORCE_PU_R {
        BBPLL_FORCE_PU_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn xtl_force_pd(&self) -> XTL_FORCE_PD_R {
        XTL_FORCE_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn xtl_force_pu(&self) -> XTL_FORCE_PU_R {
        XTL_FORCE_PU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BIAS_SLEEP follow CK8M"]
    #[inline(always)]
    pub fn bias_sleep_folw_8m(&self) -> BIAS_SLEEP_FOLW_8M_R {
        BIAS_SLEEP_FOLW_8M_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BIAS_SLEEP force sleep"]
    #[inline(always)]
    pub fn bias_force_sleep(&self) -> BIAS_FORCE_SLEEP_R {
        BIAS_FORCE_SLEEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BIAS_SLEEP force no sleep"]
    #[inline(always)]
    pub fn bias_force_nosleep(&self) -> BIAS_FORCE_NOSLEEP_R {
        BIAS_FORCE_NOSLEEP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BIAS_I2C follow CK8M"]
    #[inline(always)]
    pub fn bias_i2c_folw_8m(&self) -> BIAS_I2C_FOLW_8M_R {
        BIAS_I2C_FOLW_8M_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BIAS_I2C force power down"]
    #[inline(always)]
    pub fn bias_i2c_force_pd(&self) -> BIAS_I2C_FORCE_PD_R {
        BIAS_I2C_FORCE_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BIAS_I2C force power up"]
    #[inline(always)]
    pub fn bias_i2c_force_pu(&self) -> BIAS_I2C_FORCE_PU_R {
        BIAS_I2C_FORCE_PU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BIAS_CORE follow CK8M"]
    #[inline(always)]
    pub fn bias_core_folw_8m(&self) -> BIAS_CORE_FOLW_8M_R {
        BIAS_CORE_FOLW_8M_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BIAS_CORE force power down"]
    #[inline(always)]
    pub fn bias_core_force_pd(&self) -> BIAS_CORE_FORCE_PD_R {
        BIAS_CORE_FORCE_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - BIAS_CORE force power up"]
    #[inline(always)]
    pub fn bias_core_force_pu(&self) -> BIAS_CORE_FORCE_PU_R {
        BIAS_CORE_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn xtl_force_iso(&self) -> XTL_FORCE_ISO_R {
        XTL_FORCE_ISO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pll_force_iso(&self) -> PLL_FORCE_ISO_R {
        PLL_FORCE_ISO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn analog_force_iso(&self) -> ANALOG_FORCE_ISO_R {
        ANALOG_FORCE_ISO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn xtl_force_noiso(&self) -> XTL_FORCE_NOISO_R {
        XTL_FORCE_NOISO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pll_force_noiso(&self) -> PLL_FORCE_NOISO_R {
        PLL_FORCE_NOISO_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn analog_force_noiso(&self) -> ANALOG_FORCE_NOISO_R {
        ANALOG_FORCE_NOISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&self) -> DG_WRAP_FORCE_RST_R {
        DG_WRAP_FORCE_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&self) -> DG_WRAP_FORCE_NORST_R {
        DG_WRAP_FORCE_NORST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTIONS0")
            .field(
                "sw_stall_appcpu_c0",
                &format_args!("{}", self.sw_stall_appcpu_c0().bits()),
            )
            .field(
                "sw_stall_procpu_c0",
                &format_args!("{}", self.sw_stall_procpu_c0().bits()),
            )
            .field(
                "bb_i2c_force_pd",
                &format_args!("{}", self.bb_i2c_force_pd().bit()),
            )
            .field(
                "bb_i2c_force_pu",
                &format_args!("{}", self.bb_i2c_force_pu().bit()),
            )
            .field(
                "bbpll_i2c_force_pd",
                &format_args!("{}", self.bbpll_i2c_force_pd().bit()),
            )
            .field(
                "bbpll_i2c_force_pu",
                &format_args!("{}", self.bbpll_i2c_force_pu().bit()),
            )
            .field(
                "bbpll_force_pd",
                &format_args!("{}", self.bbpll_force_pd().bit()),
            )
            .field(
                "bbpll_force_pu",
                &format_args!("{}", self.bbpll_force_pu().bit()),
            )
            .field(
                "xtl_force_pd",
                &format_args!("{}", self.xtl_force_pd().bit()),
            )
            .field(
                "xtl_force_pu",
                &format_args!("{}", self.xtl_force_pu().bit()),
            )
            .field(
                "bias_sleep_folw_8m",
                &format_args!("{}", self.bias_sleep_folw_8m().bit()),
            )
            .field(
                "bias_force_sleep",
                &format_args!("{}", self.bias_force_sleep().bit()),
            )
            .field(
                "bias_force_nosleep",
                &format_args!("{}", self.bias_force_nosleep().bit()),
            )
            .field(
                "bias_i2c_folw_8m",
                &format_args!("{}", self.bias_i2c_folw_8m().bit()),
            )
            .field(
                "bias_i2c_force_pd",
                &format_args!("{}", self.bias_i2c_force_pd().bit()),
            )
            .field(
                "bias_i2c_force_pu",
                &format_args!("{}", self.bias_i2c_force_pu().bit()),
            )
            .field(
                "bias_core_folw_8m",
                &format_args!("{}", self.bias_core_folw_8m().bit()),
            )
            .field(
                "bias_core_force_pd",
                &format_args!("{}", self.bias_core_force_pd().bit()),
            )
            .field(
                "bias_core_force_pu",
                &format_args!("{}", self.bias_core_force_pu().bit()),
            )
            .field(
                "xtl_force_iso",
                &format_args!("{}", self.xtl_force_iso().bit()),
            )
            .field(
                "pll_force_iso",
                &format_args!("{}", self.pll_force_iso().bit()),
            )
            .field(
                "analog_force_iso",
                &format_args!("{}", self.analog_force_iso().bit()),
            )
            .field(
                "xtl_force_noiso",
                &format_args!("{}", self.xtl_force_noiso().bit()),
            )
            .field(
                "pll_force_noiso",
                &format_args!("{}", self.pll_force_noiso().bit()),
            )
            .field(
                "analog_force_noiso",
                &format_args!("{}", self.analog_force_noiso().bit()),
            )
            .field(
                "dg_wrap_force_rst",
                &format_args!("{}", self.dg_wrap_force_rst().bit()),
            )
            .field(
                "dg_wrap_force_norst",
                &format_args!("{}", self.dg_wrap_force_norst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OPTIONS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    #[must_use]
    pub fn sw_stall_appcpu_c0(&mut self) -> SW_STALL_APPCPU_C0_W<0> {
        SW_STALL_APPCPU_C0_W::new(self)
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    #[must_use]
    pub fn sw_stall_procpu_c0(&mut self) -> SW_STALL_PROCPU_C0_W<2> {
        SW_STALL_PROCPU_C0_W::new(self)
    }
    #[doc = "Bit 4 - APP CPU SW reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_appcpu_rst(&mut self) -> SW_APPCPU_RST_W<4> {
        SW_APPCPU_RST_W::new(self)
    }
    #[doc = "Bit 5 - PRO CPU SW reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_procpu_rst(&mut self) -> SW_PROCPU_RST_W<5> {
        SW_PROCPU_RST_W::new(self)
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    #[must_use]
    pub fn bb_i2c_force_pd(&mut self) -> BB_I2C_FORCE_PD_W<6> {
        BB_I2C_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    #[must_use]
    pub fn bb_i2c_force_pu(&mut self) -> BB_I2C_FORCE_PU_W<7> {
        BB_I2C_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_i2c_force_pd(&mut self) -> BBPLL_I2C_FORCE_PD_W<8> {
        BBPLL_I2C_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_i2c_force_pu(&mut self) -> BBPLL_I2C_FORCE_PU_W<9> {
        BBPLL_I2C_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_force_pd(&mut self) -> BBPLL_FORCE_PD_W<10> {
        BBPLL_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_force_pu(&mut self) -> BBPLL_FORCE_PU_W<11> {
        BBPLL_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    #[must_use]
    pub fn xtl_force_pd(&mut self) -> XTL_FORCE_PD_W<12> {
        XTL_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    #[must_use]
    pub fn xtl_force_pu(&mut self) -> XTL_FORCE_PU_W<13> {
        XTL_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 14 - BIAS_SLEEP follow CK8M"]
    #[inline(always)]
    #[must_use]
    pub fn bias_sleep_folw_8m(&mut self) -> BIAS_SLEEP_FOLW_8M_W<14> {
        BIAS_SLEEP_FOLW_8M_W::new(self)
    }
    #[doc = "Bit 15 - BIAS_SLEEP force sleep"]
    #[inline(always)]
    #[must_use]
    pub fn bias_force_sleep(&mut self) -> BIAS_FORCE_SLEEP_W<15> {
        BIAS_FORCE_SLEEP_W::new(self)
    }
    #[doc = "Bit 16 - BIAS_SLEEP force no sleep"]
    #[inline(always)]
    #[must_use]
    pub fn bias_force_nosleep(&mut self) -> BIAS_FORCE_NOSLEEP_W<16> {
        BIAS_FORCE_NOSLEEP_W::new(self)
    }
    #[doc = "Bit 17 - BIAS_I2C follow CK8M"]
    #[inline(always)]
    #[must_use]
    pub fn bias_i2c_folw_8m(&mut self) -> BIAS_I2C_FOLW_8M_W<17> {
        BIAS_I2C_FOLW_8M_W::new(self)
    }
    #[doc = "Bit 18 - BIAS_I2C force power down"]
    #[inline(always)]
    #[must_use]
    pub fn bias_i2c_force_pd(&mut self) -> BIAS_I2C_FORCE_PD_W<18> {
        BIAS_I2C_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 19 - BIAS_I2C force power up"]
    #[inline(always)]
    #[must_use]
    pub fn bias_i2c_force_pu(&mut self) -> BIAS_I2C_FORCE_PU_W<19> {
        BIAS_I2C_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 20 - BIAS_CORE follow CK8M"]
    #[inline(always)]
    #[must_use]
    pub fn bias_core_folw_8m(&mut self) -> BIAS_CORE_FOLW_8M_W<20> {
        BIAS_CORE_FOLW_8M_W::new(self)
    }
    #[doc = "Bit 21 - BIAS_CORE force power down"]
    #[inline(always)]
    #[must_use]
    pub fn bias_core_force_pd(&mut self) -> BIAS_CORE_FORCE_PD_W<21> {
        BIAS_CORE_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 22 - BIAS_CORE force power up"]
    #[inline(always)]
    #[must_use]
    pub fn bias_core_force_pu(&mut self) -> BIAS_CORE_FORCE_PU_W<22> {
        BIAS_CORE_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn xtl_force_iso(&mut self) -> XTL_FORCE_ISO_W<23> {
        XTL_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pll_force_iso(&mut self) -> PLL_FORCE_ISO_W<24> {
        PLL_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn analog_force_iso(&mut self) -> ANALOG_FORCE_ISO_W<25> {
        ANALOG_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn xtl_force_noiso(&mut self) -> XTL_FORCE_NOISO_W<26> {
        XTL_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn pll_force_noiso(&mut self) -> PLL_FORCE_NOISO_W<27> {
        PLL_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn analog_force_noiso(&mut self) -> ANALOG_FORCE_NOISO_W<28> {
        ANALOG_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_rst(&mut self) -> DG_WRAP_FORCE_RST_W<29> {
        DG_WRAP_FORCE_RST_W::new(self)
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_norst(&mut self) -> DG_WRAP_FORCE_NORST_W<30> {
        DG_WRAP_FORCE_NORST_W::new(self)
    }
    #[doc = "Bit 31 - SW system reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sys_rst(&mut self) -> SW_SYS_RST_W<31> {
        SW_SYS_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options0](index.html) module"]
pub struct OPTIONS0_SPEC;
impl crate::RegisterSpec for OPTIONS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [options0::R](R) reader structure"]
impl crate::Readable for OPTIONS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [options0::W](W) writer structure"]
impl crate::Writable for OPTIONS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTIONS0 to value 0x1c49_2000"]
impl crate::Resettable for OPTIONS0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c49_2000;
}
