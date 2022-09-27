#[doc = "Register `SLC_TX_FIFO_POP` reader"]
pub struct R(crate::R<SLC_TX_FIFO_POP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_TX_FIFO_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_TX_FIFO_POP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_TX_FIFO_POP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_TX_FIFO_POP` writer"]
pub struct W(crate::W<SLC_TX_FIFO_POP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_TX_FIFO_POP_SPEC>;
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
impl From<crate::W<SLC_TX_FIFO_POP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_TX_FIFO_POP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_TXFIFO_RDATA` reader - "]
pub type SLC_TXFIFO_RDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC_TXFIFO_RDATA` writer - "]
pub type SLC_TXFIFO_RDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLC_TX_FIFO_POP_SPEC, u16, u16, 11, O>;
#[doc = "Field `SLC_TXFIFO_POP` reader - "]
pub type SLC_TXFIFO_POP_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TXFIFO_POP` writer - "]
pub type SLC_TXFIFO_POP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_TX_FIFO_POP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc_txfifo_rdata(&self) -> SLC_TXFIFO_RDATA_R {
        SLC_TXFIFO_RDATA_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_txfifo_pop(&self) -> SLC_TXFIFO_POP_R {
        SLC_TXFIFO_POP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc_txfifo_rdata(&mut self) -> SLC_TXFIFO_RDATA_W<0> {
        SLC_TXFIFO_RDATA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_txfifo_pop(&mut self) -> SLC_TXFIFO_POP_W<16> {
        SLC_TXFIFO_POP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_TX_FIFO_POP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_tx_fifo_pop](index.html) module"]
pub struct SLC_TX_FIFO_POP_SPEC;
impl crate::RegisterSpec for SLC_TX_FIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_tx_fifo_pop::R](R) reader structure"]
impl crate::Readable for SLC_TX_FIFO_POP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_tx_fifo_pop::W](W) writer structure"]
impl crate::Writable for SLC_TX_FIFO_POP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLC_TX_FIFO_POP to value 0"]
impl crate::Resettable for SLC_TX_FIFO_POP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
