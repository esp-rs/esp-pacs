#[doc = "Register `IDSTS` reader"]
pub type R = crate::R<IDSTS_SPEC>;
#[doc = "Register `IDSTS` writer"]
pub type W = crate::W<IDSTS_SPEC>;
#[doc = "Field `TI` reader - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
pub type TI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
pub type RI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
pub type FBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\] = 0). Writing 1 clears this bit."]
pub type DU_R = crate::BitReader;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\] = 0). Writing 1 clears this bit."]
pub type DU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CES` reader - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
pub type CES_R = crate::BitReader;
#[doc = "Field `CES` writer - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
pub type CES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\] : Transmit Interrupt, IDSTS\\[1\\] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\] : Transmit Interrupt, IDSTS\\[1\\] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
pub type NIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\] : Fatal Bus Interrupt, IDSTS\\[4\\] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\] : Fatal Bus Interrupt, IDSTS\\[4\\] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
pub type AIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE_CODE` reader - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\] is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
pub type FBE_CODE_R = crate::FieldReader;
#[doc = "Field `FBE_CODE` writer - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\] is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
pub type FBE_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FSM` reader - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
pub type FSM_R = crate::FieldReader;
#[doc = "Field `FSM` writer - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
pub type FSM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\] = 0). Writing 1 clears this bit."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\] : Transmit Interrupt, IDSTS\\[1\\] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\] : Fatal Bus Interrupt, IDSTS\\[4\\] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\] is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
    #[inline(always)]
    pub fn fbe_code(&self) -> FBE_CODE_R {
        FBE_CODE_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:16 - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
    #[inline(always)]
    pub fn fsm(&self) -> FSM_R {
        FSM_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDSTS")
            .field("ti", &self.ti())
            .field("ri", &self.ri())
            .field("fbe", &self.fbe())
            .field("du", &self.du())
            .field("ces", &self.ces())
            .field("nis", &self.nis())
            .field("ais", &self.ais())
            .field("fbe_code", &self.fbe_code())
            .field("fsm", &self.fsm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W<IDSTS_SPEC> {
        TI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W<IDSTS_SPEC> {
        RI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W<IDSTS_SPEC> {
        FBE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\] = 0). Writing 1 clears this bit."]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<IDSTS_SPEC> {
        DU_W::new(self, 4)
    }
    #[doc = "Bit 5 - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W<IDSTS_SPEC> {
        CES_W::new(self, 5)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\] : Transmit Interrupt, IDSTS\\[1\\] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W<IDSTS_SPEC> {
        NIS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\] : Fatal Bus Interrupt, IDSTS\\[4\\] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W<IDSTS_SPEC> {
        AIS_W::new(self, 9)
    }
    #[doc = "Bits 10:12 - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\] is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
    #[inline(always)]
    pub fn fbe_code(&mut self) -> FBE_CODE_W<IDSTS_SPEC> {
        FBE_CODE_W::new(self, 10)
    }
    #[doc = "Bits 13:16 - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
    #[inline(always)]
    pub fn fsm(&mut self) -> FSM_W<IDSTS_SPEC> {
        FSM_W::new(self, 13)
    }
}
#[doc = "IDMAC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`idsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDSTS_SPEC;
impl crate::RegisterSpec for IDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idsts::R`](R) reader structure"]
impl crate::Readable for IDSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idsts::W`](W) writer structure"]
impl crate::Writable for IDSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDSTS to value 0"]
impl crate::Resettable for IDSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
