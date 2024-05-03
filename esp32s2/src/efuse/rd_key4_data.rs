#[doc = "Register `RD_KEY4_DATA%s` reader"]
pub type R = crate::R<RD_KEY4_DATA_SPEC>;
#[doc = "Field `KEY4_DATA` reader - Stores the %sth 32 bits of KEY4."]
pub type KEY4_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32 bits of KEY4."]
    #[inline(always)]
    pub fn key4_data(&self) -> KEY4_DATA_R {
        KEY4_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY4_DATA")
            .field("key4_data", &self.key4_data().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY4_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register %s of BLOCK8 (KEY4).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY4_DATA_SPEC;
impl crate::RegisterSpec for RD_KEY4_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key4_data::R`](R) reader structure"]
impl crate::Readable for RD_KEY4_DATA_SPEC {}
#[doc = "`reset()` method sets RD_KEY4_DATA%s to value 0"]
impl crate::Resettable for RD_KEY4_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
