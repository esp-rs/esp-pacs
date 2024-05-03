#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Field `DATE` reader - SPI register version."]
pub type DATE_R = crate::FieldReader<u32>;
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
            .field("date", &self.date().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`reset()` method sets DATE to value 0x0160_4270"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0160_4270;
}
