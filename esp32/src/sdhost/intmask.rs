#[doc = "Register `INTMASK` reader"]
pub type R = crate::R<INTMASK_SPEC>;
#[doc = "Register `INTMASK` writer"]
pub type W = crate::W<INTMASK_SPEC>;
#[doc = "Field `INT_MASK` reader - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
pub type INT_MASK_R = crate::FieldReader<u16>;
#[doc = "Field `INT_MASK` writer - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
pub type INT_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SDIO_INT_MASK` reader - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
pub type SDIO_INT_MASK_R = crate::FieldReader;
#[doc = "Field `SDIO_INT_MASK` writer - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
pub type SDIO_INT_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
            .field("int_mask", &self.int_mask())
            .field("sdio_int_mask", &self.sdio_int_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
    #[inline(always)]
    #[must_use]
    pub fn int_mask(&mut self) -> INT_MASK_W<INTMASK_SPEC> {
        INT_MASK_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_mask(&mut self) -> SDIO_INT_MASK_W<INTMASK_SPEC> {
        SDIO_INT_MASK_W::new(self, 16)
    }
}
#[doc = "SDIO interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMASK_SPEC;
impl crate::RegisterSpec for INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmask::R`](R) reader structure"]
impl crate::Readable for INTMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intmask::W`](W) writer structure"]
impl crate::Writable for INTMASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for INTMASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
