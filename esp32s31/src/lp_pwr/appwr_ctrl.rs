#[doc = "Register `APPWR_CTRL` reader"]
pub type R = crate::R<APPWR_CTRL_SPEC>;
#[doc = "Register `APPWR_CTRL` writer"]
pub type W = crate::W<APPWR_CTRL_SPEC>;
#[doc = "Field `APPWR_SW_SLEEP_REQ` writer - software sleep request config for appwr"]
pub type APPWR_SW_SLEEP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPWR_SW_WAKEUP_REQ` writer - software wakeup request config for appwr"]
pub type APPWR_SW_WAKEUP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCPU_FORCE_STALL` reader - 1: force stall hpcpu 0: no change"]
pub type HPCPU_FORCE_STALL_R = crate::BitReader;
#[doc = "Field `HPCPU_FORCE_STALL` writer - 1: force stall hpcpu 0: no change"]
pub type HPCPU_FORCE_STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - 1: force stall hpcpu 0: no change"]
    #[inline(always)]
    pub fn hpcpu_force_stall(&self) -> HPCPU_FORCE_STALL_R {
        HPCPU_FORCE_STALL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPWR_CTRL")
            .field("hpcpu_force_stall", &self.hpcpu_force_stall())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - software sleep request config for appwr"]
    #[inline(always)]
    pub fn appwr_sw_sleep_req(&mut self) -> APPWR_SW_SLEEP_REQ_W<'_, APPWR_CTRL_SPEC> {
        APPWR_SW_SLEEP_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - software wakeup request config for appwr"]
    #[inline(always)]
    pub fn appwr_sw_wakeup_req(&mut self) -> APPWR_SW_WAKEUP_REQ_W<'_, APPWR_CTRL_SPEC> {
        APPWR_SW_WAKEUP_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: force stall hpcpu 0: no change"]
    #[inline(always)]
    pub fn hpcpu_force_stall(&mut self) -> HPCPU_FORCE_STALL_W<'_, APPWR_CTRL_SPEC> {
        HPCPU_FORCE_STALL_W::new(self, 2)
    }
}
#[doc = "ctrl register for appwr power control\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPWR_CTRL_SPEC;
impl crate::RegisterSpec for APPWR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appwr_ctrl::R`](R) reader structure"]
impl crate::Readable for APPWR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`appwr_ctrl::W`](W) writer structure"]
impl crate::Writable for APPWR_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APPWR_CTRL to value 0"]
impl crate::Resettable for APPWR_CTRL_SPEC {}
