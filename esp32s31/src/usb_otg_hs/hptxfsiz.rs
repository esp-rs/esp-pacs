#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HPTXFSIZ_SPEC>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HPTXFSIZ_SPEC>;
#[doc = "Field `PTXFSTADDR` reader - Host Periodic TxFIFO Start Address (PTxFStAddr) The power-on reset value of this register is the sum of the Largest Rx Data FIFO Depth and Largest Non-periodic Tx Data FIFO Depth.These parameters are: In shared FIFO operation: - OTG_RX_DFIFO_DEPTH + OTG_TX_NPERIO_DFIFO_DEPTH In dedicated FIFO mode: - OTG_RX_DFIFO_DEPTH + OTG_TX_HNPERIO_DFIFO_DEPTH If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant."]
pub type PTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSTADDR` writer - Host Periodic TxFIFO Start Address (PTxFStAddr) The power-on reset value of this register is the sum of the Largest Rx Data FIFO Depth and Largest Non-periodic Tx Data FIFO Depth.These parameters are: In shared FIFO operation: - OTG_RX_DFIFO_DEPTH + OTG_TX_NPERIO_DFIFO_DEPTH In dedicated FIFO mode: - OTG_RX_DFIFO_DEPTH + OTG_TX_HNPERIO_DFIFO_DEPTH If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant."]
pub type PTXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PTXFSIZE` reader - Host Periodic TxFIFO Depth (PTxFSize) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest Host Mode Periodic Tx Data FIFO Depth. - If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. - If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant."]
pub type PTXFSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZE` writer - Host Periodic TxFIFO Depth (PTxFSize) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest Host Mode Periodic Tx Data FIFO Depth. - If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. - If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant."]
pub type PTXFSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:11 - Host Periodic TxFIFO Start Address (PTxFStAddr) The power-on reset value of this register is the sum of the Largest Rx Data FIFO Depth and Largest Non-periodic Tx Data FIFO Depth.These parameters are: In shared FIFO operation: - OTG_RX_DFIFO_DEPTH + OTG_TX_NPERIO_DFIFO_DEPTH In dedicated FIFO mode: - OTG_RX_DFIFO_DEPTH + OTG_TX_HNPERIO_DFIFO_DEPTH If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant."]
    #[inline(always)]
    pub fn ptxfstaddr(&self) -> PTXFSTADDR_R {
        PTXFSTADDR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:26 - Host Periodic TxFIFO Depth (PTxFSize) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest Host Mode Periodic Tx Data FIFO Depth. - If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. - If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant."]
    #[inline(always)]
    pub fn ptxfsize(&self) -> PTXFSIZE_R {
        PTXFSIZE_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXFSIZ")
            .field("ptxfstaddr", &self.ptxfstaddr())
            .field("ptxfsize", &self.ptxfsize())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Host Periodic TxFIFO Start Address (PTxFStAddr) The power-on reset value of this register is the sum of the Largest Rx Data FIFO Depth and Largest Non-periodic Tx Data FIFO Depth.These parameters are: In shared FIFO operation: - OTG_RX_DFIFO_DEPTH + OTG_TX_NPERIO_DFIFO_DEPTH In dedicated FIFO mode: - OTG_RX_DFIFO_DEPTH + OTG_TX_HNPERIO_DFIFO_DEPTH If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant."]
    #[inline(always)]
    pub fn ptxfstaddr(&mut self) -> PTXFSTADDR_W<'_, HPTXFSIZ_SPEC> {
        PTXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Host Periodic TxFIFO Depth (PTxFSize) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest Host Mode Periodic Tx Data FIFO Depth. - If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. - If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant."]
    #[inline(always)]
    pub fn ptxfsize(&mut self) -> PTXFSIZE_W<'_, HPTXFSIZ_SPEC> {
        PTXFSIZE_W::new(self, 16)
    }
}
#[doc = "This register holds the size and the memory start address of the Periodic TxFIFO. Note: Read the reset value of this register only after the following conditions: - If IDDIG_FILTER is disabled, read only after PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires.\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTXFSIZ_SPEC;
impl crate::RegisterSpec for HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HPTXFSIZ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0400_0800"]
impl crate::Resettable for HPTXFSIZ_SPEC {
    const RESET_VALUE: u32 = 0x0400_0800;
}
