#[doc = "Register `ACPU_INT1` reader"]
pub type R = crate::R<ACPU_INT1_SPEC>;
#[doc = "Field `APPCPU_INT_H` reader - GPIO32~39 APP CPU interrupt status"]
pub type APPCPU_INT_H_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 APP CPU interrupt status"]
    #[inline(always)]
    pub fn appcpu_int_h(&self) -> APPCPU_INT_H_R {
        APPCPU_INT_H_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACPU_INT1")
            .field("appcpu_int_h", &self.appcpu_int_h())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_int1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACPU_INT1_SPEC;
impl crate::RegisterSpec for ACPU_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acpu_int1::R`](R) reader structure"]
impl crate::Readable for ACPU_INT1_SPEC {}
#[doc = "`reset()` method sets ACPU_INT1 to value 0"]
impl crate::Resettable for ACPU_INT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
