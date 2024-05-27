#[doc = "Register `CLEAN` reader"]
pub type R = crate::R<CLEAN_SPEC>;
#[doc = "Field `CLEAN` reader - This bit will read 1 once the memory initialization is completed."]
pub type CLEAN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit will read 1 once the memory initialization is completed."]
    #[inline(always)]
    pub fn clean(&self) -> CLEAN_R {
        CLEAN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLEAN")
            .field("clean", &self.clean())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clean::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLEAN_SPEC;
impl crate::RegisterSpec for CLEAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clean::R`](R) reader structure"]
impl crate::Readable for CLEAN_SPEC {}
#[doc = "`reset()` method sets CLEAN to value 0"]
impl crate::Resettable for CLEAN_SPEC {
    const RESET_VALUE: u32 = 0;
}
