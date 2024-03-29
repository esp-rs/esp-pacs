#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `DB_TMP_READY` reader - The masked interrupt status of H264_DB_TMP_READY_INT. Valid only when the H264_DB_TMP_READY_INT_ENA is set to 1."]
pub type DB_TMP_READY_R = crate::BitReader;
#[doc = "Field `REC_READY` reader - The masked interrupt status of H264_REC_READY_INT. Valid only when the H264_REC_READY_INT_ENA is set to 1."]
pub type REC_READY_R = crate::BitReader;
#[doc = "Field `FRAME_DONE` reader - The masked interrupt status of H264_FRAME_DONE_INT. Valid only when the H264_FRAME_DONE_INT_ENA is set to 1."]
pub type FRAME_DONE_R = crate::BitReader;
#[doc = "Field `DMA_MOVE_2MB_LINE_DONE` reader - Masked status bit: The masked interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Valid only when the H264_DMA_MOVE_2MB_LINE_DONE_INT_ENA is set to 1."]
pub type DMA_MOVE_2MB_LINE_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of H264_DB_TMP_READY_INT. Valid only when the H264_DB_TMP_READY_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn db_tmp_ready(&self) -> DB_TMP_READY_R {
        DB_TMP_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of H264_REC_READY_INT. Valid only when the H264_REC_READY_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn rec_ready(&self) -> REC_READY_R {
        REC_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of H264_FRAME_DONE_INT. Valid only when the H264_FRAME_DONE_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn frame_done(&self) -> FRAME_DONE_R {
        FRAME_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked status bit: The masked interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Valid only when the H264_DMA_MOVE_2MB_LINE_DONE_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn dma_move_2mb_line_done(&self) -> DMA_MOVE_2MB_LINE_DONE_R {
        DMA_MOVE_2MB_LINE_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "db_tmp_ready",
                &format_args!("{}", self.db_tmp_ready().bit()),
            )
            .field("rec_ready", &format_args!("{}", self.rec_ready().bit()))
            .field("frame_done", &format_args!("{}", self.frame_done().bit()))
            .field(
                "dma_move_2mb_line_done",
                &format_args!("{}", self.dma_move_2mb_line_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Interrupt masked status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
