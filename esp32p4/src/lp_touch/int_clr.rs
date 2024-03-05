#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SCAN_DONE_INT_CLR` writer - need_des"]
pub type SCAN_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE_INT_CLR` writer - need_des"]
pub type DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_INT_CLR` writer - need_des"]
pub type ACTIVE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTIVE_INT_CLR` writer - need_des"]
pub type INACTIVE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_INT_CLR` writer - need_des"]
pub type TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPROACH_LOOP_DONE_INT_CLR` writer - need_des"]
pub type APPROACH_LOOP_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn scan_done_int_clr(&mut self) -> SCAN_DONE_INT_CLR_W<INT_CLR_SPEC> {
        SCAN_DONE_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn done_int_clr(&mut self) -> DONE_INT_CLR_W<INT_CLR_SPEC> {
        DONE_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_int_clr(&mut self) -> ACTIVE_INT_CLR_W<INT_CLR_SPEC> {
        ACTIVE_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn inactive_int_clr(&mut self) -> INACTIVE_INT_CLR_W<INT_CLR_SPEC> {
        INACTIVE_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_int_clr(&mut self) -> TIMEOUT_INT_CLR_W<INT_CLR_SPEC> {
        TIMEOUT_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn approach_loop_done_int_clr(&mut self) -> APPROACH_LOOP_DONE_INT_CLR_W<INT_CLR_SPEC> {
        APPROACH_LOOP_DONE_INT_CLR_W::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
