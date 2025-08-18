#[doc = "Register `LOW_POWER_ST` reader"]
pub type R = crate::R<LOW_POWER_ST_SPEC>;
#[doc = "Register `LOW_POWER_ST` writer"]
pub type W = crate::W<LOW_POWER_ST_SPEC>;
#[doc = "Field `XPD_DIG` reader - digital wrap power down"]
pub type XPD_DIG_R = crate::BitReader;
#[doc = "Field `XPD_DIG` writer - digital wrap power down"]
pub type XPD_DIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_STATE_START` reader - touch should start to work"]
pub type TOUCH_STATE_START_R = crate::BitReader;
#[doc = "Field `TOUCH_STATE_START` writer - touch should start to work"]
pub type TOUCH_STATE_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_STATE_SWITCH` reader - touch is about to working. Switch rtc main state"]
pub type TOUCH_STATE_SWITCH_R = crate::BitReader;
#[doc = "Field `TOUCH_STATE_SWITCH` writer - touch is about to working. Switch rtc main state"]
pub type TOUCH_STATE_SWITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_STATE_SLP` reader - touch is in sleep state"]
pub type TOUCH_STATE_SLP_R = crate::BitReader;
#[doc = "Field `TOUCH_STATE_SLP` writer - touch is in sleep state"]
pub type TOUCH_STATE_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_STATE_DONE` reader - touch is done"]
pub type TOUCH_STATE_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_STATE_DONE` writer - touch is done"]
pub type TOUCH_STATE_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_STATE_START` reader - ulp/cocpu should start to work"]
pub type COCPU_STATE_START_R = crate::BitReader;
#[doc = "Field `COCPU_STATE_START` writer - ulp/cocpu should start to work"]
pub type COCPU_STATE_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_STATE_SWITCH` reader - ulp/cocpu is about to working. Switch rtc main state"]
pub type COCPU_STATE_SWITCH_R = crate::BitReader;
#[doc = "Field `COCPU_STATE_SWITCH` writer - ulp/cocpu is about to working. Switch rtc main state"]
pub type COCPU_STATE_SWITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_STATE_SLP` reader - ulp/cocpu is in sleep state"]
pub type COCPU_STATE_SLP_R = crate::BitReader;
#[doc = "Field `COCPU_STATE_SLP` writer - ulp/cocpu is in sleep state"]
pub type COCPU_STATE_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_STATE_DONE` reader - ulp/cocpu is done"]
pub type COCPU_STATE_DONE_R = crate::BitReader;
#[doc = "Field `COCPU_STATE_DONE` writer - ulp/cocpu is done"]
pub type COCPU_STATE_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE_XTAL_ISO` reader - no use any more"]
pub type MAIN_STATE_XTAL_ISO_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_XTAL_ISO` writer - no use any more"]
pub type MAIN_STATE_XTAL_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE_PLL_ON` reader - rtc main state machine is in states that pll should be running"]
pub type MAIN_STATE_PLL_ON_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_PLL_ON` writer - rtc main state machine is in states that pll should be running"]
pub type MAIN_STATE_PLL_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDY_FOR_WAKEUP` reader - rtc is ready to receive wake up trigger from wake up source"]
pub type RDY_FOR_WAKEUP_R = crate::BitReader;
#[doc = "Field `RDY_FOR_WAKEUP` writer - rtc is ready to receive wake up trigger from wake up source"]
pub type RDY_FOR_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE_WAIT_END` reader - rtc main state machine has been waited for some cycles"]
pub type MAIN_STATE_WAIT_END_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_WAIT_END` writer - rtc main state machine has been waited for some cycles"]
pub type MAIN_STATE_WAIT_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_WAKEUP_STATE` reader - rtc main state machine is in the states of wakeup process"]
pub type IN_WAKEUP_STATE_R = crate::BitReader;
#[doc = "Field `IN_WAKEUP_STATE` writer - rtc main state machine is in the states of wakeup process"]
pub type IN_WAKEUP_STATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOW_POWER_STATE` reader - rtc main state machine is in the states of low power"]
pub type IN_LOW_POWER_STATE_R = crate::BitReader;
#[doc = "Field `IN_LOW_POWER_STATE` writer - rtc main state machine is in the states of low power"]
pub type IN_LOW_POWER_STATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE_IN_WAIT_8M` reader - rtc main state machine is in wait 8m state"]
pub type MAIN_STATE_IN_WAIT_8M_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_WAIT_8M` writer - rtc main state machine is in wait 8m state"]
pub type MAIN_STATE_IN_WAIT_8M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE_IN_WAIT_PLL` reader - rtc main state machine is in wait pll state"]
pub type MAIN_STATE_IN_WAIT_PLL_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_WAIT_PLL` writer - rtc main state machine is in wait pll state"]
pub type MAIN_STATE_IN_WAIT_PLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE_IN_WAIT_XTL` reader - rtc main state machine is in wait xtal state"]
pub type MAIN_STATE_IN_WAIT_XTL_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_WAIT_XTL` writer - rtc main state machine is in wait xtal state"]
pub type MAIN_STATE_IN_WAIT_XTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE_IN_SLP` reader - rtc main state machine is in sleep state"]
pub type MAIN_STATE_IN_SLP_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_SLP` writer - rtc main state machine is in sleep state"]
pub type MAIN_STATE_IN_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE_IN_IDLE` reader - rtc main state machine is in idle state"]
pub type MAIN_STATE_IN_IDLE_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_IDLE` writer - rtc main state machine is in idle state"]
pub type MAIN_STATE_IN_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE` reader - rtc main state machine status"]
pub type MAIN_STATE_R = crate::FieldReader;
#[doc = "Field `MAIN_STATE` writer - rtc main state machine status"]
pub type MAIN_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 8 - digital wrap power down"]
    #[inline(always)]
    pub fn xpd_dig(&self) -> XPD_DIG_R {
        XPD_DIG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - touch should start to work"]
    #[inline(always)]
    pub fn touch_state_start(&self) -> TOUCH_STATE_START_R {
        TOUCH_STATE_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - touch is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn touch_state_switch(&self) -> TOUCH_STATE_SWITCH_R {
        TOUCH_STATE_SWITCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - touch is in sleep state"]
    #[inline(always)]
    pub fn touch_state_slp(&self) -> TOUCH_STATE_SLP_R {
        TOUCH_STATE_SLP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - touch is done"]
    #[inline(always)]
    pub fn touch_state_done(&self) -> TOUCH_STATE_DONE_R {
        TOUCH_STATE_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ulp/cocpu should start to work"]
    #[inline(always)]
    pub fn cocpu_state_start(&self) -> COCPU_STATE_START_R {
        COCPU_STATE_START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ulp/cocpu is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn cocpu_state_switch(&self) -> COCPU_STATE_SWITCH_R {
        COCPU_STATE_SWITCH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ulp/cocpu is in sleep state"]
    #[inline(always)]
    pub fn cocpu_state_slp(&self) -> COCPU_STATE_SLP_R {
        COCPU_STATE_SLP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ulp/cocpu is done"]
    #[inline(always)]
    pub fn cocpu_state_done(&self) -> COCPU_STATE_DONE_R {
        COCPU_STATE_DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - no use any more"]
    #[inline(always)]
    pub fn main_state_xtal_iso(&self) -> MAIN_STATE_XTAL_ISO_R {
        MAIN_STATE_XTAL_ISO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - rtc main state machine is in states that pll should be running"]
    #[inline(always)]
    pub fn main_state_pll_on(&self) -> MAIN_STATE_PLL_ON_R {
        MAIN_STATE_PLL_ON_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - rtc is ready to receive wake up trigger from wake up source"]
    #[inline(always)]
    pub fn rdy_for_wakeup(&self) -> RDY_FOR_WAKEUP_R {
        RDY_FOR_WAKEUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - rtc main state machine has been waited for some cycles"]
    #[inline(always)]
    pub fn main_state_wait_end(&self) -> MAIN_STATE_WAIT_END_R {
        MAIN_STATE_WAIT_END_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - rtc main state machine is in the states of wakeup process"]
    #[inline(always)]
    pub fn in_wakeup_state(&self) -> IN_WAKEUP_STATE_R {
        IN_WAKEUP_STATE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - rtc main state machine is in the states of low power"]
    #[inline(always)]
    pub fn in_low_power_state(&self) -> IN_LOW_POWER_STATE_R {
        IN_LOW_POWER_STATE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - rtc main state machine is in wait 8m state"]
    #[inline(always)]
    pub fn main_state_in_wait_8m(&self) -> MAIN_STATE_IN_WAIT_8M_R {
        MAIN_STATE_IN_WAIT_8M_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - rtc main state machine is in wait pll state"]
    #[inline(always)]
    pub fn main_state_in_wait_pll(&self) -> MAIN_STATE_IN_WAIT_PLL_R {
        MAIN_STATE_IN_WAIT_PLL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - rtc main state machine is in wait xtal state"]
    #[inline(always)]
    pub fn main_state_in_wait_xtl(&self) -> MAIN_STATE_IN_WAIT_XTL_R {
        MAIN_STATE_IN_WAIT_XTL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - rtc main state machine is in sleep state"]
    #[inline(always)]
    pub fn main_state_in_slp(&self) -> MAIN_STATE_IN_SLP_R {
        MAIN_STATE_IN_SLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - rtc main state machine is in idle state"]
    #[inline(always)]
    pub fn main_state_in_idle(&self) -> MAIN_STATE_IN_IDLE_R {
        MAIN_STATE_IN_IDLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - rtc main state machine status"]
    #[inline(always)]
    pub fn main_state(&self) -> MAIN_STATE_R {
        MAIN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOW_POWER_ST")
            .field("xpd_dig", &self.xpd_dig())
            .field("touch_state_start", &self.touch_state_start())
            .field("touch_state_switch", &self.touch_state_switch())
            .field("touch_state_slp", &self.touch_state_slp())
            .field("touch_state_done", &self.touch_state_done())
            .field("cocpu_state_start", &self.cocpu_state_start())
            .field("cocpu_state_switch", &self.cocpu_state_switch())
            .field("cocpu_state_slp", &self.cocpu_state_slp())
            .field("cocpu_state_done", &self.cocpu_state_done())
            .field("main_state_xtal_iso", &self.main_state_xtal_iso())
            .field("main_state_pll_on", &self.main_state_pll_on())
            .field("rdy_for_wakeup", &self.rdy_for_wakeup())
            .field("main_state_wait_end", &self.main_state_wait_end())
            .field("in_wakeup_state", &self.in_wakeup_state())
            .field("in_low_power_state", &self.in_low_power_state())
            .field("main_state_in_wait_8m", &self.main_state_in_wait_8m())
            .field("main_state_in_wait_pll", &self.main_state_in_wait_pll())
            .field("main_state_in_wait_xtl", &self.main_state_in_wait_xtl())
            .field("main_state_in_slp", &self.main_state_in_slp())
            .field("main_state_in_idle", &self.main_state_in_idle())
            .field("main_state", &self.main_state())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - digital wrap power down"]
    #[inline(always)]
    pub fn xpd_dig(&mut self) -> XPD_DIG_W<'_, LOW_POWER_ST_SPEC> {
        XPD_DIG_W::new(self, 8)
    }
    #[doc = "Bit 9 - touch should start to work"]
    #[inline(always)]
    pub fn touch_state_start(&mut self) -> TOUCH_STATE_START_W<'_, LOW_POWER_ST_SPEC> {
        TOUCH_STATE_START_W::new(self, 9)
    }
    #[doc = "Bit 10 - touch is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn touch_state_switch(&mut self) -> TOUCH_STATE_SWITCH_W<'_, LOW_POWER_ST_SPEC> {
        TOUCH_STATE_SWITCH_W::new(self, 10)
    }
    #[doc = "Bit 11 - touch is in sleep state"]
    #[inline(always)]
    pub fn touch_state_slp(&mut self) -> TOUCH_STATE_SLP_W<'_, LOW_POWER_ST_SPEC> {
        TOUCH_STATE_SLP_W::new(self, 11)
    }
    #[doc = "Bit 12 - touch is done"]
    #[inline(always)]
    pub fn touch_state_done(&mut self) -> TOUCH_STATE_DONE_W<'_, LOW_POWER_ST_SPEC> {
        TOUCH_STATE_DONE_W::new(self, 12)
    }
    #[doc = "Bit 13 - ulp/cocpu should start to work"]
    #[inline(always)]
    pub fn cocpu_state_start(&mut self) -> COCPU_STATE_START_W<'_, LOW_POWER_ST_SPEC> {
        COCPU_STATE_START_W::new(self, 13)
    }
    #[doc = "Bit 14 - ulp/cocpu is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn cocpu_state_switch(&mut self) -> COCPU_STATE_SWITCH_W<'_, LOW_POWER_ST_SPEC> {
        COCPU_STATE_SWITCH_W::new(self, 14)
    }
    #[doc = "Bit 15 - ulp/cocpu is in sleep state"]
    #[inline(always)]
    pub fn cocpu_state_slp(&mut self) -> COCPU_STATE_SLP_W<'_, LOW_POWER_ST_SPEC> {
        COCPU_STATE_SLP_W::new(self, 15)
    }
    #[doc = "Bit 16 - ulp/cocpu is done"]
    #[inline(always)]
    pub fn cocpu_state_done(&mut self) -> COCPU_STATE_DONE_W<'_, LOW_POWER_ST_SPEC> {
        COCPU_STATE_DONE_W::new(self, 16)
    }
    #[doc = "Bit 17 - no use any more"]
    #[inline(always)]
    pub fn main_state_xtal_iso(&mut self) -> MAIN_STATE_XTAL_ISO_W<'_, LOW_POWER_ST_SPEC> {
        MAIN_STATE_XTAL_ISO_W::new(self, 17)
    }
    #[doc = "Bit 18 - rtc main state machine is in states that pll should be running"]
    #[inline(always)]
    pub fn main_state_pll_on(&mut self) -> MAIN_STATE_PLL_ON_W<'_, LOW_POWER_ST_SPEC> {
        MAIN_STATE_PLL_ON_W::new(self, 18)
    }
    #[doc = "Bit 19 - rtc is ready to receive wake up trigger from wake up source"]
    #[inline(always)]
    pub fn rdy_for_wakeup(&mut self) -> RDY_FOR_WAKEUP_W<'_, LOW_POWER_ST_SPEC> {
        RDY_FOR_WAKEUP_W::new(self, 19)
    }
    #[doc = "Bit 20 - rtc main state machine has been waited for some cycles"]
    #[inline(always)]
    pub fn main_state_wait_end(&mut self) -> MAIN_STATE_WAIT_END_W<'_, LOW_POWER_ST_SPEC> {
        MAIN_STATE_WAIT_END_W::new(self, 20)
    }
    #[doc = "Bit 21 - rtc main state machine is in the states of wakeup process"]
    #[inline(always)]
    pub fn in_wakeup_state(&mut self) -> IN_WAKEUP_STATE_W<'_, LOW_POWER_ST_SPEC> {
        IN_WAKEUP_STATE_W::new(self, 21)
    }
    #[doc = "Bit 22 - rtc main state machine is in the states of low power"]
    #[inline(always)]
    pub fn in_low_power_state(&mut self) -> IN_LOW_POWER_STATE_W<'_, LOW_POWER_ST_SPEC> {
        IN_LOW_POWER_STATE_W::new(self, 22)
    }
    #[doc = "Bit 23 - rtc main state machine is in wait 8m state"]
    #[inline(always)]
    pub fn main_state_in_wait_8m(&mut self) -> MAIN_STATE_IN_WAIT_8M_W<'_, LOW_POWER_ST_SPEC> {
        MAIN_STATE_IN_WAIT_8M_W::new(self, 23)
    }
    #[doc = "Bit 24 - rtc main state machine is in wait pll state"]
    #[inline(always)]
    pub fn main_state_in_wait_pll(&mut self) -> MAIN_STATE_IN_WAIT_PLL_W<'_, LOW_POWER_ST_SPEC> {
        MAIN_STATE_IN_WAIT_PLL_W::new(self, 24)
    }
    #[doc = "Bit 25 - rtc main state machine is in wait xtal state"]
    #[inline(always)]
    pub fn main_state_in_wait_xtl(&mut self) -> MAIN_STATE_IN_WAIT_XTL_W<'_, LOW_POWER_ST_SPEC> {
        MAIN_STATE_IN_WAIT_XTL_W::new(self, 25)
    }
    #[doc = "Bit 26 - rtc main state machine is in sleep state"]
    #[inline(always)]
    pub fn main_state_in_slp(&mut self) -> MAIN_STATE_IN_SLP_W<'_, LOW_POWER_ST_SPEC> {
        MAIN_STATE_IN_SLP_W::new(self, 26)
    }
    #[doc = "Bit 27 - rtc main state machine is in idle state"]
    #[inline(always)]
    pub fn main_state_in_idle(&mut self) -> MAIN_STATE_IN_IDLE_W<'_, LOW_POWER_ST_SPEC> {
        MAIN_STATE_IN_IDLE_W::new(self, 27)
    }
    #[doc = "Bits 28:31 - rtc main state machine status"]
    #[inline(always)]
    pub fn main_state(&mut self) -> MAIN_STATE_W<'_, LOW_POWER_ST_SPEC> {
        MAIN_STATE_W::new(self, 28)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`low_power_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_power_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOW_POWER_ST_SPEC;
impl crate::RegisterSpec for LOW_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`low_power_st::R`](R) reader structure"]
impl crate::Readable for LOW_POWER_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`low_power_st::W`](W) writer structure"]
impl crate::Writable for LOW_POWER_ST_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOW_POWER_ST to value 0"]
impl crate::Resettable for LOW_POWER_ST_SPEC {}
