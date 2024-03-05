#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `JTAG_IN_FLUSH_INT_CLR` writer - Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type JTAG_IN_FLUSH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF_INT_CLR` writer - Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt."]
pub type SOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_CLR` writer - Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SERIAL_OUT_RECV_PKT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_IN_EMPTY_INT_CLR` writer - Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type SERIAL_IN_EMPTY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt."]
pub type PID_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC5_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type CRC5_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC16_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type CRC16_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUFF_ERR_INT_CLR` writer - Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type STUFF_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_CLR` writer - Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt."]
pub type IN_TOKEN_REC_IN_EP1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_BUS_RESET_INT_CLR` writer - Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type USB_BUS_RESET_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_CLR` writer - Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_CLR` writer - Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_in_flush_int_clr(&mut self) -> JTAG_IN_FLUSH_INT_CLR_W<INT_CLR_SPEC> {
        JTAG_IN_FLUSH_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sof_int_clr(&mut self) -> SOF_INT_CLR_W<INT_CLR_SPEC> {
        SOF_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn serial_out_recv_pkt_int_clr(&mut self) -> SERIAL_OUT_RECV_PKT_INT_CLR_W<INT_CLR_SPEC> {
        SERIAL_OUT_RECV_PKT_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn serial_in_empty_int_clr(&mut self) -> SERIAL_IN_EMPTY_INT_CLR_W<INT_CLR_SPEC> {
        SERIAL_IN_EMPTY_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pid_err_int_clr(&mut self) -> PID_ERR_INT_CLR_W<INT_CLR_SPEC> {
        PID_ERR_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn crc5_err_int_clr(&mut self) -> CRC5_ERR_INT_CLR_W<INT_CLR_SPEC> {
        CRC5_ERR_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn crc16_err_int_clr(&mut self) -> CRC16_ERR_INT_CLR_W<INT_CLR_SPEC> {
        CRC16_ERR_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stuff_err_int_clr(&mut self) -> STUFF_ERR_INT_CLR_W<INT_CLR_SPEC> {
        STUFF_ERR_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_token_rec_in_ep1_int_clr(&mut self) -> IN_TOKEN_REC_IN_EP1_INT_CLR_W<INT_CLR_SPEC> {
        IN_TOKEN_REC_IN_EP1_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_bus_reset_int_clr(&mut self) -> USB_BUS_RESET_INT_CLR_W<INT_CLR_SPEC> {
        USB_BUS_RESET_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep1_zero_payload_int_clr(&mut self) -> OUT_EP1_ZERO_PAYLOAD_INT_CLR_W<INT_CLR_SPEC> {
        OUT_EP1_ZERO_PAYLOAD_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep2_zero_payload_int_clr(&mut self) -> OUT_EP2_ZERO_PAYLOAD_INT_CLR_W<INT_CLR_SPEC> {
        OUT_EP2_ZERO_PAYLOAD_INT_CLR_W::new(self, 11)
    }
}
#[doc = "USB_DEVICE_INT_CLR_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
