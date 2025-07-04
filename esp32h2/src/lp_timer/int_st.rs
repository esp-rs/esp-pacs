#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `OVERFLOW_ST` reader - need_des"]
pub type OVERFLOW_ST_R = crate::BitReader;
#[doc = "Field `SOC_WAKEUP_INT_ST` reader - need_des"]
pub type SOC_WAKEUP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn overflow_st(&self) -> OVERFLOW_ST_R {
        OVERFLOW_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_st(&self) -> SOC_WAKEUP_INT_ST_R {
        SOC_WAKEUP_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("overflow_st", &self.overflow_st())
            .field("soc_wakeup_int_st", &self.soc_wakeup_int_st())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
