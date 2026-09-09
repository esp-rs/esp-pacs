#[doc = "Register `DCMEM_VALID_1` reader"]
pub type R = crate::R<DCMEM_VALID_1_SPEC>;
#[doc = "Field `DCMEM_VALID_1` reader - "]
pub type DCMEM_VALID_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dcmem_valid_1(&self) -> DCMEM_VALID_1_R {
        DCMEM_VALID_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCMEM_VALID_1")
            .field("dcmem_valid_1", &self.dcmem_valid_1())
            .finish()
    }
}
#[doc = "DCMEM_VALID_1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcmem_valid_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCMEM_VALID_1_SPEC;
impl crate::RegisterSpec for DCMEM_VALID_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmem_valid_1::R`](R) reader structure"]
impl crate::Readable for DCMEM_VALID_1_SPEC {}
#[doc = "`reset()` method sets DCMEM_VALID_1 to value 0"]
impl crate::Resettable for DCMEM_VALID_1_SPEC {}
