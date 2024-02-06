#[doc = "Register `CH4_INTSTATUS0` reader"]
pub type R = crate::R<CH4_INTSTATUS0_SPEC>;
#[doc = "Field `CH4_BLOCK_TFR_DONE_INTSTAT` reader - NA"]
pub type CH4_BLOCK_TFR_DONE_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_DMA_TFR_DONE_INTSTAT` reader - NA"]
pub type CH4_DMA_TFR_DONE_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SRC_TRANSCOMP_INTSTAT` reader - NA"]
pub type CH4_SRC_TRANSCOMP_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_DST_TRANSCOMP_INTSTAT` reader - NA"]
pub type CH4_DST_TRANSCOMP_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SRC_DEC_ERR_INTSTAT` reader - NA"]
pub type CH4_SRC_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_DST_DEC_ERR_INTSTAT` reader - NA"]
pub type CH4_DST_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SRC_SLV_ERR_INTSTAT` reader - NA"]
pub type CH4_SRC_SLV_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_DST_SLV_ERR_INTSTAT` reader - NA"]
pub type CH4_DST_SLV_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_LLI_RD_DEC_ERR_INTSTAT` reader - NA"]
pub type CH4_LLI_RD_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_LLI_WR_DEC_ERR_INTSTAT` reader - NA"]
pub type CH4_LLI_WR_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_LLI_RD_SLV_ERR_INTSTAT` reader - NA"]
pub type CH4_LLI_RD_SLV_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_LLI_WR_SLV_ERR_INTSTAT` reader - NA"]
pub type CH4_LLI_WR_SLV_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT` reader - NA"]
pub type CH4_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SLVIF_MULTIBLKTYPE_ERR_INTSTAT` reader - NA"]
pub type CH4_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SLVIF_DEC_ERR_INTSTAT` reader - NA"]
pub type CH4_SLVIF_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SLVIF_WR2RO_ERR_INTSTAT` reader - NA"]
pub type CH4_SLVIF_WR2RO_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SLVIF_RD2RWO_ERR_INTSTAT` reader - NA"]
pub type CH4_SLVIF_RD2RWO_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SLVIF_WRONCHEN_ERR_INTSTAT` reader - NA"]
pub type CH4_SLVIF_WRONCHEN_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT` reader - NA"]
pub type CH4_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SLVIF_WRONHOLD_ERR_INTSTAT` reader - NA"]
pub type CH4_SLVIF_WRONHOLD_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_SLVIF_WRPARITY_ERR_INTSTAT` reader - NA"]
pub type CH4_SLVIF_WRPARITY_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_CH_LOCK_CLEARED_INTSTAT` reader - NA"]
pub type CH4_CH_LOCK_CLEARED_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_CH_SRC_SUSPENDED_INTSTAT` reader - NA"]
pub type CH4_CH_SRC_SUSPENDED_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_CH_SUSPENDED_INTSTAT` reader - NA"]
pub type CH4_CH_SUSPENDED_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_CH_DISABLED_INTSTAT` reader - NA"]
pub type CH4_CH_DISABLED_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_CH_ABORTED_INTSTAT` reader - NA"]
pub type CH4_CH_ABORTED_INTSTAT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch4_block_tfr_done_intstat(&self) -> CH4_BLOCK_TFR_DONE_INTSTAT_R {
        CH4_BLOCK_TFR_DONE_INTSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch4_dma_tfr_done_intstat(&self) -> CH4_DMA_TFR_DONE_INTSTAT_R {
        CH4_DMA_TFR_DONE_INTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch4_src_transcomp_intstat(&self) -> CH4_SRC_TRANSCOMP_INTSTAT_R {
        CH4_SRC_TRANSCOMP_INTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch4_dst_transcomp_intstat(&self) -> CH4_DST_TRANSCOMP_INTSTAT_R {
        CH4_DST_TRANSCOMP_INTSTAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn ch4_src_dec_err_intstat(&self) -> CH4_SRC_DEC_ERR_INTSTAT_R {
        CH4_SRC_DEC_ERR_INTSTAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch4_dst_dec_err_intstat(&self) -> CH4_DST_DEC_ERR_INTSTAT_R {
        CH4_DST_DEC_ERR_INTSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn ch4_src_slv_err_intstat(&self) -> CH4_SRC_SLV_ERR_INTSTAT_R {
        CH4_SRC_SLV_ERR_INTSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn ch4_dst_slv_err_intstat(&self) -> CH4_DST_SLV_ERR_INTSTAT_R {
        CH4_DST_SLV_ERR_INTSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn ch4_lli_rd_dec_err_intstat(&self) -> CH4_LLI_RD_DEC_ERR_INTSTAT_R {
        CH4_LLI_RD_DEC_ERR_INTSTAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn ch4_lli_wr_dec_err_intstat(&self) -> CH4_LLI_WR_DEC_ERR_INTSTAT_R {
        CH4_LLI_WR_DEC_ERR_INTSTAT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ch4_lli_rd_slv_err_intstat(&self) -> CH4_LLI_RD_SLV_ERR_INTSTAT_R {
        CH4_LLI_RD_SLV_ERR_INTSTAT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ch4_lli_wr_slv_err_intstat(&self) -> CH4_LLI_WR_SLV_ERR_INTSTAT_R {
        CH4_LLI_WR_SLV_ERR_INTSTAT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ch4_shadowreg_or_lli_invalid_err_intstat(
        &self,
    ) -> CH4_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_R {
        CH4_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ch4_slvif_multiblktype_err_intstat(&self) -> CH4_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_R {
        CH4_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn ch4_slvif_dec_err_intstat(&self) -> CH4_SLVIF_DEC_ERR_INTSTAT_R {
        CH4_SLVIF_DEC_ERR_INTSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn ch4_slvif_wr2ro_err_intstat(&self) -> CH4_SLVIF_WR2RO_ERR_INTSTAT_R {
        CH4_SLVIF_WR2RO_ERR_INTSTAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn ch4_slvif_rd2rwo_err_intstat(&self) -> CH4_SLVIF_RD2RWO_ERR_INTSTAT_R {
        CH4_SLVIF_RD2RWO_ERR_INTSTAT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn ch4_slvif_wronchen_err_intstat(&self) -> CH4_SLVIF_WRONCHEN_ERR_INTSTAT_R {
        CH4_SLVIF_WRONCHEN_ERR_INTSTAT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn ch4_slvif_shadowreg_wron_valid_err_intstat(
        &self,
    ) -> CH4_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_R {
        CH4_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - NA"]
    #[inline(always)]
    pub fn ch4_slvif_wronhold_err_intstat(&self) -> CH4_SLVIF_WRONHOLD_ERR_INTSTAT_R {
        CH4_SLVIF_WRONHOLD_ERR_INTSTAT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    pub fn ch4_slvif_wrparity_err_intstat(&self) -> CH4_SLVIF_WRPARITY_ERR_INTSTAT_R {
        CH4_SLVIF_WRPARITY_ERR_INTSTAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - NA"]
    #[inline(always)]
    pub fn ch4_ch_lock_cleared_intstat(&self) -> CH4_CH_LOCK_CLEARED_INTSTAT_R {
        CH4_CH_LOCK_CLEARED_INTSTAT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - NA"]
    #[inline(always)]
    pub fn ch4_ch_src_suspended_intstat(&self) -> CH4_CH_SRC_SUSPENDED_INTSTAT_R {
        CH4_CH_SRC_SUSPENDED_INTSTAT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - NA"]
    #[inline(always)]
    pub fn ch4_ch_suspended_intstat(&self) -> CH4_CH_SUSPENDED_INTSTAT_R {
        CH4_CH_SUSPENDED_INTSTAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch4_ch_disabled_intstat(&self) -> CH4_CH_DISABLED_INTSTAT_R {
        CH4_CH_DISABLED_INTSTAT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn ch4_ch_aborted_intstat(&self) -> CH4_CH_ABORTED_INTSTAT_R {
        CH4_CH_ABORTED_INTSTAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4_INTSTATUS0")
            .field(
                "ch4_block_tfr_done_intstat",
                &format_args!("{}", self.ch4_block_tfr_done_intstat().bit()),
            )
            .field(
                "ch4_dma_tfr_done_intstat",
                &format_args!("{}", self.ch4_dma_tfr_done_intstat().bit()),
            )
            .field(
                "ch4_src_transcomp_intstat",
                &format_args!("{}", self.ch4_src_transcomp_intstat().bit()),
            )
            .field(
                "ch4_dst_transcomp_intstat",
                &format_args!("{}", self.ch4_dst_transcomp_intstat().bit()),
            )
            .field(
                "ch4_src_dec_err_intstat",
                &format_args!("{}", self.ch4_src_dec_err_intstat().bit()),
            )
            .field(
                "ch4_dst_dec_err_intstat",
                &format_args!("{}", self.ch4_dst_dec_err_intstat().bit()),
            )
            .field(
                "ch4_src_slv_err_intstat",
                &format_args!("{}", self.ch4_src_slv_err_intstat().bit()),
            )
            .field(
                "ch4_dst_slv_err_intstat",
                &format_args!("{}", self.ch4_dst_slv_err_intstat().bit()),
            )
            .field(
                "ch4_lli_rd_dec_err_intstat",
                &format_args!("{}", self.ch4_lli_rd_dec_err_intstat().bit()),
            )
            .field(
                "ch4_lli_wr_dec_err_intstat",
                &format_args!("{}", self.ch4_lli_wr_dec_err_intstat().bit()),
            )
            .field(
                "ch4_lli_rd_slv_err_intstat",
                &format_args!("{}", self.ch4_lli_rd_slv_err_intstat().bit()),
            )
            .field(
                "ch4_lli_wr_slv_err_intstat",
                &format_args!("{}", self.ch4_lli_wr_slv_err_intstat().bit()),
            )
            .field(
                "ch4_shadowreg_or_lli_invalid_err_intstat",
                &format_args!("{}", self.ch4_shadowreg_or_lli_invalid_err_intstat().bit()),
            )
            .field(
                "ch4_slvif_multiblktype_err_intstat",
                &format_args!("{}", self.ch4_slvif_multiblktype_err_intstat().bit()),
            )
            .field(
                "ch4_slvif_dec_err_intstat",
                &format_args!("{}", self.ch4_slvif_dec_err_intstat().bit()),
            )
            .field(
                "ch4_slvif_wr2ro_err_intstat",
                &format_args!("{}", self.ch4_slvif_wr2ro_err_intstat().bit()),
            )
            .field(
                "ch4_slvif_rd2rwo_err_intstat",
                &format_args!("{}", self.ch4_slvif_rd2rwo_err_intstat().bit()),
            )
            .field(
                "ch4_slvif_wronchen_err_intstat",
                &format_args!("{}", self.ch4_slvif_wronchen_err_intstat().bit()),
            )
            .field(
                "ch4_slvif_shadowreg_wron_valid_err_intstat",
                &format_args!(
                    "{}",
                    self.ch4_slvif_shadowreg_wron_valid_err_intstat().bit()
                ),
            )
            .field(
                "ch4_slvif_wronhold_err_intstat",
                &format_args!("{}", self.ch4_slvif_wronhold_err_intstat().bit()),
            )
            .field(
                "ch4_slvif_wrparity_err_intstat",
                &format_args!("{}", self.ch4_slvif_wrparity_err_intstat().bit()),
            )
            .field(
                "ch4_ch_lock_cleared_intstat",
                &format_args!("{}", self.ch4_ch_lock_cleared_intstat().bit()),
            )
            .field(
                "ch4_ch_src_suspended_intstat",
                &format_args!("{}", self.ch4_ch_src_suspended_intstat().bit()),
            )
            .field(
                "ch4_ch_suspended_intstat",
                &format_args!("{}", self.ch4_ch_suspended_intstat().bit()),
            )
            .field(
                "ch4_ch_disabled_intstat",
                &format_args!("{}", self.ch4_ch_disabled_intstat().bit()),
            )
            .field(
                "ch4_ch_aborted_intstat",
                &format_args!("{}", self.ch4_ch_aborted_intstat().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_INTSTATUS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_intstatus0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_INTSTATUS0_SPEC;
impl crate::RegisterSpec for CH4_INTSTATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_intstatus0::R`](R) reader structure"]
impl crate::Readable for CH4_INTSTATUS0_SPEC {}
#[doc = "`reset()` method sets CH4_INTSTATUS0 to value 0"]
impl crate::Resettable for CH4_INTSTATUS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
