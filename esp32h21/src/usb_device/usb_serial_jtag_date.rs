#[doc = "Register `USB_SERIAL_JTAG_DATE` reader"]
pub type R = crate::R<USB_SERIAL_JTAG_DATE_SPEC>;
#[doc = "Register `USB_SERIAL_JTAG_DATE` writer"]
pub type W = crate::W<USB_SERIAL_JTAG_DATE_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_DATE` reader - register version."]
pub type USB_SERIAL_JTAG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `USB_SERIAL_JTAG_DATE` writer - register version."]
pub type USB_SERIAL_JTAG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - register version."]
    #[inline(always)]
    pub fn usb_serial_jtag_date(&self) -> USB_SERIAL_JTAG_DATE_R {
        USB_SERIAL_JTAG_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_SERIAL_JTAG_DATE")
            .field("usb_serial_jtag_date", &self.usb_serial_jtag_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - register version."]
    #[inline(always)]
    pub fn usb_serial_jtag_date(
        &mut self,
    ) -> USB_SERIAL_JTAG_DATE_W<'_, USB_SERIAL_JTAG_DATE_SPEC> {
        USB_SERIAL_JTAG_DATE_W::new(self, 0)
    }
}
#[doc = "Date register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_SERIAL_JTAG_DATE_SPEC;
impl crate::RegisterSpec for USB_SERIAL_JTAG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_serial_jtag_date::R`](R) reader structure"]
impl crate::Readable for USB_SERIAL_JTAG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_serial_jtag_date::W`](W) writer structure"]
impl crate::Writable for USB_SERIAL_JTAG_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_SERIAL_JTAG_DATE to value 0x0240_7030"]
impl crate::Resettable for USB_SERIAL_JTAG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0240_7030;
}
