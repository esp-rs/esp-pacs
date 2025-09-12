#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `DB_TMP_READY` reader - Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel."]
pub type DB_TMP_READY_R = crate::BitReader;
#[doc = "Field `DB_TMP_READY` writer - Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel."]
pub type DB_TMP_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REC_READY` reader - Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel."]
pub type REC_READY_R = crate::BitReader;
#[doc = "Field `REC_READY` writer - Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel."]
pub type REC_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_DONE` reader - Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done."]
pub type FRAME_DONE_R = crate::BitReader;
#[doc = "Field `FRAME_DONE` writer - Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done."]
pub type FRAME_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MOVE_2MB_LINE_DONE` reader - Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done."]
pub type DMA_MOVE_2MB_LINE_DONE_R = crate::BitReader;
#[doc = "Field `DMA_MOVE_2MB_LINE_DONE` writer - Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done."]
pub type DMA_MOVE_2MB_LINE_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel."]
    #[inline(always)]
    pub fn db_tmp_ready(&self) -> DB_TMP_READY_R {
        DB_TMP_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel."]
    #[inline(always)]
    pub fn rec_ready(&self) -> REC_READY_R {
        REC_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done."]
    #[inline(always)]
    pub fn frame_done(&self) -> FRAME_DONE_R {
        FRAME_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done."]
    #[inline(always)]
    pub fn dma_move_2mb_line_done(&self) -> DMA_MOVE_2MB_LINE_DONE_R {
        DMA_MOVE_2MB_LINE_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("db_tmp_ready", &self.db_tmp_ready())
            .field("rec_ready", &self.rec_ready())
            .field("frame_done", &self.frame_done())
            .field("dma_move_2mb_line_done", &self.dma_move_2mb_line_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel."]
    #[inline(always)]
    pub fn db_tmp_ready(&mut self) -> DB_TMP_READY_W<'_, INT_RAW_SPEC> {
        DB_TMP_READY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel."]
    #[inline(always)]
    pub fn rec_ready(&mut self) -> REC_READY_W<'_, INT_RAW_SPEC> {
        REC_READY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done."]
    #[inline(always)]
    pub fn frame_done(&mut self) -> FRAME_DONE_W<'_, INT_RAW_SPEC> {
        FRAME_DONE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done."]
    #[inline(always)]
    pub fn dma_move_2mb_line_done(&mut self) -> DMA_MOVE_2MB_LINE_DONE_W<'_, INT_RAW_SPEC> {
        DMA_MOVE_2MB_LINE_DONE_W::new(self, 3)
    }
}
#[doc = "Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
