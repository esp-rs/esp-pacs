///Register `COMMONREG_INTSTATUS_ENABLE0` reader
pub type R = crate::R<COMMONREG_INTSTATUS_ENABLE0_SPEC>;
///Register `COMMONREG_INTSTATUS_ENABLE0` writer
pub type W = crate::W<COMMONREG_INTSTATUS_ENABLE0_SPEC>;
///Field `ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSTAT` reader - NA
pub type ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSTAT` writer - NA
pub type ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT` reader - NA
pub type ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT` writer - NA
pub type ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT` reader - NA
pub type ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT` writer - NA
pub type ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT` reader - NA
pub type ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT` writer - NA
pub type ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT` reader - NA
pub type ENABLE_SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT` reader - NA
pub type ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT` writer - NA
pub type ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF1_BCH_ECCPROT_CORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF1_BCH_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF2_BCH_ECCPROT_CORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF2_BCH_ECCPROT_CORRERR_INTSTAT_R = crate::BitReader;
///Field `ENABLE_MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT` reader - NA
pub type ENABLE_MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT_R = crate::BitReader;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn enable_slvif_commonreg_dec_err_intstat(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSTAT_R {
        ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSTAT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    pub fn enable_slvif_commonreg_wr2ro_err_intstat(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_R {
        ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NA
    #[inline(always)]
    pub fn enable_slvif_commonreg_rd2wo_err_intstat(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_R {
        ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NA
    #[inline(always)]
    pub fn enable_slvif_commonreg_wronhold_err_intstat(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_R {
        ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - NA
    #[inline(always)]
    pub fn enable_slvif_commonreg_wrparity_err_intstat(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT_R {
        ENABLE_SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - NA
    #[inline(always)]
    pub fn enable_slvif_undefinedreg_dec_err_intstat(
        &self,
    ) -> ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_R {
        ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - NA
    #[inline(always)]
    pub fn enable_mxif1_rch0_eccprot_correrr_intstat(
        &self,
    ) -> ENABLE_MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT_R {
        ENABLE_MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NA
    #[inline(always)]
    pub fn enable_mxif1_rch0_eccprot_uncorrerr_intstat(
        &self,
    ) -> ENABLE_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT_R {
        ENABLE_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - NA
    #[inline(always)]
    pub fn enable_mxif1_rch1_eccprot_correrr_intstat(
        &self,
    ) -> ENABLE_MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT_R {
        ENABLE_MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - NA
    #[inline(always)]
    pub fn enable_mxif1_rch1_eccprot_uncorrerr_intstat(
        &self,
    ) -> ENABLE_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT_R {
        ENABLE_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NA
    #[inline(always)]
    pub fn enable_mxif1_bch_eccprot_correrr_intstat(
        &self,
    ) -> ENABLE_MXIF1_BCH_ECCPROT_CORRERR_INTSTAT_R {
        ENABLE_MXIF1_BCH_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - NA
    #[inline(always)]
    pub fn enable_mxif1_bch_eccprot_uncorrerr_intstat(
        &self,
    ) -> ENABLE_MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT_R {
        ENABLE_MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NA
    #[inline(always)]
    pub fn enable_mxif2_rch0_eccprot_correrr_intstat(
        &self,
    ) -> ENABLE_MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT_R {
        ENABLE_MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - NA
    #[inline(always)]
    pub fn enable_mxif2_rch0_eccprot_uncorrerr_intstat(
        &self,
    ) -> ENABLE_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT_R {
        ENABLE_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NA
    #[inline(always)]
    pub fn enable_mxif2_rch1_eccprot_correrr_intstat(
        &self,
    ) -> ENABLE_MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT_R {
        ENABLE_MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - NA
    #[inline(always)]
    pub fn enable_mxif2_rch1_eccprot_uncorrerr_intstat(
        &self,
    ) -> ENABLE_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT_R {
        ENABLE_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - NA
    #[inline(always)]
    pub fn enable_mxif2_bch_eccprot_correrr_intstat(
        &self,
    ) -> ENABLE_MXIF2_BCH_ECCPROT_CORRERR_INTSTAT_R {
        ENABLE_MXIF2_BCH_ECCPROT_CORRERR_INTSTAT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - NA
    #[inline(always)]
    pub fn enable_mxif2_bch_eccprot_uncorrerr_intstat(
        &self,
    ) -> ENABLE_MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT_R {
        ENABLE_MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMONREG_INTSTATUS_ENABLE0")
            .field(
                "enable_slvif_commonreg_dec_err_intstat",
                &self.enable_slvif_commonreg_dec_err_intstat(),
            )
            .field(
                "enable_slvif_commonreg_wr2ro_err_intstat",
                &self.enable_slvif_commonreg_wr2ro_err_intstat(),
            )
            .field(
                "enable_slvif_commonreg_rd2wo_err_intstat",
                &self.enable_slvif_commonreg_rd2wo_err_intstat(),
            )
            .field(
                "enable_slvif_commonreg_wronhold_err_intstat",
                &self.enable_slvif_commonreg_wronhold_err_intstat(),
            )
            .field(
                "enable_slvif_commonreg_wrparity_err_intstat",
                &self.enable_slvif_commonreg_wrparity_err_intstat(),
            )
            .field(
                "enable_slvif_undefinedreg_dec_err_intstat",
                &self.enable_slvif_undefinedreg_dec_err_intstat(),
            )
            .field(
                "enable_mxif1_rch0_eccprot_correrr_intstat",
                &self.enable_mxif1_rch0_eccprot_correrr_intstat(),
            )
            .field(
                "enable_mxif1_rch0_eccprot_uncorrerr_intstat",
                &self.enable_mxif1_rch0_eccprot_uncorrerr_intstat(),
            )
            .field(
                "enable_mxif1_rch1_eccprot_correrr_intstat",
                &self.enable_mxif1_rch1_eccprot_correrr_intstat(),
            )
            .field(
                "enable_mxif1_rch1_eccprot_uncorrerr_intstat",
                &self.enable_mxif1_rch1_eccprot_uncorrerr_intstat(),
            )
            .field(
                "enable_mxif1_bch_eccprot_correrr_intstat",
                &self.enable_mxif1_bch_eccprot_correrr_intstat(),
            )
            .field(
                "enable_mxif1_bch_eccprot_uncorrerr_intstat",
                &self.enable_mxif1_bch_eccprot_uncorrerr_intstat(),
            )
            .field(
                "enable_mxif2_rch0_eccprot_correrr_intstat",
                &self.enable_mxif2_rch0_eccprot_correrr_intstat(),
            )
            .field(
                "enable_mxif2_rch0_eccprot_uncorrerr_intstat",
                &self.enable_mxif2_rch0_eccprot_uncorrerr_intstat(),
            )
            .field(
                "enable_mxif2_rch1_eccprot_correrr_intstat",
                &self.enable_mxif2_rch1_eccprot_correrr_intstat(),
            )
            .field(
                "enable_mxif2_rch1_eccprot_uncorrerr_intstat",
                &self.enable_mxif2_rch1_eccprot_uncorrerr_intstat(),
            )
            .field(
                "enable_mxif2_bch_eccprot_correrr_intstat",
                &self.enable_mxif2_bch_eccprot_correrr_intstat(),
            )
            .field(
                "enable_mxif2_bch_eccprot_uncorrerr_intstat",
                &self.enable_mxif2_bch_eccprot_uncorrerr_intstat(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_commonreg_dec_err_intstat(
        &mut self,
    ) -> ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSTAT_W<COMMONREG_INTSTATUS_ENABLE0_SPEC> {
        ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSTAT_W::new(self, 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_commonreg_wr2ro_err_intstat(
        &mut self,
    ) -> ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_W<COMMONREG_INTSTATUS_ENABLE0_SPEC> {
        ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT_W::new(self, 1)
    }
    ///Bit 2 - NA
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_commonreg_rd2wo_err_intstat(
        &mut self,
    ) -> ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_W<COMMONREG_INTSTATUS_ENABLE0_SPEC> {
        ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT_W::new(self, 2)
    }
    ///Bit 3 - NA
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_commonreg_wronhold_err_intstat(
        &mut self,
    ) -> ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_W<COMMONREG_INTSTATUS_ENABLE0_SPEC> {
        ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT_W::new(self, 3)
    }
    ///Bit 8 - NA
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_undefinedreg_dec_err_intstat(
        &mut self,
    ) -> ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_W<COMMONREG_INTSTATUS_ENABLE0_SPEC> {
        ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT_W::new(self, 8)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`commonreg_intstatus_enable0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`commonreg_intstatus_enable0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COMMONREG_INTSTATUS_ENABLE0_SPEC;
impl crate::RegisterSpec for COMMONREG_INTSTATUS_ENABLE0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`commonreg_intstatus_enable0::R`](R) reader structure
impl crate::Readable for COMMONREG_INTSTATUS_ENABLE0_SPEC {}
///`write(|w| ..)` method takes [`commonreg_intstatus_enable0::W`](W) writer structure
impl crate::Writable for COMMONREG_INTSTATUS_ENABLE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COMMONREG_INTSTATUS_ENABLE0 to value 0x001f_ff8f
impl crate::Resettable for COMMONREG_INTSTATUS_ENABLE0_SPEC {
    const RESET_VALUE: u32 = 0x001f_ff8f;
}
