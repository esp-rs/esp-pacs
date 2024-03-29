#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `JTAG_IN_FLUSH` reader - The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type JTAG_IN_FLUSH_R = crate::BitReader;
#[doc = "Field `SOF` reader - The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt."]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SERIAL_OUT_RECV_PKT` reader - The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SERIAL_OUT_RECV_PKT_R = crate::BitReader;
#[doc = "Field `SERIAL_IN_EMPTY` reader - The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type SERIAL_IN_EMPTY_R = crate::BitReader;
#[doc = "Field `PID_ERR` reader - The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt."]
pub type PID_ERR_R = crate::BitReader;
#[doc = "Field `CRC5_ERR` reader - The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type CRC5_ERR_R = crate::BitReader;
#[doc = "Field `CRC16_ERR` reader - The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type CRC16_ERR_R = crate::BitReader;
#[doc = "Field `STUFF_ERR` reader - The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type STUFF_ERR_R = crate::BitReader;
#[doc = "Field `IN_TOKEN_REC_IN_EP1` reader - The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type IN_TOKEN_REC_IN_EP1_R = crate::BitReader;
#[doc = "Field `USB_BUS_RESET` reader - The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type USB_BUS_RESET_R = crate::BitReader;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD` reader - The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP1_ZERO_PAYLOAD_R = crate::BitReader;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD` reader - The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP2_ZERO_PAYLOAD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn jtag_in_flush(&self) -> JTAG_IN_FLUSH_R {
        JTAG_IN_FLUSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt."]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn serial_out_recv_pkt(&self) -> SERIAL_OUT_RECV_PKT_R {
        SERIAL_OUT_RECV_PKT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn serial_in_empty(&self) -> SERIAL_IN_EMPTY_R {
        SERIAL_IN_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn pid_err(&self) -> PID_ERR_R {
        PID_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc5_err(&self) -> CRC5_ERR_R {
        CRC5_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc16_err(&self) -> CRC16_ERR_R {
        CRC16_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn stuff_err(&self) -> STUFF_ERR_R {
        STUFF_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1(&self) -> IN_TOKEN_REC_IN_EP1_R {
        IN_TOKEN_REC_IN_EP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_bus_reset(&self) -> USB_BUS_RESET_R {
        USB_BUS_RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep1_zero_payload(&self) -> OUT_EP1_ZERO_PAYLOAD_R {
        OUT_EP1_ZERO_PAYLOAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep2_zero_payload(&self) -> OUT_EP2_ZERO_PAYLOAD_R {
        OUT_EP2_ZERO_PAYLOAD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "jtag_in_flush",
                &format_args!("{}", self.jtag_in_flush().bit()),
            )
            .field("sof", &format_args!("{}", self.sof().bit()))
            .field(
                "serial_out_recv_pkt",
                &format_args!("{}", self.serial_out_recv_pkt().bit()),
            )
            .field(
                "serial_in_empty",
                &format_args!("{}", self.serial_in_empty().bit()),
            )
            .field("pid_err", &format_args!("{}", self.pid_err().bit()))
            .field("crc5_err", &format_args!("{}", self.crc5_err().bit()))
            .field("crc16_err", &format_args!("{}", self.crc16_err().bit()))
            .field("stuff_err", &format_args!("{}", self.stuff_err().bit()))
            .field(
                "in_token_rec_in_ep1",
                &format_args!("{}", self.in_token_rec_in_ep1().bit()),
            )
            .field(
                "usb_bus_reset",
                &format_args!("{}", self.usb_bus_reset().bit()),
            )
            .field(
                "out_ep1_zero_payload",
                &format_args!("{}", self.out_ep1_zero_payload().bit()),
            )
            .field(
                "out_ep2_zero_payload",
                &format_args!("{}", self.out_ep2_zero_payload().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "USB_DEVICE_INT_ST_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
