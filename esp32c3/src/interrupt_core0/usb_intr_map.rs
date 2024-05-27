///Register `USB_INTR_MAP` reader
pub type R = crate::R<USB_INTR_MAP_SPEC>;
///Register `USB_INTR_MAP` writer
pub type W = crate::W<USB_INTR_MAP_SPEC>;
///Field `USB_INTR_MAP` reader - reg_core0_usb_intr_map
pub type USB_INTR_MAP_R = crate::FieldReader;
///Field `USB_INTR_MAP` writer - reg_core0_usb_intr_map
pub type USB_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - reg_core0_usb_intr_map
    #[inline(always)]
    pub fn usb_intr_map(&self) -> USB_INTR_MAP_R {
        USB_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_INTR_MAP")
            .field("usb_intr_map", &self.usb_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - reg_core0_usb_intr_map
    #[inline(always)]
    #[must_use]
    pub fn usb_intr_map(&mut self) -> USB_INTR_MAP_W<USB_INTR_MAP_SPEC> {
        USB_INTR_MAP_W::new(self, 0)
    }
}
/**usb intr map register

You can [`read`](crate::generic::Reg::read) this register and get [`usb_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USB_INTR_MAP_SPEC;
impl crate::RegisterSpec for USB_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`usb_intr_map::R`](R) reader structure
impl crate::Readable for USB_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`usb_intr_map::W`](W) writer structure
impl crate::Writable for USB_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USB_INTR_MAP to value 0
impl crate::Resettable for USB_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
