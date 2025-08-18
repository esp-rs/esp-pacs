#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `SCAN_DONE` reader - need_des"]
pub type SCAN_DONE_R = crate::BitReader;
#[doc = "Field `SCAN_DONE` writer - need_des"]
pub type SCAN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - need_des"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - need_des"]
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE` reader - need_des"]
pub type ACTIVE_R = crate::BitReader;
#[doc = "Field `ACTIVE` writer - need_des"]
pub type ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTIVE` reader - need_des"]
pub type INACTIVE_R = crate::BitReader;
#[doc = "Field `INACTIVE` writer - need_des"]
pub type INACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - need_des"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - need_des"]
pub type TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPROACH_LOOP_DONE` reader - need_des"]
pub type APPROACH_LOOP_DONE_R = crate::BitReader;
#[doc = "Field `APPROACH_LOOP_DONE` writer - need_des"]
pub type APPROACH_LOOP_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("INT_RAW")
            .field("scan_done", &self.scan_done())
            .field("done", &self.done())
            .field("active", &self.active())
            .field("inactive", &self.inactive())
            .field("timeout", &self.timeout())
            .field("approach_loop_done", &self.approach_loop_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn scan_done(&mut self) -> SCAN_DONE_W<'_, INT_RAW_SPEC> {
        SCAN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<'_, INT_RAW_SPEC> {
        DONE_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn active(&mut self) -> ACTIVE_W<'_, INT_RAW_SPEC> {
        ACTIVE_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn inactive(&mut self) -> INACTIVE_W<'_, INT_RAW_SPEC> {
        INACTIVE_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<'_, INT_RAW_SPEC> {
        TIMEOUT_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn approach_loop_done(&mut self) -> APPROACH_LOOP_DONE_W<'_, INT_RAW_SPEC> {
        APPROACH_LOOP_DONE_W::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
