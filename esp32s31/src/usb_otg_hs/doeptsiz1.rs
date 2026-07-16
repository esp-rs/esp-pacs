#[doc = "Register `DOEPTSIZ1` reader"]
pub type R = crate::R<DOEPTSIZ1_SPEC>;
#[doc = "Register `DOEPTSIZ1` writer"]
pub type W = crate::W<DOEPTSIZ1_SPEC>;
#[doc = "Field `XFERSIZE` reader - Transfer Size (XferSize) Indicates the transfer size in bytes for the endpoint. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be Set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the RxFIFO and written to the external memory."]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size (XferSize) Indicates the transfer size in bytes for the endpoint. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be Set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the RxFIFO and written to the external memory."]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet Count (PktCnt) This field is decremented to zero after a packet is written into the RxFIFO."]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet Count (PktCnt) This field is decremented to zero after a packet is written into the RxFIFO."]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RXDPID` reader - RxDPID Applies to isochronous OUT endpoints only. This is the data PID received in the last packet for this endpoint. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA SETUP Packet Count (SUPCnt) Applies to control OUT Endpoints only. This field specifies the number of back-to-back SETUP data packets the endpoint can receive. - 2'b01: 1 packet - 2'b10: 2 packets - 2'b11: 3 packets"]
pub type RXDPID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:18 - Transfer Size (XferSize) Indicates the transfer size in bytes for the endpoint. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be Set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the RxFIFO and written to the external memory."]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count (PktCnt) This field is decremented to zero after a packet is written into the RxFIFO."]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - RxDPID Applies to isochronous OUT endpoints only. This is the data PID received in the last packet for this endpoint. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA SETUP Packet Count (SUPCnt) Applies to control OUT Endpoints only. This field specifies the number of back-to-back SETUP data packets the endpoint can receive. - 2'b01: 1 packet - 2'b10: 2 packets - 2'b11: 3 packets"]
    #[inline(always)]
    pub fn rxdpid(&self) -> RXDPID_R {
        RXDPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ1")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpid", &self.rxdpid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size (XferSize) Indicates the transfer size in bytes for the endpoint. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be Set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the RxFIFO and written to the external memory."]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W<'_, DOEPTSIZ1_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count (PktCnt) This field is decremented to zero after a packet is written into the RxFIFO."]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, DOEPTSIZ1_SPEC> {
        PKTCNT_W::new(self, 19)
    }
}
#[doc = "This register contains the Transfer Size for the OUT Endpoint 1 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ1_SPEC;
impl crate::RegisterSpec for DOEPTSIZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz1::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz1::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPTSIZ1 to value 0"]
impl crate::Resettable for DOEPTSIZ1_SPEC {}
