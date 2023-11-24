#[doc = "Register `RD_KEY2_DATA%s` reader"]
pub type R = crate::R<RD_KEY2_DATA_SPEC>;
#[doc = "Field `KEY2_DATA` reader - Stores the %sth 32 bits of KEY2."]
pub type KEY2_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32 bits of KEY2."]
    #[inline(always)]
    pub fn key2_data(&self) -> KEY2_DATA_R {
        KEY2_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY2_DATA")
            .field("key2_data", &format_args!("{}", self.key2_data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY2_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register %s of BLOCK6 (KEY2).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY2_DATA_SPEC;
impl crate::RegisterSpec for RD_KEY2_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key2_data::R`](R) reader structure"]
impl crate::Readable for RD_KEY2_DATA_SPEC {}
#[doc = "`reset()` method sets RD_KEY2_DATA%s to value 0"]
impl crate::Resettable for RD_KEY2_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
