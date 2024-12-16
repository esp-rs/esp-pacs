#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `DB_TMP_READY` writer - Write 1 to clear H264_DB_TMP_READY_INT."]
pub type DB_TMP_READY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `REC_READY` writer - Write 1 to clear H264_REC_READY_INT."]
pub type REC_READY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FRAME_DONE` writer - Write 1 to clear H264_FRAME_DONE_INT."]
pub type FRAME_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMA_MOVE_2MB_LINE_DONE` writer - Clear bit: Write 1 to clear H264_DMA_MOVE_2MB_LINE_DONE_INT."]
pub type DMA_MOVE_2MB_LINE_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear H264_DB_TMP_READY_INT."]
    #[inline(always)]
    pub fn db_tmp_ready(&mut self) -> DB_TMP_READY_W<INT_CLR_SPEC> {
        DB_TMP_READY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear H264_REC_READY_INT."]
    #[inline(always)]
    pub fn rec_ready(&mut self) -> REC_READY_W<INT_CLR_SPEC> {
        REC_READY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear H264_FRAME_DONE_INT."]
    #[inline(always)]
    pub fn frame_done(&mut self) -> FRAME_DONE_W<INT_CLR_SPEC> {
        FRAME_DONE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit: Write 1 to clear H264_DMA_MOVE_2MB_LINE_DONE_INT."]
    #[inline(always)]
    pub fn dma_move_2mb_line_done(&mut self) -> DMA_MOVE_2MB_LINE_DONE_W<INT_CLR_SPEC> {
        DMA_MOVE_2MB_LINE_DONE_W::new(self, 3)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
