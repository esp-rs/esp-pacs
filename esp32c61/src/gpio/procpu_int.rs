#[doc = "Register `PROCPU_INT` reader"]
pub type R = crate::R<PROCPU_INT_SPEC>;
#[doc = "Field `PROCPU_INT` reader - GPIO_PROCPU_INT interrupt status register for GPIO0-31"]
pub type PROCPU_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO_PROCPU_INT interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub fn procpu_int(&self) -> PROCPU_INT_R {
        PROCPU_INT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PROCPU_INT")
            .field("procpu_int", &self.procpu_int())
            .finish()
    }
}
#[doc = "GPIO_PROCPU_INT interrupt status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_int::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROCPU_INT_SPEC;
impl crate::RegisterSpec for PROCPU_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procpu_int::R`](R) reader structure"]
impl crate::Readable for PROCPU_INT_SPEC {}
#[doc = "`reset()` method sets PROCPU_INT to value 0"]
impl crate::Resettable for PROCPU_INT_SPEC {}
