#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JTAG_IN_FLUSH_INT_RAW` reader - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
pub type JTAG_IN_FLUSH_INT_RAW_R = crate::BitReader;
#[doc = "Field `JTAG_IN_FLUSH_INT_RAW` writer - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
pub type JTAG_IN_FLUSH_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `SOF_INT_RAW` reader - The raw interrupt bit turns to high level when SOF frame is received."]
pub type SOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SOF_INT_RAW` writer - The raw interrupt bit turns to high level when SOF frame is received."]
pub type SOF_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_RAW` reader - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
pub type SERIAL_OUT_RECV_PKT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_RAW` writer - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
pub type SERIAL_OUT_RECV_PKT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `SERIAL_IN_EMPTY_INT_RAW` reader - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
pub type SERIAL_IN_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `SERIAL_IN_EMPTY_INT_RAW` writer - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
pub type SERIAL_IN_EMPTY_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `PID_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when pid error is detected."]
pub type PID_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `PID_ERR_INT_RAW` writer - The raw interrupt bit turns to high level when pid error is detected."]
pub type PID_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CRC5_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when CRC5 error is detected."]
pub type CRC5_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CRC5_ERR_INT_RAW` writer - The raw interrupt bit turns to high level when CRC5 error is detected."]
pub type CRC5_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CRC16_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when CRC16 error is detected."]
pub type CRC16_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CRC16_ERR_INT_RAW` writer - The raw interrupt bit turns to high level when CRC16 error is detected."]
pub type CRC16_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `STUFF_ERR_INT_RAW` reader - The raw interrupt bit turns to high level when stuff error is detected."]
pub type STUFF_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `STUFF_ERR_INT_RAW` writer - The raw interrupt bit turns to high level when stuff error is detected."]
pub type STUFF_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_RAW` reader - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
pub type IN_TOKEN_REC_IN_EP1_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_RAW` writer - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
pub type IN_TOKEN_REC_IN_EP1_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `USB_BUS_RESET_INT_RAW` reader - The raw interrupt bit turns to high level when usb bus reset is detected."]
pub type USB_BUS_RESET_INT_RAW_R = crate::BitReader;
#[doc = "Field `USB_BUS_RESET_INT_RAW` writer - The raw interrupt bit turns to high level when usb bus reset is detected."]
pub type USB_BUS_RESET_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_RAW` reader - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_RAW` writer - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_RAW` reader - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_RAW` writer - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
    #[inline(always)]
    pub fn jtag_in_flush_int_raw(&self) -> JTAG_IN_FLUSH_INT_RAW_R {
        JTAG_IN_FLUSH_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when SOF frame is received."]
    #[inline(always)]
    pub fn sof_int_raw(&self) -> SOF_INT_RAW_R {
        SOF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
    #[inline(always)]
    pub fn serial_out_recv_pkt_int_raw(&self) -> SERIAL_OUT_RECV_PKT_INT_RAW_R {
        SERIAL_OUT_RECV_PKT_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
    #[inline(always)]
    pub fn serial_in_empty_int_raw(&self) -> SERIAL_IN_EMPTY_INT_RAW_R {
        SERIAL_IN_EMPTY_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when pid error is detected."]
    #[inline(always)]
    pub fn pid_err_int_raw(&self) -> PID_ERR_INT_RAW_R {
        PID_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when CRC5 error is detected."]
    #[inline(always)]
    pub fn crc5_err_int_raw(&self) -> CRC5_ERR_INT_RAW_R {
        CRC5_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when CRC16 error is detected."]
    #[inline(always)]
    pub fn crc16_err_int_raw(&self) -> CRC16_ERR_INT_RAW_R {
        CRC16_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when stuff error is detected."]
    #[inline(always)]
    pub fn stuff_err_int_raw(&self) -> STUFF_ERR_INT_RAW_R {
        STUFF_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1_int_raw(&self) -> IN_TOKEN_REC_IN_EP1_INT_RAW_R {
        IN_TOKEN_REC_IN_EP1_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when usb bus reset is detected."]
    #[inline(always)]
    pub fn usb_bus_reset_int_raw(&self) -> USB_BUS_RESET_INT_RAW_R {
        USB_BUS_RESET_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
    #[inline(always)]
    pub fn out_ep1_zero_payload_int_raw(&self) -> OUT_EP1_ZERO_PAYLOAD_INT_RAW_R {
        OUT_EP1_ZERO_PAYLOAD_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
    #[inline(always)]
    pub fn out_ep2_zero_payload_int_raw(&self) -> OUT_EP2_ZERO_PAYLOAD_INT_RAW_R {
        OUT_EP2_ZERO_PAYLOAD_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "jtag_in_flush_int_raw",
                &format_args!("{}", self.jtag_in_flush_int_raw().bit()),
            )
            .field("sof_int_raw", &format_args!("{}", self.sof_int_raw().bit()))
            .field(
                "serial_out_recv_pkt_int_raw",
                &format_args!("{}", self.serial_out_recv_pkt_int_raw().bit()),
            )
            .field(
                "serial_in_empty_int_raw",
                &format_args!("{}", self.serial_in_empty_int_raw().bit()),
            )
            .field(
                "pid_err_int_raw",
                &format_args!("{}", self.pid_err_int_raw().bit()),
            )
            .field(
                "crc5_err_int_raw",
                &format_args!("{}", self.crc5_err_int_raw().bit()),
            )
            .field(
                "crc16_err_int_raw",
                &format_args!("{}", self.crc16_err_int_raw().bit()),
            )
            .field(
                "stuff_err_int_raw",
                &format_args!("{}", self.stuff_err_int_raw().bit()),
            )
            .field(
                "in_token_rec_in_ep1_int_raw",
                &format_args!("{}", self.in_token_rec_in_ep1_int_raw().bit()),
            )
            .field(
                "usb_bus_reset_int_raw",
                &format_args!("{}", self.usb_bus_reset_int_raw().bit()),
            )
            .field(
                "out_ep1_zero_payload_int_raw",
                &format_args!("{}", self.out_ep1_zero_payload_int_raw().bit()),
            )
            .field(
                "out_ep2_zero_payload_int_raw",
                &format_args!("{}", self.out_ep2_zero_payload_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_in_flush_int_raw(&mut self) -> JTAG_IN_FLUSH_INT_RAW_W<0> {
        JTAG_IN_FLUSH_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when SOF frame is received."]
    #[inline(always)]
    #[must_use]
    pub fn sof_int_raw(&mut self) -> SOF_INT_RAW_W<1> {
        SOF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
    #[inline(always)]
    #[must_use]
    pub fn serial_out_recv_pkt_int_raw(&mut self) -> SERIAL_OUT_RECV_PKT_INT_RAW_W<2> {
        SERIAL_OUT_RECV_PKT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
    #[inline(always)]
    #[must_use]
    pub fn serial_in_empty_int_raw(&mut self) -> SERIAL_IN_EMPTY_INT_RAW_W<3> {
        SERIAL_IN_EMPTY_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when pid error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn pid_err_int_raw(&mut self) -> PID_ERR_INT_RAW_W<4> {
        PID_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when CRC5 error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn crc5_err_int_raw(&mut self) -> CRC5_ERR_INT_RAW_W<5> {
        CRC5_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when CRC16 error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn crc16_err_int_raw(&mut self) -> CRC16_ERR_INT_RAW_W<6> {
        CRC16_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when stuff error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn stuff_err_int_raw(&mut self) -> STUFF_ERR_INT_RAW_W<7> {
        STUFF_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
    #[inline(always)]
    #[must_use]
    pub fn in_token_rec_in_ep1_int_raw(&mut self) -> IN_TOKEN_REC_IN_EP1_INT_RAW_W<8> {
        IN_TOKEN_REC_IN_EP1_INT_RAW_W::new(self)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when usb bus reset is detected."]
    #[inline(always)]
    #[must_use]
    pub fn usb_bus_reset_int_raw(&mut self) -> USB_BUS_RESET_INT_RAW_W<9> {
        USB_BUS_RESET_INT_RAW_W::new(self)
    }
    #[doc = "Bit 10 - The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep1_zero_payload_int_raw(&mut self) -> OUT_EP1_ZERO_PAYLOAD_INT_RAW_W<10> {
        OUT_EP1_ZERO_PAYLOAD_INT_RAW_W::new(self)
    }
    #[doc = "Bit 11 - The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep2_zero_payload_int_raw(&mut self) -> OUT_EP2_ZERO_PAYLOAD_INT_RAW_W<11> {
        OUT_EP2_ZERO_PAYLOAD_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_DEVICE_INT_RAW_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0x08"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
