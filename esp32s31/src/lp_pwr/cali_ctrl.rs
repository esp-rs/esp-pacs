#[doc = "Register `CALI_CTRL` writer"]
pub type W = crate::W<CALI_CTRL_SPEC>;
#[doc = "Field `CALI_SW_SLEEP_REQ` writer - software sleep request config for cali"]
pub type CALI_SW_SLEEP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALI_SW_WAKEUP_REQ` writer - software wakeup request config for cali"]
pub type CALI_SW_WAKEUP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CALI_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - software sleep request config for cali"]
    #[inline(always)]
    pub fn cali_sw_sleep_req(&mut self) -> CALI_SW_SLEEP_REQ_W<'_, CALI_CTRL_SPEC> {
        CALI_SW_SLEEP_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - software wakeup request config for cali"]
    #[inline(always)]
    pub fn cali_sw_wakeup_req(&mut self) -> CALI_SW_WAKEUP_REQ_W<'_, CALI_CTRL_SPEC> {
        CALI_SW_WAKEUP_REQ_W::new(self, 1)
    }
}
#[doc = "ctrl register for cali power control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cali_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI_CTRL_SPEC;
impl crate::RegisterSpec for CALI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cali_ctrl::W`](W) writer structure"]
impl crate::Writable for CALI_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALI_CTRL to value 0"]
impl crate::Resettable for CALI_CTRL_SPEC {}
