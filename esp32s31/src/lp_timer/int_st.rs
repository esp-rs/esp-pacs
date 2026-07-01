#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TICK` reader - need_des"]
pub type TICK_R = crate::BitReader;
#[doc = "Field `UPDTAE_DONE` reader - need_des"]
pub type UPDTAE_DONE_R = crate::BitReader;
#[doc = "Field `OVERFLOW` reader - need_des"]
pub type OVERFLOW_R = crate::BitReader;
#[doc = "Field `SOC_WAKEUP` reader - need_des"]
pub type SOC_WAKEUP_R = crate::BitReader;
impl R {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tick(&self) -> TICK_R {
        TICK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn updtae_done(&self) -> UPDTAE_DONE_R {
        UPDTAE_DONE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup(&self) -> SOC_WAKEUP_R {
        SOC_WAKEUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("tick", &self.tick())
            .field("updtae_done", &self.updtae_done())
            .field("overflow", &self.overflow())
            .field("soc_wakeup", &self.soc_wakeup())
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
