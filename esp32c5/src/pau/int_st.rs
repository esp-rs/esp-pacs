#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `DONE_INT_ST` reader - backup done flag"]
pub type DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `ERROR_INT_ST` reader - error flag"]
pub type ERROR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - backup done flag"]
    #[inline(always)]
    pub fn done_int_st(&self) -> DONE_INT_ST_R {
        DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - error flag"]
    #[inline(always)]
    pub fn error_int_st(&self) -> ERROR_INT_ST_R {
        ERROR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("done_int_st", &self.done_int_st())
            .field("error_int_st", &self.error_int_st())
            .finish()
    }
}
#[doc = "Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
