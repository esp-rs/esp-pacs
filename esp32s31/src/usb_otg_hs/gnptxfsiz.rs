#[doc = "Register `GNPTXFSIZ` reader"]
pub type R = crate::R<GNPTXFSIZ_SPEC>;
#[doc = "Register `GNPTXFSIZ` writer"]
pub type W = crate::W<GNPTXFSIZ_SPEC>;
#[doc = "Field `NPTXFSTADDR` reader - Non-periodic Transmit RAM Start Address (NPTxFStAddr) For host mode, this field is always valid. This field contains the memory start address for Non-periodic Transmit FIFO RAM. - This field is determined during coreConsultant configuration by Enable Dynamic FIFO Sizing? (OTG_DFIFO_DYNAMIC):OTG_DFIFO_DYNAMIC = 0 These flops are optimized, and reads return the power-on value. - OTG_DFIFO_DYNAMIC = 1 The application can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant. Programmed values must not exceed the power-on value set in coreConsultant. The power-on reset value of this field is specified during coreConsultant configuration by Largest Rx Data FIFO Depth (parameter OTG_RX_DFIFO_DEPTH)."]
pub type NPTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSTADDR` writer - Non-periodic Transmit RAM Start Address (NPTxFStAddr) For host mode, this field is always valid. This field contains the memory start address for Non-periodic Transmit FIFO RAM. - This field is determined during coreConsultant configuration by Enable Dynamic FIFO Sizing? (OTG_DFIFO_DYNAMIC):OTG_DFIFO_DYNAMIC = 0 These flops are optimized, and reads return the power-on value. - OTG_DFIFO_DYNAMIC = 1 The application can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant. Programmed values must not exceed the power-on value set in coreConsultant. The power-on reset value of this field is specified during coreConsultant configuration by Largest Rx Data FIFO Depth (parameter OTG_RX_DFIFO_DEPTH)."]
pub type NPTXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NPTXFDEP` reader - Mode: Host only Non-periodic TxFIFO Depth (NPTxFDep) For host mode, this field is always valid. For device mode, this field is valid only when OTG_EN_DED_TX_FIFO=0. This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 This attribute of field is determined during coreConsultant configuration by Enable Dynamic FIFO Sizing? (OTG_DFIFO_DYNAMIC): - OTG_DFIFO_DYNAMIC = 0: These flops are optimized, and reads return the power-on value. - OTG_DFIFO_DYNAMIC = 1: The application can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant. The power-on reset value of this field is specified during coreConsultant configuration as Largest IN Endpoint FIFO 0 Depth (parameter OTG_TX_DINEP_DFIFO_DEPTH_0)."]
pub type NPTXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFDEP` writer - Mode: Host only Non-periodic TxFIFO Depth (NPTxFDep) For host mode, this field is always valid. For device mode, this field is valid only when OTG_EN_DED_TX_FIFO=0. This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 This attribute of field is determined during coreConsultant configuration by Enable Dynamic FIFO Sizing? (OTG_DFIFO_DYNAMIC): - OTG_DFIFO_DYNAMIC = 0: These flops are optimized, and reads return the power-on value. - OTG_DFIFO_DYNAMIC = 1: The application can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant. The power-on reset value of this field is specified during coreConsultant configuration as Largest IN Endpoint FIFO 0 Depth (parameter OTG_TX_DINEP_DFIFO_DEPTH_0)."]
pub type NPTXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Non-periodic Transmit RAM Start Address (NPTxFStAddr) For host mode, this field is always valid. This field contains the memory start address for Non-periodic Transmit FIFO RAM. - This field is determined during coreConsultant configuration by Enable Dynamic FIFO Sizing? (OTG_DFIFO_DYNAMIC):OTG_DFIFO_DYNAMIC = 0 These flops are optimized, and reads return the power-on value. - OTG_DFIFO_DYNAMIC = 1 The application can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant. Programmed values must not exceed the power-on value set in coreConsultant. The power-on reset value of this field is specified during coreConsultant configuration by Largest Rx Data FIFO Depth (parameter OTG_RX_DFIFO_DEPTH)."]
    #[inline(always)]
    pub fn nptxfstaddr(&self) -> NPTXFSTADDR_R {
        NPTXFSTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Mode: Host only Non-periodic TxFIFO Depth (NPTxFDep) For host mode, this field is always valid. For device mode, this field is valid only when OTG_EN_DED_TX_FIFO=0. This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 This attribute of field is determined during coreConsultant configuration by Enable Dynamic FIFO Sizing? (OTG_DFIFO_DYNAMIC): - OTG_DFIFO_DYNAMIC = 0: These flops are optimized, and reads return the power-on value. - OTG_DFIFO_DYNAMIC = 1: The application can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant. The power-on reset value of this field is specified during coreConsultant configuration as Largest IN Endpoint FIFO 0 Depth (parameter OTG_TX_DINEP_DFIFO_DEPTH_0)."]
    #[inline(always)]
    pub fn nptxfdep(&self) -> NPTXFDEP_R {
        NPTXFDEP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXFSIZ")
            .field("nptxfstaddr", &self.nptxfstaddr())
            .field("nptxfdep", &self.nptxfdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Non-periodic Transmit RAM Start Address (NPTxFStAddr) For host mode, this field is always valid. This field contains the memory start address for Non-periodic Transmit FIFO RAM. - This field is determined during coreConsultant configuration by Enable Dynamic FIFO Sizing? (OTG_DFIFO_DYNAMIC):OTG_DFIFO_DYNAMIC = 0 These flops are optimized, and reads return the power-on value. - OTG_DFIFO_DYNAMIC = 1 The application can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant. Programmed values must not exceed the power-on value set in coreConsultant. The power-on reset value of this field is specified during coreConsultant configuration by Largest Rx Data FIFO Depth (parameter OTG_RX_DFIFO_DEPTH)."]
    #[inline(always)]
    pub fn nptxfstaddr(&mut self) -> NPTXFSTADDR_W<'_, GNPTXFSIZ_SPEC> {
        NPTXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Mode: Host only Non-periodic TxFIFO Depth (NPTxFDep) For host mode, this field is always valid. For device mode, this field is valid only when OTG_EN_DED_TX_FIFO=0. This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 This attribute of field is determined during coreConsultant configuration by Enable Dynamic FIFO Sizing? (OTG_DFIFO_DYNAMIC): - OTG_DFIFO_DYNAMIC = 0: These flops are optimized, and reads return the power-on value. - OTG_DFIFO_DYNAMIC = 1: The application can write a new value in this field. Programmed values must not exceed the power-on value set in coreConsultant. The power-on reset value of this field is specified during coreConsultant configuration as Largest IN Endpoint FIFO 0 Depth (parameter OTG_TX_DINEP_DFIFO_DEPTH_0)."]
    #[inline(always)]
    pub fn nptxfdep(&mut self) -> NPTXFDEP_W<'_, GNPTXFSIZ_SPEC> {
        NPTXFDEP_W::new(self, 16)
    }
}
#[doc = "The application can program the RAM size and the memory start address for the Non-periodic TxFIFO Note: The fields of this register change depending on host or device mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXFSIZ_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz::R`](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz::W`](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GNPTXFSIZ to value 0x0400_0400"]
impl crate::Resettable for GNPTXFSIZ_SPEC {
    const RESET_VALUE: u32 = 0x0400_0400;
}
