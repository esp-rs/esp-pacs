#[doc = "Register `DATA%s` reader"]
pub type R = crate::R<DATA_SPEC>;
#[doc = "Field `DATA` reader - Stores the zeroth 32 bits of KEY0."]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the zeroth 32 bits of KEY0."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA").field("data", &self.data()).finish()
    }
}
#[doc = "Register %s of BLOCK4 (KEY0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DATA_SPEC {}
#[doc = "`reset()` method sets DATA%s to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
