#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_START_FORCE` reader - Need add description"]
pub type SARADC_START_FORCE_R = crate::BitReader;
#[doc = "Field `SARADC_START_FORCE` writer - Need add description"]
pub type SARADC_START_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SARADC_START` reader - Need add description"]
pub type SARADC_START_R = crate::BitReader;
#[doc = "Field `SARADC_START` writer - Need add description"]
pub type SARADC_START_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SARADC_SAR_CLK_GATED` reader - Need add description"]
pub type SARADC_SAR_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SARADC_SAR_CLK_GATED` writer - Need add description"]
pub type SARADC_SAR_CLK_GATED_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SARADC_SAR_CLK_DIV` reader - SAR clock divider"]
pub type SARADC_SAR_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SARADC_SAR_CLK_DIV` writer - SAR clock divider"]
pub type SARADC_SAR_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_SPEC, 8, O>;
#[doc = "Field `SARADC_SAR_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SARADC_SAR_PATT_LEN_R = crate::FieldReader;
#[doc = "Field `SARADC_SAR_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SARADC_SAR_PATT_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_SPEC, 3, O>;
#[doc = "Field `SARADC_SAR_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SARADC_SAR_PATT_P_CLEAR_R = crate::BitReader;
#[doc = "Field `SARADC_SAR_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SARADC_SAR_PATT_P_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SARADC_XPD_SAR_FORCE` reader - force option to xpd sar blocks"]
pub type SARADC_XPD_SAR_FORCE_R = crate::FieldReader;
#[doc = "Field `SARADC_XPD_SAR_FORCE` writer - force option to xpd sar blocks"]
pub type SARADC_XPD_SAR_FORCE_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_SPEC, 2, O>;
#[doc = "Field `SARADC_WAIT_ARB_CYCLE` reader - wait arbit signal stable after sar_done"]
pub type SARADC_WAIT_ARB_CYCLE_R = crate::FieldReader;
#[doc = "Field `SARADC_WAIT_ARB_CYCLE` writer - wait arbit signal stable after sar_done"]
pub type SARADC_WAIT_ARB_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn saradc_start_force(&self) -> SARADC_START_FORCE_R {
        SARADC_START_FORCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Need add description"]
    #[inline(always)]
    pub fn saradc_start(&self) -> SARADC_START_R {
        SARADC_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Need add description"]
    #[inline(always)]
    pub fn saradc_sar_clk_gated(&self) -> SARADC_SAR_CLK_GATED_R {
        SARADC_SAR_CLK_GATED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn saradc_sar_clk_div(&self) -> SARADC_SAR_CLK_DIV_R {
        SARADC_SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn saradc_sar_patt_len(&self) -> SARADC_SAR_PATT_LEN_R {
        SARADC_SAR_PATT_LEN_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn saradc_sar_patt_p_clear(&self) -> SARADC_SAR_PATT_P_CLEAR_R {
        SARADC_SAR_PATT_P_CLEAR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 27:28 - force option to xpd sar blocks"]
    #[inline(always)]
    pub fn saradc_xpd_sar_force(&self) -> SARADC_XPD_SAR_FORCE_R {
        SARADC_XPD_SAR_FORCE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 30:31 - wait arbit signal stable after sar_done"]
    #[inline(always)]
    pub fn saradc_wait_arb_cycle(&self) -> SARADC_WAIT_ARB_CYCLE_R {
        SARADC_WAIT_ARB_CYCLE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field(
                "saradc_start_force",
                &format_args!("{}", self.saradc_start_force().bit()),
            )
            .field(
                "saradc_start",
                &format_args!("{}", self.saradc_start().bit()),
            )
            .field(
                "saradc_sar_clk_gated",
                &format_args!("{}", self.saradc_sar_clk_gated().bit()),
            )
            .field(
                "saradc_sar_clk_div",
                &format_args!("{}", self.saradc_sar_clk_div().bits()),
            )
            .field(
                "saradc_sar_patt_len",
                &format_args!("{}", self.saradc_sar_patt_len().bits()),
            )
            .field(
                "saradc_sar_patt_p_clear",
                &format_args!("{}", self.saradc_sar_patt_p_clear().bit()),
            )
            .field(
                "saradc_xpd_sar_force",
                &format_args!("{}", self.saradc_xpd_sar_force().bits()),
            )
            .field(
                "saradc_wait_arb_cycle",
                &format_args!("{}", self.saradc_wait_arb_cycle().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_start_force(&mut self) -> SARADC_START_FORCE_W<0> {
        SARADC_START_FORCE_W::new(self)
    }
    #[doc = "Bit 1 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_start(&mut self) -> SARADC_START_W<1> {
        SARADC_START_W::new(self)
    }
    #[doc = "Bit 6 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_clk_gated(&mut self) -> SARADC_SAR_CLK_GATED_W<6> {
        SARADC_SAR_CLK_GATED_W::new(self)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_clk_div(&mut self) -> SARADC_SAR_CLK_DIV_W<7> {
        SARADC_SAR_CLK_DIV_W::new(self)
    }
    #[doc = "Bits 15:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_patt_len(&mut self) -> SARADC_SAR_PATT_LEN_W<15> {
        SARADC_SAR_PATT_LEN_W::new(self)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_patt_p_clear(&mut self) -> SARADC_SAR_PATT_P_CLEAR_W<23> {
        SARADC_SAR_PATT_P_CLEAR_W::new(self)
    }
    #[doc = "Bits 27:28 - force option to xpd sar blocks"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_xpd_sar_force(&mut self) -> SARADC_XPD_SAR_FORCE_W<27> {
        SARADC_XPD_SAR_FORCE_W::new(self)
    }
    #[doc = "Bits 30:31 - wait arbit signal stable after sar_done"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_wait_arb_cycle(&mut self) -> SARADC_WAIT_ARB_CYCLE_W<30> {
        SARADC_WAIT_ARB_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x4003_8240"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4003_8240;
}
