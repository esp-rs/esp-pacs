#[doc = "Register `LPCPU_CTRL` reader"]
pub type R = crate::R<LPCPU_CTRL_SPEC>;
#[doc = "Register `LPCPU_CTRL` writer"]
pub type W = crate::W<LPCPU_CTRL_SPEC>;
#[doc = "Field `LPCPU_SW_SLEEP_REQ` writer - software sleep request config for lpcpu"]
pub type LPCPU_SW_SLEEP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_SW_WAKEUP_REQ` writer - software wakeup request config for lpcpu"]
pub type LPCPU_SW_WAKEUP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_FORCE_STALL` reader - 1: force stall lpcpu 0: no change"]
pub type LPCPU_FORCE_STALL_R = crate::BitReader;
#[doc = "Field `LPCPU_FORCE_STALL` writer - 1: force stall lpcpu 0: no change"]
pub type LPCPU_FORCE_STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_FORCE_RESET` reader - 1: force reset lpcpu 0: no change"]
pub type LPCPU_FORCE_RESET_R = crate::BitReader;
#[doc = "Field `LPCPU_FORCE_RESET` writer - 1: force reset lpcpu 0: no change"]
pub type LPCPU_FORCE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_FORCE_NO_RESET` reader - 1: force no reset lpcpu 0: no change"]
pub type LPCPU_FORCE_NO_RESET_R = crate::BitReader;
#[doc = "Field `LPCPU_FORCE_NO_RESET` writer - 1: force no reset lpcpu 0: no change"]
pub type LPCPU_FORCE_NO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - 1: force stall lpcpu 0: no change"]
    #[inline(always)]
    pub fn lpcpu_force_stall(&self) -> LPCPU_FORCE_STALL_R {
        LPCPU_FORCE_STALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: force reset lpcpu 0: no change"]
    #[inline(always)]
    pub fn lpcpu_force_reset(&self) -> LPCPU_FORCE_RESET_R {
        LPCPU_FORCE_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: force no reset lpcpu 0: no change"]
    #[inline(always)]
    pub fn lpcpu_force_no_reset(&self) -> LPCPU_FORCE_NO_RESET_R {
        LPCPU_FORCE_NO_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCPU_CTRL")
            .field("lpcpu_force_stall", &self.lpcpu_force_stall())
            .field("lpcpu_force_reset", &self.lpcpu_force_reset())
            .field("lpcpu_force_no_reset", &self.lpcpu_force_no_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - software sleep request config for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_sw_sleep_req(&mut self) -> LPCPU_SW_SLEEP_REQ_W<'_, LPCPU_CTRL_SPEC> {
        LPCPU_SW_SLEEP_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - software wakeup request config for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_sw_wakeup_req(&mut self) -> LPCPU_SW_WAKEUP_REQ_W<'_, LPCPU_CTRL_SPEC> {
        LPCPU_SW_WAKEUP_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: force stall lpcpu 0: no change"]
    #[inline(always)]
    pub fn lpcpu_force_stall(&mut self) -> LPCPU_FORCE_STALL_W<'_, LPCPU_CTRL_SPEC> {
        LPCPU_FORCE_STALL_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: force reset lpcpu 0: no change"]
    #[inline(always)]
    pub fn lpcpu_force_reset(&mut self) -> LPCPU_FORCE_RESET_W<'_, LPCPU_CTRL_SPEC> {
        LPCPU_FORCE_RESET_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: force no reset lpcpu 0: no change"]
    #[inline(always)]
    pub fn lpcpu_force_no_reset(&mut self) -> LPCPU_FORCE_NO_RESET_W<'_, LPCPU_CTRL_SPEC> {
        LPCPU_FORCE_NO_RESET_W::new(self, 4)
    }
}
#[doc = "ctrl register for lpcpu power control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCPU_CTRL_SPEC;
impl crate::RegisterSpec for LPCPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcpu_ctrl::R`](R) reader structure"]
impl crate::Readable for LPCPU_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpcpu_ctrl::W`](W) writer structure"]
impl crate::Writable for LPCPU_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPCPU_CTRL to value 0"]
impl crate::Resettable for LPCPU_CTRL_SPEC {}
