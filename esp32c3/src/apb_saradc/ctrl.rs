#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SARADC_START_FORCE` reader - select software enable saradc sample"]
pub type SARADC_START_FORCE_R = crate::BitReader;
#[doc = "Field `SARADC_START_FORCE` writer - select software enable saradc sample"]
pub type SARADC_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_START` reader - software enable saradc sample"]
pub type SARADC_START_R = crate::BitReader;
#[doc = "Field `SARADC_START` writer - software enable saradc sample"]
pub type SARADC_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_SAR_CLK_GATED` reader - SAR clock gated"]
pub type SARADC_SAR_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SARADC_SAR_CLK_GATED` writer - SAR clock gated"]
pub type SARADC_SAR_CLK_GATED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_SAR_CLK_DIV` reader - SAR clock divider"]
pub type SARADC_SAR_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SARADC_SAR_CLK_DIV` writer - SAR clock divider"]
pub type SARADC_SAR_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SARADC_SAR_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SARADC_SAR_PATT_LEN_R = crate::FieldReader;
#[doc = "Field `SARADC_SAR_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SARADC_SAR_PATT_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SARADC_SAR_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SARADC_SAR_PATT_P_CLEAR_R = crate::BitReader;
#[doc = "Field `SARADC_SAR_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SARADC_SAR_PATT_P_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_XPD_SAR_FORCE` reader - force option to xpd sar blocks"]
pub type SARADC_XPD_SAR_FORCE_R = crate::FieldReader;
#[doc = "Field `SARADC_XPD_SAR_FORCE` writer - force option to xpd sar blocks"]
pub type SARADC_XPD_SAR_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SARADC_WAIT_ARB_CYCLE` reader - wait arbit signal stable after sar_done"]
pub type SARADC_WAIT_ARB_CYCLE_R = crate::FieldReader;
#[doc = "Field `SARADC_WAIT_ARB_CYCLE` writer - wait arbit signal stable after sar_done"]
pub type SARADC_WAIT_ARB_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - select software enable saradc sample"]
    #[inline(always)]
    pub fn saradc_start_force(&self) -> SARADC_START_FORCE_R {
        SARADC_START_FORCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - software enable saradc sample"]
    #[inline(always)]
    pub fn saradc_start(&self) -> SARADC_START_R {
        SARADC_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - SAR clock gated"]
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - select software enable saradc sample"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_start_force(&mut self) -> SARADC_START_FORCE_W<CTRL_SPEC> {
        SARADC_START_FORCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - software enable saradc sample"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_start(&mut self) -> SARADC_START_W<CTRL_SPEC> {
        SARADC_START_W::new(self, 1)
    }
    #[doc = "Bit 6 - SAR clock gated"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_clk_gated(&mut self) -> SARADC_SAR_CLK_GATED_W<CTRL_SPEC> {
        SARADC_SAR_CLK_GATED_W::new(self, 6)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_clk_div(&mut self) -> SARADC_SAR_CLK_DIV_W<CTRL_SPEC> {
        SARADC_SAR_CLK_DIV_W::new(self, 7)
    }
    #[doc = "Bits 15:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_patt_len(&mut self) -> SARADC_SAR_PATT_LEN_W<CTRL_SPEC> {
        SARADC_SAR_PATT_LEN_W::new(self, 15)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_patt_p_clear(&mut self) -> SARADC_SAR_PATT_P_CLEAR_W<CTRL_SPEC> {
        SARADC_SAR_PATT_P_CLEAR_W::new(self, 23)
    }
    #[doc = "Bits 27:28 - force option to xpd sar blocks"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_xpd_sar_force(&mut self) -> SARADC_XPD_SAR_FORCE_W<CTRL_SPEC> {
        SARADC_XPD_SAR_FORCE_W::new(self, 27)
    }
    #[doc = "Bits 30:31 - wait arbit signal stable after sar_done"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_wait_arb_cycle(&mut self) -> SARADC_WAIT_ARB_CYCLE_W<CTRL_SPEC> {
        SARADC_WAIT_ARB_CYCLE_W::new(self, 30)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x4003_8240"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4003_8240;
}
