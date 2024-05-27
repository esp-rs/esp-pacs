#[doc = "Register `TIMER5` reader"]
pub type R = crate::R<TIMER5_SPEC>;
#[doc = "Register `TIMER5` writer"]
pub type W = crate::W<TIMER5_SPEC>;
#[doc = "Field `ULP_CP_SUBTIMER_PREDIV` reader - "]
pub type ULP_CP_SUBTIMER_PREDIV_R = crate::FieldReader;
#[doc = "Field `ULP_CP_SUBTIMER_PREDIV` writer - "]
pub type ULP_CP_SUBTIMER_PREDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MIN_SLP_VAL` reader - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_R = crate::FieldReader;
#[doc = "Field `MIN_SLP_VAL` writer - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RTCMEM_WAIT_TIMER` reader - "]
pub type RTCMEM_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `RTCMEM_WAIT_TIMER` writer - "]
pub type RTCMEM_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RTCMEM_POWERUP_TIMER` reader - "]
pub type RTCMEM_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `RTCMEM_POWERUP_TIMER` writer - "]
pub type RTCMEM_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ulp_cp_subtimer_prediv(&self) -> ULP_CP_SUBTIMER_PREDIV_R {
        ULP_CP_SUBTIMER_PREDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&self) -> MIN_SLP_VAL_R {
        MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtcmem_wait_timer(&self) -> RTCMEM_WAIT_TIMER_R {
        RTCMEM_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtcmem_powerup_timer(&self) -> RTCMEM_POWERUP_TIMER_R {
        RTCMEM_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER5")
            .field("ulp_cp_subtimer_prediv", &self.ulp_cp_subtimer_prediv())
            .field("min_slp_val", &self.min_slp_val())
            .field("rtcmem_wait_timer", &self.rtcmem_wait_timer())
            .field("rtcmem_powerup_timer", &self.rtcmem_powerup_timer())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_subtimer_prediv(&mut self) -> ULP_CP_SUBTIMER_PREDIV_W<TIMER5_SPEC> {
        ULP_CP_SUBTIMER_PREDIV_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn min_slp_val(&mut self) -> MIN_SLP_VAL_W<TIMER5_SPEC> {
        MIN_SLP_VAL_W::new(self, 8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    #[must_use]
    pub fn rtcmem_wait_timer(&mut self) -> RTCMEM_WAIT_TIMER_W<TIMER5_SPEC> {
        RTCMEM_WAIT_TIMER_W::new(self, 16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    #[must_use]
    pub fn rtcmem_powerup_timer(&mut self) -> RTCMEM_POWERUP_TIMER_W<TIMER5_SPEC> {
        RTCMEM_POWERUP_TIMER_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER5_SPEC;
impl crate::RegisterSpec for TIMER5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer5::R`](R) reader structure"]
impl crate::Readable for TIMER5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer5::W`](W) writer structure"]
impl crate::Writable for TIMER5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER5 to value 0x1214_8001"]
impl crate::Resettable for TIMER5_SPEC {
    const RESET_VALUE: u32 = 0x1214_8001;
}
