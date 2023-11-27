#[doc = "Register `EP1` reader"]
pub type R = crate::R<EP1_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_RDWR_BYTE` reader - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO."]
pub type USB_SERIAL_JTAG_RDWR_BYTE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO."]
    #[inline(always)]
    pub fn usb_serial_jtag_rdwr_byte(&self) -> USB_SERIAL_JTAG_RDWR_BYTE_R {
        USB_SERIAL_JTAG_RDWR_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EP1")
            .field(
                "usb_serial_jtag_rdwr_byte",
                &format_args!("{}", self.usb_serial_jtag_rdwr_byte().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EP1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "FIFO access for the CDC-ACM data IN and OUT endpoints.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP1_SPEC;
impl crate::RegisterSpec for EP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep1::R`](R) reader structure"]
impl crate::Readable for EP1_SPEC {}
#[doc = "`reset()` method sets EP1 to value 0"]
impl crate::Resettable for EP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
