#[doc = "Register `RD_USR_DATA4` reader"]
pub type R = crate::R<RD_USR_DATA4_SPEC>;
#[doc = "Field `USR_DATA4` reader - Stores the fourth 32 bits of BLOCK3 (user)."]
pub type USR_DATA4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the fourth 32 bits of BLOCK3 (user)."]
    #[inline(always)]
    pub fn usr_data4(&self) -> USR_DATA4_R {
        USR_DATA4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_USR_DATA4")
            .field("usr_data4", &format_args!("{}", self.usr_data4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_USR_DATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register 4 of BLOCK3 (user).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_USR_DATA4_SPEC;
impl crate::RegisterSpec for RD_USR_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_usr_data4::R`](R) reader structure"]
impl crate::Readable for RD_USR_DATA4_SPEC {}
#[doc = "`reset()` method sets RD_USR_DATA4 to value 0"]
impl crate::Resettable for RD_USR_DATA4_SPEC {
    const RESET_VALUE: u32 = 0;
}
