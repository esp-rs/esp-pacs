#[doc = "Register `DATE` reader"]
pub struct R(crate::R<DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATE` reader - SPI register version."]
pub type DATE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:27 - SPI register version."]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("date", &format_args!("{}", self.date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](index.html) module"]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [date::R](R) reader structure"]
impl crate::Readable for DATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATE to value 0x0160_4270"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0160_4270;
}
