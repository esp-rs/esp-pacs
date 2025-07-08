#[doc = "Register `SDIO_INT` reader"]
pub type R = crate::R<SDIO_INT_SPEC>;
#[doc = "Field `SDIO_INT` reader - GPIO_SDIO_INT interrupt status register for GPIO0-31"]
pub type SDIO_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO_SDIO_INT interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub fn sdio_int(&self) -> SDIO_INT_R {
        SDIO_INT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_INT")
            .field("sdio_int", &self.sdio_int())
            .finish()
    }
}
#[doc = "GPIO_SDIO_INT interrupt status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_int::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_INT_SPEC;
impl crate::RegisterSpec for SDIO_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_int::R`](R) reader structure"]
impl crate::Readable for SDIO_INT_SPEC {}
#[doc = "`reset()` method sets SDIO_INT to value 0"]
impl crate::Resettable for SDIO_INT_SPEC {}
