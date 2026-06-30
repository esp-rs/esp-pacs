#[doc = "Register `TOUCH_CTRL` writer"]
pub type W = crate::W<TOUCH_CTRL_SPEC>;
#[doc = "Field `TOUCH_SW_SLEEP_REQ` writer - software sleep request config for touch"]
pub type TOUCH_SW_SLEEP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SW_WAKEUP_REQ` writer - software wakeup request config for touch"]
pub type TOUCH_SW_WAKEUP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - software sleep request config for touch"]
    #[inline(always)]
    pub fn touch_sw_sleep_req(&mut self) -> TOUCH_SW_SLEEP_REQ_W<'_, TOUCH_CTRL_SPEC> {
        TOUCH_SW_SLEEP_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - software wakeup request config for touch"]
    #[inline(always)]
    pub fn touch_sw_wakeup_req(&mut self) -> TOUCH_SW_WAKEUP_REQ_W<'_, TOUCH_CTRL_SPEC> {
        TOUCH_SW_WAKEUP_REQ_W::new(self, 1)
    }
}
#[doc = "ctrl register for touch power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`touch_ctrl::W`](W) writer structure"]
impl crate::Writable for TOUCH_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_CTRL to value 0"]
impl crate::Resettable for TOUCH_CTRL_SPEC {}
