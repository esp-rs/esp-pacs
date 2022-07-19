#[doc = "Register `RTC_CLK_CONF` reader"]
pub struct R(crate::R<RTC_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CLK_CONF` writer"]
pub struct W(crate::W<RTC_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CLK_CONF_SPEC>;
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
impl From<crate::W<RTC_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFUSE_CLK_FORCE_GATING` reader - Need add desc"]
pub type EFUSE_CLK_FORCE_GATING_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_CLK_FORCE_GATING` writer - Need add desc"]
pub type EFUSE_CLK_FORCE_GATING_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 1>;
#[doc = "Field `EFUSE_CLK_FORCE_NOGATING` reader - Need add desc"]
pub type EFUSE_CLK_FORCE_NOGATING_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_CLK_FORCE_NOGATING` writer - Need add desc"]
pub type EFUSE_CLK_FORCE_NOGATING_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 2>;
#[doc = "Field `CK8M_DIV_SEL_VLD` reader - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel"]
pub type CK8M_DIV_SEL_VLD_R = crate::BitReader<bool>;
#[doc = "Field `CK8M_DIV_SEL_VLD` writer - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel"]
pub type CK8M_DIV_SEL_VLD_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 3>;
#[doc = "Field `CK8M_DIV` reader - CK8M_D256_OUT divider. 00: div128"]
pub type CK8M_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CK8M_DIV` writer - CK8M_D256_OUT divider. 00: div128"]
pub type CK8M_DIV_W<'a> = crate::FieldWriter<'a, u32, RTC_CLK_CONF_SPEC, u8, u8, 2, 4>;
#[doc = "Field `ENB_CK8M` reader - disable CK8M and CK8M_D256_OUT"]
pub type ENB_CK8M_R = crate::BitReader<bool>;
#[doc = "Field `ENB_CK8M` writer - disable CK8M and CK8M_D256_OUT"]
pub type ENB_CK8M_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 6>;
#[doc = "Field `ENB_CK8M_DIV` reader - 1: CK8M_D256_OUT is actually CK8M"]
pub type ENB_CK8M_DIV_R = crate::BitReader<bool>;
#[doc = "Field `ENB_CK8M_DIV` writer - 1: CK8M_D256_OUT is actually CK8M"]
pub type ENB_CK8M_DIV_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 7>;
#[doc = "Field `DIG_XTAL32K_EN` reader - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
pub type DIG_XTAL32K_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIG_XTAL32K_EN` writer - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
pub type DIG_XTAL32K_EN_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 8>;
#[doc = "Field `DIG_CLK8M_D256_EN` reader - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_D256_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIG_CLK8M_D256_EN` writer - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_D256_EN_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 9>;
#[doc = "Field `DIG_CLK8M_EN` reader - enable CK8M for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIG_CLK8M_EN` writer - enable CK8M for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_EN_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 10>;
#[doc = "Field `CK8M_DIV_SEL` reader - divider = reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CK8M_DIV_SEL` writer - divider = reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_W<'a> = crate::FieldWriter<'a, u32, RTC_CLK_CONF_SPEC, u8, u8, 3, 12>;
#[doc = "Field `XTAL_FORCE_NOGATING` reader - XTAL force no gating during sleep"]
pub type XTAL_FORCE_NOGATING_R = crate::BitReader<bool>;
#[doc = "Field `XTAL_FORCE_NOGATING` writer - XTAL force no gating during sleep"]
pub type XTAL_FORCE_NOGATING_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 15>;
#[doc = "Field `CK8M_FORCE_NOGATING` reader - CK8M force no gating during sleep"]
pub type CK8M_FORCE_NOGATING_R = crate::BitReader<bool>;
#[doc = "Field `CK8M_FORCE_NOGATING` writer - CK8M force no gating during sleep"]
pub type CK8M_FORCE_NOGATING_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 16>;
#[doc = "Field `CK8M_DFREQ` reader - CK8M_DFREQ"]
pub type CK8M_DFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CK8M_DFREQ` writer - CK8M_DFREQ"]
pub type CK8M_DFREQ_W<'a> = crate::FieldWriter<'a, u32, RTC_CLK_CONF_SPEC, u8, u8, 8, 17>;
#[doc = "Field `CK8M_FORCE_PD` reader - CK8M force power down"]
pub type CK8M_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `CK8M_FORCE_PD` writer - CK8M force power down"]
pub type CK8M_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 25>;
#[doc = "Field `CK8M_FORCE_PU` reader - CK8M force power up"]
pub type CK8M_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `CK8M_FORCE_PU` writer - CK8M force power up"]
pub type CK8M_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 26>;
#[doc = "Field `XTAL_GLOBAL_FORCE_GATING` reader - Need add desc"]
pub type XTAL_GLOBAL_FORCE_GATING_R = crate::BitReader<bool>;
#[doc = "Field `XTAL_GLOBAL_FORCE_GATING` writer - Need add desc"]
pub type XTAL_GLOBAL_FORCE_GATING_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 27>;
#[doc = "Field `XTAL_GLOBAL_FORCE_NOGATING` reader - Need add desc"]
pub type XTAL_GLOBAL_FORCE_NOGATING_R = crate::BitReader<bool>;
#[doc = "Field `XTAL_GLOBAL_FORCE_NOGATING` writer - Need add desc"]
pub type XTAL_GLOBAL_FORCE_NOGATING_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 28>;
#[doc = "Field `FAST_CLK_RTC_SEL` reader - fast_clk_rtc sel. 0: XTAL div 4"]
pub type FAST_CLK_RTC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `FAST_CLK_RTC_SEL` writer - fast_clk_rtc sel. 0: XTAL div 4"]
pub type FAST_CLK_RTC_SEL_W<'a> = crate::BitWriter<'a, u32, RTC_CLK_CONF_SPEC, bool, 29>;
#[doc = "Field `ANA_CLK_RTC_SEL` reader - Need add desc"]
pub type ANA_CLK_RTC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANA_CLK_RTC_SEL` writer - Need add desc"]
pub type ANA_CLK_RTC_SEL_W<'a> = crate::FieldWriter<'a, u32, RTC_CLK_CONF_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn efuse_clk_force_gating(&self) -> EFUSE_CLK_FORCE_GATING_R {
        EFUSE_CLK_FORCE_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Need add desc"]
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
    #[doc = "Bit 27 - Need add desc"]
    #[inline(always)]
    pub fn xtal_global_force_gating(&self) -> XTAL_GLOBAL_FORCE_GATING_R {
        XTAL_GLOBAL_FORCE_GATING_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn xtal_global_force_nogating(&self) -> XTAL_GLOBAL_FORCE_NOGATING_R {
        XTAL_GLOBAL_FORCE_NOGATING_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Need add desc"]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&self) -> ANA_CLK_RTC_SEL_R {
        ANA_CLK_RTC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn efuse_clk_force_gating(&mut self) -> EFUSE_CLK_FORCE_GATING_W {
        EFUSE_CLK_FORCE_GATING_W::new(self)
    }
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    pub fn efuse_clk_force_nogating(&mut self) -> EFUSE_CLK_FORCE_NOGATING_W {
        EFUSE_CLK_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bit 3 - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel"]
    #[inline(always)]
    pub fn ck8m_div_sel_vld(&mut self) -> CK8M_DIV_SEL_VLD_W {
        CK8M_DIV_SEL_VLD_W::new(self)
    }
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128"]
    #[inline(always)]
    pub fn ck8m_div(&mut self) -> CK8M_DIV_W {
        CK8M_DIV_W::new(self)
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    pub fn enb_ck8m(&mut self) -> ENB_CK8M_W {
        ENB_CK8M_W::new(self)
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M"]
    #[inline(always)]
    pub fn enb_ck8m_div(&mut self) -> ENB_CK8M_DIV_W {
        ENB_CK8M_DIV_W::new(self)
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_xtal32k_en(&mut self) -> DIG_XTAL32K_EN_W {
        DIG_XTAL32K_EN_W::new(self)
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&mut self) -> DIG_CLK8M_D256_EN_W {
        DIG_CLK8M_D256_EN_W::new(self)
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_en(&mut self) -> DIG_CLK8M_EN_W {
        DIG_CLK8M_EN_W::new(self)
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn ck8m_div_sel(&mut self) -> CK8M_DIV_SEL_W {
        CK8M_DIV_SEL_W::new(self)
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    pub fn xtal_force_nogating(&mut self) -> XTAL_FORCE_NOGATING_W {
        XTAL_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    pub fn ck8m_force_nogating(&mut self) -> CK8M_FORCE_NOGATING_W {
        CK8M_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn ck8m_dfreq(&mut self) -> CK8M_DFREQ_W {
        CK8M_DFREQ_W::new(self)
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    pub fn ck8m_force_pd(&mut self) -> CK8M_FORCE_PD_W {
        CK8M_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    pub fn ck8m_force_pu(&mut self) -> CK8M_FORCE_PU_W {
        CK8M_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 27 - Need add desc"]
    #[inline(always)]
    pub fn xtal_global_force_gating(&mut self) -> XTAL_GLOBAL_FORCE_GATING_W {
        XTAL_GLOBAL_FORCE_GATING_W::new(self)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn xtal_global_force_nogating(&mut self) -> XTAL_GLOBAL_FORCE_NOGATING_W {
        XTAL_GLOBAL_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W {
        FAST_CLK_RTC_SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - Need add desc"]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&mut self) -> ANA_CLK_RTC_SEL_W {
        ANA_CLK_RTC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_clk_conf](index.html) module"]
pub struct RTC_CLK_CONF_SPEC;
impl crate::RegisterSpec for RTC_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_clk_conf::R](R) reader structure"]
impl crate::Readable for RTC_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_clk_conf::W](W) writer structure"]
impl crate::Writable for RTC_CLK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CLK_CONF to value 0x1158_3218"]
impl crate::Resettable for RTC_CLK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1158_3218
    }
}
