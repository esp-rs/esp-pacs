#[doc = "Register `INTMASK` reader"]
pub struct R(crate::R<INTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTMASK` writer"]
pub struct W(crate::W<INTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTMASK_SPEC>;
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
impl From<crate::W<INTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_MASK` reader - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
pub type INT_MASK_R = crate::FieldReader<u16>;
#[doc = "Field `INT_MASK` writer - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
pub type INT_MASK_W<'a, const O: u8> = crate::FieldWriter<'a, INTMASK_SPEC, 16, O, u16>;
#[doc = "Field `SDIO_INT_MASK` reader - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
pub type SDIO_INT_MASK_R = crate::FieldReader;
#[doc = "Field `SDIO_INT_MASK` writer - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
pub type SDIO_INT_MASK_W<'a, const O: u8> = crate::FieldWriter<'a, INTMASK_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:15 - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
    #[inline(always)]
    pub fn int_mask(&self) -> INT_MASK_R {
        INT_MASK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SDIO_INT_MASK_R {
        SDIO_INT_MASK_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMASK")
            .field("int_mask", &format_args!("{}", self.int_mask().bits()))
            .field(
                "sdio_int_mask",
                &format_args!("{}", self.sdio_int_mask().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTMASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
    #[inline(always)]
    #[must_use]
    pub fn int_mask(&mut self) -> INT_MASK_W<0> {
        INT_MASK_W::new(self)
    }
    #[doc = "Bits 16:17 - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_mask(&mut self) -> SDIO_INT_MASK_W<16> {
        SDIO_INT_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intmask](index.html) module"]
pub struct INTMASK_SPEC;
impl crate::RegisterSpec for INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intmask::R](R) reader structure"]
impl crate::Readable for INTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intmask::W](W) writer structure"]
impl crate::Writable for INTMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for INTMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
