#[doc = "Register `ZB_CTRL` writer"]
pub type W = crate::W<ZB_CTRL_SPEC>;
#[doc = "Field `ZB_SW_SLEEP_REQ` writer - software sleep request config for zb"]
pub type ZB_SW_SLEEP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZB_SW_WAKEUP_REQ` writer - software wakeup request config for zb"]
pub type ZB_SW_WAKEUP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ZB_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - software sleep request config for zb"]
    #[inline(always)]
    pub fn zb_sw_sleep_req(&mut self) -> ZB_SW_SLEEP_REQ_W<'_, ZB_CTRL_SPEC> {
        ZB_SW_SLEEP_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - software wakeup request config for zb"]
    #[inline(always)]
    pub fn zb_sw_wakeup_req(&mut self) -> ZB_SW_WAKEUP_REQ_W<'_, ZB_CTRL_SPEC> {
        ZB_SW_WAKEUP_REQ_W::new(self, 1)
    }
}
#[doc = "ctrl register for zb power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zb_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZB_CTRL_SPEC;
impl crate::RegisterSpec for ZB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`zb_ctrl::W`](W) writer structure"]
impl crate::Writable for ZB_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ZB_CTRL to value 0"]
impl crate::Resettable for ZB_CTRL_SPEC {}
