#[doc = "Register `RD_KEY_DATA%s` reader"]
pub type R = crate::R<RD_KEY_DATA_SPEC>;
#[doc = "Field `DATA` reader - Stores the %sth 32 bits of KEY0."]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32 bits of KEY0."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY_DATA")
            .field("data", &self.data())
            .finish()
    }
}
#[doc = "Register %s of BLOCK4 (KEY0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY_DATA_SPEC;
impl crate::RegisterSpec for RD_KEY_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key_data::R`](R) reader structure"]
impl crate::Readable for RD_KEY_DATA_SPEC {}
#[doc = "`reset()` method sets RD_KEY_DATA%s to value 0"]
impl crate::Resettable for RD_KEY_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
