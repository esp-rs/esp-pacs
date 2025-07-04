#[doc = "Register `RD_KEY0_DATA%s` reader"]
pub type R = crate::R<RD_KEY0_DATA_SPEC>;
#[doc = "Field `KEY0_DATA` reader - Stores the %sth 32 bits of KEY0."]
pub type KEY0_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32 bits of KEY0."]
    #[inline(always)]
    pub fn key0_data(&self) -> KEY0_DATA_R {
        KEY0_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY0_DATA")
            .field("key0_data", &self.key0_data())
            .finish()
    }
}
#[doc = "Register %s of BLOCK4 (KEY0).\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key0_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY0_DATA_SPEC;
impl crate::RegisterSpec for RD_KEY0_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key0_data::R`](R) reader structure"]
impl crate::Readable for RD_KEY0_DATA_SPEC {}
#[doc = "`reset()` method sets RD_KEY0_DATA%s to value 0"]
impl crate::Resettable for RD_KEY0_DATA_SPEC {}
