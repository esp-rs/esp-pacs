#[doc = "Register `T0_MEM[%s]` reader"]
pub type R = crate::R<T0_MEM_SPEC>;
#[doc = "Field `T0` reader - This register stores the %sth 32-bit piece of 128-bit T0"]
pub type T0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit T0"]
    #[inline(always)]
    pub fn t0(&self) -> T0_R {
        T0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0_MEM").field("t0", &self.t0()).finish()
    }
}
#[doc = "T0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_mem::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0_MEM_SPEC;
impl crate::RegisterSpec for T0_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0_mem::R`](R) reader structure"]
impl crate::Readable for T0_MEM_SPEC {}
#[doc = "`reset()` method sets T0_MEM[%s] to value 0"]
impl crate::Resettable for T0_MEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
