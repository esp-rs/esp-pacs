#[doc = "Register `TIME_UPDATE` reader"]
pub type R = crate::R<TIME_UPDATE_SPEC>;
#[doc = "Register `TIME_UPDATE` writer"]
pub type W = crate::W<TIME_UPDATE_SPEC>;
#[doc = "Field `TIMER_SYS_STALL` reader - Enable to record system stall time"]
pub type TIMER_SYS_STALL_R = crate::BitReader;
#[doc = "Field `TIMER_SYS_STALL` writer - Enable to record system stall time"]
pub type TIMER_SYS_STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_XTL_OFF` reader - Enable to record 40M XTAL OFF time"]
pub type TIMER_XTL_OFF_R = crate::BitReader;
#[doc = "Field `TIMER_XTL_OFF` writer - Enable to record 40M XTAL OFF time"]
pub type TIMER_XTL_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_SYS_RST` reader - enable to record system reset time"]
pub type TIMER_SYS_RST_R = crate::BitReader;
#[doc = "Field `TIMER_SYS_RST` writer - enable to record system reset time"]
pub type TIMER_SYS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_UPDATE` writer - Set 1: to update register with RTC timer"]
pub type TIME_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - Enable to record system stall time"]
    #[inline(always)]
    pub fn timer_sys_stall(&self) -> TIMER_SYS_STALL_R {
        TIMER_SYS_STALL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable to record 40M XTAL OFF time"]
    #[inline(always)]
    pub fn timer_xtl_off(&self) -> TIMER_XTL_OFF_R {
        TIMER_XTL_OFF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable to record system reset time"]
    #[inline(always)]
    pub fn timer_sys_rst(&self) -> TIMER_SYS_RST_R {
        TIMER_SYS_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_UPDATE")
            .field(
                "timer_sys_stall",
                &format_args!("{}", self.timer_sys_stall().bit()),
            )
            .field(
                "timer_xtl_off",
                &format_args!("{}", self.timer_xtl_off().bit()),
            )
            .field(
                "timer_sys_rst",
                &format_args!("{}", self.timer_sys_rst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIME_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 27 - Enable to record system stall time"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sys_stall(&mut self) -> TIMER_SYS_STALL_W<TIME_UPDATE_SPEC> {
        TIMER_SYS_STALL_W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable to record 40M XTAL OFF time"]
    #[inline(always)]
    #[must_use]
    pub fn timer_xtl_off(&mut self) -> TIMER_XTL_OFF_W<TIME_UPDATE_SPEC> {
        TIMER_XTL_OFF_W::new(self, 28)
    }
    #[doc = "Bit 29 - enable to record system reset time"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sys_rst(&mut self) -> TIMER_SYS_RST_W<TIME_UPDATE_SPEC> {
        TIMER_SYS_RST_W::new(self, 29)
    }
    #[doc = "Bit 31 - Set 1: to update register with RTC timer"]
    #[inline(always)]
    #[must_use]
    pub fn time_update(&mut self) -> TIME_UPDATE_W<TIME_UPDATE_SPEC> {
        TIME_UPDATE_W::new(self, 31)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_UPDATE_SPEC;
impl crate::RegisterSpec for TIME_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_update::R`](R) reader structure"]
impl crate::Readable for TIME_UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_update::W`](W) writer structure"]
impl crate::Writable for TIME_UPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME_UPDATE to value 0"]
impl crate::Resettable for TIME_UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
