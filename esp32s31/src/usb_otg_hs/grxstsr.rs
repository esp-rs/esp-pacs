#[doc = "Register `GRXSTSR` reader"]
pub type R = crate::R<GRXSTSR_SPEC>;
#[doc = "Field `CHNUM` reader - Channel Number (ChNum) Mode: Host only Indicates the channel number to which the current received packet belongs. Endpoint Number (EPNum) Mode: Device only Indicates the endpoint number to which the current received packet belongs."]
pub type CHNUM_R = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte Count (BCnt) In host mode, indicates the byte count of the received IN data packet. In device mode, indicates the byte count of the received data packet."]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID (DPID) In host mode, indicates the Data PID of the received packet. In device mode, indicates the Data PID of the received OUT data packet. - 2'b00: DATA0 - 2'b10: DATA1 - 2'b01: DATA2 - 2'b11: MDATA Reset: 2'h0"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet Status (PktSts) indicates the status of the received packet. In host mode, - 4'b0010: IN data packet received - 4'b0011: IN transfer completed (triggers an interrupt) - 4'b0101: Data toggle error (triggers an interrupt) - 4'b0111: Channel halted (triggers an interrupt) - Others: Reserved Reset:4'b0 In device mode, - 4'b0001: Global OUT NAK (triggers an interrupt) - 4'b0010: OUT data packet received - 4'b0011: OUT transfer completed (triggers an interrupt) - 4'b0100: SETUP transaction completed (triggers an interrupt) - 4'b0110: SETUP data packet received - Others: Reserved Reset:4'h0"]
pub type PKTSTS_R = crate::FieldReader;
#[doc = "Field `FN` reader - Mode: Device only Frame Number (FN) This is the least significant 4 bits of the (micro)Frame number in which the packet is received on the USB. This field is supported only when isochronous OUT endpoints are supported."]
pub type FN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Channel Number (ChNum) Mode: Host only Indicates the channel number to which the current received packet belongs. Endpoint Number (EPNum) Mode: Device only Indicates the endpoint number to which the current received packet belongs."]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count (BCnt) In host mode, indicates the byte count of the received IN data packet. In device mode, indicates the byte count of the received data packet."]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID (DPID) In host mode, indicates the Data PID of the received packet. In device mode, indicates the Data PID of the received OUT data packet. - 2'b00: DATA0 - 2'b10: DATA1 - 2'b01: DATA2 - 2'b11: MDATA Reset: 2'h0"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status (PktSts) indicates the status of the received packet. In host mode, - 4'b0010: IN data packet received - 4'b0011: IN transfer completed (triggers an interrupt) - 4'b0101: Data toggle error (triggers an interrupt) - 4'b0111: Channel halted (triggers an interrupt) - Others: Reserved Reset:4'b0 In device mode, - 4'b0001: Global OUT NAK (triggers an interrupt) - 4'b0010: OUT data packet received - 4'b0011: OUT transfer completed (triggers an interrupt) - 4'b0100: SETUP transaction completed (triggers an interrupt) - 4'b0110: SETUP data packet received - Others: Reserved Reset:4'h0"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Mode: Device only Frame Number (FN) This is the least significant 4 bits of the (micro)Frame number in which the packet is received on the USB. This field is supported only when isochronous OUT endpoints are supported."]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSR")
            .field("chnum", &self.chnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .field("fn_", &self.fn_())
            .finish()
    }
}
#[doc = "A read to the Receive Status Debug Read register returns the contents of the top of the Receive FIFO. The receive status contents must be interpreted differently in Host and Device modes. The core ignores the receive status read when the receive FIFO is empty and returns a value of 32'h0000_0000. Note: - Use of these fields vary based on whether the core is functioning as a host or a device. - Do not read this register's reset value before configuring the core because the read value is 'X' in the simulation.\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXSTSR_SPEC;
impl crate::RegisterSpec for GRXSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr::R`](R) reader structure"]
impl crate::Readable for GRXSTSR_SPEC {}
#[doc = "`reset()` method sets GRXSTSR to value 0"]
impl crate::Resettable for GRXSTSR_SPEC {}
