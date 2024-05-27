///Register `EP1` reader
pub type R = crate::R<EP1_SPEC>;
///Register `EP1` writer
pub type W = crate::W<EP1_SPEC>;
///Field `RDWR_BYTE` reader - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO.
pub type RDWR_BYTE_R = crate::FieldReader;
///Field `RDWR_BYTE` writer - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO.
pub type RDWR_BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO.
    #[inline(always)]
    pub fn rdwr_byte(&self) -> RDWR_BYTE_R {
        RDWR_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EP1")
            .field("rdwr_byte", &self.rdwr_byte())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO.
    #[inline(always)]
    #[must_use]
    pub fn rdwr_byte(&mut self) -> RDWR_BYTE_W<EP1_SPEC> {
        RDWR_BYTE_W::new(self, 0)
    }
}
/**FIFO access for the CDC-ACM data IN and OUT endpoints.

You can [`read`](crate::generic::Reg::read) this register and get [`ep1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EP1_SPEC;
impl crate::RegisterSpec for EP1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ep1::R`](R) reader structure
impl crate::Readable for EP1_SPEC {}
///`write(|w| ..)` method takes [`ep1::W`](W) writer structure
impl crate::Writable for EP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EP1 to value 0
impl crate::Resettable for EP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
