///Register `TIMER_SYNCI_CFG` reader
pub type R = crate::R<TIMER_SYNCI_CFG_SPEC>;
///Register `TIMER_SYNCI_CFG` writer
pub type W = crate::W<TIMER_SYNCI_CFG_SPEC>;
///Field `TIMER0_SYNCISEL` reader - select sync input for PWM timer0, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
pub type TIMER0_SYNCISEL_R = crate::FieldReader;
///Field `TIMER0_SYNCISEL` writer - select sync input for PWM timer0, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
pub type TIMER0_SYNCISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TIMER1_SYNCISEL` reader - select sync input for PWM timer1, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
pub type TIMER1_SYNCISEL_R = crate::FieldReader;
///Field `TIMER1_SYNCISEL` writer - select sync input for PWM timer1, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
pub type TIMER1_SYNCISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TIMER2_SYNCISEL` reader - select sync input for PWM timer2, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
pub type TIMER2_SYNCISEL_R = crate::FieldReader;
///Field `TIMER2_SYNCISEL` writer - select sync input for PWM timer2, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
pub type TIMER2_SYNCISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EXTERNAL_SYNCI0_INVERT` reader - invert SYNC0 from GPIO matrix
pub type EXTERNAL_SYNCI0_INVERT_R = crate::BitReader;
///Field `EXTERNAL_SYNCI0_INVERT` writer - invert SYNC0 from GPIO matrix
pub type EXTERNAL_SYNCI0_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTERNAL_SYNCI1_INVERT` reader - invert SYNC1 from GPIO matrix
pub type EXTERNAL_SYNCI1_INVERT_R = crate::BitReader;
///Field `EXTERNAL_SYNCI1_INVERT` writer - invert SYNC1 from GPIO matrix
pub type EXTERNAL_SYNCI1_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTERNAL_SYNCI2_INVERT` reader - invert SYNC2 from GPIO matrix
pub type EXTERNAL_SYNCI2_INVERT_R = crate::BitReader;
///Field `EXTERNAL_SYNCI2_INVERT` writer - invert SYNC2 from GPIO matrix
pub type EXTERNAL_SYNCI2_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - select sync input for PWM timer0, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
    #[inline(always)]
    pub fn timer0_syncisel(&self) -> TIMER0_SYNCISEL_R {
        TIMER0_SYNCISEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - select sync input for PWM timer1, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
    #[inline(always)]
    pub fn timer1_syncisel(&self) -> TIMER1_SYNCISEL_R {
        TIMER1_SYNCISEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - select sync input for PWM timer2, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
    #[inline(always)]
    pub fn timer2_syncisel(&self) -> TIMER2_SYNCISEL_R {
        TIMER2_SYNCISEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bit 9 - invert SYNC0 from GPIO matrix
    #[inline(always)]
    pub fn external_synci0_invert(&self) -> EXTERNAL_SYNCI0_INVERT_R {
        EXTERNAL_SYNCI0_INVERT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - invert SYNC1 from GPIO matrix
    #[inline(always)]
    pub fn external_synci1_invert(&self) -> EXTERNAL_SYNCI1_INVERT_R {
        EXTERNAL_SYNCI1_INVERT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - invert SYNC2 from GPIO matrix
    #[inline(always)]
    pub fn external_synci2_invert(&self) -> EXTERNAL_SYNCI2_INVERT_R {
        EXTERNAL_SYNCI2_INVERT_R::new(((self.bits >> 11) & 1) != 0)
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
    ///Bits 0:2 - select sync input for PWM timer0, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
    #[inline(always)]
    #[must_use]
    pub fn timer0_syncisel(&mut self) -> TIMER0_SYNCISEL_W<TIMER_SYNCI_CFG_SPEC> {
        TIMER0_SYNCISEL_W::new(self, 0)
    }
    ///Bits 3:5 - select sync input for PWM timer1, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
    #[inline(always)]
    #[must_use]
    pub fn timer1_syncisel(&mut self) -> TIMER1_SYNCISEL_W<TIMER_SYNCI_CFG_SPEC> {
        TIMER1_SYNCISEL_W::new(self, 3)
    }
    ///Bits 6:8 - select sync input for PWM timer2, 1: PWM timer0 sync_out, 2: PWM timer1 sync_out, 3: PWM timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix, other values: no sync input selected
    #[inline(always)]
    #[must_use]
    pub fn timer2_syncisel(&mut self) -> TIMER2_SYNCISEL_W<TIMER_SYNCI_CFG_SPEC> {
        TIMER2_SYNCISEL_W::new(self, 6)
    }
    ///Bit 9 - invert SYNC0 from GPIO matrix
    #[inline(always)]
    #[must_use]
    pub fn external_synci0_invert(&mut self) -> EXTERNAL_SYNCI0_INVERT_W<TIMER_SYNCI_CFG_SPEC> {
        EXTERNAL_SYNCI0_INVERT_W::new(self, 9)
    }
    ///Bit 10 - invert SYNC1 from GPIO matrix
    #[inline(always)]
    #[must_use]
    pub fn external_synci1_invert(&mut self) -> EXTERNAL_SYNCI1_INVERT_W<TIMER_SYNCI_CFG_SPEC> {
        EXTERNAL_SYNCI1_INVERT_W::new(self, 10)
    }
    ///Bit 11 - invert SYNC2 from GPIO matrix
    #[inline(always)]
    #[must_use]
    pub fn external_synci2_invert(&mut self) -> EXTERNAL_SYNCI2_INVERT_W<TIMER_SYNCI_CFG_SPEC> {
        EXTERNAL_SYNCI2_INVERT_W::new(self, 11)
    }
}
/**Synchronization input selection for three PWM timers.

You can [`read`](crate::generic::Reg::read) this register and get [`timer_synci_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_synci_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIMER_SYNCI_CFG_SPEC;
impl crate::RegisterSpec for TIMER_SYNCI_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`timer_synci_cfg::R`](R) reader structure
impl crate::Readable for TIMER_SYNCI_CFG_SPEC {}
///`write(|w| ..)` method takes [`timer_synci_cfg::W`](W) writer structure
impl crate::Writable for TIMER_SYNCI_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMER_SYNCI_CFG to value 0
impl crate::Resettable for TIMER_SYNCI_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
