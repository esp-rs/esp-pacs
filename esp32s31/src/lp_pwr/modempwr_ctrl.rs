#[doc = "Register `MODEMPWR_CTRL` writer"]
pub type W = crate::W<MODEMPWR_CTRL_SPEC>;
#[doc = "Field `MODEMPWR_SW_SLEEP_REQ` writer - software sleep request config for modempwr"]
pub type MODEMPWR_SW_SLEEP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEMPWR_SW_WAKEUP_REQ` writer - software wakeup request config for modempwr"]
pub type MODEMPWR_SW_WAKEUP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEMPWR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - software sleep request config for modempwr"]
    #[inline(always)]
    pub fn modempwr_sw_sleep_req(&mut self) -> MODEMPWR_SW_SLEEP_REQ_W<'_, MODEMPWR_CTRL_SPEC> {
        MODEMPWR_SW_SLEEP_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - software wakeup request config for modempwr"]
    #[inline(always)]
    pub fn modempwr_sw_wakeup_req(&mut self) -> MODEMPWR_SW_WAKEUP_REQ_W<'_, MODEMPWR_CTRL_SPEC> {
        MODEMPWR_SW_WAKEUP_REQ_W::new(self, 1)
    }
}
#[doc = "ctrl register for modempwr power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modempwr_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEMPWR_CTRL_SPEC;
impl crate::RegisterSpec for MODEMPWR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`modempwr_ctrl::W`](W) writer structure"]
impl crate::Writable for MODEMPWR_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEMPWR_CTRL to value 0"]
impl crate::Resettable for MODEMPWR_CTRL_SPEC {}
