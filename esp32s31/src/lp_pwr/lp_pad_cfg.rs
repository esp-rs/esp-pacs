#[doc = "Register `LP_PAD_CFG` reader"]
pub type R = crate::R<LP_PAD_CFG_SPEC>;
#[doc = "Register `LP_PAD_CFG` writer"]
pub type W = crate::W<LP_PAD_CFG_SPEC>;
#[doc = "Field `LPPAD_HOLD_CPU_PAUSE_EN` reader - 1: hold lppad when hpcpu stalls 0: not hold lppad when hpcpu stalls"]
pub type LPPAD_HOLD_CPU_PAUSE_EN_R = crate::BitReader;
#[doc = "Field `LPPAD_HOLD_CPU_PAUSE_EN` writer - 1: hold lppad when hpcpu stalls 0: not hold lppad when hpcpu stalls"]
pub type LPPAD_HOLD_CPU_PAUSE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HIGH_LP_PAD_HOLD` reader - 1:force pull up lppad hold signal 0:no effect"]
pub type FORCE_HIGH_LP_PAD_HOLD_R = crate::BitReader;
#[doc = "Field `FORCE_HIGH_LP_PAD_HOLD` writer - 1:force pull up lppad hold signal 0:no effect"]
pub type FORCE_HIGH_LP_PAD_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_LOW_LP_PAD_HOLD` reader - 1:force pull low lppad hold signal 0:no effect"]
pub type FORCE_LOW_LP_PAD_HOLD_R = crate::BitReader;
#[doc = "Field `FORCE_LOW_LP_PAD_HOLD` writer - 1:force pull low lppad hold signal 0:no effect"]
pub type FORCE_LOW_LP_PAD_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: hold lppad when hpcpu stalls 0: not hold lppad when hpcpu stalls"]
    #[inline(always)]
    pub fn lppad_hold_cpu_pause_en(&self) -> LPPAD_HOLD_CPU_PAUSE_EN_R {
        LPPAD_HOLD_CPU_PAUSE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:force pull up lppad hold signal 0:no effect"]
    #[inline(always)]
    pub fn force_high_lp_pad_hold(&self) -> FORCE_HIGH_LP_PAD_HOLD_R {
        FORCE_HIGH_LP_PAD_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:force pull low lppad hold signal 0:no effect"]
    #[inline(always)]
    pub fn force_low_lp_pad_hold(&self) -> FORCE_LOW_LP_PAD_HOLD_R {
        FORCE_LOW_LP_PAD_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PAD_CFG")
            .field("lppad_hold_cpu_pause_en", &self.lppad_hold_cpu_pause_en())
            .field("force_high_lp_pad_hold", &self.force_high_lp_pad_hold())
            .field("force_low_lp_pad_hold", &self.force_low_lp_pad_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: hold lppad when hpcpu stalls 0: not hold lppad when hpcpu stalls"]
    #[inline(always)]
    pub fn lppad_hold_cpu_pause_en(&mut self) -> LPPAD_HOLD_CPU_PAUSE_EN_W<'_, LP_PAD_CFG_SPEC> {
        LPPAD_HOLD_CPU_PAUSE_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:force pull up lppad hold signal 0:no effect"]
    #[inline(always)]
    pub fn force_high_lp_pad_hold(&mut self) -> FORCE_HIGH_LP_PAD_HOLD_W<'_, LP_PAD_CFG_SPEC> {
        FORCE_HIGH_LP_PAD_HOLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1:force pull low lppad hold signal 0:no effect"]
    #[inline(always)]
    pub fn force_low_lp_pad_hold(&mut self) -> FORCE_LOW_LP_PAD_HOLD_W<'_, LP_PAD_CFG_SPEC> {
        FORCE_LOW_LP_PAD_HOLD_W::new(self, 2)
    }
}
#[doc = "config register for lppad hold signal\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pad_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pad_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PAD_CFG_SPEC;
impl crate::RegisterSpec for LP_PAD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_pad_cfg::R`](R) reader structure"]
impl crate::Readable for LP_PAD_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_pad_cfg::W`](W) writer structure"]
impl crate::Writable for LP_PAD_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_PAD_CFG to value 0"]
impl crate::Resettable for LP_PAD_CFG_SPEC {}
