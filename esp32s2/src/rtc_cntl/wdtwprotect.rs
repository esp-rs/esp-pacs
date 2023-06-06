#[doc = "Register `WDTWPROTECT` reader"]
pub struct R(crate::R<WDTWPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTWPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTWPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTWPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTWPROTECT` writer"]
pub struct W(crate::W<WDTWPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTWPROTECT_SPEC>;
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
impl From<crate::W<WDTWPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTWPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_WKEY` reader - Sets the write protection key of the watchdog."]
pub type WDT_WKEY_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_WKEY` writer - Sets the write protection key of the watchdog."]
pub type WDT_WKEY_W<'a, const O: u8> = crate::FieldWriter<'a, WDTWPROTECT_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Sets the write protection key of the watchdog."]
    #[inline(always)]
    pub fn wdt_wkey(&self) -> WDT_WKEY_R {
        WDT_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTWPROTECT")
            .field("wdt_wkey", &format_args!("{}", self.wdt_wkey().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTWPROTECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sets the write protection key of the watchdog."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_wkey(&mut self) -> WDT_WKEY_W<0> {
        WDT_WKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC watchdog write protection configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtwprotect](index.html) module"]
pub struct WDTWPROTECT_SPEC;
impl crate::RegisterSpec for WDTWPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtwprotect::R](R) reader structure"]
impl crate::Readable for WDTWPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtwprotect::W](W) writer structure"]
impl crate::Writable for WDTWPROTECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTWPROTECT to value 0x50d8_3aa1"]
impl crate::Resettable for WDTWPROTECT_SPEC {
    const RESET_VALUE: Self::Ux = 0x50d8_3aa1;
}
