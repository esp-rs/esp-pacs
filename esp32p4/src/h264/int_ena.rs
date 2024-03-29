#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `DB_TMP_READY` reader - Write 1 to enable H264_DB_TMP_READY_INT."]
pub type DB_TMP_READY_R = crate::BitReader;
#[doc = "Field `DB_TMP_READY` writer - Write 1 to enable H264_DB_TMP_READY_INT."]
pub type DB_TMP_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REC_READY` reader - Write 1 to enable H264_REC_READY_INT."]
pub type REC_READY_R = crate::BitReader;
#[doc = "Field `REC_READY` writer - Write 1 to enable H264_REC_READY_INT."]
pub type REC_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_DONE` reader - Write 1 to enable H264_FRAME_DONE_INT."]
pub type FRAME_DONE_R = crate::BitReader;
#[doc = "Field `FRAME_DONE` writer - Write 1 to enable H264_FRAME_DONE_INT."]
pub type FRAME_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MOVE_2MB_LINE_DONE` reader - Enable bit: Write 1 to enable H264_DMA_MOVE_2MB_LINE_DONE_INT."]
pub type DMA_MOVE_2MB_LINE_DONE_R = crate::BitReader;
#[doc = "Field `DMA_MOVE_2MB_LINE_DONE` writer - Enable bit: Write 1 to enable H264_DMA_MOVE_2MB_LINE_DONE_INT."]
pub type DMA_MOVE_2MB_LINE_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable H264_DB_TMP_READY_INT."]
    #[inline(always)]
    pub fn db_tmp_ready(&self) -> DB_TMP_READY_R {
        DB_TMP_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable H264_REC_READY_INT."]
    #[inline(always)]
    pub fn rec_ready(&self) -> REC_READY_R {
        REC_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable H264_FRAME_DONE_INT."]
    #[inline(always)]
    pub fn frame_done(&self) -> FRAME_DONE_R {
        FRAME_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable bit: Write 1 to enable H264_DMA_MOVE_2MB_LINE_DONE_INT."]
    #[inline(always)]
    pub fn dma_move_2mb_line_done(&self) -> DMA_MOVE_2MB_LINE_DONE_R {
        DMA_MOVE_2MB_LINE_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
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
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable H264_DB_TMP_READY_INT."]
    #[inline(always)]
    #[must_use]
    pub fn db_tmp_ready(&mut self) -> DB_TMP_READY_W<INT_ENA_SPEC> {
        DB_TMP_READY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable H264_REC_READY_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rec_ready(&mut self) -> REC_READY_W<INT_ENA_SPEC> {
        REC_READY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable H264_FRAME_DONE_INT."]
    #[inline(always)]
    #[must_use]
    pub fn frame_done(&mut self) -> FRAME_DONE_W<INT_ENA_SPEC> {
        FRAME_DONE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable bit: Write 1 to enable H264_DMA_MOVE_2MB_LINE_DONE_INT."]
    #[inline(always)]
    #[must_use]
    pub fn dma_move_2mb_line_done(&mut self) -> DMA_MOVE_2MB_LINE_DONE_W<INT_ENA_SPEC> {
        DMA_MOVE_2MB_LINE_DONE_W::new(self, 3)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
