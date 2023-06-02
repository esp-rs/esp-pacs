#[doc = "Register `DMA_OUT_PUSH` reader"]
pub struct R(crate::R<DMA_OUT_PUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_PUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_PUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_PUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_OUT_PUSH` writer"]
pub struct W(crate::W<DMA_OUT_PUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_OUT_PUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_OUT_PUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_OUT_PUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTFIFO_WDATA` reader - This is the data that need to be pushed into TX FIFO."]
pub type OUTFIFO_WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTFIFO_WDATA` writer - This is the data that need to be pushed into TX FIFO."]
pub type OUTFIFO_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_OUT_PUSH_SPEC, 9, O, u16, u16>;
#[doc = "Field `OUTFIFO_PUSH` reader - Set this bit to push data into TX FIFO."]
pub type OUTFIFO_PUSH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_PUSH` writer - Set this bit to push data into TX FIFO."]
pub type OUTFIFO_PUSH_W<'a, const O: u8> = crate::BitWriter<'a, DMA_OUT_PUSH_SPEC, O>;
impl R {
    #[doc = "Bits 0:8 - This is the data that need to be pushed into TX FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Set this bit to push data into TX FIFO."]
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - This is the data that need to be pushed into TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W<0> {
        OUTFIFO_WDATA_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to push data into TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W<16> {
        OUTFIFO_PUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Push control register of TX FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_push](index.html) module"]
pub struct DMA_OUT_PUSH_SPEC;
impl crate::RegisterSpec for DMA_OUT_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_push::R](R) reader structure"]
impl crate::Readable for DMA_OUT_PUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_out_push::W](W) writer structure"]
impl crate::Writable for DMA_OUT_PUSH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_OUT_PUSH to value 0"]
impl crate::Resettable for DMA_OUT_PUSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
