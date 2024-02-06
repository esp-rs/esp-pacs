#[doc = "Register `RD_USR_DATA%s` reader"]
pub type R = crate::R<RD_USR_DATA_SPEC>;
#[doc = "Field `USR_DATA` reader - Stores the %sth 32 bits of BLOCK3 (user)."]
pub type USR_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32 bits of BLOCK3 (user)."]
    #[inline(always)]
    pub fn usr_data(&self) -> USR_DATA_R {
        USR_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_USR_DATA")
            .field("usr_data", &format_args!("{}", self.usr_data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_USR_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register %s of BLOCK3 (user).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_USR_DATA_SPEC;
impl crate::RegisterSpec for RD_USR_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_usr_data::R`](R) reader structure"]
impl crate::Readable for RD_USR_DATA_SPEC {}
#[doc = "`reset()` method sets RD_USR_DATA%s to value 0"]
impl crate::Resettable for RD_USR_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
