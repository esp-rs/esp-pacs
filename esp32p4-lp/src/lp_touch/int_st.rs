#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SCAN_DONE_INT_ST` reader - need_des"]
pub type SCAN_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `DONE_INT_ST` reader - need_des"]
pub type DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `ACTIVE_INT_ST` reader - need_des"]
pub type ACTIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `INACTIVE_INT_ST` reader - need_des"]
pub type INACTIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `TIMEOUT_INT_ST` reader - need_des"]
pub type TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `APPROACH_LOOP_DONE_INT_ST` reader - need_des"]
pub type APPROACH_LOOP_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `BASELINE_UPDATE_INT_ST` reader - need_des"]
pub type BASELINE_UPDATE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn scan_done_int_st(&self) -> SCAN_DONE_INT_ST_R {
        SCAN_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn done_int_st(&self) -> DONE_INT_ST_R {
        DONE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn active_int_st(&self) -> ACTIVE_INT_ST_R {
        ACTIVE_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn inactive_int_st(&self) -> INACTIVE_INT_ST_R {
        INACTIVE_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn timeout_int_st(&self) -> TIMEOUT_INT_ST_R {
        TIMEOUT_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn approach_loop_done_int_st(&self) -> APPROACH_LOOP_DONE_INT_ST_R {
        APPROACH_LOOP_DONE_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn baseline_update_int_st(&self) -> BASELINE_UPDATE_INT_ST_R {
        BASELINE_UPDATE_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("scan_done_int_st", &self.scan_done_int_st())
            .field("done_int_st", &self.done_int_st())
            .field("active_int_st", &self.active_int_st())
            .field("inactive_int_st", &self.inactive_int_st())
            .field("timeout_int_st", &self.timeout_int_st())
            .field(
                "approach_loop_done_int_st",
                &self.approach_loop_done_int_st(),
            )
            .field("baseline_update_int_st", &self.baseline_update_int_st())
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
