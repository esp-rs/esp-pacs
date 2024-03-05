#[doc = "Register `CH1_INTSTATUS_ENABLE0` reader"]
pub type R = crate::R<CH1_INTSTATUS_ENABLE0_SPEC>;
#[doc = "Register `CH1_INTSTATUS_ENABLE0` writer"]
pub type W = crate::W<CH1_INTSTATUS_ENABLE0_SPEC>;
#[doc = "Field `CH1_ENABLE_BLOCK_TFR_DONE_INTSTAT` reader - NA"]
pub type CH1_ENABLE_BLOCK_TFR_DONE_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_BLOCK_TFR_DONE_INTSTAT` writer - NA"]
pub type CH1_ENABLE_BLOCK_TFR_DONE_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_DMA_TFR_DONE_INTSTAT` reader - NA"]
pub type CH1_ENABLE_DMA_TFR_DONE_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_DMA_TFR_DONE_INTSTAT` writer - NA"]
pub type CH1_ENABLE_DMA_TFR_DONE_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SRC_TRANSCOMP_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SRC_TRANSCOMP_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SRC_TRANSCOMP_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SRC_TRANSCOMP_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_DST_TRANSCOMP_INTSTAT` reader - NA"]
pub type CH1_ENABLE_DST_TRANSCOMP_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_DST_TRANSCOMP_INTSTAT` writer - NA"]
pub type CH1_ENABLE_DST_TRANSCOMP_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SRC_DEC_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SRC_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SRC_DEC_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SRC_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_DST_DEC_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_DST_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_DST_DEC_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_DST_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SRC_SLV_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SRC_SLV_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SRC_SLV_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SRC_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_DST_SLV_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_DST_SLV_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_DST_SLV_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_DST_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_LLI_RD_DEC_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_LLI_RD_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_LLI_RD_DEC_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_LLI_RD_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_LLI_WR_DEC_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_LLI_WR_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_LLI_WR_DEC_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_LLI_WR_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_LLI_RD_SLV_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_LLI_RD_SLV_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_LLI_RD_SLV_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_LLI_RD_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_LLI_WR_SLV_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_LLI_WR_SLV_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_LLI_WR_SLV_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_LLI_WR_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SLVIF_MULTIBLKTYPE_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SLVIF_MULTIBLKTYPE_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SLVIF_DEC_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SLVIF_DEC_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SLVIF_DEC_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SLVIF_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SLVIF_WR2RO_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SLVIF_WR2RO_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SLVIF_WR2RO_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SLVIF_WR2RO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SLVIF_RD2RWO_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SLVIF_RD2RWO_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SLVIF_RD2RWO_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SLVIF_RD2RWO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SLVIF_WRONCHEN_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SLVIF_WRONCHEN_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SLVIF_WRONCHEN_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SLVIF_WRONCHEN_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SLVIF_WRONHOLD_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SLVIF_WRONHOLD_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_SLVIF_WRONHOLD_ERR_INTSTAT` writer - NA"]
pub type CH1_ENABLE_SLVIF_WRONHOLD_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_SLVIF_WRPARITY_ERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_SLVIF_WRPARITY_ERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_CH_LOCK_CLEARED_INTSTAT` reader - NA"]
pub type CH1_ENABLE_CH_LOCK_CLEARED_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_CH_LOCK_CLEARED_INTSTAT` writer - NA"]
pub type CH1_ENABLE_CH_LOCK_CLEARED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_CH_SRC_SUSPENDED_INTSTAT` reader - NA"]
pub type CH1_ENABLE_CH_SRC_SUSPENDED_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_CH_SRC_SUSPENDED_INTSTAT` writer - NA"]
pub type CH1_ENABLE_CH_SRC_SUSPENDED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_CH_SUSPENDED_INTSTAT` reader - NA"]
pub type CH1_ENABLE_CH_SUSPENDED_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_CH_SUSPENDED_INTSTAT` writer - NA"]
pub type CH1_ENABLE_CH_SUSPENDED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_CH_DISABLED_INTSTAT` reader - NA"]
pub type CH1_ENABLE_CH_DISABLED_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_CH_DISABLED_INTSTAT` writer - NA"]
pub type CH1_ENABLE_CH_DISABLED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ENABLE_CH_ABORTED_INTSTAT` reader - NA"]
pub type CH1_ENABLE_CH_ABORTED_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_CH_ABORTED_INTSTAT` writer - NA"]
pub type CH1_ENABLE_CH_ABORTED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_enable_block_tfr_done_intstat(&self) -> CH1_ENABLE_BLOCK_TFR_DONE_INTSTAT_R {
        CH1_ENABLE_BLOCK_TFR_DONE_INTSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch1_enable_dma_tfr_done_intstat(&self) -> CH1_ENABLE_DMA_TFR_DONE_INTSTAT_R {
        CH1_ENABLE_DMA_TFR_DONE_INTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch1_enable_src_transcomp_intstat(&self) -> CH1_ENABLE_SRC_TRANSCOMP_INTSTAT_R {
        CH1_ENABLE_SRC_TRANSCOMP_INTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch1_enable_dst_transcomp_intstat(&self) -> CH1_ENABLE_DST_TRANSCOMP_INTSTAT_R {
        CH1_ENABLE_DST_TRANSCOMP_INTSTAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn ch1_enable_src_dec_err_intstat(&self) -> CH1_ENABLE_SRC_DEC_ERR_INTSTAT_R {
        CH1_ENABLE_SRC_DEC_ERR_INTSTAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch1_enable_dst_dec_err_intstat(&self) -> CH1_ENABLE_DST_DEC_ERR_INTSTAT_R {
        CH1_ENABLE_DST_DEC_ERR_INTSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn ch1_enable_src_slv_err_intstat(&self) -> CH1_ENABLE_SRC_SLV_ERR_INTSTAT_R {
        CH1_ENABLE_SRC_SLV_ERR_INTSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn ch1_enable_dst_slv_err_intstat(&self) -> CH1_ENABLE_DST_SLV_ERR_INTSTAT_R {
        CH1_ENABLE_DST_SLV_ERR_INTSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn ch1_enable_lli_rd_dec_err_intstat(&self) -> CH1_ENABLE_LLI_RD_DEC_ERR_INTSTAT_R {
        CH1_ENABLE_LLI_RD_DEC_ERR_INTSTAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn ch1_enable_lli_wr_dec_err_intstat(&self) -> CH1_ENABLE_LLI_WR_DEC_ERR_INTSTAT_R {
        CH1_ENABLE_LLI_WR_DEC_ERR_INTSTAT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ch1_enable_lli_rd_slv_err_intstat(&self) -> CH1_ENABLE_LLI_RD_SLV_ERR_INTSTAT_R {
        CH1_ENABLE_LLI_RD_SLV_ERR_INTSTAT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ch1_enable_lli_wr_slv_err_intstat(&self) -> CH1_ENABLE_LLI_WR_SLV_ERR_INTSTAT_R {
        CH1_ENABLE_LLI_WR_SLV_ERR_INTSTAT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ch1_enable_shadowreg_or_lli_invalid_err_intstat(
        &self,
    ) -> CH1_ENABLE_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_R {
        CH1_ENABLE_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ch1_enable_slvif_multiblktype_err_intstat(
        &self,
    ) -> CH1_ENABLE_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_R {
        CH1_ENABLE_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn ch1_enable_slvif_dec_err_intstat(&self) -> CH1_ENABLE_SLVIF_DEC_ERR_INTSTAT_R {
        CH1_ENABLE_SLVIF_DEC_ERR_INTSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn ch1_enable_slvif_wr2ro_err_intstat(&self) -> CH1_ENABLE_SLVIF_WR2RO_ERR_INTSTAT_R {
        CH1_ENABLE_SLVIF_WR2RO_ERR_INTSTAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn ch1_enable_slvif_rd2rwo_err_intstat(&self) -> CH1_ENABLE_SLVIF_RD2RWO_ERR_INTSTAT_R {
        CH1_ENABLE_SLVIF_RD2RWO_ERR_INTSTAT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn ch1_enable_slvif_wronchen_err_intstat(&self) -> CH1_ENABLE_SLVIF_WRONCHEN_ERR_INTSTAT_R {
        CH1_ENABLE_SLVIF_WRONCHEN_ERR_INTSTAT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn ch1_enable_slvif_shadowreg_wron_valid_err_intstat(
        &self,
    ) -> CH1_ENABLE_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_R {
        CH1_ENABLE_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - NA"]
    #[inline(always)]
    pub fn ch1_enable_slvif_wronhold_err_intstat(&self) -> CH1_ENABLE_SLVIF_WRONHOLD_ERR_INTSTAT_R {
        CH1_ENABLE_SLVIF_WRONHOLD_ERR_INTSTAT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    pub fn ch1_enable_slvif_wrparity_err_intstat(&self) -> CH1_ENABLE_SLVIF_WRPARITY_ERR_INTSTAT_R {
        CH1_ENABLE_SLVIF_WRPARITY_ERR_INTSTAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - NA"]
    #[inline(always)]
    pub fn ch1_enable_ch_lock_cleared_intstat(&self) -> CH1_ENABLE_CH_LOCK_CLEARED_INTSTAT_R {
        CH1_ENABLE_CH_LOCK_CLEARED_INTSTAT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - NA"]
    #[inline(always)]
    pub fn ch1_enable_ch_src_suspended_intstat(&self) -> CH1_ENABLE_CH_SRC_SUSPENDED_INTSTAT_R {
        CH1_ENABLE_CH_SRC_SUSPENDED_INTSTAT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - NA"]
    #[inline(always)]
    pub fn ch1_enable_ch_suspended_intstat(&self) -> CH1_ENABLE_CH_SUSPENDED_INTSTAT_R {
        CH1_ENABLE_CH_SUSPENDED_INTSTAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch1_enable_ch_disabled_intstat(&self) -> CH1_ENABLE_CH_DISABLED_INTSTAT_R {
        CH1_ENABLE_CH_DISABLED_INTSTAT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn ch1_enable_ch_aborted_intstat(&self) -> CH1_ENABLE_CH_ABORTED_INTSTAT_R {
        CH1_ENABLE_CH_ABORTED_INTSTAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1_INTSTATUS_ENABLE0")
            .field(
                "ch1_enable_block_tfr_done_intstat",
                &format_args!("{}", self.ch1_enable_block_tfr_done_intstat().bit()),
            )
            .field(
                "ch1_enable_dma_tfr_done_intstat",
                &format_args!("{}", self.ch1_enable_dma_tfr_done_intstat().bit()),
            )
            .field(
                "ch1_enable_src_transcomp_intstat",
                &format_args!("{}", self.ch1_enable_src_transcomp_intstat().bit()),
            )
            .field(
                "ch1_enable_dst_transcomp_intstat",
                &format_args!("{}", self.ch1_enable_dst_transcomp_intstat().bit()),
            )
            .field(
                "ch1_enable_src_dec_err_intstat",
                &format_args!("{}", self.ch1_enable_src_dec_err_intstat().bit()),
            )
            .field(
                "ch1_enable_dst_dec_err_intstat",
                &format_args!("{}", self.ch1_enable_dst_dec_err_intstat().bit()),
            )
            .field(
                "ch1_enable_src_slv_err_intstat",
                &format_args!("{}", self.ch1_enable_src_slv_err_intstat().bit()),
            )
            .field(
                "ch1_enable_dst_slv_err_intstat",
                &format_args!("{}", self.ch1_enable_dst_slv_err_intstat().bit()),
            )
            .field(
                "ch1_enable_lli_rd_dec_err_intstat",
                &format_args!("{}", self.ch1_enable_lli_rd_dec_err_intstat().bit()),
            )
            .field(
                "ch1_enable_lli_wr_dec_err_intstat",
                &format_args!("{}", self.ch1_enable_lli_wr_dec_err_intstat().bit()),
            )
            .field(
                "ch1_enable_lli_rd_slv_err_intstat",
                &format_args!("{}", self.ch1_enable_lli_rd_slv_err_intstat().bit()),
            )
            .field(
                "ch1_enable_lli_wr_slv_err_intstat",
                &format_args!("{}", self.ch1_enable_lli_wr_slv_err_intstat().bit()),
            )
            .field(
                "ch1_enable_shadowreg_or_lli_invalid_err_intstat",
                &format_args!(
                    "{}",
                    self.ch1_enable_shadowreg_or_lli_invalid_err_intstat().bit()
                ),
            )
            .field(
                "ch1_enable_slvif_multiblktype_err_intstat",
                &format_args!("{}", self.ch1_enable_slvif_multiblktype_err_intstat().bit()),
            )
            .field(
                "ch1_enable_slvif_dec_err_intstat",
                &format_args!("{}", self.ch1_enable_slvif_dec_err_intstat().bit()),
            )
            .field(
                "ch1_enable_slvif_wr2ro_err_intstat",
                &format_args!("{}", self.ch1_enable_slvif_wr2ro_err_intstat().bit()),
            )
            .field(
                "ch1_enable_slvif_rd2rwo_err_intstat",
                &format_args!("{}", self.ch1_enable_slvif_rd2rwo_err_intstat().bit()),
            )
            .field(
                "ch1_enable_slvif_wronchen_err_intstat",
                &format_args!("{}", self.ch1_enable_slvif_wronchen_err_intstat().bit()),
            )
            .field(
                "ch1_enable_slvif_shadowreg_wron_valid_err_intstat",
                &format_args!(
                    "{}",
                    self.ch1_enable_slvif_shadowreg_wron_valid_err_intstat()
                        .bit()
                ),
            )
            .field(
                "ch1_enable_slvif_wronhold_err_intstat",
                &format_args!("{}", self.ch1_enable_slvif_wronhold_err_intstat().bit()),
            )
            .field(
                "ch1_enable_slvif_wrparity_err_intstat",
                &format_args!("{}", self.ch1_enable_slvif_wrparity_err_intstat().bit()),
            )
            .field(
                "ch1_enable_ch_lock_cleared_intstat",
                &format_args!("{}", self.ch1_enable_ch_lock_cleared_intstat().bit()),
            )
            .field(
                "ch1_enable_ch_src_suspended_intstat",
                &format_args!("{}", self.ch1_enable_ch_src_suspended_intstat().bit()),
            )
            .field(
                "ch1_enable_ch_suspended_intstat",
                &format_args!("{}", self.ch1_enable_ch_suspended_intstat().bit()),
            )
            .field(
                "ch1_enable_ch_disabled_intstat",
                &format_args!("{}", self.ch1_enable_ch_disabled_intstat().bit()),
            )
            .field(
                "ch1_enable_ch_aborted_intstat",
                &format_args!("{}", self.ch1_enable_ch_aborted_intstat().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH1_INTSTATUS_ENABLE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_block_tfr_done_intstat(
        &mut self,
    ) -> CH1_ENABLE_BLOCK_TFR_DONE_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_BLOCK_TFR_DONE_INTSTAT_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_dma_tfr_done_intstat(
        &mut self,
    ) -> CH1_ENABLE_DMA_TFR_DONE_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_DMA_TFR_DONE_INTSTAT_W::new(self, 1)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_src_transcomp_intstat(
        &mut self,
    ) -> CH1_ENABLE_SRC_TRANSCOMP_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SRC_TRANSCOMP_INTSTAT_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_dst_transcomp_intstat(
        &mut self,
    ) -> CH1_ENABLE_DST_TRANSCOMP_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_DST_TRANSCOMP_INTSTAT_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_src_dec_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SRC_DEC_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SRC_DEC_ERR_INTSTAT_W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_dst_dec_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_DST_DEC_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_DST_DEC_ERR_INTSTAT_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_src_slv_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SRC_SLV_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SRC_SLV_ERR_INTSTAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_dst_slv_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_DST_SLV_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_DST_SLV_ERR_INTSTAT_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_lli_rd_dec_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_LLI_RD_DEC_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_LLI_RD_DEC_ERR_INTSTAT_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_lli_wr_dec_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_LLI_WR_DEC_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_LLI_WR_DEC_ERR_INTSTAT_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_lli_rd_slv_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_LLI_RD_SLV_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_LLI_RD_SLV_ERR_INTSTAT_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_lli_wr_slv_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_LLI_WR_SLV_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_LLI_WR_SLV_ERR_INTSTAT_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_shadowreg_or_lli_invalid_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_slvif_multiblktype_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_W::new(self, 14)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_slvif_dec_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SLVIF_DEC_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SLVIF_DEC_ERR_INTSTAT_W::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_slvif_wr2ro_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SLVIF_WR2RO_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SLVIF_WR2RO_ERR_INTSTAT_W::new(self, 17)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_slvif_rd2rwo_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SLVIF_RD2RWO_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SLVIF_RD2RWO_ERR_INTSTAT_W::new(self, 18)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_slvif_wronchen_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SLVIF_WRONCHEN_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SLVIF_WRONCHEN_ERR_INTSTAT_W::new(self, 19)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_slvif_shadowreg_wron_valid_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_W::new(self, 20)
    }
    #[doc = "Bit 21 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_slvif_wronhold_err_intstat(
        &mut self,
    ) -> CH1_ENABLE_SLVIF_WRONHOLD_ERR_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_SLVIF_WRONHOLD_ERR_INTSTAT_W::new(self, 21)
    }
    #[doc = "Bit 27 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_ch_lock_cleared_intstat(
        &mut self,
    ) -> CH1_ENABLE_CH_LOCK_CLEARED_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_CH_LOCK_CLEARED_INTSTAT_W::new(self, 27)
    }
    #[doc = "Bit 28 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_ch_src_suspended_intstat(
        &mut self,
    ) -> CH1_ENABLE_CH_SRC_SUSPENDED_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_CH_SRC_SUSPENDED_INTSTAT_W::new(self, 28)
    }
    #[doc = "Bit 29 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_ch_suspended_intstat(
        &mut self,
    ) -> CH1_ENABLE_CH_SUSPENDED_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_CH_SUSPENDED_INTSTAT_W::new(self, 29)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_ch_disabled_intstat(
        &mut self,
    ) -> CH1_ENABLE_CH_DISABLED_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_CH_DISABLED_INTSTAT_W::new(self, 30)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_enable_ch_aborted_intstat(
        &mut self,
    ) -> CH1_ENABLE_CH_ABORTED_INTSTAT_W<CH1_INTSTATUS_ENABLE0_SPEC> {
        CH1_ENABLE_CH_ABORTED_INTSTAT_W::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_intstatus_enable0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_intstatus_enable0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1_INTSTATUS_ENABLE0_SPEC;
impl crate::RegisterSpec for CH1_INTSTATUS_ENABLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_intstatus_enable0::R`](R) reader structure"]
impl crate::Readable for CH1_INTSTATUS_ENABLE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch1_intstatus_enable0::W`](W) writer structure"]
impl crate::Writable for CH1_INTSTATUS_ENABLE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1_INTSTATUS_ENABLE0 to value 0xfa3f_7ffb"]
impl crate::Resettable for CH1_INTSTATUS_ENABLE0_SPEC {
    const RESET_VALUE: u32 = 0xfa3f_7ffb;
}
