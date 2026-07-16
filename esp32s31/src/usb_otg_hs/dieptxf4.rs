#[doc = "Register `DIEPTXF4` reader"]
pub type R = crate::R<DIEPTXF4_SPEC>;
#[doc = "Register `DIEPTXF4` writer"]
pub type W = crate::W<DIEPTXF4_SPEC>;
#[doc = "Field `INEPNTXFSTADDR` reader - IN Endpoint FIFOn Transmit RAM Start Address (INEPnTxFStAddr) This field contains the memory start address for the IN endpoint Transmit FIFO that this register corresponds to. The power-on reset value of this register is calculated according to the following formula: OTG_RX_DFIFO_DEPTH + SUM of OTG_TX_DINEP_DFIFO_DEPTH_'n', where n = 0 to (FIFO number-1). For example, Start address of INEP FIFO 1 (DIEPTXF1) is OTG_RX_DFIFO_DEPTH + OTG_TX_DINEP_DFIFO_DEPTH_0, start address of INEP FIFO 2 (DIEPTXF2) is OTG_RX_DFIFO_DEPTH + OTG_TX_DINEP_DFIFO_DEPTH_0 + OTG_TX_DINEP_DFIFO_DEPTH_1, and so on. If at POR the calculated value (C) exceeds 65535, then the Reset value becomes Reset Value(A) = (C-65536). - If Enable Dynamic FIFO Sizing is deselected in coreConsultant (OTG_DFIFO_DYNAMIC = 0), this field is read-only and read value is the power-on reset value. - If Enable Dynamic FIFO Sizing is selected in coreConsultant (OTG_DFIFO_DYNAMIC = 1), and you have calculated or programmed a new value for RxFIFO depth or TX FIFO depths, you can program their values according to the above formula. Programmed values must not exceed the power-on value set in coreConsultant."]
pub type INEPNTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFSTADDR` writer - IN Endpoint FIFOn Transmit RAM Start Address (INEPnTxFStAddr) This field contains the memory start address for the IN endpoint Transmit FIFO that this register corresponds to. The power-on reset value of this register is calculated according to the following formula: OTG_RX_DFIFO_DEPTH + SUM of OTG_TX_DINEP_DFIFO_DEPTH_'n', where n = 0 to (FIFO number-1). For example, Start address of INEP FIFO 1 (DIEPTXF1) is OTG_RX_DFIFO_DEPTH + OTG_TX_DINEP_DFIFO_DEPTH_0, start address of INEP FIFO 2 (DIEPTXF2) is OTG_RX_DFIFO_DEPTH + OTG_TX_DINEP_DFIFO_DEPTH_0 + OTG_TX_DINEP_DFIFO_DEPTH_1, and so on. If at POR the calculated value (C) exceeds 65535, then the Reset value becomes Reset Value(A) = (C-65536). - If Enable Dynamic FIFO Sizing is deselected in coreConsultant (OTG_DFIFO_DYNAMIC = 0), this field is read-only and read value is the power-on reset value. - If Enable Dynamic FIFO Sizing is selected in coreConsultant (OTG_DFIFO_DYNAMIC = 1), and you have calculated or programmed a new value for RxFIFO depth or TX FIFO depths, you can program their values according to the above formula. Programmed values must not exceed the power-on value set in coreConsultant."]
pub type INEPNTXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `INEPNTXFDEP` reader - IN Endpoint TxFIFO Depth (INEPnTxFDep) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest IN Endpoint FIFO number Depth (parameter OTG_TX_DINEP_DFIFO_DEPTH_i) set during coreConsultant configuration, where i is the FIFO number this register corresponds to. - If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. - If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value."]
pub type INEPNTXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFDEP` writer - IN Endpoint TxFIFO Depth (INEPnTxFDep) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest IN Endpoint FIFO number Depth (parameter OTG_TX_DINEP_DFIFO_DEPTH_i) set during coreConsultant configuration, where i is the FIFO number this register corresponds to. - If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. - If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value."]
pub type INEPNTXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:11 - IN Endpoint FIFOn Transmit RAM Start Address (INEPnTxFStAddr) This field contains the memory start address for the IN endpoint Transmit FIFO that this register corresponds to. The power-on reset value of this register is calculated according to the following formula: OTG_RX_DFIFO_DEPTH + SUM of OTG_TX_DINEP_DFIFO_DEPTH_'n', where n = 0 to (FIFO number-1). For example, Start address of INEP FIFO 1 (DIEPTXF1) is OTG_RX_DFIFO_DEPTH + OTG_TX_DINEP_DFIFO_DEPTH_0, start address of INEP FIFO 2 (DIEPTXF2) is OTG_RX_DFIFO_DEPTH + OTG_TX_DINEP_DFIFO_DEPTH_0 + OTG_TX_DINEP_DFIFO_DEPTH_1, and so on. If at POR the calculated value (C) exceeds 65535, then the Reset value becomes Reset Value(A) = (C-65536). - If Enable Dynamic FIFO Sizing is deselected in coreConsultant (OTG_DFIFO_DYNAMIC = 0), this field is read-only and read value is the power-on reset value. - If Enable Dynamic FIFO Sizing is selected in coreConsultant (OTG_DFIFO_DYNAMIC = 1), and you have calculated or programmed a new value for RxFIFO depth or TX FIFO depths, you can program their values according to the above formula. Programmed values must not exceed the power-on value set in coreConsultant."]
    #[inline(always)]
    pub fn inepntxfstaddr(&self) -> INEPNTXFSTADDR_R {
        INEPNTXFSTADDR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth (INEPnTxFDep) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest IN Endpoint FIFO number Depth (parameter OTG_TX_DINEP_DFIFO_DEPTH_i) set during coreConsultant configuration, where i is the FIFO number this register corresponds to. - If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. - If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value."]
    #[inline(always)]
    pub fn inepntxfdep(&self) -> INEPNTXFDEP_R {
        INEPNTXFDEP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF4")
            .field("inepntxfstaddr", &self.inepntxfstaddr())
            .field("inepntxfdep", &self.inepntxfdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - IN Endpoint FIFOn Transmit RAM Start Address (INEPnTxFStAddr) This field contains the memory start address for the IN endpoint Transmit FIFO that this register corresponds to. The power-on reset value of this register is calculated according to the following formula: OTG_RX_DFIFO_DEPTH + SUM of OTG_TX_DINEP_DFIFO_DEPTH_'n', where n = 0 to (FIFO number-1). For example, Start address of INEP FIFO 1 (DIEPTXF1) is OTG_RX_DFIFO_DEPTH + OTG_TX_DINEP_DFIFO_DEPTH_0, start address of INEP FIFO 2 (DIEPTXF2) is OTG_RX_DFIFO_DEPTH + OTG_TX_DINEP_DFIFO_DEPTH_0 + OTG_TX_DINEP_DFIFO_DEPTH_1, and so on. If at POR the calculated value (C) exceeds 65535, then the Reset value becomes Reset Value(A) = (C-65536). - If Enable Dynamic FIFO Sizing is deselected in coreConsultant (OTG_DFIFO_DYNAMIC = 0), this field is read-only and read value is the power-on reset value. - If Enable Dynamic FIFO Sizing is selected in coreConsultant (OTG_DFIFO_DYNAMIC = 1), and you have calculated or programmed a new value for RxFIFO depth or TX FIFO depths, you can program their values according to the above formula. Programmed values must not exceed the power-on value set in coreConsultant."]
    #[inline(always)]
    pub fn inepntxfstaddr(&mut self) -> INEPNTXFSTADDR_W<'_, DIEPTXF4_SPEC> {
        INEPNTXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth (INEPnTxFDep) This value is in terms of 32-bit words. - Minimum value is 16 - Maximum value is 32,768 The power-on reset value of this register is specified as the Largest IN Endpoint FIFO number Depth (parameter OTG_TX_DINEP_DFIFO_DEPTH_i) set during coreConsultant configuration, where i is the FIFO number this register corresponds to. - If Enable Dynamic FIFO Sizing? was deselected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 0), these flops are optimized, and reads return the power-on value. - If Enable Dynamic FIFO Sizing? was selected in coreConsultant (parameter OTG_DFIFO_DYNAMIC = 1), you can write a new value in this field. Programmed values must not exceed the power-on value."]
    #[inline(always)]
    pub fn inepntxfdep(&mut self) -> INEPNTXFDEP_W<'_, DIEPTXF4_SPEC> {
        INEPNTXFDEP_W::new(self, 16)
    }
}
#[doc = "This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF4_SPEC;
impl crate::RegisterSpec for DIEPTXF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf4::R`](R) reader structure"]
impl crate::Readable for DIEPTXF4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf4::W`](W) writer structure"]
impl crate::Writable for DIEPTXF4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPTXF4 to value 0x0200_0c00"]
impl crate::Resettable for DIEPTXF4_SPEC {
    const RESET_VALUE: u32 = 0x0200_0c00;
}
