#[doc = "Register `CNNT_PAD_CFG` reader"]
pub type R = crate::R<CNNT_PAD_CFG_SPEC>;
#[doc = "Register `CNNT_PAD_CFG` writer"]
pub type W = crate::W<CNNT_PAD_CFG_SPEC>;
#[doc = "Field `CNNT_PAD_HOLD_CPU_PAUSE_EN` reader - 1: hold cnntpad when hpcpu stalls 0: not hold cnntpad when hpcpu stalls"]
pub type CNNT_PAD_HOLD_CPU_PAUSE_EN_R = crate::BitReader;
#[doc = "Field `CNNT_PAD_HOLD_CPU_PAUSE_EN` writer - 1: hold cnntpad when hpcpu stalls 0: not hold cnntpad when hpcpu stalls"]
pub type CNNT_PAD_HOLD_CPU_PAUSE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HIGH_CNNT_PAD_HOLD` reader - 1:force pull up cnntpad hold signal 0:no effect"]
pub type FORCE_HIGH_CNNT_PAD_HOLD_R = crate::BitReader;
#[doc = "Field `FORCE_HIGH_CNNT_PAD_HOLD` writer - 1:force pull up cnntpad hold signal 0:no effect"]
pub type FORCE_HIGH_CNNT_PAD_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_LOW_CNNT_PAD_HOLD` reader - 1:force pull low cnntpad hold signal 0:no effect"]
pub type FORCE_LOW_CNNT_PAD_HOLD_R = crate::BitReader;
#[doc = "Field `FORCE_LOW_CNNT_PAD_HOLD` writer - 1:force pull low cnntpad hold signal 0:no effect"]
pub type FORCE_LOW_CNNT_PAD_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: hold cnntpad when hpcpu stalls 0: not hold cnntpad when hpcpu stalls"]
    #[inline(always)]
    pub fn cnnt_pad_hold_cpu_pause_en(&self) -> CNNT_PAD_HOLD_CPU_PAUSE_EN_R {
        CNNT_PAD_HOLD_CPU_PAUSE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:force pull up cnntpad hold signal 0:no effect"]
    #[inline(always)]
    pub fn force_high_cnnt_pad_hold(&self) -> FORCE_HIGH_CNNT_PAD_HOLD_R {
        FORCE_HIGH_CNNT_PAD_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:force pull low cnntpad hold signal 0:no effect"]
    #[inline(always)]
    pub fn force_low_cnnt_pad_hold(&self) -> FORCE_LOW_CNNT_PAD_HOLD_R {
        FORCE_LOW_CNNT_PAD_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNNT_PAD_CFG")
            .field(
                "cnnt_pad_hold_cpu_pause_en",
                &self.cnnt_pad_hold_cpu_pause_en(),
            )
            .field("force_high_cnnt_pad_hold", &self.force_high_cnnt_pad_hold())
            .field("force_low_cnnt_pad_hold", &self.force_low_cnnt_pad_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: hold cnntpad when hpcpu stalls 0: not hold cnntpad when hpcpu stalls"]
    #[inline(always)]
    pub fn cnnt_pad_hold_cpu_pause_en(
        &mut self,
    ) -> CNNT_PAD_HOLD_CPU_PAUSE_EN_W<'_, CNNT_PAD_CFG_SPEC> {
        CNNT_PAD_HOLD_CPU_PAUSE_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:force pull up cnntpad hold signal 0:no effect"]
    #[inline(always)]
    pub fn force_high_cnnt_pad_hold(
        &mut self,
    ) -> FORCE_HIGH_CNNT_PAD_HOLD_W<'_, CNNT_PAD_CFG_SPEC> {
        FORCE_HIGH_CNNT_PAD_HOLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1:force pull low cnntpad hold signal 0:no effect"]
    #[inline(always)]
    pub fn force_low_cnnt_pad_hold(&mut self) -> FORCE_LOW_CNNT_PAD_HOLD_W<'_, CNNT_PAD_CFG_SPEC> {
        FORCE_LOW_CNNT_PAD_HOLD_W::new(self, 2)
    }
}
#[doc = "config register for cnntpad hold signal\n\nYou can [`read`](crate::Reg::read) this register and get [`cnnt_pad_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnnt_pad_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNNT_PAD_CFG_SPEC;
impl crate::RegisterSpec for CNNT_PAD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnnt_pad_cfg::R`](R) reader structure"]
impl crate::Readable for CNNT_PAD_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnnt_pad_cfg::W`](W) writer structure"]
impl crate::Writable for CNNT_PAD_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNNT_PAD_CFG to value 0"]
impl crate::Resettable for CNNT_PAD_CFG_SPEC {}
