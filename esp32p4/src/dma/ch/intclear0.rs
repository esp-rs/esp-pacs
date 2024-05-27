///Register `INTCLEAR0` writer
pub type W = crate::W<INTCLEAR0_SPEC>;
///Field `CH1_CLEAR_BLOCK_TFR_DONE_INTSTAT` writer - NA
pub type CH1_CLEAR_BLOCK_TFR_DONE_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_DMA_TFR_DONE_INTSTAT` writer - NA
pub type CH1_CLEAR_DMA_TFR_DONE_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SRC_TRANSCOMP_INTSTAT` writer - NA
pub type CH1_CLEAR_SRC_TRANSCOMP_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_DST_TRANSCOMP_INTSTAT` writer - NA
pub type CH1_CLEAR_DST_TRANSCOMP_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SRC_DEC_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SRC_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_DST_DEC_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_DST_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SRC_SLV_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SRC_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_DST_SLV_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_DST_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_LLI_RD_DEC_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_LLI_RD_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_LLI_WR_DEC_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_LLI_WR_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_LLI_RD_SLV_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_LLI_RD_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_LLI_WR_SLV_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_LLI_WR_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SLVIF_MULTIBLKTYPE_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SLVIF_DEC_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SLVIF_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SLVIF_WR2RO_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SLVIF_WR2RO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SLVIF_RD2RWO_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SLVIF_RD2RWO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SLVIF_WRONCHEN_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SLVIF_WRONCHEN_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SLVIF_WRONHOLD_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SLVIF_WRONHOLD_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_SLVIF_WRPARITY_ERR_INTSTAT` writer - NA
pub type CH1_CLEAR_SLVIF_WRPARITY_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_CH_LOCK_CLEARED_INTSTAT` writer - NA
pub type CH1_CLEAR_CH_LOCK_CLEARED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_CH_SRC_SUSPENDED_INTSTAT` writer - NA
pub type CH1_CLEAR_CH_SRC_SUSPENDED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_CH_SUSPENDED_INTSTAT` writer - NA
pub type CH1_CLEAR_CH_SUSPENDED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_CH_DISABLED_INTSTAT` writer - NA
pub type CH1_CLEAR_CH_DISABLED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_CLEAR_CH_ABORTED_INTSTAT` writer - NA
pub type CH1_CLEAR_CH_ABORTED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTCLEAR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_block_tfr_done_intstat(
        &mut self,
    ) -> CH1_CLEAR_BLOCK_TFR_DONE_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_BLOCK_TFR_DONE_INTSTAT_W::new(self, 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_dma_tfr_done_intstat(
        &mut self,
    ) -> CH1_CLEAR_DMA_TFR_DONE_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_DMA_TFR_DONE_INTSTAT_W::new(self, 1)
    }
    ///Bit 3 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_src_transcomp_intstat(
        &mut self,
    ) -> CH1_CLEAR_SRC_TRANSCOMP_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SRC_TRANSCOMP_INTSTAT_W::new(self, 3)
    }
    ///Bit 4 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_dst_transcomp_intstat(
        &mut self,
    ) -> CH1_CLEAR_DST_TRANSCOMP_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_DST_TRANSCOMP_INTSTAT_W::new(self, 4)
    }
    ///Bit 5 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_src_dec_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SRC_DEC_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SRC_DEC_ERR_INTSTAT_W::new(self, 5)
    }
    ///Bit 6 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_dst_dec_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_DST_DEC_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_DST_DEC_ERR_INTSTAT_W::new(self, 6)
    }
    ///Bit 7 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_src_slv_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SRC_SLV_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SRC_SLV_ERR_INTSTAT_W::new(self, 7)
    }
    ///Bit 8 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_dst_slv_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_DST_SLV_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_DST_SLV_ERR_INTSTAT_W::new(self, 8)
    }
    ///Bit 9 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_lli_rd_dec_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_LLI_RD_DEC_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_LLI_RD_DEC_ERR_INTSTAT_W::new(self, 9)
    }
    ///Bit 10 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_lli_wr_dec_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_LLI_WR_DEC_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_LLI_WR_DEC_ERR_INTSTAT_W::new(self, 10)
    }
    ///Bit 11 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_lli_rd_slv_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_LLI_RD_SLV_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_LLI_RD_SLV_ERR_INTSTAT_W::new(self, 11)
    }
    ///Bit 12 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_lli_wr_slv_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_LLI_WR_SLV_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_LLI_WR_SLV_ERR_INTSTAT_W::new(self, 12)
    }
    ///Bit 13 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_shadowreg_or_lli_invalid_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_W::new(self, 13)
    }
    ///Bit 14 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_slvif_multiblktype_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_W::new(self, 14)
    }
    ///Bit 16 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_slvif_dec_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SLVIF_DEC_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SLVIF_DEC_ERR_INTSTAT_W::new(self, 16)
    }
    ///Bit 17 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_slvif_wr2ro_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SLVIF_WR2RO_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SLVIF_WR2RO_ERR_INTSTAT_W::new(self, 17)
    }
    ///Bit 18 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_slvif_rd2rwo_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SLVIF_RD2RWO_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SLVIF_RD2RWO_ERR_INTSTAT_W::new(self, 18)
    }
    ///Bit 19 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_slvif_wronchen_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SLVIF_WRONCHEN_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SLVIF_WRONCHEN_ERR_INTSTAT_W::new(self, 19)
    }
    ///Bit 20 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_slvif_shadowreg_wron_valid_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_W::new(self, 20)
    }
    ///Bit 21 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_slvif_wronhold_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SLVIF_WRONHOLD_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SLVIF_WRONHOLD_ERR_INTSTAT_W::new(self, 21)
    }
    ///Bit 25 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_slvif_wrparity_err_intstat(
        &mut self,
    ) -> CH1_CLEAR_SLVIF_WRPARITY_ERR_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_SLVIF_WRPARITY_ERR_INTSTAT_W::new(self, 25)
    }
    ///Bit 27 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_ch_lock_cleared_intstat(
        &mut self,
    ) -> CH1_CLEAR_CH_LOCK_CLEARED_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_CH_LOCK_CLEARED_INTSTAT_W::new(self, 27)
    }
    ///Bit 28 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_ch_src_suspended_intstat(
        &mut self,
    ) -> CH1_CLEAR_CH_SRC_SUSPENDED_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_CH_SRC_SUSPENDED_INTSTAT_W::new(self, 28)
    }
    ///Bit 29 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_ch_suspended_intstat(
        &mut self,
    ) -> CH1_CLEAR_CH_SUSPENDED_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_CH_SUSPENDED_INTSTAT_W::new(self, 29)
    }
    ///Bit 30 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_ch_disabled_intstat(
        &mut self,
    ) -> CH1_CLEAR_CH_DISABLED_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_CH_DISABLED_INTSTAT_W::new(self, 30)
    }
    ///Bit 31 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_clear_ch_aborted_intstat(
        &mut self,
    ) -> CH1_CLEAR_CH_ABORTED_INTSTAT_W<INTCLEAR0_SPEC> {
        CH1_CLEAR_CH_ABORTED_INTSTAT_W::new(self, 31)
    }
}
/**NA

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclear0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTCLEAR0_SPEC;
impl crate::RegisterSpec for INTCLEAR0_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`intclear0::W`](W) writer structure
impl crate::Writable for INTCLEAR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INTCLEAR0 to value 0
impl crate::Resettable for INTCLEAR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
