#[doc = "Register `HCCHAR10` reader"]
pub type R = crate::R<HCCHAR10_SPEC>;
#[doc = "Register `HCCHAR10` writer"]
pub type W = crate::W<HCCHAR10_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size (MPS) Indicates the maximum packet size of the associated endpoint."]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size (MPS) Indicates the maximum packet size of the associated endpoint."]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - Endpoint Number (EPNum) Indicates the endpoint number on the device serving as the data source or sink."]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint Number (EPNum) Indicates the endpoint number on the device serving as the data source or sink."]
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - Endpoint Direction (EPDir) Indicates whether the transaction is IN or OUT. - 1'b0: OUT - 1'b1: IN"]
pub type EPDIR_R = crate::BitReader;
#[doc = "Field `EPDIR` writer - Endpoint Direction (EPDir) Indicates whether the transaction is IN or OUT. - 1'b0: OUT - 1'b1: IN"]
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPDDEV` reader - Low-Speed Device (LSpdDev) This field is Set by the application to indicate that this channel is communicating to a low-speed device. The application must program this bit when a low speed device is connected to the host through an FS HUB. The DWC_otg Host core uses this field to drive the XCVR_SELECT signal to 2'b11 while communicating to the LS Device through the FS hub. Note: In a peer to peer setup, the DWC_otg Host core ignores this bit even if it is set by the application software."]
pub type LSPDDEV_R = crate::BitReader;
#[doc = "Field `LSPDDEV` writer - Low-Speed Device (LSpdDev) This field is Set by the application to indicate that this channel is communicating to a low-speed device. The application must program this bit when a low speed device is connected to the host through an FS HUB. The DWC_otg Host core uses this field to drive the XCVR_SELECT signal to 2'b11 while communicating to the LS Device through the FS hub. Note: In a peer to peer setup, the DWC_otg Host core ignores this bit even if it is set by the application software."]
pub type LSPDDEV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYPE` reader - Endpoint Type (EPType) Indicates the transfer type selected. - 2'b00: Control - 2'b01: Isochronous - 2'b10: Bulk - 2'b11: Interrupt"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint Type (EPType) Indicates the transfer type selected. - 2'b00: Control - 2'b01: Isochronous - 2'b10: Bulk - 2'b11: Interrupt"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EC` reader - Multi Count (MC) / Error Count (EC) When the Split Enable bit of the Host Channel-n Split Control register (HCSPLTn.SpltEna) is reset (1'b0), this field indicates to the host the number of transactions that must be executed per microframe for this periodic endpoint. For non periodic transfers, this field is used only in DMA mode, and specifies the number packets to be fetched for this channel before the internal DMA engine changes arbitration. - 2'b00: Reserved This field yields undefined results. - 2'b01: 1 transaction - 2'b10: 2 transactions to be issued for this endpoint per microframe - 2'b11: 3 transactions to be issued for this endpoint per microframe When HCSPLTn.SpltEna is Set (1'b1), this field indicates the number of immediate retries to be performed for a periodic split transactions on transaction errors. This field must be Set to at least 2'b01."]
pub type EC_R = crate::FieldReader;
#[doc = "Field `EC` writer - Multi Count (MC) / Error Count (EC) When the Split Enable bit of the Host Channel-n Split Control register (HCSPLTn.SpltEna) is reset (1'b0), this field indicates to the host the number of transactions that must be executed per microframe for this periodic endpoint. For non periodic transfers, this field is used only in DMA mode, and specifies the number packets to be fetched for this channel before the internal DMA engine changes arbitration. - 2'b00: Reserved This field yields undefined results. - 2'b01: 1 transaction - 2'b10: 2 transactions to be issued for this endpoint per microframe - 2'b11: 3 transactions to be issued for this endpoint per microframe When HCSPLTn.SpltEna is Set (1'b1), this field indicates the number of immediate retries to be performed for a periodic split transactions on transaction errors. This field must be Set to at least 2'b01."]
pub type EC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVADDR` reader - Device Address (DevAddr) This field selects the specific device serving as the data source or sink."]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device Address (DevAddr) This field selects the specific device serving as the data source or sink."]
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - Odd Frame (OddFrm) This field is set (reset) by the application to indicate that the OTG host must perform a transfer in an odd (micro)Frame. This field is applicable for only periodic (isochronous and interrupt) transactions. - 1'b0: Even (micro)Frame - 1'b1: Odd (micro)Frame"]
pub type ODDFRM_R = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd Frame (OddFrm) This field is set (reset) by the application to indicate that the OTG host must perform a transfer in an odd (micro)Frame. This field is applicable for only periodic (isochronous and interrupt) transactions. - 1'b0: Even (micro)Frame - 1'b1: Odd (micro)Frame"]
pub type ODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDIS` reader - Channel Disable (ChDis) The application sets this bit to stop transmitting/receiving data on a channel, even before the transfer for that channel is complete. The application must wait for the Channel Disabled interrupt before treating the channel as disabled."]
pub type CHDIS_R = crate::BitReader;
#[doc = "Field `CHDIS` writer - Channel Disable (ChDis) The application sets this bit to stop transmitting/receiving data on a channel, even before the transfer for that channel is complete. The application must wait for the Channel Disabled interrupt before treating the channel as disabled."]
pub type CHDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - Channel Enable (ChEna) When Scatter/Gather mode is enabled - 1'b0: Indicates that the descriptor structure is not yet ready. - 1'b1: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. When Scatter/Gather mode is disabled This field is set by the application and cleared by the OTG host. - 1'b0: Channel disabled - 1'b1: Channel enabled"]
pub type CHENA_R = crate::BitReader;
#[doc = "Field `CHENA` writer - Channel Enable (ChEna) When Scatter/Gather mode is enabled - 1'b0: Indicates that the descriptor structure is not yet ready. - 1'b1: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. When Scatter/Gather mode is disabled This field is set by the application and cleared by the OTG host. - 1'b0: Channel disabled - 1'b1: Channel enabled"]
pub type CHENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size (MPS) Indicates the maximum packet size of the associated endpoint."]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint Number (EPNum) Indicates the endpoint number on the device serving as the data source or sink."]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint Direction (EPDir) Indicates whether the transaction is IN or OUT. - 1'b0: OUT - 1'b1: IN"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-Speed Device (LSpdDev) This field is Set by the application to indicate that this channel is communicating to a low-speed device. The application must program this bit when a low speed device is connected to the host through an FS HUB. The DWC_otg Host core uses this field to drive the XCVR_SELECT signal to 2'b11 while communicating to the LS Device through the FS hub. Note: In a peer to peer setup, the DWC_otg Host core ignores this bit even if it is set by the application software."]
    #[inline(always)]
    pub fn lspddev(&self) -> LSPDDEV_R {
        LSPDDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type (EPType) Indicates the transfer type selected. - 2'b00: Control - 2'b01: Isochronous - 2'b10: Bulk - 2'b11: Interrupt"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC) When the Split Enable bit of the Host Channel-n Split Control register (HCSPLTn.SpltEna) is reset (1'b0), this field indicates to the host the number of transactions that must be executed per microframe for this periodic endpoint. For non periodic transfers, this field is used only in DMA mode, and specifies the number packets to be fetched for this channel before the internal DMA engine changes arbitration. - 2'b00: Reserved This field yields undefined results. - 2'b01: 1 transaction - 2'b10: 2 transactions to be issued for this endpoint per microframe - 2'b11: 3 transactions to be issued for this endpoint per microframe When HCSPLTn.SpltEna is Set (1'b1), this field indicates the number of immediate retries to be performed for a periodic split transactions on transaction errors. This field must be Set to at least 2'b01."]
    #[inline(always)]
    pub fn ec(&self) -> EC_R {
        EC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device Address (DevAddr) This field selects the specific device serving as the data source or sink."]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd Frame (OddFrm) This field is set (reset) by the application to indicate that the OTG host must perform a transfer in an odd (micro)Frame. This field is applicable for only periodic (isochronous and interrupt) transactions. - 1'b0: Even (micro)Frame - 1'b1: Odd (micro)Frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel Disable (ChDis) The application sets this bit to stop transmitting/receiving data on a channel, even before the transfer for that channel is complete. The application must wait for the Channel Disabled interrupt before treating the channel as disabled."]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel Enable (ChEna) When Scatter/Gather mode is enabled - 1'b0: Indicates that the descriptor structure is not yet ready. - 1'b1: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. When Scatter/Gather mode is disabled This field is set by the application and cleared by the OTG host. - 1'b0: Channel disabled - 1'b1: Channel enabled"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR10")
            .field("mps", &self.mps())
            .field("epnum", &self.epnum())
            .field("epdir", &self.epdir())
            .field("lspddev", &self.lspddev())
            .field("eptype", &self.eptype())
            .field("ec", &self.ec())
            .field("devaddr", &self.devaddr())
            .field("oddfrm", &self.oddfrm())
            .field("chdis", &self.chdis())
            .field("chena", &self.chena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size (MPS) Indicates the maximum packet size of the associated endpoint."]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W<'_, HCCHAR10_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint Number (EPNum) Indicates the endpoint number on the device serving as the data source or sink."]
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W<'_, HCCHAR10_SPEC> {
        EPNUM_W::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint Direction (EPDir) Indicates whether the transaction is IN or OUT. - 1'b0: OUT - 1'b1: IN"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W<'_, HCCHAR10_SPEC> {
        EPDIR_W::new(self, 15)
    }
    #[doc = "Bit 17 - Low-Speed Device (LSpdDev) This field is Set by the application to indicate that this channel is communicating to a low-speed device. The application must program this bit when a low speed device is connected to the host through an FS HUB. The DWC_otg Host core uses this field to drive the XCVR_SELECT signal to 2'b11 while communicating to the LS Device through the FS hub. Note: In a peer to peer setup, the DWC_otg Host core ignores this bit even if it is set by the application software."]
    #[inline(always)]
    pub fn lspddev(&mut self) -> LSPDDEV_W<'_, HCCHAR10_SPEC> {
        LSPDDEV_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Endpoint Type (EPType) Indicates the transfer type selected. - 2'b00: Control - 2'b01: Isochronous - 2'b10: Bulk - 2'b11: Interrupt"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W<'_, HCCHAR10_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC) When the Split Enable bit of the Host Channel-n Split Control register (HCSPLTn.SpltEna) is reset (1'b0), this field indicates to the host the number of transactions that must be executed per microframe for this periodic endpoint. For non periodic transfers, this field is used only in DMA mode, and specifies the number packets to be fetched for this channel before the internal DMA engine changes arbitration. - 2'b00: Reserved This field yields undefined results. - 2'b01: 1 transaction - 2'b10: 2 transactions to be issued for this endpoint per microframe - 2'b11: 3 transactions to be issued for this endpoint per microframe When HCSPLTn.SpltEna is Set (1'b1), this field indicates the number of immediate retries to be performed for a periodic split transactions on transaction errors. This field must be Set to at least 2'b01."]
    #[inline(always)]
    pub fn ec(&mut self) -> EC_W<'_, HCCHAR10_SPEC> {
        EC_W::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device Address (DevAddr) This field selects the specific device serving as the data source or sink."]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W<'_, HCCHAR10_SPEC> {
        DEVADDR_W::new(self, 22)
    }
    #[doc = "Bit 29 - Odd Frame (OddFrm) This field is set (reset) by the application to indicate that the OTG host must perform a transfer in an odd (micro)Frame. This field is applicable for only periodic (isochronous and interrupt) transactions. - 1'b0: Even (micro)Frame - 1'b1: Odd (micro)Frame"]
    #[inline(always)]
    pub fn oddfrm(&mut self) -> ODDFRM_W<'_, HCCHAR10_SPEC> {
        ODDFRM_W::new(self, 29)
    }
    #[doc = "Bit 30 - Channel Disable (ChDis) The application sets this bit to stop transmitting/receiving data on a channel, even before the transfer for that channel is complete. The application must wait for the Channel Disabled interrupt before treating the channel as disabled."]
    #[inline(always)]
    pub fn chdis(&mut self) -> CHDIS_W<'_, HCCHAR10_SPEC> {
        CHDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Channel Enable (ChEna) When Scatter/Gather mode is enabled - 1'b0: Indicates that the descriptor structure is not yet ready. - 1'b1: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. When Scatter/Gather mode is disabled This field is set by the application and cleared by the OTG host. - 1'b0: Channel disabled - 1'b1: Channel enabled"]
    #[inline(always)]
    pub fn chena(&mut self) -> CHENA_W<'_, HCCHAR10_SPEC> {
        CHENA_W::new(self, 31)
    }
}
#[doc = "This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCCHAR10_SPEC;
impl crate::RegisterSpec for HCCHAR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar10::R`](R) reader structure"]
impl crate::Readable for HCCHAR10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcchar10::W`](W) writer structure"]
impl crate::Writable for HCCHAR10_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCCHAR10 to value 0"]
impl crate::Resettable for HCCHAR10_SPEC {}
