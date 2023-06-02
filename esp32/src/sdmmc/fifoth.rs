#[doc = "Register `FIFOTH` reader"]
pub struct R(crate::R<FIFOTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOTH` writer"]
pub struct W(crate::W<FIFOTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOTH_SPEC>;
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
impl From<crate::W<FIFOTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_WMARK` reader - FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred."]
pub type TX_WMARK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_WMARK` writer - FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred."]
pub type TX_WMARK_W<'a, const O: u8> = crate::FieldWriter<'a, FIFOTH_SPEC, 12, O, u16, u16>;
#[doc = "Field `RX_WMARK` reader - FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set."]
pub type RX_WMARK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_WMARK` writer - FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set."]
pub type RX_WMARK_W<'a, const O: u8> = crate::FieldWriter<'a, FIFOTH_SPEC, 11, O, u16, u16>;
#[doc = "Field `DMA_MULTIPLE_TRANSACTION_SIZE` reader - Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE. 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer."]
pub type DMA_MULTIPLE_TRANSACTION_SIZE_R = crate::FieldReader;
#[doc = "Field `DMA_MULTIPLE_TRANSACTION_SIZE` writer - Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE. 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer."]
pub type DMA_MULTIPLE_TRANSACTION_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, FIFOTH_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred."]
    #[inline(always)]
    pub fn tx_wmark(&self) -> TX_WMARK_R {
        TX_WMARK_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:26 - FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set."]
    #[inline(always)]
    pub fn rx_wmark(&self) -> RX_WMARK_R {
        RX_WMARK_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE. 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer."]
    #[inline(always)]
    pub fn dma_multiple_transaction_size(&self) -> DMA_MULTIPLE_TRANSACTION_SIZE_R {
        DMA_MULTIPLE_TRANSACTION_SIZE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOTH")
            .field("tx_wmark", &format_args!("{}", self.tx_wmark().bits()))
            .field("rx_wmark", &format_args!("{}", self.rx_wmark().bits()))
            .field(
                "dma_multiple_transaction_size",
                &format_args!("{}", self.dma_multiple_transaction_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFOTH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming.In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wmark(&mut self) -> TX_WMARK_W<0> {
        TX_WMARK_W::new(self)
    }
    #[doc = "Bits 16:26 - FIFO threshold watermark level when receiving data to card.When FIFO data count reaches greater than this number , DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data.In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request.During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt.In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wmark(&mut self) -> RX_WMARK_W<16> {
        RX_WMARK_W::new(self)
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction, should be programmed same as DMA controller multiple-transaction-size SDHOST_SRC/DEST_MSIZE. 000: 1-byte transfer; 001: 4-byte transfer; 010: 8-byte transfer; 011: 16-byte transfer; 100: 32-byte transfer; 101: 64-byte transfer; 110: 128-byte transfer; 111: 256-byte transfer."]
    #[inline(always)]
    #[must_use]
    pub fn dma_multiple_transaction_size(&mut self) -> DMA_MULTIPLE_TRANSACTION_SIZE_W<28> {
        DMA_MULTIPLE_TRANSACTION_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoth](index.html) module"]
pub struct FIFOTH_SPEC;
impl crate::RegisterSpec for FIFOTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoth::R](R) reader structure"]
impl crate::Readable for FIFOTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoth::W](W) writer structure"]
impl crate::Writable for FIFOTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOTH to value 0"]
impl crate::Resettable for FIFOTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
