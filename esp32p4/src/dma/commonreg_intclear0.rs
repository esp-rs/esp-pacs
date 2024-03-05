#[doc = "Register `COMMONREG_INTCLEAR0` writer"]
pub type W = crate::W<COMMONREG_INTCLEAR0_SPEC>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_DEC_ERR_INTSTAT` writer - NA"]
pub type CLEAR_SLVIF_COMMONREG_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT` writer - NA"]
pub type CLEAR_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT` writer - NA"]
pub type CLEAR_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT` writer - NA"]
pub type CLEAR_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT` writer - NA"]
pub type CLEAR_SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT` writer - NA"]
pub type CLEAR_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_BCH_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF1_BCH_ECCPROT_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_BCH_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF2_BCH_ECCPROT_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type CLEAR_MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMMONREG_INTCLEAR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_slvif_commonreg_dec_err_intstat(
        &mut self,
    ) -> CLEAR_SLVIF_COMMONREG_DEC_ERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_SLVIF_COMMONREG_DEC_ERR_INTSTAT_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_slvif_commonreg_wr2ro_err_intstat(
        &mut self,
    ) -> CLEAR_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_slvif_commonreg_rd2wo_err_intstat(
        &mut self,
    ) -> CLEAR_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_slvif_commonreg_wronhold_err_intstat(
        &mut self,
    ) -> CLEAR_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_W::new(self, 3)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_slvif_commonreg_wrparity_err_intstat(
        &mut self,
    ) -> CLEAR_SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_slvif_undefinedreg_dec_err_intstat(
        &mut self,
    ) -> CLEAR_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif1_rch0_eccprot_correrr_intstat(
        &mut self,
    ) -> CLEAR_MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif1_rch0_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> CLEAR_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif1_rch1_eccprot_correrr_intstat(
        &mut self,
    ) -> CLEAR_MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif1_rch1_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> CLEAR_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif1_bch_eccprot_correrr_intstat(
        &mut self,
    ) -> CLEAR_MXIF1_BCH_ECCPROT_CORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF1_BCH_ECCPROT_CORRERR_INTSTAT_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif1_bch_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> CLEAR_MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT_W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif2_rch0_eccprot_correrr_intstat(
        &mut self,
    ) -> CLEAR_MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT_W::new(self, 15)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif2_rch0_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> CLEAR_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT_W::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif2_rch1_eccprot_correrr_intstat(
        &mut self,
    ) -> CLEAR_MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT_W::new(self, 17)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif2_rch1_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> CLEAR_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT_W::new(self, 18)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif2_bch_eccprot_correrr_intstat(
        &mut self,
    ) -> CLEAR_MXIF2_BCH_ECCPROT_CORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF2_BCH_ECCPROT_CORRERR_INTSTAT_W::new(self, 19)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn clear_mxif2_bch_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> CLEAR_MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT_W<COMMONREG_INTCLEAR0_SPEC> {
        CLEAR_MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT_W::new(self, 20)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`commonreg_intclear0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMMONREG_INTCLEAR0_SPEC;
impl crate::RegisterSpec for COMMONREG_INTCLEAR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`commonreg_intclear0::W`](W) writer structure"]
impl crate::Writable for COMMONREG_INTCLEAR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMMONREG_INTCLEAR0 to value 0"]
impl crate::Resettable for COMMONREG_INTCLEAR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
