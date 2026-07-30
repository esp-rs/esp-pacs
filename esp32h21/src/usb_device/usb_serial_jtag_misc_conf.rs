#[doc = "Register `USB_SERIAL_JTAG_MISC_CONF` reader"]
pub type R = crate::R<USB_SERIAL_JTAG_MISC_CONF_SPEC>;
#[doc = "Register `USB_SERIAL_JTAG_MISC_CONF` writer"]
pub type W = crate::W<USB_SERIAL_JTAG_MISC_CONF_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type USB_SERIAL_JTAG_CLK_EN_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type USB_SERIAL_JTAG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn usb_serial_jtag_clk_en(&self) -> USB_SERIAL_JTAG_CLK_EN_R {
        USB_SERIAL_JTAG_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_SERIAL_JTAG_MISC_CONF")
            .field("usb_serial_jtag_clk_en", &self.usb_serial_jtag_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn usb_serial_jtag_clk_en(
        &mut self,
    ) -> USB_SERIAL_JTAG_CLK_EN_W<'_, USB_SERIAL_JTAG_MISC_CONF_SPEC> {
        USB_SERIAL_JTAG_CLK_EN_W::new(self, 0)
    }
}
#[doc = "Clock enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_misc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_misc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_SERIAL_JTAG_MISC_CONF_SPEC;
impl crate::RegisterSpec for USB_SERIAL_JTAG_MISC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_serial_jtag_misc_conf::R`](R) reader structure"]
impl crate::Readable for USB_SERIAL_JTAG_MISC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_serial_jtag_misc_conf::W`](W) writer structure"]
impl crate::Writable for USB_SERIAL_JTAG_MISC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_SERIAL_JTAG_MISC_CONF to value 0"]
impl crate::Resettable for USB_SERIAL_JTAG_MISC_CONF_SPEC {}
