#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Field `USB_WRAP_DATE` reader - Date register."]
pub type USB_WRAP_DATE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Date register."]
    #[inline(always)]
    pub fn usb_wrap_date(&self) -> USB_WRAP_DATE_R {
        USB_WRAP_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("usb_wrap_date", &self.usb_wrap_date().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Date register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`reset()` method sets DATE to value 0x2303_0504"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x2303_0504;
}
