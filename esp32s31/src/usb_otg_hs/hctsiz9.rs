#[doc = "Register `HCTSIZ9` reader"]
pub type R = crate::R<HCTSIZ9_SPEC>;
#[doc = "Register `HCTSIZ9` writer"]
pub type W = crate::W<HCTSIZ9_SPEC>;
#[doc = "Field `XFERSIZE` reader - Non-Scatter/Gather DMA Mode: Transfer Size (XferSize) For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has Reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic). The width of this counter is specified as Width of Transfer Size Counters during coreConsultant configuration (parameter OTG_TRANS_COUNT_WIDTH). Scatter/Gather DMA Mode: \\[18:16\\]: Reserved \\[15:8\\]: NTD (Number of Transfer Descriptors) (Non Isochronous) This value is in terms of number of descriptors. Maximum number of descriptor that can be present in the list is 64. The values can be from 0 to 63. - 0: 1 descriptor - 63: 64 descriptors This field indicates the total number of descriptors present in that list. The core wraps around after servicing NTD number of descriptors for that list. (Isochronous) This field indicates the number of descriptors present in that list microframe. The possible values for FS are - 1: 2 descriptors - 3: 4 descriptors - 7: 8 descriptors - 15: 16 descriptors - 31: 32 descriptors - 63: 64 descriptors The possible values for HS are - 7: 8 descriptors - 15: 16 descriptors - 31: 32 descriptors - 63: 64 descriptors - 127: 128 descriptors - 255: 256 descriptors \\[7:0\\]: SCHED_INFO (Schedule information) Every bit in this 8 bit register indicates scheduling for that microframe. Bit 0 indicates scheduling for 1st microframe and bit 7 indicates scheduling for 8th microframe in that frame. A value of 8'b11111111 indicates that the corresponding interrupt channel is scheduled to issue a token every microframe in that frame. A value of 8'b10101010 indicates that the corresponding interrupt channel is scheduled to issue a token every alternate microframe starting with second microframe. Note that this field is applicable only for periodic (Isochronous and Interrupt) channels."]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Non-Scatter/Gather DMA Mode: Transfer Size (XferSize) For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has Reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic). The width of this counter is specified as Width of Transfer Size Counters during coreConsultant configuration (parameter OTG_TRANS_COUNT_WIDTH). Scatter/Gather DMA Mode: \\[18:16\\]: Reserved \\[15:8\\]: NTD (Number of Transfer Descriptors) (Non Isochronous) This value is in terms of number of descriptors. Maximum number of descriptor that can be present in the list is 64. The values can be from 0 to 63. - 0: 1 descriptor - 63: 64 descriptors This field indicates the total number of descriptors present in that list. The core wraps around after servicing NTD number of descriptors for that list. (Isochronous) This field indicates the number of descriptors present in that list microframe. The possible values for FS are - 1: 2 descriptors - 3: 4 descriptors - 7: 8 descriptors - 15: 16 descriptors - 31: 32 descriptors - 63: 64 descriptors The possible values for HS are - 7: 8 descriptors - 15: 16 descriptors - 31: 32 descriptors - 63: 64 descriptors - 127: 128 descriptors - 255: 256 descriptors \\[7:0\\]: SCHED_INFO (Schedule information) Every bit in this 8 bit register indicates scheduling for that microframe. Bit 0 indicates scheduling for 1st microframe and bit 7 indicates scheduling for 8th microframe in that frame. A value of 8'b11111111 indicates that the corresponding interrupt channel is scheduled to issue a token every microframe in that frame. A value of 8'b10101010 indicates that the corresponding interrupt channel is scheduled to issue a token every alternate microframe starting with second microframe. Note that this field is applicable only for periodic (Isochronous and Interrupt) channels."]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Non-Scatter/Gather DMA Mode: Packet Count (PktCnt) This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion. The width of this counter is specified as Width of Packet Counters during coreConsultant configuration (parameter OTG_PACKET_COUNT_WIDTH). Scatter/Gather DMA Mode: \\[28:19\\]: Reserved"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Non-Scatter/Gather DMA Mode: Packet Count (PktCnt) This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion. The width of this counter is specified as Width of Packet Counters during coreConsultant configuration (parameter OTG_PACKET_COUNT_WIDTH). Scatter/Gather DMA Mode: \\[28:19\\]: Reserved"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PID` reader - PID (Pid) The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA (non-control)/SETUP (control)"]
pub type PID_R = crate::FieldReader;
#[doc = "Field `PID` writer - PID (Pid) The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA (non-control)/SETUP (control)"]
pub type PID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOPNG` reader - Do Ping (DoPng) This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for IN transfers it disables the channel."]
pub type DOPNG_R = crate::BitReader;
#[doc = "Field `DOPNG` writer - Do Ping (DoPng) This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for IN transfers it disables the channel."]
pub type DOPNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - Non-Scatter/Gather DMA Mode: Transfer Size (XferSize) For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has Reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic). The width of this counter is specified as Width of Transfer Size Counters during coreConsultant configuration (parameter OTG_TRANS_COUNT_WIDTH). Scatter/Gather DMA Mode: \\[18:16\\]: Reserved \\[15:8\\]: NTD (Number of Transfer Descriptors) (Non Isochronous) This value is in terms of number of descriptors. Maximum number of descriptor that can be present in the list is 64. The values can be from 0 to 63. - 0: 1 descriptor - 63: 64 descriptors This field indicates the total number of descriptors present in that list. The core wraps around after servicing NTD number of descriptors for that list. (Isochronous) This field indicates the number of descriptors present in that list microframe. The possible values for FS are - 1: 2 descriptors - 3: 4 descriptors - 7: 8 descriptors - 15: 16 descriptors - 31: 32 descriptors - 63: 64 descriptors The possible values for HS are - 7: 8 descriptors - 15: 16 descriptors - 31: 32 descriptors - 63: 64 descriptors - 127: 128 descriptors - 255: 256 descriptors \\[7:0\\]: SCHED_INFO (Schedule information) Every bit in this 8 bit register indicates scheduling for that microframe. Bit 0 indicates scheduling for 1st microframe and bit 7 indicates scheduling for 8th microframe in that frame. A value of 8'b11111111 indicates that the corresponding interrupt channel is scheduled to issue a token every microframe in that frame. A value of 8'b10101010 indicates that the corresponding interrupt channel is scheduled to issue a token every alternate microframe starting with second microframe. Note that this field is applicable only for periodic (Isochronous and Interrupt) channels."]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Non-Scatter/Gather DMA Mode: Packet Count (PktCnt) This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion. The width of this counter is specified as Width of Packet Counters during coreConsultant configuration (parameter OTG_PACKET_COUNT_WIDTH). Scatter/Gather DMA Mode: \\[28:19\\]: Reserved"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - PID (Pid) The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Do Ping (DoPng) This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for IN transfers it disables the channel."]
    #[inline(always)]
    pub fn dopng(&self) -> DOPNG_R {
        DOPNG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ9")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("pid", &self.pid())
            .field("dopng", &self.dopng())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18 - Non-Scatter/Gather DMA Mode: Transfer Size (XferSize) For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has Reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic). The width of this counter is specified as Width of Transfer Size Counters during coreConsultant configuration (parameter OTG_TRANS_COUNT_WIDTH). Scatter/Gather DMA Mode: \\[18:16\\]: Reserved \\[15:8\\]: NTD (Number of Transfer Descriptors) (Non Isochronous) This value is in terms of number of descriptors. Maximum number of descriptor that can be present in the list is 64. The values can be from 0 to 63. - 0: 1 descriptor - 63: 64 descriptors This field indicates the total number of descriptors present in that list. The core wraps around after servicing NTD number of descriptors for that list. (Isochronous) This field indicates the number of descriptors present in that list microframe. The possible values for FS are - 1: 2 descriptors - 3: 4 descriptors - 7: 8 descriptors - 15: 16 descriptors - 31: 32 descriptors - 63: 64 descriptors The possible values for HS are - 7: 8 descriptors - 15: 16 descriptors - 31: 32 descriptors - 63: 64 descriptors - 127: 128 descriptors - 255: 256 descriptors \\[7:0\\]: SCHED_INFO (Schedule information) Every bit in this 8 bit register indicates scheduling for that microframe. Bit 0 indicates scheduling for 1st microframe and bit 7 indicates scheduling for 8th microframe in that frame. A value of 8'b11111111 indicates that the corresponding interrupt channel is scheduled to issue a token every microframe in that frame. A value of 8'b10101010 indicates that the corresponding interrupt channel is scheduled to issue a token every alternate microframe starting with second microframe. Note that this field is applicable only for periodic (Isochronous and Interrupt) channels."]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W<'_, HCTSIZ9_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Non-Scatter/Gather DMA Mode: Packet Count (PktCnt) This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion. The width of this counter is specified as Width of Packet Counters during coreConsultant configuration (parameter OTG_PACKET_COUNT_WIDTH). Scatter/Gather DMA Mode: \\[28:19\\]: Reserved"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, HCTSIZ9_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - PID (Pid) The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W<'_, HCTSIZ9_SPEC> {
        PID_W::new(self, 29)
    }
    #[doc = "Bit 31 - Do Ping (DoPng) This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for IN transfers it disables the channel."]
    #[inline(always)]
    pub fn dopng(&mut self) -> DOPNG_W<'_, HCTSIZ9_SPEC> {
        DOPNG_W::new(self, 31)
    }
}
#[doc = "This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTSIZ9_SPEC;
impl crate::RegisterSpec for HCTSIZ9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz9::R`](R) reader structure"]
impl crate::Readable for HCTSIZ9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctsiz9::W`](W) writer structure"]
impl crate::Writable for HCTSIZ9_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCTSIZ9 to value 0"]
impl crate::Resettable for HCTSIZ9_SPEC {}
