#[doc = "Register `PCPU_INT` reader"]
pub type R = crate::R<PCPU_INT_SPEC>;
#[doc = "Field `PROCPU_INT` reader - GPIO0~31 PRO CPU interrupt status"]
pub type PROCPU_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 PRO CPU interrupt status"]
    #[inline(always)]
    pub fn procpu_int(&self) -> PROCPU_INT_R {
        PROCPU_INT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCPU_INT")
            .field("procpu_int", &self.procpu_int())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCPU_INT_SPEC;
impl crate::RegisterSpec for PCPU_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcpu_int::R`](R) reader structure"]
impl crate::Readable for PCPU_INT_SPEC {}
#[doc = "`reset()` method sets PCPU_INT to value 0"]
impl crate::Resettable for PCPU_INT_SPEC {
    const RESET_VALUE: u32 = 0;
}
