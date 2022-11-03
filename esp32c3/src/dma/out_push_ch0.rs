#[doc = "Register `OUT_PUSH_CH0` reader"]
pub struct R(crate::R<OUT_PUSH_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PUSH_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PUSH_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PUSH_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PUSH_CH0` writer"]
pub struct W(crate::W<OUT_PUSH_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PUSH_CH0_SPEC>;
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
impl From<crate::W<OUT_PUSH_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PUSH_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTFIFO_WDATA` reader - This register stores the data that need to be pushed into DMA FIFO."]
pub type OUTFIFO_WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTFIFO_WDATA` writer - This register stores the data that need to be pushed into DMA FIFO."]
pub type OUTFIFO_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUT_PUSH_CH0_SPEC, u16, u16, 9, O>;
#[doc = "Field `OUTFIFO_PUSH` reader - Set this bit to push data into DMA FIFO."]
pub type OUTFIFO_PUSH_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_PUSH` writer - Set this bit to push data into DMA FIFO."]
pub type OUTFIFO_PUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUT_PUSH_CH0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - This register stores the data that need to be pushed into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Set this bit to push data into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_push(&self) -> OUTFIFO_PUSH_R {
        OUTFIFO_PUSH_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register stores the data that need to be pushed into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W<0> {
        OUTFIFO_WDATA_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to push data into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W<9> {
        OUTFIFO_PUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_PUSH_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_push_ch0](index.html) module"]
pub struct OUT_PUSH_CH0_SPEC;
impl crate::RegisterSpec for OUT_PUSH_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_push_ch0::R](R) reader structure"]
impl crate::Readable for OUT_PUSH_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_push_ch0::W](W) writer structure"]
impl crate::Writable for OUT_PUSH_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_PUSH_CH0 to value 0"]
impl crate::Resettable for OUT_PUSH_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
