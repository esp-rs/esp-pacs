#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `DB_TMP_READY_INT_CLR` writer - Write 1 to clear H264_DB_TMP_READY_INT."]
pub type DB_TMP_READY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REC_READY_INT_CLR` writer - Write 1 to clear H264_REC_READY_INT."]
pub type REC_READY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_DONE_INT_CLR` writer - Write 1 to clear H264_FRAME_DONE_INT."]
pub type FRAME_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MOVE_2MB_LINE_DONE_INT_CLR` writer - Clear bit: Write 1 to clear H264_DMA_MOVE_2MB_LINE_DONE_INT."]
pub type DMA_MOVE_2MB_LINE_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear H264_DB_TMP_READY_INT."]
    #[inline(always)]
    #[must_use]
    pub fn db_tmp_ready_int_clr(&mut self) -> DB_TMP_READY_INT_CLR_W<INT_CLR_SPEC> {
        DB_TMP_READY_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear H264_REC_READY_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rec_ready_int_clr(&mut self) -> REC_READY_INT_CLR_W<INT_CLR_SPEC> {
        REC_READY_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear H264_FRAME_DONE_INT."]
    #[inline(always)]
    #[must_use]
    pub fn frame_done_int_clr(&mut self) -> FRAME_DONE_INT_CLR_W<INT_CLR_SPEC> {
        FRAME_DONE_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit: Write 1 to clear H264_DMA_MOVE_2MB_LINE_DONE_INT."]
    #[inline(always)]
    #[must_use]
    pub fn dma_move_2mb_line_done_int_clr(
        &mut self,
    ) -> DMA_MOVE_2MB_LINE_DONE_INT_CLR_W<INT_CLR_SPEC> {
        DMA_MOVE_2MB_LINE_DONE_INT_CLR_W::new(self, 3)
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
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
