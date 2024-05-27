#[doc = "Register `STATE0` reader"]
pub type R = crate::R<STATE0_SPEC>;
#[doc = "Register `STATE0` writer"]
pub type W = crate::W<STATE0_SPEC>;
#[doc = "Field `TOUCH_WAKEUP_FORCE_EN` reader - touch controller force wake up"]
pub type TOUCH_WAKEUP_FORCE_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_WAKEUP_FORCE_EN` writer - touch controller force wake up"]
pub type TOUCH_WAKEUP_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_WAKEUP_FORCE_EN` reader - ULP-coprocessor force wake up"]
pub type ULP_CP_WAKEUP_FORCE_EN_R = crate::BitReader;
#[doc = "Field `ULP_CP_WAKEUP_FORCE_EN` writer - ULP-coprocessor force wake up"]
pub type ULP_CP_WAKEUP_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB2RTC_BRIDGE_SEL` reader - 1: APB to RTC using bridge 0: APB to RTC using sync"]
pub type APB2RTC_BRIDGE_SEL_R = crate::BitReader;
#[doc = "Field `APB2RTC_BRIDGE_SEL` writer - 1: APB to RTC using bridge 0: APB to RTC using sync"]
pub type APB2RTC_BRIDGE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SLP_TIMER_EN` reader - touch timer enable bit"]
pub type TOUCH_SLP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SLP_TIMER_EN` writer - touch timer enable bit"]
pub type TOUCH_SLP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_SLP_TIMER_EN` reader - ULP-coprocessor timer enable bit"]
pub type ULP_CP_SLP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `ULP_CP_SLP_TIMER_EN` writer - ULP-coprocessor timer enable bit"]
pub type ULP_CP_SLP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_ACTIVE_IND` reader - SDIO active indication"]
pub type SDIO_ACTIVE_IND_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP` reader - sleep wakeup bit"]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP` writer - sleep wakeup bit"]
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT` reader - sleep reject bit"]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` writer - sleep reject bit"]
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_EN` reader - sleep enable bit"]
pub type SLEEP_EN_R = crate::BitReader;
#[doc = "Field `SLEEP_EN` writer - sleep enable bit"]
pub type SLEEP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20 - touch controller force wake up"]
    #[inline(always)]
    pub fn touch_wakeup_force_en(&self) -> TOUCH_WAKEUP_FORCE_EN_R {
        TOUCH_WAKEUP_FORCE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ULP-coprocessor force wake up"]
    #[inline(always)]
    pub fn ulp_cp_wakeup_force_en(&self) -> ULP_CP_WAKEUP_FORCE_EN_R {
        ULP_CP_WAKEUP_FORCE_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&self) -> APB2RTC_BRIDGE_SEL_R {
        APB2RTC_BRIDGE_SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - touch timer enable bit"]
    #[inline(always)]
    pub fn touch_slp_timer_en(&self) -> TOUCH_SLP_TIMER_EN_R {
        TOUCH_SLP_TIMER_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&self) -> ULP_CP_SLP_TIMER_EN_R {
        ULP_CP_SLP_TIMER_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - SDIO active indication"]
    #[inline(always)]
    pub fn sdio_active_ind(&self) -> SDIO_ACTIVE_IND_R {
        SDIO_ACTIVE_IND_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - sleep wakeup bit"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - sleep reject bit"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE0")
            .field("touch_wakeup_force_en", &self.touch_wakeup_force_en())
            .field("ulp_cp_wakeup_force_en", &self.ulp_cp_wakeup_force_en())
            .field("apb2rtc_bridge_sel", &self.apb2rtc_bridge_sel())
            .field("touch_slp_timer_en", &self.touch_slp_timer_en())
            .field("ulp_cp_slp_timer_en", &self.ulp_cp_slp_timer_en())
            .field("sdio_active_ind", &self.sdio_active_ind())
            .field("slp_wakeup", &self.slp_wakeup())
            .field("slp_reject", &self.slp_reject())
            .field("sleep_en", &self.sleep_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 20 - touch controller force wake up"]
    #[inline(always)]
    #[must_use]
    pub fn touch_wakeup_force_en(&mut self) -> TOUCH_WAKEUP_FORCE_EN_W<STATE0_SPEC> {
        TOUCH_WAKEUP_FORCE_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - ULP-coprocessor force wake up"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_wakeup_force_en(&mut self) -> ULP_CP_WAKEUP_FORCE_EN_W<STATE0_SPEC> {
        ULP_CP_WAKEUP_FORCE_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    #[must_use]
    pub fn apb2rtc_bridge_sel(&mut self) -> APB2RTC_BRIDGE_SEL_W<STATE0_SPEC> {
        APB2RTC_BRIDGE_SEL_W::new(self, 22)
    }
    #[doc = "Bit 23 - touch timer enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn touch_slp_timer_en(&mut self) -> TOUCH_SLP_TIMER_EN_W<STATE0_SPEC> {
        TOUCH_SLP_TIMER_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_slp_timer_en(&mut self) -> ULP_CP_SLP_TIMER_EN_W<STATE0_SPEC> {
        ULP_CP_SLP_TIMER_EN_W::new(self, 24)
    }
    #[doc = "Bit 29 - sleep wakeup bit"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<STATE0_SPEC> {
        SLP_WAKEUP_W::new(self, 29)
    }
    #[doc = "Bit 30 - sleep reject bit"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<STATE0_SPEC> {
        SLP_REJECT_W::new(self, 30)
    }
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W<STATE0_SPEC> {
        SLEEP_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state0::R`](R) reader structure"]
impl crate::Readable for STATE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`state0::W`](W) writer structure"]
impl crate::Writable for STATE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATE0 to value 0x0030_0000"]
impl crate::Resettable for STATE0_SPEC {
    const RESET_VALUE: u32 = 0x0030_0000;
}
