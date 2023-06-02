#[doc = "Register `STATE0` reader"]
pub struct R(crate::R<STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_ERR_CAUSE` reader - This register indicates the error type when DMA has received a packet with error. 3'b001: Checksum error in HCI packet. 3'b010: Sequence number error in HCI packet. 3'b011: CRC bit error in HCI packet. 3'b100: 0xc0 is found but received HCI packet is not end. 3'b101: 0xc0 is not found when receiving HCI packet is end. 3'b110: CRC check error."]
pub type RX_ERR_CAUSE_R = crate::FieldReader;
#[doc = "Field `DECODE_STATE` reader - UHCI decoder status."]
pub type DECODE_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - This register indicates the error type when DMA has received a packet with error. 3'b001: Checksum error in HCI packet. 3'b010: Sequence number error in HCI packet. 3'b011: CRC bit error in HCI packet. 3'b100: 0xc0 is found but received HCI packet is not end. 3'b101: 0xc0 is not found when receiving HCI packet is end. 3'b110: CRC check error."]
    #[inline(always)]
    pub fn rx_err_cause(&self) -> RX_ERR_CAUSE_R {
        RX_ERR_CAUSE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - UHCI decoder status."]
    #[inline(always)]
    pub fn decode_state(&self) -> DECODE_STATE_R {
        DECODE_STATE_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE0")
            .field(
                "rx_err_cause",
                &format_args!("{}", self.rx_err_cause().bits()),
            )
            .field(
                "decode_state",
                &format_args!("{}", self.decode_state().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "UHCI receive status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state0](index.html) module"]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state0::R](R) reader structure"]
impl crate::Readable for STATE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE0 to value 0"]
impl crate::Resettable for STATE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
