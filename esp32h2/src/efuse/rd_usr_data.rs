#[doc = "Register `RD_USR_DATA%s` reader"]
pub type R = crate::R<RD_USR_DATA_SPEC>;
#[doc = "Field `DATA` reader - Stores the zeroth 32 bits of BLOCK3 (user)."]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the zeroth 32 bits of BLOCK3 (user)."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_USR_DATA")
            .field("data", &self.data())
            .finish()
    }
}
#[doc = "Register $n of BLOCK3 (user).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
