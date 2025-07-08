#[doc = "Register `SDIO_INT1` reader"]
pub type R = crate::R<SDIO_INT1_SPEC>;
#[doc = "Field `SDIO_INT1` reader - GPIO_SDIO_INT interrupt status register for GPIO32-33"]
pub type SDIO_INT1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - GPIO_SDIO_INT interrupt status register for GPIO32-33"]
    #[inline(always)]
    pub fn sdio_int1(&self) -> SDIO_INT1_R {
        SDIO_INT1_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_INT1")
            .field("sdio_int1", &self.sdio_int1())
            .finish()
    }
}
#[doc = "GPIO_SDIO_INT interrupt status register for GPIO32-33\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_int1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_INT1_SPEC;
impl crate::RegisterSpec for SDIO_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_int1::R`](R) reader structure"]
impl crate::Readable for SDIO_INT1_SPEC {}
#[doc = "`reset()` method sets SDIO_INT1 to value 0"]
impl crate::Resettable for SDIO_INT1_SPEC {}
