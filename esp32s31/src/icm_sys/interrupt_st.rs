#[doc = "Register `INTERRUPT_ST` reader"]
pub type R = crate::R<INTERRUPT_ST_SPEC>;
#[doc = "Field `REG_DEC_FAILURE_INT_ST` reader - "]
pub type REG_DEC_FAILURE_INT_ST_R = crate::BitReader;
#[doc = "Field `REG_TIMEOUT_INT_ST` reader - "]
pub type REG_TIMEOUT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dec_failure_int_st(&self) -> REG_DEC_FAILURE_INT_ST_R {
        REG_DEC_FAILURE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_timeout_int_st(&self) -> REG_TIMEOUT_INT_ST_R {
        REG_TIMEOUT_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_ST")
            .field("reg_dec_failure_int_st", &self.reg_dec_failure_int_st())
            .field("reg_timeout_int_st", &self.reg_timeout_int_st())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_ST_SPEC;
impl crate::RegisterSpec for INTERRUPT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_st::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_ST_SPEC {}
#[doc = "`reset()` method sets INTERRUPT_ST to value 0"]
impl crate::Resettable for INTERRUPT_ST_SPEC {}
