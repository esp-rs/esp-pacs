#[doc = "Register `DMA_OUT_PUSH` reader"]
pub type R = crate::R<DMA_OUT_PUSH_SPEC>;
#[doc = "Register `DMA_OUT_PUSH` writer"]
pub type W = crate::W<DMA_OUT_PUSH_SPEC>;
#[doc = "Field `OUTFIFO_WDATA` reader - This is the data need to be pushed into out link descriptor's fifo."]
pub type OUTFIFO_WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFO_WDATA` writer - This is the data need to be pushed into out link descriptor's fifo."]
pub type OUTFIFO_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `OUTFIFO_PUSH` reader - Set this bit to push data in out link descriptor's fifo."]
pub type OUTFIFO_PUSH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_PUSH` writer - Set this bit to push data in out link descriptor's fifo."]
pub type OUTFIFO_PUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - This is the data need to be pushed into out link descriptor's fifo."]
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Set this bit to push data in out link descriptor's fifo."]
    #[inline(always)]
    pub fn outfifo_push(&self) -> OUTFIFO_PUSH_R {
        OUTFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_PUSH")
            .field(
                "outfifo_wdata",
                &format_args!("{}", self.outfifo_wdata().bits()),
            )
            .field(
                "outfifo_push",
                &format_args!("{}", self.outfifo_push().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_OUT_PUSH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - This is the data need to be pushed into out link descriptor's fifo."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W<DMA_OUT_PUSH_SPEC> {
        OUTFIFO_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 16 - Set this bit to push data in out link descriptor's fifo."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W<DMA_OUT_PUSH_SPEC> {
        OUTFIFO_PUSH_W::new(self, 16)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_push::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_push::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_OUT_PUSH_SPEC;
impl crate::RegisterSpec for DMA_OUT_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_out_push::R`](R) reader structure"]
impl crate::Readable for DMA_OUT_PUSH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_out_push::W`](W) writer structure"]
impl crate::Writable for DMA_OUT_PUSH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_OUT_PUSH to value 0"]
impl crate::Resettable for DMA_OUT_PUSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
