#[doc = "Register `CPUSDIO_INT1` reader"]
pub struct R(crate::R<CPUSDIO_INT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUSDIO_INT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUSDIO_INT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUSDIO_INT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDIO_INT1` reader - GPIO CPUSDIO interrupt status register for GPIO32-34"]
pub type SDIO_INT1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - GPIO CPUSDIO interrupt status register for GPIO32-34"]
    #[inline(always)]
    pub fn sdio_int1(&self) -> SDIO_INT1_R {
        SDIO_INT1_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUSDIO_INT1")
            .field("sdio_int1", &format_args!("{}", self.sdio_int1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPUSDIO_INT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GPIO CPUSDIO interrupt status register for GPIO32-34\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpusdio_int1](index.html) module"]
pub struct CPUSDIO_INT1_SPEC;
impl crate::RegisterSpec for CPUSDIO_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpusdio_int1::R](R) reader structure"]
impl crate::Readable for CPUSDIO_INT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPUSDIO_INT1 to value 0"]
impl crate::Resettable for CPUSDIO_INT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
