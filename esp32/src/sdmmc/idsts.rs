#[doc = "Register `IDSTS` reader"]
pub struct R(crate::R<IDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDSTS` writer"]
pub struct W(crate::W<IDSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDSTS_SPEC>;
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
impl From<crate::W<IDSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
pub type TI_W<'a, const O: u8> = crate::BitWriter<'a, IDSTS_SPEC, O>;
#[doc = "Field `RI` reader - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
pub type RI_W<'a, const O: u8> = crate::BitWriter<'a, IDSTS_SPEC, O>;
#[doc = "Field `FBE` reader - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
pub type FBE_W<'a, const O: u8> = crate::BitWriter<'a, IDSTS_SPEC, O>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\] = 0). Writing 1 clears this bit."]
pub type DU_R = crate::BitReader;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\] = 0). Writing 1 clears this bit."]
pub type DU_W<'a, const O: u8> = crate::BitWriter<'a, IDSTS_SPEC, O>;
#[doc = "Field `CES` reader - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
pub type CES_R = crate::BitReader;
#[doc = "Field `CES` writer - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
pub type CES_W<'a, const O: u8> = crate::BitWriter<'a, IDSTS_SPEC, O>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\] : Transmit Interrupt, IDSTS\\[1\\] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\] : Transmit Interrupt, IDSTS\\[1\\] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
pub type NIS_W<'a, const O: u8> = crate::BitWriter<'a, IDSTS_SPEC, O>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\] : Fatal Bus Interrupt, IDSTS\\[4\\] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\] : Fatal Bus Interrupt, IDSTS\\[4\\] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
pub type AIS_W<'a, const O: u8> = crate::BitWriter<'a, IDSTS_SPEC, O>;
#[doc = "Field `FBE_CODE` reader - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\] is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
pub type FBE_CODE_R = crate::FieldReader;
#[doc = "Field `FBE_CODE` writer - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\] is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
pub type FBE_CODE_W<'a, const O: u8> = crate::FieldWriter<'a, IDSTS_SPEC, 3, O>;
#[doc = "Field `FSM` reader - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
pub type FSM_R = crate::FieldReader;
#[doc = "Field `FSM` writer - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
pub type FSM_W<'a, const O: u8> = crate::FieldWriter<'a, IDSTS_SPEC, 4, O>;
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
            .field("ti", &format_args!("{}", self.ti().bit()))
            .field("ri", &format_args!("{}", self.ri().bit()))
            .field("fbe", &format_args!("{}", self.fbe().bit()))
            .field("du", &format_args!("{}", self.du().bit()))
            .field("ces", &format_args!("{}", self.ces().bit()))
            .field("nis", &format_args!("{}", self.nis().bit()))
            .field("ais", &format_args!("{}", self.ais().bit()))
            .field("fbe_code", &format_args!("{}", self.fbe_code().bits()))
            .field("fsm", &format_args!("{}", self.fsm().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IDSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<0> {
        TI_W::new(self)
    }
    #[doc = "Bit 1 - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<1> {
        RI_W::new(self)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<2> {
        FBE_W::new(self)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\] = 0). Writing 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<4> {
        DU_W::new(self)
    }
    #[doc = "Bit 5 - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
    #[inline(always)]
    #[must_use]
    pub fn ces(&mut self) -> CES_W<5> {
        CES_W::new(self)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\] : Transmit Interrupt, IDSTS\\[1\\] : Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<8> {
        NIS_W::new(self)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\] : Fatal Bus Interrupt, IDSTS\\[4\\] : DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<9> {
        AIS_W::new(self)
    }
    #[doc = "Bits 10:12 - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\] is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn fbe_code(&mut self) -> FBE_CODE_W<10> {
        FBE_CODE_W::new(self)
    }
    #[doc = "Bits 13:16 - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
    #[inline(always)]
    #[must_use]
    pub fn fsm(&mut self) -> FSM_W<13> {
        FSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDMAC status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idsts](index.html) module"]
pub struct IDSTS_SPEC;
impl crate::RegisterSpec for IDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idsts::R](R) reader structure"]
impl crate::Readable for IDSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idsts::W](W) writer structure"]
impl crate::Writable for IDSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDSTS to value 0"]
impl crate::Resettable for IDSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
