///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `HSTIMER_OVF(0-3)` writer - Set this bit to clear high speed channel%s counter overflow interrupt.
pub type HSTIMER_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `LSTIMER_OVF(0-3)` writer - Set this bit to clear low speed channel%s counter overflow interrupt.
pub type LSTIMER_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DUTY_CHNG_END_HSCH(0-7)` writer - Set this bit to clear high speed channel %s duty change done interrupt.
pub type DUTY_CHNG_END_HSCH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DUTY_CHNG_END_LSCH(0-7)` writer - Set this bit to clear low speed channel %s duty change done interrupt.
pub type DUTY_CHNG_END_LSCH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Set this bit to clear high speed channel(0-3) counter overflow interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `HSTIMER0_OVF` field
    #[inline(always)]
    #[must_use]
    pub fn hstimer_ovf(&mut self, n: u8) -> HSTIMER_OVF_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        HSTIMER_OVF_W::new(self, n)
    }
    ///Bit 0 - Set this bit to clear high speed channel0 counter overflow interrupt.
    #[inline(always)]
    #[must_use]
    pub fn hstimer0_ovf(&mut self) -> HSTIMER_OVF_W<INT_CLR_SPEC> {
        HSTIMER_OVF_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear high speed channel1 counter overflow interrupt.
    #[inline(always)]
    #[must_use]
    pub fn hstimer1_ovf(&mut self) -> HSTIMER_OVF_W<INT_CLR_SPEC> {
        HSTIMER_OVF_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to clear high speed channel2 counter overflow interrupt.
    #[inline(always)]
    #[must_use]
    pub fn hstimer2_ovf(&mut self) -> HSTIMER_OVF_W<INT_CLR_SPEC> {
        HSTIMER_OVF_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear high speed channel3 counter overflow interrupt.
    #[inline(always)]
    #[must_use]
    pub fn hstimer3_ovf(&mut self) -> HSTIMER_OVF_W<INT_CLR_SPEC> {
        HSTIMER_OVF_W::new(self, 3)
    }
    ///Set this bit to clear low speed channel(0-3) counter overflow interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `LSTIMER0_OVF` field
    #[inline(always)]
    #[must_use]
    pub fn lstimer_ovf(&mut self, n: u8) -> LSTIMER_OVF_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        LSTIMER_OVF_W::new(self, n + 4)
    }
    ///Bit 4 - Set this bit to clear low speed channel0 counter overflow interrupt.
    #[inline(always)]
    #[must_use]
    pub fn lstimer0_ovf(&mut self) -> LSTIMER_OVF_W<INT_CLR_SPEC> {
        LSTIMER_OVF_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to clear low speed channel1 counter overflow interrupt.
    #[inline(always)]
    #[must_use]
    pub fn lstimer1_ovf(&mut self) -> LSTIMER_OVF_W<INT_CLR_SPEC> {
        LSTIMER_OVF_W::new(self, 5)
    }
    ///Bit 6 - Set this bit to clear low speed channel2 counter overflow interrupt.
    #[inline(always)]
    #[must_use]
    pub fn lstimer2_ovf(&mut self) -> LSTIMER_OVF_W<INT_CLR_SPEC> {
        LSTIMER_OVF_W::new(self, 6)
    }
    ///Bit 7 - Set this bit to clear low speed channel3 counter overflow interrupt.
    #[inline(always)]
    #[must_use]
    pub fn lstimer3_ovf(&mut self) -> LSTIMER_OVF_W<INT_CLR_SPEC> {
        LSTIMER_OVF_W::new(self, 7)
    }
    ///Set this bit to clear high speed channel (0-7) duty change done interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_HSCH0` field
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch(&mut self, n: u8) -> DUTY_CHNG_END_HSCH_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DUTY_CHNG_END_HSCH_W::new(self, n + 8)
    }
    ///Bit 8 - Set this bit to clear high speed channel 0 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch0(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 8)
    }
    ///Bit 9 - Set this bit to clear high speed channel 1 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch1(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 9)
    }
    ///Bit 10 - Set this bit to clear high speed channel 2 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch2(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 10)
    }
    ///Bit 11 - Set this bit to clear high speed channel 3 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch3(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 11)
    }
    ///Bit 12 - Set this bit to clear high speed channel 4 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch4(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 12)
    }
    ///Bit 13 - Set this bit to clear high speed channel 5 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch5(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 13)
    }
    ///Bit 14 - Set this bit to clear high speed channel 6 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch6(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 14)
    }
    ///Bit 15 - Set this bit to clear high speed channel 7 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch7(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 15)
    }
    ///Set this bit to clear low speed channel (0-7) duty change done interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_LSCH0` field
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch(&mut self, n: u8) -> DUTY_CHNG_END_LSCH_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DUTY_CHNG_END_LSCH_W::new(self, n + 16)
    }
    ///Bit 16 - Set this bit to clear low speed channel 0 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch0(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 16)
    }
    ///Bit 17 - Set this bit to clear low speed channel 1 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch1(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 17)
    }
    ///Bit 18 - Set this bit to clear low speed channel 2 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch2(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 18)
    }
    ///Bit 19 - Set this bit to clear low speed channel 3 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch3(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 19)
    }
    ///Bit 20 - Set this bit to clear low speed channel 4 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch4(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 20)
    }
    ///Bit 21 - Set this bit to clear low speed channel 5 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch5(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 21)
    }
    ///Bit 22 - Set this bit to clear low speed channel 6 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch6(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 22)
    }
    ///Bit 23 - Set this bit to clear low speed channel 7 duty change done interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch7(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 23)
    }
}
/**

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0111;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
