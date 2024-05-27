#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SCAN_DONE` reader - need_des"]
pub type SCAN_DONE_R = crate::BitReader;
#[doc = "Field `DONE` reader - need_des"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `ACTIVE` reader - need_des"]
pub type ACTIVE_R = crate::BitReader;
#[doc = "Field `INACTIVE` reader - need_des"]
pub type INACTIVE_R = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - need_des"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `APPROACH_LOOP_DONE` reader - need_des"]
pub type APPROACH_LOOP_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn scan_done(&self) -> SCAN_DONE_R {
        SCAN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn inactive(&self) -> INACTIVE_R {
        INACTIVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn approach_loop_done(&self) -> APPROACH_LOOP_DONE_R {
        APPROACH_LOOP_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("scan_done", &self.scan_done())
            .field("done", &self.done())
            .field("active", &self.active())
            .field("inactive", &self.inactive())
            .field("timeout", &self.timeout())
            .field("approach_loop_done", &self.approach_loop_done())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
