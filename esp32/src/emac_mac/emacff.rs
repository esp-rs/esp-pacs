#[doc = "Register `EMACFF` reader"]
pub type R = crate::R<EMACFF_SPEC>;
#[doc = "Register `EMACFF` writer"]
pub type W = crate::W<EMACFF_SPEC>;
#[doc = "Field `PMODE` reader - When this bit is set the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR(PRI_RATIO) is set."]
pub type PMODE_R = crate::BitReader;
#[doc = "Field `PMODE` writer - When this bit is set the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR(PRI_RATIO) is set."]
pub type PMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAIF` reader - When this bit is set the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset normal filtering of frames is performed."]
pub type DAIF_R = crate::BitReader;
#[doc = "Field `DAIF` writer - When this bit is set the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset normal filtering of frames is performed."]
pub type DAIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAM` reader - When set this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
pub type PAM_R = crate::BitReader;
#[doc = "Field `PAM` writer - When set this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
pub type PAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBF` reader - When this bit is set the AFM(Address Filtering Module) module blocks all incoming broadcast frames. In addition it overrides all other filter settings. When this bit is reset the AFM module passes all received broadcast Frames."]
pub type DBF_R = crate::BitReader;
#[doc = "Field `DBF` writer - When this bit is set the AFM(Address Filtering Module) module blocks all incoming broadcast frames. In addition it overrides all other filter settings. When this bit is reset the AFM module passes all received broadcast Frames."]
pub type DBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCF` reader - These bits control the forwarding of all control frames (including unicast and multicast Pause frames). 2'b00: MAC filters all control frames from reaching the application. 2'b01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. 2'b10: MAC forwards all control frames to application even if they fail the Address Filter. 2'b11: MAC forwards control frames that pass the Address Filter.The following conditions should be true for the Pause frames processing: Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register (Flow Control Register) to 1. Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register(Flow Control Register) is set. Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001."]
pub type PCF_R = crate::FieldReader;
#[doc = "Field `PCF` writer - These bits control the forwarding of all control frames (including unicast and multicast Pause frames). 2'b00: MAC filters all control frames from reaching the application. 2'b01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. 2'b10: MAC forwards all control frames to application even if they fail the Address Filter. 2'b11: MAC forwards control frames that pass the Address Filter.The following conditions should be true for the Pause frames processing: Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register (Flow Control Register) to 1. Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register(Flow Control Register) is set. Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001."]
pub type PCF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SAIF` reader - When this bit is set the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
pub type SAIF_R = crate::BitReader;
#[doc = "Field `SAIF` writer - When this bit is set the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
pub type SAIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAFE` reader - When this bit is set the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails the MAC drops the frame. When this bit is reset the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
pub type SAFE_R = crate::BitReader;
#[doc = "Field `SAFE` writer - When this bit is set the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails the MAC drops the frame. When this bit is reset the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
pub type SAFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RECEIVE_ALL` reader - When this bit is set the MAC Receiver module passes all received frames irrespective of whether they pass the address filter or not to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset the Receiver module passes only those frames to the Application that pass the SA or DA address Filter."]
pub type RECEIVE_ALL_R = crate::BitReader;
#[doc = "Field `RECEIVE_ALL` writer - When this bit is set the MAC Receiver module passes all received frames irrespective of whether they pass the address filter or not to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset the Receiver module passes only those frames to the Application that pass the SA or DA address Filter."]
pub type RECEIVE_ALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - When this bit is set the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR(PRI_RATIO) is set."]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is set the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset normal filtering of frames is performed."]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When this bit is set the AFM(Address Filtering Module) module blocks all incoming broadcast frames. In addition it overrides all other filter settings. When this bit is reset the AFM module passes all received broadcast Frames."]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - These bits control the forwarding of all control frames (including unicast and multicast Pause frames). 2'b00: MAC filters all control frames from reaching the application. 2'b01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. 2'b10: MAC forwards all control frames to application even if they fail the Address Filter. 2'b11: MAC forwards control frames that pass the Address Filter.The following conditions should be true for the Pause frames processing: Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register (Flow Control Register) to 1. Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register(Flow Control Register) is set. Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001."]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - When this bit is set the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When this bit is set the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails the MAC drops the frame. When this bit is reset the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
    #[inline(always)]
    pub fn safe(&self) -> SAFE_R {
        SAFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - When this bit is set the MAC Receiver module passes all received frames irrespective of whether they pass the address filter or not to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset the Receiver module passes only those frames to the Application that pass the SA or DA address Filter."]
    #[inline(always)]
    pub fn receive_all(&self) -> RECEIVE_ALL_R {
        RECEIVE_ALL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACFF")
            .field("pmode", &format_args!("{}", self.pmode().bit()))
            .field("daif", &format_args!("{}", self.daif().bit()))
            .field("pam", &format_args!("{}", self.pam().bit()))
            .field("dbf", &format_args!("{}", self.dbf().bit()))
            .field("pcf", &format_args!("{}", self.pcf().bits()))
            .field("saif", &format_args!("{}", self.saif().bit()))
            .field("safe", &format_args!("{}", self.safe().bit()))
            .field("receive_all", &format_args!("{}", self.receive_all().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACFF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is set the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR(PRI_RATIO) is set."]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<EMACFF_SPEC, 0> {
        PMODE_W::new(self)
    }
    #[doc = "Bit 3 - When this bit is set the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset normal filtering of frames is performed."]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<EMACFF_SPEC, 3> {
        DAIF_W::new(self)
    }
    #[doc = "Bit 4 - When set this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<EMACFF_SPEC, 4> {
        PAM_W::new(self)
    }
    #[doc = "Bit 5 - When this bit is set the AFM(Address Filtering Module) module blocks all incoming broadcast frames. In addition it overrides all other filter settings. When this bit is reset the AFM module passes all received broadcast Frames."]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<EMACFF_SPEC, 5> {
        DBF_W::new(self)
    }
    #[doc = "Bits 6:7 - These bits control the forwarding of all control frames (including unicast and multicast Pause frames). 2'b00: MAC filters all control frames from reaching the application. 2'b01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. 2'b10: MAC forwards all control frames to application even if they fail the Address Filter. 2'b11: MAC forwards control frames that pass the Address Filter.The following conditions should be true for the Pause frames processing: Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register (Flow Control Register) to 1. Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register(Flow Control Register) is set. Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001."]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<EMACFF_SPEC, 6> {
        PCF_W::new(self)
    }
    #[doc = "Bit 8 - When this bit is set the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<EMACFF_SPEC, 8> {
        SAIF_W::new(self)
    }
    #[doc = "Bit 9 - When this bit is set the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails the MAC drops the frame. When this bit is reset the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
    #[inline(always)]
    #[must_use]
    pub fn safe(&mut self) -> SAFE_W<EMACFF_SPEC, 9> {
        SAFE_W::new(self)
    }
    #[doc = "Bit 31 - When this bit is set the MAC Receiver module passes all received frames irrespective of whether they pass the address filter or not to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset the Receiver module passes only those frames to the Application that pass the SA or DA address Filter."]
    #[inline(always)]
    #[must_use]
    pub fn receive_all(&mut self) -> RECEIVE_ALL_W<EMACFF_SPEC, 31> {
        RECEIVE_ALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Frame filter settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACFF_SPEC;
impl crate::RegisterSpec for EMACFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacff::R`](R) reader structure"]
impl crate::Readable for EMACFF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacff::W`](W) writer structure"]
impl crate::Writable for EMACFF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMACFF to value 0"]
impl crate::Resettable for EMACFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
