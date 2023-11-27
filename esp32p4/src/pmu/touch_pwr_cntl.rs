#[doc = "Register `TOUCH_PWR_CNTL` reader"]
pub type R = crate::R<TOUCH_PWR_CNTL_SPEC>;
#[doc = "Register `TOUCH_PWR_CNTL` writer"]
pub type W = crate::W<TOUCH_PWR_CNTL_SPEC>;
#[doc = "Field `TOUCH_WAIT_CYCLES` reader - need_des"]
pub type TOUCH_WAIT_CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_WAIT_CYCLES` writer - need_des"]
pub type TOUCH_WAIT_CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` reader - need_des"]
pub type TOUCH_SLEEP_CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` writer - need_des"]
pub type TOUCH_SLEEP_CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_FORCE_DONE` reader - need_des"]
pub type TOUCH_FORCE_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_FORCE_DONE` writer - need_des"]
pub type TOUCH_FORCE_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SLEEP_TIMER_EN` reader - need_des"]
pub type TOUCH_SLEEP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SLEEP_TIMER_EN` writer - need_des"]
pub type TOUCH_SLEEP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    pub fn touch_wait_cycles(&self) -> TOUCH_WAIT_CYCLES_R {
        TOUCH_WAIT_CYCLES_R::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    #[doc = "Bits 14:29 - need_des"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&self) -> TOUCH_SLEEP_CYCLES_R {
        TOUCH_SLEEP_CYCLES_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn touch_force_done(&self) -> TOUCH_FORCE_DONE_R {
        TOUCH_FORCE_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn touch_sleep_timer_en(&self) -> TOUCH_SLEEP_TIMER_EN_R {
        TOUCH_SLEEP_TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PWR_CNTL")
            .field(
                "touch_wait_cycles",
                &format_args!("{}", self.touch_wait_cycles().bits()),
            )
            .field(
                "touch_sleep_cycles",
                &format_args!("{}", self.touch_sleep_cycles().bits()),
            )
            .field(
                "touch_force_done",
                &format_args!("{}", self.touch_force_done().bit()),
            )
            .field(
                "touch_sleep_timer_en",
                &format_args!("{}", self.touch_sleep_timer_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_PWR_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_wait_cycles(&mut self) -> TOUCH_WAIT_CYCLES_W<TOUCH_PWR_CNTL_SPEC> {
        TOUCH_WAIT_CYCLES_W::new(self, 5)
    }
    #[doc = "Bits 14:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_sleep_cycles(&mut self) -> TOUCH_SLEEP_CYCLES_W<TOUCH_PWR_CNTL_SPEC> {
        TOUCH_SLEEP_CYCLES_W::new(self, 14)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_force_done(&mut self) -> TOUCH_FORCE_DONE_W<TOUCH_PWR_CNTL_SPEC> {
        TOUCH_FORCE_DONE_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_sleep_timer_en(&mut self) -> TOUCH_SLEEP_TIMER_EN_W<TOUCH_PWR_CNTL_SPEC> {
        TOUCH_SLEEP_TIMER_EN_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pwr_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pwr_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_PWR_CNTL_SPEC;
impl crate::RegisterSpec for TOUCH_PWR_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_pwr_cntl::R`](R) reader structure"]
impl crate::Readable for TOUCH_PWR_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_pwr_cntl::W`](W) writer structure"]
impl crate::Writable for TOUCH_PWR_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_PWR_CNTL to value 0x0019_0140"]
impl crate::Resettable for TOUCH_PWR_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0019_0140;
}
