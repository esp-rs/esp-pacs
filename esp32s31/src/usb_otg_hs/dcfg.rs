#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DCFG_SPEC>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DCFG_SPEC>;
#[doc = "Field `DEVSPD` reader - Device Speed (DevSpd) Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the connect sequence is completed, and is based on the speed of the USB host to which the core is connected."]
pub type DEVSPD_R = crate::FieldReader;
#[doc = "Field `DEVSPD` writer - Device Speed (DevSpd) Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the connect sequence is completed, and is based on the speed of the USB host to which the core is connected."]
pub type DEVSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZSTSOUTHSHK` reader - Non-Zero-Length Status OUT Handshake (NZStsOUTHShk) The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfer's Status stage. - 1'b1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application. - 1'b0: Send the received OUT packet to the application (zerolength or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
pub type NZSTSOUTHSHK_R = crate::BitReader;
#[doc = "Field `NZSTSOUTHSHK` writer - Non-Zero-Length Status OUT Handshake (NZStsOUTHShk) The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfer's Status stage. - 1'b1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application. - 1'b0: Send the received OUT packet to the application (zerolength or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
pub type NZSTSOUTHSHK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA32KHZSUSP` reader - Enable 32 KHz Suspend mode (Ena32KHzSusp) This bit can be set only if FS PHY interface is selected. Otherwise, this bit needs to be set to zero. If FS PHY interface is chosen and this bit is set, the PHY clock during Suspend must be switched from 48 MHz to 32 KHz."]
pub type ENA32KHZSUSP_R = crate::BitReader;
#[doc = "Field `ENA32KHZSUSP` writer - Enable 32 KHz Suspend mode (Ena32KHzSusp) This bit can be set only if FS PHY interface is selected. Otherwise, this bit needs to be set to zero. If FS PHY interface is chosen and this bit is set, the PHY clock during Suspend must be switched from 48 MHz to 32 KHz."]
pub type ENA32KHZSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDR` reader - Device Address (DevAddr) The application must program this field after every SetAddress control command."]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device Address (DevAddr) The application must program this field after every SetAddress control command."]
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PERFRINT` reader - Periodic Frame Interval (PerFrInt) Indicates the time within a (micro)Frame at which the application must be notified using the End Of Periodic Frame Interrupt. This can be used to determine If all the isochronous traffic for that (micro)Frame is complete. - 2'b00: 80% of the (micro)Frame interval - 2'b01: 85% of the (micro)Frame interval - 2'b10: 90% of the (micro)Frame interval - 2'b11: 95% of the (micro)Frame interval"]
pub type PERFRINT_R = crate::FieldReader;
#[doc = "Field `PERFRINT` writer - Periodic Frame Interval (PerFrInt) Indicates the time within a (micro)Frame at which the application must be notified using the End Of Periodic Frame Interrupt. This can be used to determine If all the isochronous traffic for that (micro)Frame is complete. - 2'b00: 80% of the (micro)Frame interval - 2'b01: 85% of the (micro)Frame interval - 2'b10: 90% of the (micro)Frame interval - 2'b11: 95% of the (micro)Frame interval"]
pub type PERFRINT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENDEVOUTNAK` reader - Enable Device OUT NAK (EnDevOutNak) This bit enables setting NAK for Bulk OUT endpoints after the transfer is completed for Device mode Descriptor DMA - 1'b0 : The core does not set NAK after Bulk OUT transfer complete - 1'b1 : The core sets NAK after Bulk OUT transfer complete This bit is one time programmable after reset like any other DCFG register bits."]
pub type ENDEVOUTNAK_R = crate::BitReader;
#[doc = "Field `ENDEVOUTNAK` writer - Enable Device OUT NAK (EnDevOutNak) This bit enables setting NAK for Bulk OUT endpoints after the transfer is completed for Device mode Descriptor DMA - 1'b0 : The core does not set NAK after Bulk OUT transfer complete - 1'b1 : The core sets NAK after Bulk OUT transfer complete This bit is one time programmable after reset like any other DCFG register bits."]
pub type ENDEVOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCVRDLY` reader - XCVRDLY Enables or disables delay between xcvr_sel and txvalid during device chirp"]
pub type XCVRDLY_R = crate::BitReader;
#[doc = "Field `XCVRDLY` writer - XCVRDLY Enables or disables delay between xcvr_sel and txvalid during device chirp"]
pub type XCVRDLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRATICINTMSK` reader - Erratic Error Interrupt Mask"]
pub type ERRATICINTMSK_R = crate::BitReader;
#[doc = "Field `ERRATICINTMSK` writer - Erratic Error Interrupt Mask"]
pub type ERRATICINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPGISOCSUPT` reader - Worst-Case Inter-Packet Gap ISOC OUT Support (ipgisocSupt) This bit indicates that the controller supports the worst-case scenario of Rx followed by Rx Inter-Packet Gap (IPG) (32 bit times) as per the UTMI Specification for any token following an ISOC OUT token. Without this support, when any token follows an ISOC OUT token with the worst-case IPG, the controller does not detect the followed token. The worst-case IPG of the controller without this support depends on the AHB and PHY clock frequency."]
pub type IPGISOCSUPT_R = crate::BitReader;
#[doc = "Field `IPGISOCSUPT` writer - Worst-Case Inter-Packet Gap ISOC OUT Support (ipgisocSupt) This bit indicates that the controller supports the worst-case scenario of Rx followed by Rx Inter-Packet Gap (IPG) (32 bit times) as per the UTMI Specification for any token following an ISOC OUT token. Without this support, when any token follows an ISOC OUT token with the worst-case IPG, the controller does not detect the followed token. The worst-case IPG of the controller without this support depends on the AHB and PHY clock frequency."]
pub type IPGISOCSUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESCDMA` reader - Enable Scatter/gather DMA in device mode (DescDMA). When the Scatter/Gather DMA option selected during configuration of the RTL, the application can Set this bit during initialization to enable the Scatter/Gather DMA operation. Note: This bit must be modified only once after a reset. The following combinations are available for programming: - GAHBCFG.DMAEn=0,DCFG.DescDMA=0 => Slave mode - GAHBCFG.DMAEn=0,DCFG.DescDMA=1 => Invalid - GAHBCFG.DMAEn=1,DCFG.DescDMA=0 => Buffered DMA mode - GAHBCFG.DMAEn=1,DCFG.DescDMA=1 => Scatter/Gather DMA mode"]
pub type DESCDMA_R = crate::BitReader;
#[doc = "Field `DESCDMA` writer - Enable Scatter/gather DMA in device mode (DescDMA). When the Scatter/Gather DMA option selected during configuration of the RTL, the application can Set this bit during initialization to enable the Scatter/Gather DMA operation. Note: This bit must be modified only once after a reset. The following combinations are available for programming: - GAHBCFG.DMAEn=0,DCFG.DescDMA=0 => Slave mode - GAHBCFG.DMAEn=0,DCFG.DescDMA=1 => Invalid - GAHBCFG.DMAEn=1,DCFG.DescDMA=0 => Buffered DMA mode - GAHBCFG.DMAEn=1,DCFG.DescDMA=1 => Scatter/Gather DMA mode"]
pub type DESCDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERSCHINTVL` reader - Periodic Scheduling Interval (PerSchIntvl) PerSchIntvl must be programmed for Scatter/Gather DMA mode. This field specifies the amount of time the Internal DMA engine must allocate for fetching periodic IN endpoint data. Based on the number of periodic endpoints, this value must be specified as 25,50 or 75% of (micro)Frame. - When any periodic endpoints are active, the internal DMA engine allocates the specified amount of time in fetching periodic IN endpoint data . - When no periodic endpoints are active, Then the internal DMA engine services non-periodic endpoints, ignoring this field. - After the specified time within a (micro)Frame, the DMA switches to fetching for non-periodic endpoints. -- 2'b00: 25% of (micro)Frame. -- 2'b01: 50% of (micro)Frame. -- 2'b10: 75% of (micro)Frame. -- 2'b11: Reserved. Reset: 2'b00"]
pub type PERSCHINTVL_R = crate::FieldReader;
#[doc = "Field `PERSCHINTVL` writer - Periodic Scheduling Interval (PerSchIntvl) PerSchIntvl must be programmed for Scatter/Gather DMA mode. This field specifies the amount of time the Internal DMA engine must allocate for fetching periodic IN endpoint data. Based on the number of periodic endpoints, this value must be specified as 25,50 or 75% of (micro)Frame. - When any periodic endpoints are active, the internal DMA engine allocates the specified amount of time in fetching periodic IN endpoint data . - When no periodic endpoints are active, Then the internal DMA engine services non-periodic endpoints, ignoring this field. - After the specified time within a (micro)Frame, the DMA switches to fetching for non-periodic endpoints. -- 2'b00: 25% of (micro)Frame. -- 2'b01: 50% of (micro)Frame. -- 2'b10: 75% of (micro)Frame. -- 2'b11: Reserved. Reset: 2'b00"]
pub type PERSCHINTVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESVALID` reader - Resume Validation Period (ResValid) This field is effective only when DCFG.Ena32KHzSusp is set. It controls the resume period when the core resumes from suspend. The core counts for ResValid number of clock cycles to detect a valid resume when this bit is set"]
pub type RESVALID_R = crate::FieldReader;
#[doc = "Field `RESVALID` writer - Resume Validation Period (ResValid) This field is effective only when DCFG.Ena32KHzSusp is set. It controls the resume period when the core resumes from suspend. The core counts for ResValid number of clock cycles to detect a valid resume when this bit is set"]
pub type RESVALID_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Device Speed (DevSpd) Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the connect sequence is completed, and is based on the speed of the USB host to which the core is connected."]
    #[inline(always)]
    pub fn devspd(&self) -> DEVSPD_R {
        DEVSPD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake (NZStsOUTHShk) The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfer's Status stage. - 1'b1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application. - 1'b0: Send the received OUT packet to the application (zerolength or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NZSTSOUTHSHK_R {
        NZSTSOUTHSHK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode (Ena32KHzSusp) This bit can be set only if FS PHY interface is selected. Otherwise, this bit needs to be set to zero. If FS PHY interface is chosen and this bit is set, the PHY clock during Suspend must be switched from 48 MHz to 32 KHz."]
    #[inline(always)]
    pub fn ena32khzsusp(&self) -> ENA32KHZSUSP_R {
        ENA32KHZSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device Address (DevAddr) The application must program this field after every SetAddress control command."]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval (PerFrInt) Indicates the time within a (micro)Frame at which the application must be notified using the End Of Periodic Frame Interrupt. This can be used to determine If all the isochronous traffic for that (micro)Frame is complete. - 2'b00: 80% of the (micro)Frame interval - 2'b01: 85% of the (micro)Frame interval - 2'b10: 90% of the (micro)Frame interval - 2'b11: 95% of the (micro)Frame interval"]
    #[inline(always)]
    pub fn perfrint(&self) -> PERFRINT_R {
        PERFRINT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Enable Device OUT NAK (EnDevOutNak) This bit enables setting NAK for Bulk OUT endpoints after the transfer is completed for Device mode Descriptor DMA - 1'b0 : The core does not set NAK after Bulk OUT transfer complete - 1'b1 : The core sets NAK after Bulk OUT transfer complete This bit is one time programmable after reset like any other DCFG register bits."]
    #[inline(always)]
    pub fn endevoutnak(&self) -> ENDEVOUTNAK_R {
        ENDEVOUTNAK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XCVRDLY Enables or disables delay between xcvr_sel and txvalid during device chirp"]
    #[inline(always)]
    pub fn xcvrdly(&self) -> XCVRDLY_R {
        XCVRDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Erratic Error Interrupt Mask"]
    #[inline(always)]
    pub fn erraticintmsk(&self) -> ERRATICINTMSK_R {
        ERRATICINTMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Worst-Case Inter-Packet Gap ISOC OUT Support (ipgisocSupt) This bit indicates that the controller supports the worst-case scenario of Rx followed by Rx Inter-Packet Gap (IPG) (32 bit times) as per the UTMI Specification for any token following an ISOC OUT token. Without this support, when any token follows an ISOC OUT token with the worst-case IPG, the controller does not detect the followed token. The worst-case IPG of the controller without this support depends on the AHB and PHY clock frequency."]
    #[inline(always)]
    pub fn ipgisocsupt(&self) -> IPGISOCSUPT_R {
        IPGISOCSUPT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in device mode (DescDMA). When the Scatter/Gather DMA option selected during configuration of the RTL, the application can Set this bit during initialization to enable the Scatter/Gather DMA operation. Note: This bit must be modified only once after a reset. The following combinations are available for programming: - GAHBCFG.DMAEn=0,DCFG.DescDMA=0 => Slave mode - GAHBCFG.DMAEn=0,DCFG.DescDMA=1 => Invalid - GAHBCFG.DMAEn=1,DCFG.DescDMA=0 => Buffered DMA mode - GAHBCFG.DMAEn=1,DCFG.DescDMA=1 => Scatter/Gather DMA mode"]
    #[inline(always)]
    pub fn descdma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval (PerSchIntvl) PerSchIntvl must be programmed for Scatter/Gather DMA mode. This field specifies the amount of time the Internal DMA engine must allocate for fetching periodic IN endpoint data. Based on the number of periodic endpoints, this value must be specified as 25,50 or 75% of (micro)Frame. - When any periodic endpoints are active, the internal DMA engine allocates the specified amount of time in fetching periodic IN endpoint data . - When no periodic endpoints are active, Then the internal DMA engine services non-periodic endpoints, ignoring this field. - After the specified time within a (micro)Frame, the DMA switches to fetching for non-periodic endpoints. -- 2'b00: 25% of (micro)Frame. -- 2'b01: 50% of (micro)Frame. -- 2'b10: 75% of (micro)Frame. -- 2'b11: Reserved. Reset: 2'b00"]
    #[inline(always)]
    pub fn perschintvl(&self) -> PERSCHINTVL_R {
        PERSCHINTVL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - Resume Validation Period (ResValid) This field is effective only when DCFG.Ena32KHzSusp is set. It controls the resume period when the core resumes from suspend. The core counts for ResValid number of clock cycles to detect a valid resume when this bit is set"]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG")
            .field("devspd", &self.devspd())
            .field("nzstsouthshk", &self.nzstsouthshk())
            .field("ena32khzsusp", &self.ena32khzsusp())
            .field("devaddr", &self.devaddr())
            .field("perfrint", &self.perfrint())
            .field("endevoutnak", &self.endevoutnak())
            .field("xcvrdly", &self.xcvrdly())
            .field("erraticintmsk", &self.erraticintmsk())
            .field("ipgisocsupt", &self.ipgisocsupt())
            .field("descdma", &self.descdma())
            .field("perschintvl", &self.perschintvl())
            .field("resvalid", &self.resvalid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed (DevSpd) Indicates the speed at which the application requires the core to enumerate, or the maximum speed the application can support. However, the actual bus speed is determined only after the connect sequence is completed, and is based on the speed of the USB host to which the core is connected."]
    #[inline(always)]
    pub fn devspd(&mut self) -> DEVSPD_W<'_, DCFG_SPEC> {
        DEVSPD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake (NZStsOUTHShk) The application can use this field to select the handshake the core sends on receiving a nonzero-length data packet during the OUT transaction of a control transfer's Status stage. - 1'b1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application. - 1'b0: Send the received OUT packet to the application (zerolength or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline(always)]
    pub fn nzstsouthshk(&mut self) -> NZSTSOUTHSHK_W<'_, DCFG_SPEC> {
        NZSTSOUTHSHK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode (Ena32KHzSusp) This bit can be set only if FS PHY interface is selected. Otherwise, this bit needs to be set to zero. If FS PHY interface is chosen and this bit is set, the PHY clock during Suspend must be switched from 48 MHz to 32 KHz."]
    #[inline(always)]
    pub fn ena32khzsusp(&mut self) -> ENA32KHZSUSP_W<'_, DCFG_SPEC> {
        ENA32KHZSUSP_W::new(self, 3)
    }
    #[doc = "Bits 4:10 - Device Address (DevAddr) The application must program this field after every SetAddress control command."]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W<'_, DCFG_SPEC> {
        DEVADDR_W::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval (PerFrInt) Indicates the time within a (micro)Frame at which the application must be notified using the End Of Periodic Frame Interrupt. This can be used to determine If all the isochronous traffic for that (micro)Frame is complete. - 2'b00: 80% of the (micro)Frame interval - 2'b01: 85% of the (micro)Frame interval - 2'b10: 90% of the (micro)Frame interval - 2'b11: 95% of the (micro)Frame interval"]
    #[inline(always)]
    pub fn perfrint(&mut self) -> PERFRINT_W<'_, DCFG_SPEC> {
        PERFRINT_W::new(self, 11)
    }
    #[doc = "Bit 13 - Enable Device OUT NAK (EnDevOutNak) This bit enables setting NAK for Bulk OUT endpoints after the transfer is completed for Device mode Descriptor DMA - 1'b0 : The core does not set NAK after Bulk OUT transfer complete - 1'b1 : The core sets NAK after Bulk OUT transfer complete This bit is one time programmable after reset like any other DCFG register bits."]
    #[inline(always)]
    pub fn endevoutnak(&mut self) -> ENDEVOUTNAK_W<'_, DCFG_SPEC> {
        ENDEVOUTNAK_W::new(self, 13)
    }
    #[doc = "Bit 14 - XCVRDLY Enables or disables delay between xcvr_sel and txvalid during device chirp"]
    #[inline(always)]
    pub fn xcvrdly(&mut self) -> XCVRDLY_W<'_, DCFG_SPEC> {
        XCVRDLY_W::new(self, 14)
    }
    #[doc = "Bit 15 - Erratic Error Interrupt Mask"]
    #[inline(always)]
    pub fn erraticintmsk(&mut self) -> ERRATICINTMSK_W<'_, DCFG_SPEC> {
        ERRATICINTMSK_W::new(self, 15)
    }
    #[doc = "Bit 17 - Worst-Case Inter-Packet Gap ISOC OUT Support (ipgisocSupt) This bit indicates that the controller supports the worst-case scenario of Rx followed by Rx Inter-Packet Gap (IPG) (32 bit times) as per the UTMI Specification for any token following an ISOC OUT token. Without this support, when any token follows an ISOC OUT token with the worst-case IPG, the controller does not detect the followed token. The worst-case IPG of the controller without this support depends on the AHB and PHY clock frequency."]
    #[inline(always)]
    pub fn ipgisocsupt(&mut self) -> IPGISOCSUPT_W<'_, DCFG_SPEC> {
        IPGISOCSUPT_W::new(self, 17)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in device mode (DescDMA). When the Scatter/Gather DMA option selected during configuration of the RTL, the application can Set this bit during initialization to enable the Scatter/Gather DMA operation. Note: This bit must be modified only once after a reset. The following combinations are available for programming: - GAHBCFG.DMAEn=0,DCFG.DescDMA=0 => Slave mode - GAHBCFG.DMAEn=0,DCFG.DescDMA=1 => Invalid - GAHBCFG.DMAEn=1,DCFG.DescDMA=0 => Buffered DMA mode - GAHBCFG.DMAEn=1,DCFG.DescDMA=1 => Scatter/Gather DMA mode"]
    #[inline(always)]
    pub fn descdma(&mut self) -> DESCDMA_W<'_, DCFG_SPEC> {
        DESCDMA_W::new(self, 23)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval (PerSchIntvl) PerSchIntvl must be programmed for Scatter/Gather DMA mode. This field specifies the amount of time the Internal DMA engine must allocate for fetching periodic IN endpoint data. Based on the number of periodic endpoints, this value must be specified as 25,50 or 75% of (micro)Frame. - When any periodic endpoints are active, the internal DMA engine allocates the specified amount of time in fetching periodic IN endpoint data . - When no periodic endpoints are active, Then the internal DMA engine services non-periodic endpoints, ignoring this field. - After the specified time within a (micro)Frame, the DMA switches to fetching for non-periodic endpoints. -- 2'b00: 25% of (micro)Frame. -- 2'b01: 50% of (micro)Frame. -- 2'b10: 75% of (micro)Frame. -- 2'b11: Reserved. Reset: 2'b00"]
    #[inline(always)]
    pub fn perschintvl(&mut self) -> PERSCHINTVL_W<'_, DCFG_SPEC> {
        PERSCHINTVL_W::new(self, 24)
    }
    #[doc = "Bits 26:31 - Resume Validation Period (ResValid) This field is effective only when DCFG.Ena32KHzSusp is set. It controls the resume period when the core resumes from suspend. The core counts for ResValid number of clock cycles to detect a valid resume when this bit is set"]
    #[inline(always)]
    pub fn resvalid(&mut self) -> RESVALID_W<'_, DCFG_SPEC> {
        RESVALID_W::new(self, 26)
    }
}
#[doc = "This register configures the core in Device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCFG to value 0x0802_0000"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: u32 = 0x0802_0000;
}
