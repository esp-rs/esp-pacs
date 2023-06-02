#[doc = "Register `CLK_CONF` reader"]
pub struct R(crate::R<CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF` writer"]
pub struct W(crate::W<CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF_SPEC>;
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
impl From<crate::W<CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFUSE_CLK_FORCE_GATING` reader - efuse_clk_force_gating"]
pub type EFUSE_CLK_FORCE_GATING_R = crate::BitReader;
#[doc = "Field `EFUSE_CLK_FORCE_GATING` writer - efuse_clk_force_gating"]
pub type EFUSE_CLK_FORCE_GATING_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `EFUSE_CLK_FORCE_NOGATING` reader - efuse_clk_force_nogating"]
pub type EFUSE_CLK_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `EFUSE_CLK_FORCE_NOGATING` writer - efuse_clk_force_nogating"]
pub type EFUSE_CLK_FORCE_NOGATING_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_DIV_SEL_VLD` reader - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel"]
pub type CK8M_DIV_SEL_VLD_R = crate::BitReader;
#[doc = "Field `CK8M_DIV_SEL_VLD` writer - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel"]
pub type CK8M_DIV_SEL_VLD_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_DIV` reader - CK8M_D256_OUT divider. 00: div128"]
pub type CK8M_DIV_R = crate::FieldReader;
#[doc = "Field `CK8M_DIV` writer - CK8M_D256_OUT divider. 00: div128"]
pub type CK8M_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_SPEC, 2, O>;
#[doc = "Field `ENB_CK8M` reader - disable CK8M and CK8M_D256_OUT"]
pub type ENB_CK8M_R = crate::BitReader;
#[doc = "Field `ENB_CK8M` writer - disable CK8M and CK8M_D256_OUT"]
pub type ENB_CK8M_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `ENB_CK8M_DIV` reader - 1: CK8M_D256_OUT is actually CK8M"]
pub type ENB_CK8M_DIV_R = crate::BitReader;
#[doc = "Field `ENB_CK8M_DIV` writer - 1: CK8M_D256_OUT is actually CK8M"]
pub type ENB_CK8M_DIV_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `DIG_XTAL32K_EN` reader - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
pub type DIG_XTAL32K_EN_R = crate::BitReader;
#[doc = "Field `DIG_XTAL32K_EN` writer - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
pub type DIG_XTAL32K_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `DIG_CLK8M_D256_EN` reader - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_D256_EN_R = crate::BitReader;
#[doc = "Field `DIG_CLK8M_D256_EN` writer - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_D256_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `DIG_CLK8M_EN` reader - enable CK8M for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_EN_R = crate::BitReader;
#[doc = "Field `DIG_CLK8M_EN` writer - enable CK8M for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_DIV_SEL` reader - divider = reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_R = crate::FieldReader;
#[doc = "Field `CK8M_DIV_SEL` writer - divider = reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_SPEC, 3, O>;
#[doc = "Field `XTAL_FORCE_NOGATING` reader - XTAL force no gating during sleep"]
pub type XTAL_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `XTAL_FORCE_NOGATING` writer - XTAL force no gating during sleep"]
pub type XTAL_FORCE_NOGATING_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_FORCE_NOGATING` reader - CK8M force no gating during sleep"]
pub type CK8M_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_NOGATING` writer - CK8M force no gating during sleep"]
pub type CK8M_FORCE_NOGATING_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_DFREQ` reader - CK8M_DFREQ"]
pub type CK8M_DFREQ_R = crate::FieldReader;
#[doc = "Field `CK8M_DFREQ` writer - CK8M_DFREQ"]
pub type CK8M_DFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_SPEC, 8, O>;
#[doc = "Field `CK8M_FORCE_PD` reader - CK8M force power down"]
pub type CK8M_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_PD` writer - CK8M force power down"]
pub type CK8M_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_FORCE_PU` reader - CK8M force power up"]
pub type CK8M_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_PU` writer - CK8M force power up"]
pub type CK8M_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `XTAL_GLOBAL_FORCE_GATING` reader - force enable xtal clk gating"]
pub type XTAL_GLOBAL_FORCE_GATING_R = crate::BitReader;
#[doc = "Field `XTAL_GLOBAL_FORCE_GATING` writer - force enable xtal clk gating"]
pub type XTAL_GLOBAL_FORCE_GATING_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `XTAL_GLOBAL_FORCE_NOGATING` reader - force bypass xtal clk gating"]
pub type XTAL_GLOBAL_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `XTAL_GLOBAL_FORCE_NOGATING` writer - force bypass xtal clk gating"]
pub type XTAL_GLOBAL_FORCE_NOGATING_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `FAST_CLK_RTC_SEL` reader - fast_clk_rtc sel. 0: XTAL div 4"]
pub type FAST_CLK_RTC_SEL_R = crate::BitReader;
#[doc = "Field `FAST_CLK_RTC_SEL` writer - fast_clk_rtc sel. 0: XTAL div 4"]
pub type FAST_CLK_RTC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `ANA_CLK_RTC_SEL` reader - slelect rtc slow clk"]
pub type ANA_CLK_RTC_SEL_R = crate::FieldReader;
#[doc = "Field `ANA_CLK_RTC_SEL` writer - slelect rtc slow clk"]
pub type ANA_CLK_RTC_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_SPEC, 2, O>;
impl R {
    #[doc = "Bit 1 - efuse_clk_force_gating"]
    #[inline(always)]
    pub fn efuse_clk_force_gating(&self) -> EFUSE_CLK_FORCE_GATING_R {
        EFUSE_CLK_FORCE_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - efuse_clk_force_nogating"]
    #[inline(always)]
    pub fn efuse_clk_force_nogating(&self) -> EFUSE_CLK_FORCE_NOGATING_R {
        EFUSE_CLK_FORCE_NOGATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel"]
    #[inline(always)]
    pub fn ck8m_div_sel_vld(&self) -> CK8M_DIV_SEL_VLD_R {
        CK8M_DIV_SEL_VLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128"]
    #[inline(always)]
    pub fn ck8m_div(&self) -> CK8M_DIV_R {
        CK8M_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    pub fn enb_ck8m(&self) -> ENB_CK8M_R {
        ENB_CK8M_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M"]
    #[inline(always)]
    pub fn enb_ck8m_div(&self) -> ENB_CK8M_DIV_R {
        ENB_CK8M_DIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_xtal32k_en(&self) -> DIG_XTAL32K_EN_R {
        DIG_XTAL32K_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&self) -> DIG_CLK8M_D256_EN_R {
        DIG_CLK8M_D256_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_en(&self) -> DIG_CLK8M_EN_R {
        DIG_CLK8M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn ck8m_div_sel(&self) -> CK8M_DIV_SEL_R {
        CK8M_DIV_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    pub fn xtal_force_nogating(&self) -> XTAL_FORCE_NOGATING_R {
        XTAL_FORCE_NOGATING_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    pub fn ck8m_force_nogating(&self) -> CK8M_FORCE_NOGATING_R {
        CK8M_FORCE_NOGATING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn ck8m_dfreq(&self) -> CK8M_DFREQ_R {
        CK8M_DFREQ_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    pub fn ck8m_force_pd(&self) -> CK8M_FORCE_PD_R {
        CK8M_FORCE_PD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    pub fn ck8m_force_pu(&self) -> CK8M_FORCE_PU_R {
        CK8M_FORCE_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - force enable xtal clk gating"]
    #[inline(always)]
    pub fn xtal_global_force_gating(&self) -> XTAL_GLOBAL_FORCE_GATING_R {
        XTAL_GLOBAL_FORCE_GATING_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - force bypass xtal clk gating"]
    #[inline(always)]
    pub fn xtal_global_force_nogating(&self) -> XTAL_GLOBAL_FORCE_NOGATING_R {
        XTAL_GLOBAL_FORCE_NOGATING_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - slelect rtc slow clk"]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&self) -> ANA_CLK_RTC_SEL_R {
        ANA_CLK_RTC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field(
                "efuse_clk_force_gating",
                &format_args!("{}", self.efuse_clk_force_gating().bit()),
            )
            .field(
                "efuse_clk_force_nogating",
                &format_args!("{}", self.efuse_clk_force_nogating().bit()),
            )
            .field(
                "ck8m_div_sel_vld",
                &format_args!("{}", self.ck8m_div_sel_vld().bit()),
            )
            .field("ck8m_div", &format_args!("{}", self.ck8m_div().bits()))
            .field("enb_ck8m", &format_args!("{}", self.enb_ck8m().bit()))
            .field(
                "enb_ck8m_div",
                &format_args!("{}", self.enb_ck8m_div().bit()),
            )
            .field(
                "dig_xtal32k_en",
                &format_args!("{}", self.dig_xtal32k_en().bit()),
            )
            .field(
                "dig_clk8m_d256_en",
                &format_args!("{}", self.dig_clk8m_d256_en().bit()),
            )
            .field(
                "dig_clk8m_en",
                &format_args!("{}", self.dig_clk8m_en().bit()),
            )
            .field(
                "ck8m_div_sel",
                &format_args!("{}", self.ck8m_div_sel().bits()),
            )
            .field(
                "xtal_force_nogating",
                &format_args!("{}", self.xtal_force_nogating().bit()),
            )
            .field(
                "ck8m_force_nogating",
                &format_args!("{}", self.ck8m_force_nogating().bit()),
            )
            .field("ck8m_dfreq", &format_args!("{}", self.ck8m_dfreq().bits()))
            .field(
                "ck8m_force_pd",
                &format_args!("{}", self.ck8m_force_pd().bit()),
            )
            .field(
                "ck8m_force_pu",
                &format_args!("{}", self.ck8m_force_pu().bit()),
            )
            .field(
                "xtal_global_force_gating",
                &format_args!("{}", self.xtal_global_force_gating().bit()),
            )
            .field(
                "xtal_global_force_nogating",
                &format_args!("{}", self.xtal_global_force_nogating().bit()),
            )
            .field(
                "fast_clk_rtc_sel",
                &format_args!("{}", self.fast_clk_rtc_sel().bit()),
            )
            .field(
                "ana_clk_rtc_sel",
                &format_args!("{}", self.ana_clk_rtc_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - efuse_clk_force_gating"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_clk_force_gating(&mut self) -> EFUSE_CLK_FORCE_GATING_W<1> {
        EFUSE_CLK_FORCE_GATING_W::new(self)
    }
    #[doc = "Bit 2 - efuse_clk_force_nogating"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_clk_force_nogating(&mut self) -> EFUSE_CLK_FORCE_NOGATING_W<2> {
        EFUSE_CLK_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bit 3 - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div_sel_vld(&mut self) -> CK8M_DIV_SEL_VLD_W<3> {
        CK8M_DIV_SEL_VLD_W::new(self)
    }
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div(&mut self) -> CK8M_DIV_W<4> {
        CK8M_DIV_W::new(self)
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn enb_ck8m(&mut self) -> ENB_CK8M_W<6> {
        ENB_CK8M_W::new(self)
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M"]
    #[inline(always)]
    #[must_use]
    pub fn enb_ck8m_div(&mut self) -> ENB_CK8M_DIV_W<7> {
        ENB_CK8M_DIV_W::new(self)
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    #[must_use]
    pub fn dig_xtal32k_en(&mut self) -> DIG_XTAL32K_EN_W<8> {
        DIG_XTAL32K_EN_W::new(self)
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    #[must_use]
    pub fn dig_clk8m_d256_en(&mut self) -> DIG_CLK8M_D256_EN_W<9> {
        DIG_CLK8M_D256_EN_W::new(self)
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    #[must_use]
    pub fn dig_clk8m_en(&mut self) -> DIG_CLK8M_EN_W<10> {
        DIG_CLK8M_EN_W::new(self)
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div_sel(&mut self) -> CK8M_DIV_SEL_W<12> {
        CK8M_DIV_SEL_W::new(self)
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_force_nogating(&mut self) -> XTAL_FORCE_NOGATING_W<15> {
        XTAL_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_nogating(&mut self) -> CK8M_FORCE_NOGATING_W<16> {
        CK8M_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_dfreq(&mut self) -> CK8M_DFREQ_W<17> {
        CK8M_DFREQ_W::new(self)
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_pd(&mut self) -> CK8M_FORCE_PD_W<25> {
        CK8M_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_pu(&mut self) -> CK8M_FORCE_PU_W<26> {
        CK8M_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 27 - force enable xtal clk gating"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_global_force_gating(&mut self) -> XTAL_GLOBAL_FORCE_GATING_W<27> {
        XTAL_GLOBAL_FORCE_GATING_W::new(self)
    }
    #[doc = "Bit 28 - force bypass xtal clk gating"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_global_force_nogating(&mut self) -> XTAL_GLOBAL_FORCE_NOGATING_W<28> {
        XTAL_GLOBAL_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4"]
    #[inline(always)]
    #[must_use]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W<29> {
        FAST_CLK_RTC_SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - slelect rtc slow clk"]
    #[inline(always)]
    #[must_use]
    pub fn ana_clk_rtc_sel(&mut self) -> ANA_CLK_RTC_SEL_W<30> {
        ANA_CLK_RTC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf](index.html) module"]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf::R](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf::W](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x1158_3218"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1158_3218;
}
