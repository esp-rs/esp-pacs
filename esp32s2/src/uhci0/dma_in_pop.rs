#[doc = "Register `DMA_IN_POP` reader"]
pub struct R(crate::R<DMA_IN_POP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_POP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_POP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_IN_POP` writer"]
pub struct W(crate::W<DMA_IN_POP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_IN_POP_SPEC>;
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
impl From<crate::W<DMA_IN_POP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_IN_POP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INFIFO_RDATA` reader - This register stores the data popping from RX FIFO."]
pub type INFIFO_RDATA_R = crate::FieldReader<u16>;
#[doc = "Field `INFIFO_POP` reader - Set this bit to pop data from RX FIFO."]
pub type INFIFO_POP_R = crate::BitReader;
#[doc = "Field `INFIFO_POP` writer - Set this bit to pop data from RX FIFO."]
pub type INFIFO_POP_W<'a, const O: u8> = crate::BitWriter<'a, DMA_IN_POP_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - This register stores the data popping from RX FIFO."]
    #[inline(always)]
    pub fn infifo_rdata(&self) -> INFIFO_RDATA_R {
        INFIFO_RDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Set this bit to pop data from RX FIFO."]
    #[inline(always)]
    pub fn infifo_pop(&self) -> INFIFO_POP_R {
        INFIFO_POP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_POP")
            .field(
                "infifo_rdata",
                &format_args!("{}", self.infifo_rdata().bits()),
            )
            .field("infifo_pop", &format_args!("{}", self.infifo_pop().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_IN_POP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 16 - Set this bit to pop data from RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_pop(&mut self) -> INFIFO_POP_W<16> {
        INFIFO_POP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pop control register of RX FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_pop](index.html) module"]
pub struct DMA_IN_POP_SPEC;
impl crate::RegisterSpec for DMA_IN_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_pop::R](R) reader structure"]
impl crate::Readable for DMA_IN_POP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_in_pop::W](W) writer structure"]
impl crate::Writable for DMA_IN_POP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_IN_POP to value 0"]
impl crate::Resettable for DMA_IN_POP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
