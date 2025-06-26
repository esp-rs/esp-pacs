#[doc = "Register `CPUSDIO_INT1` reader"]
pub type R = crate::R<CPUSDIO_INT1_SPEC>;
#[doc = "Field `SDIO_INT1` reader - GPIO CPUSDIO interrupt status register for GPIO32-53"]
pub type SDIO_INT1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - GPIO CPUSDIO interrupt status register for GPIO32-53"]
    #[inline(always)]
    pub fn sdio_int1(&self) -> SDIO_INT1_R {
        SDIO_INT1_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUSDIO_INT1")
            .field("sdio_int1", &self.sdio_int1())
            .finish()
    }
}
#[doc = "GPIO CPUSDIO interrupt status register for GPIO32-53\n\nYou can [`read`](crate::Reg::read) this register and get [`cpusdio_int1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUSDIO_INT1_SPEC;
impl crate::RegisterSpec for CPUSDIO_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpusdio_int1::R`](R) reader structure"]
impl crate::Readable for CPUSDIO_INT1_SPEC {}
#[doc = "`reset()` method sets CPUSDIO_INT1 to value 0"]
impl crate::Resettable for CPUSDIO_INT1_SPEC {}
