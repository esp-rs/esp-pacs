#[doc = "Register `EP1_CONF` reader"]
pub type R = crate::R<EP1_CONF_SPEC>;
#[doc = "Register `EP1_CONF` writer"]
pub type W = crate::W<EP1_CONF_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_WR_DONE` writer - Set this bit to indicate writing byte data to UART Tx FIFO is done."]
pub type USB_SERIAL_JTAG_WR_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE` reader - 1'b1: Indicate UART Tx FIFO is not full and can write data into in. After writing USB_SERIAL_JTAG_WR_DONE, this bit would be 0 until data in UART Tx FIFO is read by USB Host."]
pub type USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL` reader - 1'b1: Indicate there is data in UART Rx FIFO."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - 1'b1: Indicate UART Tx FIFO is not full and can write data into in. After writing USB_SERIAL_JTAG_WR_DONE, this bit would be 0 until data in UART Tx FIFO is read by USB Host."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_ep_data_free(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE_R {
        USB_SERIAL_JTAG_SERIAL_IN_EP_DATA_FREE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1'b1: Indicate there is data in UART Rx FIFO."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_ep_data_avail(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL_R {
        USB_SERIAL_JTAG_SERIAL_OUT_EP_DATA_AVAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EP1_CONF")
            .field(
                "usb_serial_jtag_serial_in_ep_data_free",
                &self.usb_serial_jtag_serial_in_ep_data_free(),
            )
            .field(
                "usb_serial_jtag_serial_out_ep_data_avail",
                &self.usb_serial_jtag_serial_out_ep_data_avail(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to indicate writing byte data to UART Tx FIFO is done."]
    #[inline(always)]
    pub fn usb_serial_jtag_wr_done(&mut self) -> USB_SERIAL_JTAG_WR_DONE_W<'_, EP1_CONF_SPEC> {
        USB_SERIAL_JTAG_WR_DONE_W::new(self, 0)
    }
}
#[doc = "Configuration and control registers for the CDC-ACM FIFOs.\n\nYou can [`read`](crate::Reg::read) this register and get [`ep1_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP1_CONF_SPEC;
impl crate::RegisterSpec for EP1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep1_conf::R`](R) reader structure"]
impl crate::Readable for EP1_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep1_conf::W`](W) writer structure"]
impl crate::Writable for EP1_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP1_CONF to value 0x02"]
impl crate::Resettable for EP1_CONF_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
