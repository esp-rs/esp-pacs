#[doc = "Register `PROCPU_INT1` reader"]
pub type R = crate::R<PROCPU_INT1_SPEC>;
#[doc = "Field `PROCPU_INT1` reader - GPIO_PROCPU_INT interrupt status register for GPIO32-33"]
pub type PROCPU_INT1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - GPIO_PROCPU_INT interrupt status register for GPIO32-33"]
    #[inline(always)]
    pub fn procpu_int1(&self) -> PROCPU_INT1_R {
        PROCPU_INT1_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PROCPU_INT1")
            .field("procpu_int1", &self.procpu_int1())
            .finish()
    }
}
#[doc = "GPIO_PROCPU_INT interrupt status register for GPIO32-33\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_int1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROCPU_INT1_SPEC;
impl crate::RegisterSpec for PROCPU_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procpu_int1::R`](R) reader structure"]
impl crate::Readable for PROCPU_INT1_SPEC {}
#[doc = "`reset()` method sets PROCPU_INT1 to value 0"]
impl crate::Resettable for PROCPU_INT1_SPEC {}
