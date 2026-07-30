#[doc = "Register `USB_SERIAL_JTAG_CONFIG_UPDATE` writer"]
pub type W = crate::W<USB_SERIAL_JTAG_CONFIG_UPDATE_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_CONFIG_UPDATE` writer - Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain."]
pub type USB_SERIAL_JTAG_CONFIG_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USB_SERIAL_JTAG_CONFIG_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_config_update(
        &mut self,
    ) -> USB_SERIAL_JTAG_CONFIG_UPDATE_W<'_, USB_SERIAL_JTAG_CONFIG_UPDATE_SPEC> {
        USB_SERIAL_JTAG_CONFIG_UPDATE_W::new(self, 0)
    }
}
#[doc = "Configuration registers' value update\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_config_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_SERIAL_JTAG_CONFIG_UPDATE_SPEC;
impl crate::RegisterSpec for USB_SERIAL_JTAG_CONFIG_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`usb_serial_jtag_config_update::W`](W) writer structure"]
impl crate::Writable for USB_SERIAL_JTAG_CONFIG_UPDATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_SERIAL_JTAG_CONFIG_UPDATE to value 0"]
impl crate::Resettable for USB_SERIAL_JTAG_CONFIG_UPDATE_SPEC {}
