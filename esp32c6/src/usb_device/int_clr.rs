#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR` writer - Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_SOF_INT_CLR` writer - Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt."]
pub type USB_SERIAL_JTAG_SOF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR` writer - Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR` writer - Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_PID_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_PID_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_CRC5_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC5_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_CRC16_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_CRC16_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_STUFF_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type USB_SERIAL_JTAG_STUFF_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR` writer - Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt."]
pub type USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR` writer - Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR` writer - Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR` writer - Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_RTS_CHG_INT_CLR` writer - Set this bit to clear the USB_DEVICE_RTS_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_RTS_CHG_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_DTR_CHG_INT_CLR` writer - Set this bit to clear the USB_DEVICE_DTR_CHG_INT interrupt."]
pub type USB_SERIAL_JTAG_DTR_CHG_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR` writer - Set this bit to clear the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR` writer - Set this bit to clear the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_jtag_in_flush_int_clr(
        &mut self,
    ) -> USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR_W<0> {
        USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_sof_int_clr(&mut self) -> USB_SERIAL_JTAG_SOF_INT_CLR_W<1> {
        USB_SERIAL_JTAG_SOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_serial_out_recv_pkt_int_clr(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR_W<2> {
        USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_serial_in_empty_int_clr(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR_W<3> {
        USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_pid_err_int_clr(&mut self) -> USB_SERIAL_JTAG_PID_ERR_INT_CLR_W<4> {
        USB_SERIAL_JTAG_PID_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_crc5_err_int_clr(&mut self) -> USB_SERIAL_JTAG_CRC5_ERR_INT_CLR_W<5> {
        USB_SERIAL_JTAG_CRC5_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_crc16_err_int_clr(&mut self) -> USB_SERIAL_JTAG_CRC16_ERR_INT_CLR_W<6> {
        USB_SERIAL_JTAG_CRC16_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_stuff_err_int_clr(&mut self) -> USB_SERIAL_JTAG_STUFF_ERR_INT_CLR_W<7> {
        USB_SERIAL_JTAG_STUFF_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_in_token_rec_in_ep1_int_clr(
        &mut self,
    ) -> USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR_W<8> {
        USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_usb_bus_reset_int_clr(
        &mut self,
    ) -> USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR_W<9> {
        USB_SERIAL_JTAG_USB_BUS_RESET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_out_ep1_zero_payload_int_clr(
        &mut self,
    ) -> USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR_W<10> {
        USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_out_ep2_zero_payload_int_clr(
        &mut self,
    ) -> USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR_W<11> {
        USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear the USB_DEVICE_RTS_CHG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_rts_chg_int_clr(&mut self) -> USB_SERIAL_JTAG_RTS_CHG_INT_CLR_W<12> {
        USB_SERIAL_JTAG_RTS_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear the USB_DEVICE_DTR_CHG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_dtr_chg_int_clr(&mut self) -> USB_SERIAL_JTAG_DTR_CHG_INT_CLR_W<13> {
        USB_SERIAL_JTAG_DTR_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_get_line_code_int_clr(
        &mut self,
    ) -> USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR_W<14> {
        USB_SERIAL_JTAG_GET_LINE_CODE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_set_line_code_int_clr(
        &mut self,
    ) -> USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR_W<15> {
        USB_SERIAL_JTAG_SET_LINE_CODE_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear status register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
