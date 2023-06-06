#[doc = "Register `CPUSDIO_INT` reader"]
pub struct R(crate::R<CPUSDIO_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUSDIO_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUSDIO_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUSDIO_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDIO_INT` reader - GPIO CPUSDIO interrupt status register for GPIO0-25"]
pub type SDIO_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25 - GPIO CPUSDIO interrupt status register for GPIO0-25"]
    #[inline(always)]
    pub fn sdio_int(&self) -> SDIO_INT_R {
        SDIO_INT_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUSDIO_INT")
            .field("sdio_int", &format_args!("{}", self.sdio_int().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPUSDIO_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GPIO CPUSDIO interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpusdio_int](index.html) module"]
pub struct CPUSDIO_INT_SPEC;
impl crate::RegisterSpec for CPUSDIO_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpusdio_int::R](R) reader structure"]
impl crate::Readable for CPUSDIO_INT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPUSDIO_INT to value 0"]
impl crate::Resettable for CPUSDIO_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
