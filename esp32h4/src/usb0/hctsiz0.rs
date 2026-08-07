#[doc = "Register `HCTSIZ0` reader"]
pub type R = crate::R<HCTSIZ0_SPEC>;
#[doc = "Register `HCTSIZ0` writer"]
pub type W = crate::W<HCTSIZ0_SPEC>;
#[doc = "Field `XFERSIZE` reader - Transfer Size (XferSize) For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has Reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic). The width of this counter is specified as Width of Transfer Size Counters during coreConsultant configuration (parameter OTG_TRANS_COUNT_WIDTH)."]
pub type XFERSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `XFERSIZE` writer - Transfer Size (XferSize) For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has Reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic). The width of this counter is specified as Width of Transfer Size Counters during coreConsultant configuration (parameter OTG_TRANS_COUNT_WIDTH)."]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PKTCNT` reader - Packet Count (PktCnt) This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion. The width of this counter is specified as Width of Packet Counters during coreConsultant configuration (parameter OTG_PACKET_COUNT_WIDTH)."]
pub type PKTCNT_R = crate::FieldReader;
#[doc = "Field `PKTCNT` writer - Packet Count (PktCnt) This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion. The width of this counter is specified as Width of Packet Counters during coreConsultant configuration (parameter OTG_PACKET_COUNT_WIDTH)."]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PID` reader - PID (Pid) The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA (non-control)/SETUP (control)"]
pub type PID_R = crate::FieldReader;
#[doc = "Field `PID` writer - PID (Pid) The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA (non-control)/SETUP (control)"]
pub type PID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOPNG` reader - Do Ping (DoPng) This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for for IN transfers it disables the channel."]
pub type DOPNG_R = crate::BitReader;
#[doc = "Field `DOPNG` writer - Do Ping (DoPng) This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for for IN transfers it disables the channel."]
pub type DOPNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Transfer Size (XferSize) For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has Reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic). The width of this counter is specified as Width of Transfer Size Counters during coreConsultant configuration (parameter OTG_TRANS_COUNT_WIDTH)."]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 19:25 - Packet Count (PktCnt) This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion. The width of this counter is specified as Width of Packet Counters during coreConsultant configuration (parameter OTG_PACKET_COUNT_WIDTH)."]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x7f) as u8)
    }
    #[doc = "Bits 29:30 - PID (Pid) The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Do Ping (DoPng) This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for for IN transfers it disables the channel."]
    #[inline(always)]
    pub fn dopng(&self) -> DOPNG_R {
        DOPNG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ0")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("pid", &self.pid())
            .field("dopng", &self.dopng())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Transfer Size (XferSize) For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has Reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic). The width of this counter is specified as Width of Transfer Size Counters during coreConsultant configuration (parameter OTG_TRANS_COUNT_WIDTH)."]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W<'_, HCTSIZ0_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:25 - Packet Count (PktCnt) This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion. The width of this counter is specified as Width of Packet Counters during coreConsultant configuration (parameter OTG_PACKET_COUNT_WIDTH)."]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, HCTSIZ0_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - PID (Pid) The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer. - 2'b00: DATA0 - 2'b01: DATA2 - 2'b10: DATA1 - 2'b11: MDATA (non-control)/SETUP (control)"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W<'_, HCTSIZ0_SPEC> {
        PID_W::new(self, 29)
    }
    #[doc = "Bit 31 - Do Ping (DoPng) This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for for IN transfers it disables the channel."]
    #[inline(always)]
    pub fn dopng(&mut self) -> DOPNG_W<'_, HCTSIZ0_SPEC> {
        DOPNG_W::new(self, 31)
    }
}
#[doc = "Host Channel 0 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTSIZ0_SPEC;
impl crate::RegisterSpec for HCTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz0::R`](R) reader structure"]
impl crate::Readable for HCTSIZ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctsiz0::W`](W) writer structure"]
impl crate::Writable for HCTSIZ0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCTSIZ0 to value 0"]
impl crate::Resettable for HCTSIZ0_SPEC {}
