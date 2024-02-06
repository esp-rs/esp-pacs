#[doc = "Register `COMMONREG_INTSTATUS0` reader"]
pub type R = crate::R<COMMONREG_INTSTATUS0_SPEC>;
#[doc = "Field `SLVIF_COMMONREG_DEC_ERR_INTSTAT` reader - NA"]
pub type SLVIF_COMMONREG_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `SLVIF_COMMONREG_WR2RO_ERR_INTSTAT` reader - NA"]
pub type SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `SLVIF_COMMONREG_RD2WO_ERR_INTSTAT` reader - NA"]
pub type SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT` reader - NA"]
pub type SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT` reader - NA"]
pub type SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT` reader - NA"]
pub type SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF1_BCH_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type MXIF1_BCH_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF2_BCH_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type MXIF2_BCH_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_dec_err_intstat(&self) -> SLVIF_COMMONREG_DEC_ERR_INTSTAT_R {
        SLVIF_COMMONREG_DEC_ERR_INTSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_wr2ro_err_intstat(&self) -> SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_R {
        SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_rd2wo_err_intstat(&self) -> SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_R {
        SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_wronhold_err_intstat(&self) -> SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_R {
        SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_wrparity_err_intstat(&self) -> SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT_R {
        SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn slvif_undefinedreg_dec_err_intstat(&self) -> SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_R {
        SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn mxif1_rch0_eccprot_correrr_intstat(&self) -> MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT_R {
        MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn mxif1_rch0_eccprot_uncorrerr_intstat(&self) -> MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT_R {
        MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn mxif1_rch1_eccprot_correrr_intstat(&self) -> MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT_R {
        MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn mxif1_rch1_eccprot_uncorrerr_intstat(&self) -> MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT_R {
        MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn mxif1_bch_eccprot_correrr_intstat(&self) -> MXIF1_BCH_ECCPROT_CORRERR_INTSTAT_R {
        MXIF1_BCH_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn mxif1_bch_eccprot_uncorrerr_intstat(&self) -> MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT_R {
        MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn mxif2_rch0_eccprot_correrr_intstat(&self) -> MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT_R {
        MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn mxif2_rch0_eccprot_uncorrerr_intstat(&self) -> MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT_R {
        MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn mxif2_rch1_eccprot_correrr_intstat(&self) -> MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT_R {
        MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn mxif2_rch1_eccprot_uncorrerr_intstat(&self) -> MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT_R {
        MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn mxif2_bch_eccprot_correrr_intstat(&self) -> MXIF2_BCH_ECCPROT_CORRERR_INTSTAT_R {
        MXIF2_BCH_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn mxif2_bch_eccprot_uncorrerr_intstat(&self) -> MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT_R {
        MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMONREG_INTSTATUS0")
            .field(
                "slvif_commonreg_dec_err_intstat",
                &format_args!("{}", self.slvif_commonreg_dec_err_intstat().bit()),
            )
            .field(
                "slvif_commonreg_wr2ro_err_intstat",
                &format_args!("{}", self.slvif_commonreg_wr2ro_err_intstat().bit()),
            )
            .field(
                "slvif_commonreg_rd2wo_err_intstat",
                &format_args!("{}", self.slvif_commonreg_rd2wo_err_intstat().bit()),
            )
            .field(
                "slvif_commonreg_wronhold_err_intstat",
                &format_args!("{}", self.slvif_commonreg_wronhold_err_intstat().bit()),
            )
            .field(
                "slvif_commonreg_wrparity_err_intstat",
                &format_args!("{}", self.slvif_commonreg_wrparity_err_intstat().bit()),
            )
            .field(
                "slvif_undefinedreg_dec_err_intstat",
                &format_args!("{}", self.slvif_undefinedreg_dec_err_intstat().bit()),
            )
            .field(
                "mxif1_rch0_eccprot_correrr_intstat",
                &format_args!("{}", self.mxif1_rch0_eccprot_correrr_intstat().bit()),
            )
            .field(
                "mxif1_rch0_eccprot_uncorrerr_intstat",
                &format_args!("{}", self.mxif1_rch0_eccprot_uncorrerr_intstat().bit()),
            )
            .field(
                "mxif1_rch1_eccprot_correrr_intstat",
                &format_args!("{}", self.mxif1_rch1_eccprot_correrr_intstat().bit()),
            )
            .field(
                "mxif1_rch1_eccprot_uncorrerr_intstat",
                &format_args!("{}", self.mxif1_rch1_eccprot_uncorrerr_intstat().bit()),
            )
            .field(
                "mxif1_bch_eccprot_correrr_intstat",
                &format_args!("{}", self.mxif1_bch_eccprot_correrr_intstat().bit()),
            )
            .field(
                "mxif1_bch_eccprot_uncorrerr_intstat",
                &format_args!("{}", self.mxif1_bch_eccprot_uncorrerr_intstat().bit()),
            )
            .field(
                "mxif2_rch0_eccprot_correrr_intstat",
                &format_args!("{}", self.mxif2_rch0_eccprot_correrr_intstat().bit()),
            )
            .field(
                "mxif2_rch0_eccprot_uncorrerr_intstat",
                &format_args!("{}", self.mxif2_rch0_eccprot_uncorrerr_intstat().bit()),
            )
            .field(
                "mxif2_rch1_eccprot_correrr_intstat",
                &format_args!("{}", self.mxif2_rch1_eccprot_correrr_intstat().bit()),
            )
            .field(
                "mxif2_rch1_eccprot_uncorrerr_intstat",
                &format_args!("{}", self.mxif2_rch1_eccprot_uncorrerr_intstat().bit()),
            )
            .field(
                "mxif2_bch_eccprot_correrr_intstat",
                &format_args!("{}", self.mxif2_bch_eccprot_correrr_intstat().bit()),
            )
            .field(
                "mxif2_bch_eccprot_uncorrerr_intstat",
                &format_args!("{}", self.mxif2_bch_eccprot_uncorrerr_intstat().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMMONREG_INTSTATUS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`commonreg_intstatus0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMMONREG_INTSTATUS0_SPEC;
impl crate::RegisterSpec for COMMONREG_INTSTATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`commonreg_intstatus0::R`](R) reader structure"]
impl crate::Readable for COMMONREG_INTSTATUS0_SPEC {}
#[doc = "`reset()` method sets COMMONREG_INTSTATUS0 to value 0"]
impl crate::Resettable for COMMONREG_INTSTATUS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
