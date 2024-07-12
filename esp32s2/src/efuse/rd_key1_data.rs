#[doc = "Register `RD_KEY1_DATA%s` reader"]
pub type R = crate::R<RD_KEY1_DATA_SPEC>;
#[doc = "Field `KEY1_DATA` reader - Stores the %sth 32 bits of KEY1."]
pub type KEY1_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32 bits of KEY1."]
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
#[doc = "Register %s of BLOCK5 (KEY1).\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key1_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY1_DATA_SPEC;
impl crate::RegisterSpec for RD_KEY1_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key1_data::R`](R) reader structure"]
impl crate::Readable for RD_KEY1_DATA_SPEC {}
#[doc = "`reset()` method sets RD_KEY1_DATA%s to value 0"]
impl crate::Resettable for RD_KEY1_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
