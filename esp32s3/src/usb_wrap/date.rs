///Register `DATE` reader
pub type R = crate::R<DATE_SPEC>;
///Register `DATE` writer
pub type W = crate::W<DATE_SPEC>;
///Field `USB_WRAP_DATE` reader - Date register
pub type USB_WRAP_DATE_R = crate::FieldReader<u32>;
///Field `USB_WRAP_DATE` writer - Date register
pub type USB_WRAP_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Date register
    #[inline(always)]
    pub fn usb_wrap_date(&self) -> USB_WRAP_DATE_R {
        USB_WRAP_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("usb_wrap_date", &self.usb_wrap_date())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Date register
    #[inline(always)]
    #[must_use]
    pub fn usb_wrap_date(&mut self) -> USB_WRAP_DATE_W<DATE_SPEC> {
        USB_WRAP_DATE_W::new(self, 0)
    }
}
/**Version Control Register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`date::R`](R) reader structure
impl crate::Readable for DATE_SPEC {}
///`write(|w| ..)` method takes [`date::W`](W) writer structure
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATE to value 0x0210_2010
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0210_2010;
}
