#[doc = "Register `RD_KEY1_DATA%s` reader"]
pub type R = crate::R<RD_KEY1_DATA_SPEC>;
#[doc = "Field `KEY1_DATA` reader - Represents the zeroth 32-bit of key1."]
pub type KEY1_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the zeroth 32-bit of key1."]
    #[inline(always)]
    pub fn key1_data(&self) -> KEY1_DATA_R {
        KEY1_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY1_DATA")
            .field("key1_data", &self.key1_data())
            .finish()
    }
}
#[doc = "Represents rd_key1_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key1_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY1_DATA_SPEC;
impl crate::RegisterSpec for RD_KEY1_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key1_data::R`](R) reader structure"]
impl crate::Readable for RD_KEY1_DATA_SPEC {}
#[doc = "`reset()` method sets RD_KEY1_DATA%s to value 0"]
impl crate::Resettable for RD_KEY1_DATA_SPEC {}
