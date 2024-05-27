#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `COCPU_TSENS_WAKE` reader - Tsens wakeup interrupt status."]
pub type COCPU_TSENS_WAKE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Tsens wakeup interrupt status."]
    #[inline(always)]
    pub fn cocpu_tsens_wake(&self) -> COCPU_TSENS_WAKE_R {
        COCPU_TSENS_WAKE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("cocpu_tsens_wake", &self.cocpu_tsens_wake())
            .finish()
    }
}
#[doc = "Tsens interrupt status registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
