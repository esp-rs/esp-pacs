#[doc = "Register `CH4_INTCLEAR0` writer"]
pub type W = crate::W<CH4_INTCLEAR0_SPEC>;
#[doc = "Field `CH4_CLEAR_BLOCK_TFR_DONE_INTSTAT` writer - NA"]
pub type CH4_CLEAR_BLOCK_TFR_DONE_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_DMA_TFR_DONE_INTSTAT` writer - NA"]
pub type CH4_CLEAR_DMA_TFR_DONE_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SRC_TRANSCOMP_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SRC_TRANSCOMP_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_DST_TRANSCOMP_INTSTAT` writer - NA"]
pub type CH4_CLEAR_DST_TRANSCOMP_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SRC_DEC_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SRC_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_DST_DEC_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_DST_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SRC_SLV_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SRC_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_DST_SLV_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_DST_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_LLI_RD_DEC_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_LLI_RD_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_LLI_WR_DEC_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_LLI_WR_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_LLI_RD_SLV_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_LLI_RD_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_LLI_WR_SLV_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_LLI_WR_SLV_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SLVIF_MULTIBLKTYPE_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SLVIF_DEC_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SLVIF_DEC_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SLVIF_WR2RO_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SLVIF_WR2RO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SLVIF_RD2RWO_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SLVIF_RD2RWO_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SLVIF_WRONCHEN_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SLVIF_WRONCHEN_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SLVIF_WRONHOLD_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SLVIF_WRONHOLD_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_SLVIF_WRPARITY_ERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_SLVIF_WRPARITY_ERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_CH_LOCK_CLEARED_INTSTAT` writer - NA"]
pub type CH4_CLEAR_CH_LOCK_CLEARED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_CH_SRC_SUSPENDED_INTSTAT` writer - NA"]
pub type CH4_CLEAR_CH_SRC_SUSPENDED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_CH_SUSPENDED_INTSTAT` writer - NA"]
pub type CH4_CLEAR_CH_SUSPENDED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_CH_DISABLED_INTSTAT` writer - NA"]
pub type CH4_CLEAR_CH_DISABLED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_CH_ABORTED_INTSTAT` writer - NA"]
pub type CH4_CLEAR_CH_ABORTED_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_INTCLEAR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_block_tfr_done_intstat(
        &mut self,
    ) -> CH4_CLEAR_BLOCK_TFR_DONE_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_BLOCK_TFR_DONE_INTSTAT_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_dma_tfr_done_intstat(
        &mut self,
    ) -> CH4_CLEAR_DMA_TFR_DONE_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_DMA_TFR_DONE_INTSTAT_W::new(self, 1)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_src_transcomp_intstat(
        &mut self,
    ) -> CH4_CLEAR_SRC_TRANSCOMP_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SRC_TRANSCOMP_INTSTAT_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_dst_transcomp_intstat(
        &mut self,
    ) -> CH4_CLEAR_DST_TRANSCOMP_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_DST_TRANSCOMP_INTSTAT_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_src_dec_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SRC_DEC_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SRC_DEC_ERR_INTSTAT_W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_dst_dec_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_DST_DEC_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_DST_DEC_ERR_INTSTAT_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_src_slv_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SRC_SLV_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SRC_SLV_ERR_INTSTAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_dst_slv_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_DST_SLV_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_DST_SLV_ERR_INTSTAT_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_lli_rd_dec_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_LLI_RD_DEC_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_LLI_RD_DEC_ERR_INTSTAT_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_lli_wr_dec_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_LLI_WR_DEC_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_LLI_WR_DEC_ERR_INTSTAT_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_lli_rd_slv_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_LLI_RD_SLV_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_LLI_RD_SLV_ERR_INTSTAT_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_lli_wr_slv_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_LLI_WR_SLV_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_LLI_WR_SLV_ERR_INTSTAT_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_shadowreg_or_lli_invalid_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_slvif_multiblktype_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SLVIF_MULTIBLKTYPE_ERR_INTSTAT_W::new(self, 14)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_slvif_dec_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SLVIF_DEC_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SLVIF_DEC_ERR_INTSTAT_W::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_slvif_wr2ro_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SLVIF_WR2RO_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SLVIF_WR2RO_ERR_INTSTAT_W::new(self, 17)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_slvif_rd2rwo_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SLVIF_RD2RWO_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SLVIF_RD2RWO_ERR_INTSTAT_W::new(self, 18)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_slvif_wronchen_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SLVIF_WRONCHEN_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SLVIF_WRONCHEN_ERR_INTSTAT_W::new(self, 19)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_slvif_shadowreg_wron_valid_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT_W::new(self, 20)
    }
    #[doc = "Bit 21 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_slvif_wronhold_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SLVIF_WRONHOLD_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SLVIF_WRONHOLD_ERR_INTSTAT_W::new(self, 21)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_slvif_wrparity_err_intstat(
        &mut self,
    ) -> CH4_CLEAR_SLVIF_WRPARITY_ERR_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_SLVIF_WRPARITY_ERR_INTSTAT_W::new(self, 25)
    }
    #[doc = "Bit 27 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_ch_lock_cleared_intstat(
        &mut self,
    ) -> CH4_CLEAR_CH_LOCK_CLEARED_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_CH_LOCK_CLEARED_INTSTAT_W::new(self, 27)
    }
    #[doc = "Bit 28 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_ch_src_suspended_intstat(
        &mut self,
    ) -> CH4_CLEAR_CH_SRC_SUSPENDED_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_CH_SRC_SUSPENDED_INTSTAT_W::new(self, 28)
    }
    #[doc = "Bit 29 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_ch_suspended_intstat(
        &mut self,
    ) -> CH4_CLEAR_CH_SUSPENDED_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_CH_SUSPENDED_INTSTAT_W::new(self, 29)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_ch_disabled_intstat(
        &mut self,
    ) -> CH4_CLEAR_CH_DISABLED_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_CH_DISABLED_INTSTAT_W::new(self, 30)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_ch_aborted_intstat(
        &mut self,
    ) -> CH4_CLEAR_CH_ABORTED_INTSTAT_W<CH4_INTCLEAR0_SPEC> {
        CH4_CLEAR_CH_ABORTED_INTSTAT_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_intclear0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_INTCLEAR0_SPEC;
impl crate::RegisterSpec for CH4_INTCLEAR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch4_intclear0::W`](W) writer structure"]
impl crate::Writable for CH4_INTCLEAR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4_INTCLEAR0 to value 0"]
impl crate::Resettable for CH4_INTCLEAR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
