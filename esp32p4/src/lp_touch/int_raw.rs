#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `SCAN_DONE_INT_RAW` reader - need_des"]
pub type SCAN_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SCAN_DONE_INT_RAW` writer - need_des"]
pub type SCAN_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE_INT_RAW` reader - need_des"]
pub type DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `DONE_INT_RAW` writer - need_des"]
pub type DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_INT_RAW` reader - need_des"]
pub type ACTIVE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ACTIVE_INT_RAW` writer - need_des"]
pub type ACTIVE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTIVE_INT_RAW` reader - need_des"]
pub type INACTIVE_INT_RAW_R = crate::BitReader;
#[doc = "Field `INACTIVE_INT_RAW` writer - need_des"]
pub type INACTIVE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_INT_RAW` reader - need_des"]
pub type TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMEOUT_INT_RAW` writer - need_des"]
pub type TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPROACH_LOOP_DONE_INT_RAW` reader - need_des"]
pub type APPROACH_LOOP_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `APPROACH_LOOP_DONE_INT_RAW` writer - need_des"]
pub type APPROACH_LOOP_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn scan_done_int_raw(&self) -> SCAN_DONE_INT_RAW_R {
        SCAN_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn done_int_raw(&self) -> DONE_INT_RAW_R {
        DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn active_int_raw(&self) -> ACTIVE_INT_RAW_R {
        ACTIVE_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn inactive_int_raw(&self) -> INACTIVE_INT_RAW_R {
        INACTIVE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn timeout_int_raw(&self) -> TIMEOUT_INT_RAW_R {
        TIMEOUT_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn approach_loop_done_int_raw(&self) -> APPROACH_LOOP_DONE_INT_RAW_R {
        APPROACH_LOOP_DONE_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "scan_done_int_raw",
                &format_args!("{}", self.scan_done_int_raw().bit()),
            )
            .field(
                "done_int_raw",
                &format_args!("{}", self.done_int_raw().bit()),
            )
            .field(
                "active_int_raw",
                &format_args!("{}", self.active_int_raw().bit()),
            )
            .field(
                "inactive_int_raw",
                &format_args!("{}", self.inactive_int_raw().bit()),
            )
            .field(
                "timeout_int_raw",
                &format_args!("{}", self.timeout_int_raw().bit()),
            )
            .field(
                "approach_loop_done_int_raw",
                &format_args!("{}", self.approach_loop_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn scan_done_int_raw(&mut self) -> SCAN_DONE_INT_RAW_W<INT_RAW_SPEC> {
        SCAN_DONE_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn done_int_raw(&mut self) -> DONE_INT_RAW_W<INT_RAW_SPEC> {
        DONE_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_int_raw(&mut self) -> ACTIVE_INT_RAW_W<INT_RAW_SPEC> {
        ACTIVE_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn inactive_int_raw(&mut self) -> INACTIVE_INT_RAW_W<INT_RAW_SPEC> {
        INACTIVE_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_int_raw(&mut self) -> TIMEOUT_INT_RAW_W<INT_RAW_SPEC> {
        TIMEOUT_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn approach_loop_done_int_raw(&mut self) -> APPROACH_LOOP_DONE_INT_RAW_W<INT_RAW_SPEC> {
        APPROACH_LOOP_DONE_INT_RAW_W::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
