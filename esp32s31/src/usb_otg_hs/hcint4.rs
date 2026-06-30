#[doc = "Register `HCINT4` reader"]
pub type R = crate::R<HCINT4_SPEC>;
#[doc = "Register `HCINT4` writer"]
pub type W = crate::W<HCINT4_SPEC>;
#[doc = "Field `XFERCOMPL` reader - Transfer Completed (XferCompl) Transfer completed normally without any errors.This bit can be set only by the core and the application must write 1 to clear it. - For Scatter/Gather DMA mode, it indicates that current descriptor processing got completed with IOC bit set in its descriptor. - In non Scatter/Gather DMA mode, it indicates that Transfer completed normally without any errors."]
pub type XFERCOMPL_R = crate::BitReader;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed (XferCompl) Transfer completed normally without any errors.This bit can be set only by the core and the application must write 1 to clear it. - For Scatter/Gather DMA mode, it indicates that current descriptor processing got completed with IOC bit set in its descriptor. - In non Scatter/Gather DMA mode, it indicates that Transfer completed normally without any errors."]
pub type XFERCOMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTD` reader - Channel Halted (ChHltd) In non Scatter/Gather DMA mode, it indicates the transfer completed abnormally either because of any USB transaction error or in response to disable request by the application or because of a completed transfer. In Scatter/gather DMA mode, this indicates that transfer completed due to any of the following - EOL being set in descriptor - AHB error - Excessive transaction errors - Babble - Stall"]
pub type CHHLTD_R = crate::BitReader;
#[doc = "Field `CHHLTD` writer - Channel Halted (ChHltd) In non Scatter/Gather DMA mode, it indicates the transfer completed abnormally either because of any USB transaction error or in response to disable request by the application or because of a completed transfer. In Scatter/gather DMA mode, this indicates that transfer completed due to any of the following - EOL being set in descriptor - AHB error - Excessive transaction errors - Babble - Stall"]
pub type CHHLTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB Error (AHBErr) This is generated only in Internal DMA mode when there is an AHB error during AHB read/write. The application can read the corresponding channel's DMA address register to get the error address. For details, see AHB Error Handling in the Programming Guide."]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB Error (AHBErr) This is generated only in Internal DMA mode when there is an AHB error during AHB read/write. The application can read the corresponding channel's DMA address register to get the error address. For details, see AHB Error Handling in the Programming Guide."]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL Response Received Interrupt (STALL) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL Response Received Interrupt (STALL) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK Response Received Interrupt (NAK) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK Response Received Interrupt (NAK) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK Response Received/Transmitted Interrupt (ACK) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK Response Received/Transmitted Interrupt (ACK) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - NYET Response Received Interrupt (NYET) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type NYET_R = crate::BitReader;
#[doc = "Field `NYET` writer - NYET Response Received Interrupt (NYET) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERR` reader - Transaction Error (XactErr) Indicates one of the following errors occurred on the USB. - CRC check failure - Timeout - Bit stuff error - False EOP In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type XACTERR_R = crate::BitReader;
#[doc = "Field `XACTERR` writer - Transaction Error (XactErr) Indicates one of the following errors occurred on the USB. - CRC check failure - Timeout - Bit stuff error - False EOP In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
pub type XACTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERR` reader - Babble Error (BblErr) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core. This bit can be set only by the core and the application must write 1 to clear it."]
pub type BBLERR_R = crate::BitReader;
#[doc = "Field `BBLERR` writer - Babble Error (BblErr) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core. This bit can be set only by the core and the application must write 1 to clear it."]
pub type BBLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUN` reader - Frame Overrun (FrmOvrun). In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core. This bit can be set only by the core and the application must write 1 to clear it."]
pub type FRMOVRUN_R = crate::BitReader;
#[doc = "Field `FRMOVRUN` writer - Frame Overrun (FrmOvrun). In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core. This bit can be set only by the core and the application must write 1 to clear it."]
pub type FRMOVRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATGLERR` reader - Data Toggle Error (DataTglErr).This bit can be set only by the core and the application must write 1 to clear it.In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core."]
pub type DATATGLERR_R = crate::BitReader;
#[doc = "Field `DATATGLERR` writer - Data Toggle Error (DataTglErr).This bit can be set only by the core and the application must write 1 to clear it.In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core."]
pub type DATATGLERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINTR` reader - BNA (Buffer Not Available) Interrupt (BNAIntr) This bit is valid only when Scatter/Gather DMA mode is enabled. The core generates this interrupt when the descriptor accessed is not ready for the Core to process. BNA is not generated for Isochronous channels. For non Scatter/Gather DMA mode, this bit is reserved."]
pub type BNAINTR_R = crate::BitReader;
#[doc = "Field `BNAINTR` writer - BNA (Buffer Not Available) Interrupt (BNAIntr) This bit is valid only when Scatter/Gather DMA mode is enabled. The core generates this interrupt when the descriptor accessed is not ready for the Core to process. BNA is not generated for Isochronous channels. For non Scatter/Gather DMA mode, this bit is reserved."]
pub type BNAINTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCS_XACT_ERR` reader - Excessive Transaction Error (XCS_XACT_ERR) This bit is valid only when Scatter/Gather DMA mode is enabled. The core sets this bit when 3 consecutive transaction errors occurred on the USB bus. XCS_XACT_ERR is not generated for Isochronous channels. For non Scatter/Gather DMA mode, this bit is reserved."]
pub type XCS_XACT_ERR_R = crate::BitReader;
#[doc = "Field `XCS_XACT_ERR` writer - Excessive Transaction Error (XCS_XACT_ERR) This bit is valid only when Scatter/Gather DMA mode is enabled. The core sets this bit when 3 consecutive transaction errors occurred on the USB bus. XCS_XACT_ERR is not generated for Isochronous channels. For non Scatter/Gather DMA mode, this bit is reserved."]
pub type XCS_XACT_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LST_ROLLINTR` reader - Descriptor rollover interrupt (DESC_LST_ROLLIntr) This bit is valid only when Scatter/Gather DMA mode is enabled. The core sets this bit when the corresponding channel's descriptor list rolls over. For non Scatter/Gather DMA mode, this bit is reserved."]
pub type DESC_LST_ROLLINTR_R = crate::BitReader;
#[doc = "Field `DESC_LST_ROLLINTR` writer - Descriptor rollover interrupt (DESC_LST_ROLLIntr) This bit is valid only when Scatter/Gather DMA mode is enabled. The core sets this bit when the corresponding channel's descriptor list rolls over. For non Scatter/Gather DMA mode, this bit is reserved."]
pub type DESC_LST_ROLLINTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed (XferCompl) Transfer completed normally without any errors.This bit can be set only by the core and the application must write 1 to clear it. - For Scatter/Gather DMA mode, it indicates that current descriptor processing got completed with IOC bit set in its descriptor. - In non Scatter/Gather DMA mode, it indicates that Transfer completed normally without any errors."]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted (ChHltd) In non Scatter/Gather DMA mode, it indicates the transfer completed abnormally either because of any USB transaction error or in response to disable request by the application or because of a completed transfer. In Scatter/gather DMA mode, this indicates that transfer completed due to any of the following - EOL being set in descriptor - AHB error - Excessive transaction errors - Babble - Stall"]
    #[inline(always)]
    pub fn chhltd(&self) -> CHHLTD_R {
        CHHLTD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error (AHBErr) This is generated only in Internal DMA mode when there is an AHB error during AHB read/write. The application can read the corresponding channel's DMA address register to get the error address. For details, see AHB Error Handling in the Programming Guide."]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt (STALL) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt (NAK) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt (ACK) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt (NYET) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error (XactErr) Indicates one of the following errors occurred on the USB. - CRC check failure - Timeout - Bit stuff error - False EOP In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn xacterr(&self) -> XACTERR_R {
        XACTERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error (BblErr) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn bblerr(&self) -> BBLERR_R {
        BBLERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun (FrmOvrun). In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn frmovrun(&self) -> FRMOVRUN_R {
        FRMOVRUN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error (DataTglErr).This bit can be set only by the core and the application must write 1 to clear it.In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core."]
    #[inline(always)]
    pub fn datatglerr(&self) -> DATATGLERR_R {
        DATATGLERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt (BNAIntr) This bit is valid only when Scatter/Gather DMA mode is enabled. The core generates this interrupt when the descriptor accessed is not ready for the Core to process. BNA is not generated for Isochronous channels. For non Scatter/Gather DMA mode, this bit is reserved."]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Excessive Transaction Error (XCS_XACT_ERR) This bit is valid only when Scatter/Gather DMA mode is enabled. The core sets this bit when 3 consecutive transaction errors occurred on the USB bus. XCS_XACT_ERR is not generated for Isochronous channels. For non Scatter/Gather DMA mode, this bit is reserved."]
    #[inline(always)]
    pub fn xcs_xact_err(&self) -> XCS_XACT_ERR_R {
        XCS_XACT_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt (DESC_LST_ROLLIntr) This bit is valid only when Scatter/Gather DMA mode is enabled. The core sets this bit when the corresponding channel's descriptor list rolls over. For non Scatter/Gather DMA mode, this bit is reserved."]
    #[inline(always)]
    pub fn desc_lst_rollintr(&self) -> DESC_LST_ROLLINTR_R {
        DESC_LST_ROLLINTR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT4")
            .field("xfercompl", &self.xfercompl())
            .field("chhltd", &self.chhltd())
            .field("ahberr", &self.ahberr())
            .field("stall", &self.stall())
            .field("nak", &self.nak())
            .field("ack", &self.ack())
            .field("nyet", &self.nyet())
            .field("xacterr", &self.xacterr())
            .field("bblerr", &self.bblerr())
            .field("frmovrun", &self.frmovrun())
            .field("datatglerr", &self.datatglerr())
            .field("bnaintr", &self.bnaintr())
            .field("xcs_xact_err", &self.xcs_xact_err())
            .field("desc_lst_rollintr", &self.desc_lst_rollintr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed (XferCompl) Transfer completed normally without any errors.This bit can be set only by the core and the application must write 1 to clear it. - For Scatter/Gather DMA mode, it indicates that current descriptor processing got completed with IOC bit set in its descriptor. - In non Scatter/Gather DMA mode, it indicates that Transfer completed normally without any errors."]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W<'_, HCINT4_SPEC> {
        XFERCOMPL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Halted (ChHltd) In non Scatter/Gather DMA mode, it indicates the transfer completed abnormally either because of any USB transaction error or in response to disable request by the application or because of a completed transfer. In Scatter/gather DMA mode, this indicates that transfer completed due to any of the following - EOL being set in descriptor - AHB error - Excessive transaction errors - Babble - Stall"]
    #[inline(always)]
    pub fn chhltd(&mut self) -> CHHLTD_W<'_, HCINT4_SPEC> {
        CHHLTD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error (AHBErr) This is generated only in Internal DMA mode when there is an AHB error during AHB read/write. The application can read the corresponding channel's DMA address register to get the error address. For details, see AHB Error Handling in the Programming Guide."]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<'_, HCINT4_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt (STALL) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, HCINT4_SPEC> {
        STALL_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt (NAK) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<'_, HCINT4_SPEC> {
        NAK_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt (ACK) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<'_, HCINT4_SPEC> {
        ACK_W::new(self, 5)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt (NYET) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<'_, HCINT4_SPEC> {
        NYET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transaction Error (XactErr) Indicates one of the following errors occurred on the USB. - CRC check failure - Timeout - Bit stuff error - False EOP In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn xacterr(&mut self) -> XACTERR_W<'_, HCINT4_SPEC> {
        XACTERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Babble Error (BblErr) In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn bblerr(&mut self) -> BBLERR_W<'_, HCINT4_SPEC> {
        BBLERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Frame Overrun (FrmOvrun). In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn frmovrun(&mut self) -> FRMOVRUN_W<'_, HCINT4_SPEC> {
        FRMOVRUN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data Toggle Error (DataTglErr).This bit can be set only by the core and the application must write 1 to clear it.In Scatter/Gather DMA mode, the interrupt due to this bit is masked in the core."]
    #[inline(always)]
    pub fn datatglerr(&mut self) -> DATATGLERR_W<'_, HCINT4_SPEC> {
        DATATGLERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt (BNAIntr) This bit is valid only when Scatter/Gather DMA mode is enabled. The core generates this interrupt when the descriptor accessed is not ready for the Core to process. BNA is not generated for Isochronous channels. For non Scatter/Gather DMA mode, this bit is reserved."]
    #[inline(always)]
    pub fn bnaintr(&mut self) -> BNAINTR_W<'_, HCINT4_SPEC> {
        BNAINTR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Excessive Transaction Error (XCS_XACT_ERR) This bit is valid only when Scatter/Gather DMA mode is enabled. The core sets this bit when 3 consecutive transaction errors occurred on the USB bus. XCS_XACT_ERR is not generated for Isochronous channels. For non Scatter/Gather DMA mode, this bit is reserved."]
    #[inline(always)]
    pub fn xcs_xact_err(&mut self) -> XCS_XACT_ERR_W<'_, HCINT4_SPEC> {
        XCS_XACT_ERR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt (DESC_LST_ROLLIntr) This bit is valid only when Scatter/Gather DMA mode is enabled. The core sets this bit when the corresponding channel's descriptor list rolls over. For non Scatter/Gather DMA mode, this bit is reserved."]
    #[inline(always)]
    pub fn desc_lst_rollintr(&mut self) -> DESC_LST_ROLLINTR_W<'_, HCINT4_SPEC> {
        DESC_LST_ROLLINTR_W::new(self, 13)
    }
}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINT4_SPEC;
impl crate::RegisterSpec for HCINT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint4::R`](R) reader structure"]
impl crate::Readable for HCINT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcint4::W`](W) writer structure"]
impl crate::Writable for HCINT4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCINT4 to value 0"]
impl crate::Resettable for HCINT4_SPEC {}
