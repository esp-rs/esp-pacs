#[doc = "Register `TIMER_SYNCI_CFG` reader"]
pub type R = crate::R<TIMER_SYNCI_CFG_SPEC>;
#[doc = "Register `TIMER_SYNCI_CFG` writer"]
pub type W = crate::W<TIMER_SYNCI_CFG_SPEC>;
#[doc = "Field `TIMER_SYNCISEL(0-2)` reader - select sync input for PWM timer%s, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
pub type TIMER_SYNCISEL_R = crate::FieldReader;
#[doc = "Field `TIMER_SYNCISEL(0-2)` writer - select sync input for PWM timer%s, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
pub type TIMER_SYNCISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTERNAL_SYNCI_INVERT(0-2)` reader - invert SYNC%s from GPIO matrix"]
pub type EXTERNAL_SYNCI_INVERT_R = crate::BitReader;
#[doc = "Field `EXTERNAL_SYNCI_INVERT(0-2)` writer - invert SYNC%s from GPIO matrix"]
pub type EXTERNAL_SYNCI_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "select sync input for PWM timer(0-2), 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_SYNCISEL` field.</div>"]
    #[inline(always)]
    pub fn timer_syncisel(&self, n: u8) -> TIMER_SYNCISEL_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_SYNCISEL_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "select sync input for PWM timer(0-2), 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
    #[inline(always)]
    pub fn timer_syncisel_iter(&self) -> impl Iterator<Item = TIMER_SYNCISEL_R> + '_ {
        (0..3).map(move |n| TIMER_SYNCISEL_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - select sync input for PWM timer0, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
    #[inline(always)]
    pub fn timer0_syncisel(&self) -> TIMER_SYNCISEL_R {
        TIMER_SYNCISEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - select sync input for PWM timer1, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
    #[inline(always)]
    pub fn timer1_syncisel(&self) -> TIMER_SYNCISEL_R {
        TIMER_SYNCISEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - select sync input for PWM timer2, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
    #[inline(always)]
    pub fn timer2_syncisel(&self) -> TIMER_SYNCISEL_R {
        TIMER_SYNCISEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "invert SYNC(0-2) from GPIO matrix"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXTERNAL_SYNCI0_INVERT` field.</div>"]
    #[inline(always)]
    pub fn external_synci_invert(&self, n: u8) -> EXTERNAL_SYNCI_INVERT_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        EXTERNAL_SYNCI_INVERT_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "invert SYNC(0-2) from GPIO matrix"]
    #[inline(always)]
    pub fn external_synci_invert_iter(&self) -> impl Iterator<Item = EXTERNAL_SYNCI_INVERT_R> + '_ {
        (0..3).map(move |n| EXTERNAL_SYNCI_INVERT_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - invert SYNC0 from GPIO matrix"]
    #[inline(always)]
    pub fn external_synci0_invert(&self) -> EXTERNAL_SYNCI_INVERT_R {
        EXTERNAL_SYNCI_INVERT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - invert SYNC1 from GPIO matrix"]
    #[inline(always)]
    pub fn external_synci1_invert(&self) -> EXTERNAL_SYNCI_INVERT_R {
        EXTERNAL_SYNCI_INVERT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - invert SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn external_synci2_invert(&self) -> EXTERNAL_SYNCI_INVERT_R {
        EXTERNAL_SYNCI_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_SYNCI_CFG")
            .field("timer0_syncisel", &self.timer0_syncisel())
            .field("timer1_syncisel", &self.timer1_syncisel())
            .field("timer2_syncisel", &self.timer2_syncisel())
            .field("external_synci0_invert", &self.external_synci0_invert())
            .field("external_synci1_invert", &self.external_synci1_invert())
            .field("external_synci2_invert", &self.external_synci2_invert())
            .finish()
    }
}
impl W {
    #[doc = "select sync input for PWM timer(0-2), 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_SYNCISEL` field.</div>"]
    #[inline(always)]
    pub fn timer_syncisel(&mut self, n: u8) -> TIMER_SYNCISEL_W<'_, TIMER_SYNCI_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TIMER_SYNCISEL_W::new(self, n * 3)
    }
    #[doc = "Bits 0:2 - select sync input for PWM timer0, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
    #[inline(always)]
    pub fn timer0_syncisel(&mut self) -> TIMER_SYNCISEL_W<'_, TIMER_SYNCI_CFG_SPEC> {
        TIMER_SYNCISEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - select sync input for PWM timer1, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
    #[inline(always)]
    pub fn timer1_syncisel(&mut self) -> TIMER_SYNCISEL_W<'_, TIMER_SYNCI_CFG_SPEC> {
        TIMER_SYNCISEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - select sync input for PWM timer2, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected"]
    #[inline(always)]
    pub fn timer2_syncisel(&mut self) -> TIMER_SYNCISEL_W<'_, TIMER_SYNCI_CFG_SPEC> {
        TIMER_SYNCISEL_W::new(self, 6)
    }
    #[doc = "invert SYNC(0-2) from GPIO matrix"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXTERNAL_SYNCI0_INVERT` field.</div>"]
    #[inline(always)]
    pub fn external_synci_invert(
        &mut self,
        n: u8,
    ) -> EXTERNAL_SYNCI_INVERT_W<'_, TIMER_SYNCI_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        EXTERNAL_SYNCI_INVERT_W::new(self, n + 9)
    }
    #[doc = "Bit 9 - invert SYNC0 from GPIO matrix"]
    #[inline(always)]
    pub fn external_synci0_invert(&mut self) -> EXTERNAL_SYNCI_INVERT_W<'_, TIMER_SYNCI_CFG_SPEC> {
        EXTERNAL_SYNCI_INVERT_W::new(self, 9)
    }
    #[doc = "Bit 10 - invert SYNC1 from GPIO matrix"]
    #[inline(always)]
    pub fn external_synci1_invert(&mut self) -> EXTERNAL_SYNCI_INVERT_W<'_, TIMER_SYNCI_CFG_SPEC> {
        EXTERNAL_SYNCI_INVERT_W::new(self, 10)
    }
    #[doc = "Bit 11 - invert SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn external_synci2_invert(&mut self) -> EXTERNAL_SYNCI_INVERT_W<'_, TIMER_SYNCI_CFG_SPEC> {
        EXTERNAL_SYNCI_INVERT_W::new(self, 11)
    }
}
#[doc = "Synchronization input selection for three PWM timers.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_synci_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_synci_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_SYNCI_CFG_SPEC;
impl crate::RegisterSpec for TIMER_SYNCI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_synci_cfg::R`](R) reader structure"]
impl crate::Readable for TIMER_SYNCI_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_synci_cfg::W`](W) writer structure"]
impl crate::Writable for TIMER_SYNCI_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_SYNCI_CFG to value 0"]
impl crate::Resettable for TIMER_SYNCI_CFG_SPEC {}
