#[doc = "Register `COMMONREG_INTSIGNAL_ENABLE0` reader"]
pub type R = crate::R<COMMONREG_INTSIGNAL_ENABLE0_SPEC>;
#[doc = "Register `COMMONREG_INTSIGNAL_ENABLE0` writer"]
pub type W = crate::W<COMMONREG_INTSIGNAL_ENABLE0_SPEC>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL` reader - NA"]
pub type ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL` writer - NA"]
pub type ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL` reader - NA"]
pub type ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL` writer - NA"]
pub type ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL` reader - NA"]
pub type ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL` writer - NA"]
pub type ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL` reader - NA"]
pub type ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL` writer - NA"]
pub type ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WRPARITY_ERR_INTSIGNAL` reader - NA"]
pub type ENABLE_SLVIF_COMMONREG_WRPARITY_ERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL` reader - NA"]
pub type ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL` writer - NA"]
pub type ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_MXIF1_RCH0_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF1_RCH0_ECCPROT_CORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_RCH1_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF1_RCH1_ECCPROT_CORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_BCH_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF1_BCH_ECCPROT_CORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_BCH_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF1_BCH_ECCPROT_UNCORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_RCH0_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF2_RCH0_ECCPROT_CORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_RCH1_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF2_RCH1_ECCPROT_CORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_BCH_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF2_BCH_ECCPROT_CORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_BCH_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type ENABLE_MXIF2_BCH_ECCPROT_UNCORRERR_INTSIGNAL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_dec_err_intsignal(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL_R {
        ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_wr2ro_err_intsignal(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL_R {
        ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_rd2wo_err_intsignal(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL_R {
        ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_wronhold_err_intsignal(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL_R {
        ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_wrparity_err_intsignal(
        &self,
    ) -> ENABLE_SLVIF_COMMONREG_WRPARITY_ERR_INTSIGNAL_R {
        ENABLE_SLVIF_COMMONREG_WRPARITY_ERR_INTSIGNAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn enable_slvif_undefinedreg_dec_err_intsignal(
        &self,
    ) -> ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL_R {
        ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_rch0_eccprot_correrr_intsignal(
        &self,
    ) -> ENABLE_MXIF1_RCH0_ECCPROT_CORRERR_INTSIGNAL_R {
        ENABLE_MXIF1_RCH0_ECCPROT_CORRERR_INTSIGNAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_rch0_eccprot_uncorrerr_intsignal(
        &self,
    ) -> ENABLE_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSIGNAL_R {
        ENABLE_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_rch1_eccprot_correrr_intsignal(
        &self,
    ) -> ENABLE_MXIF1_RCH1_ECCPROT_CORRERR_INTSIGNAL_R {
        ENABLE_MXIF1_RCH1_ECCPROT_CORRERR_INTSIGNAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_rch1_eccprot_uncorrerr_intsignal(
        &self,
    ) -> ENABLE_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSIGNAL_R {
        ENABLE_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_bch_eccprot_correrr_intsignal(
        &self,
    ) -> ENABLE_MXIF1_BCH_ECCPROT_CORRERR_INTSIGNAL_R {
        ENABLE_MXIF1_BCH_ECCPROT_CORRERR_INTSIGNAL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_bch_eccprot_uncorrerr_intsignal(
        &self,
    ) -> ENABLE_MXIF1_BCH_ECCPROT_UNCORRERR_INTSIGNAL_R {
        ENABLE_MXIF1_BCH_ECCPROT_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_rch0_eccprot_correrr_intsignal(
        &self,
    ) -> ENABLE_MXIF2_RCH0_ECCPROT_CORRERR_INTSIGNAL_R {
        ENABLE_MXIF2_RCH0_ECCPROT_CORRERR_INTSIGNAL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_rch0_eccprot_uncorrerr_intsignal(
        &self,
    ) -> ENABLE_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSIGNAL_R {
        ENABLE_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_rch1_eccprot_correrr_intsignal(
        &self,
    ) -> ENABLE_MXIF2_RCH1_ECCPROT_CORRERR_INTSIGNAL_R {
        ENABLE_MXIF2_RCH1_ECCPROT_CORRERR_INTSIGNAL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_rch1_eccprot_uncorrerr_intsignal(
        &self,
    ) -> ENABLE_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSIGNAL_R {
        ENABLE_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_bch_eccprot_correrr_intsignal(
        &self,
    ) -> ENABLE_MXIF2_BCH_ECCPROT_CORRERR_INTSIGNAL_R {
        ENABLE_MXIF2_BCH_ECCPROT_CORRERR_INTSIGNAL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_bch_eccprot_uncorrerr_intsignal(
        &self,
    ) -> ENABLE_MXIF2_BCH_ECCPROT_UNCORRERR_INTSIGNAL_R {
        ENABLE_MXIF2_BCH_ECCPROT_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMONREG_INTSIGNAL_ENABLE0")
            .field(
                "enable_slvif_commonreg_dec_err_intsignal",
                &self.enable_slvif_commonreg_dec_err_intsignal().bit(),
            )
            .field(
                "enable_slvif_commonreg_wr2ro_err_intsignal",
                &self.enable_slvif_commonreg_wr2ro_err_intsignal().bit(),
            )
            .field(
                "enable_slvif_commonreg_rd2wo_err_intsignal",
                &self.enable_slvif_commonreg_rd2wo_err_intsignal().bit(),
            )
            .field(
                "enable_slvif_commonreg_wronhold_err_intsignal",
                &self.enable_slvif_commonreg_wronhold_err_intsignal().bit(),
            )
            .field(
                "enable_slvif_commonreg_wrparity_err_intsignal",
                &self.enable_slvif_commonreg_wrparity_err_intsignal().bit(),
            )
            .field(
                "enable_slvif_undefinedreg_dec_err_intsignal",
                &self.enable_slvif_undefinedreg_dec_err_intsignal().bit(),
            )
            .field(
                "enable_mxif1_rch0_eccprot_correrr_intsignal",
                &self.enable_mxif1_rch0_eccprot_correrr_intsignal().bit(),
            )
            .field(
                "enable_mxif1_rch0_eccprot_uncorrerr_intsignal",
                &self.enable_mxif1_rch0_eccprot_uncorrerr_intsignal().bit(),
            )
            .field(
                "enable_mxif1_rch1_eccprot_correrr_intsignal",
                &self.enable_mxif1_rch1_eccprot_correrr_intsignal().bit(),
            )
            .field(
                "enable_mxif1_rch1_eccprot_uncorrerr_intsignal",
                &self.enable_mxif1_rch1_eccprot_uncorrerr_intsignal().bit(),
            )
            .field(
                "enable_mxif1_bch_eccprot_correrr_intsignal",
                &self.enable_mxif1_bch_eccprot_correrr_intsignal().bit(),
            )
            .field(
                "enable_mxif1_bch_eccprot_uncorrerr_intsignal",
                &self.enable_mxif1_bch_eccprot_uncorrerr_intsignal().bit(),
            )
            .field(
                "enable_mxif2_rch0_eccprot_correrr_intsignal",
                &self.enable_mxif2_rch0_eccprot_correrr_intsignal().bit(),
            )
            .field(
                "enable_mxif2_rch0_eccprot_uncorrerr_intsignal",
                &self.enable_mxif2_rch0_eccprot_uncorrerr_intsignal().bit(),
            )
            .field(
                "enable_mxif2_rch1_eccprot_correrr_intsignal",
                &self.enable_mxif2_rch1_eccprot_correrr_intsignal().bit(),
            )
            .field(
                "enable_mxif2_rch1_eccprot_uncorrerr_intsignal",
                &self.enable_mxif2_rch1_eccprot_uncorrerr_intsignal().bit(),
            )
            .field(
                "enable_mxif2_bch_eccprot_correrr_intsignal",
                &self.enable_mxif2_bch_eccprot_correrr_intsignal().bit(),
            )
            .field(
                "enable_mxif2_bch_eccprot_uncorrerr_intsignal",
                &self.enable_mxif2_bch_eccprot_uncorrerr_intsignal().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMMONREG_INTSIGNAL_ENABLE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_commonreg_dec_err_intsignal(
        &mut self,
    ) -> ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL_W<COMMONREG_INTSIGNAL_ENABLE0_SPEC> {
        ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_commonreg_wr2ro_err_intsignal(
        &mut self,
    ) -> ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL_W<COMMONREG_INTSIGNAL_ENABLE0_SPEC> {
        ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_commonreg_rd2wo_err_intsignal(
        &mut self,
    ) -> ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL_W<COMMONREG_INTSIGNAL_ENABLE0_SPEC> {
        ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_commonreg_wronhold_err_intsignal(
        &mut self,
    ) -> ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL_W<COMMONREG_INTSIGNAL_ENABLE0_SPEC> {
        ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL_W::new(self, 3)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn enable_slvif_undefinedreg_dec_err_intsignal(
        &mut self,
    ) -> ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL_W<COMMONREG_INTSIGNAL_ENABLE0_SPEC> {
        ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL_W::new(self, 8)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`commonreg_intsignal_enable0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`commonreg_intsignal_enable0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMMONREG_INTSIGNAL_ENABLE0_SPEC;
impl crate::RegisterSpec for COMMONREG_INTSIGNAL_ENABLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`commonreg_intsignal_enable0::R`](R) reader structure"]
impl crate::Readable for COMMONREG_INTSIGNAL_ENABLE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`commonreg_intsignal_enable0::W`](W) writer structure"]
impl crate::Writable for COMMONREG_INTSIGNAL_ENABLE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMMONREG_INTSIGNAL_ENABLE0 to value 0x001f_ff8f"]
impl crate::Resettable for COMMONREG_INTSIGNAL_ENABLE0_SPEC {
    const RESET_VALUE: u32 = 0x001f_ff8f;
}
