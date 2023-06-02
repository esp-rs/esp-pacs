#[doc = "Register `USB_INTR_MAP` reader"]
pub struct R(crate::R<USB_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_INTR_MAP` writer"]
pub struct W(crate::W<USB_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USB_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_INTR_MAP` reader - this register used to map usb interrupt to one of core0's external interrupt"]
pub type USB_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `USB_INTR_MAP` writer - this register used to map usb interrupt to one of core0's external interrupt"]
pub type USB_INTR_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, USB_INTR_MAP_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map usb interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn usb_intr_map(&self) -> USB_INTR_MAP_R {
        USB_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_INTR_MAP")
            .field(
                "usb_intr_map",
                &format_args!("{}", self.usb_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USB_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map usb interrupt to one of core0's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn usb_intr_map(&mut self) -> USB_INTR_MAP_W<0> {
        USB_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_intr_map](index.html) module"]
pub struct USB_INTR_MAP_SPEC;
impl crate::RegisterSpec for USB_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_intr_map::R](R) reader structure"]
impl crate::Readable for USB_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_intr_map::W](W) writer structure"]
impl crate::Writable for USB_INTR_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_INTR_MAP to value 0x10"]
impl crate::Resettable for USB_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
