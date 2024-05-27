///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `TIMER_OVF(0-3)` reader - The interrupt enable bit for the TIMER%s_OVF interrupt.
pub type TIMER_OVF_R = crate::BitReader;
///Field `TIMER_OVF(0-3)` writer - The interrupt enable bit for the TIMER%s_OVF interrupt.
pub type TIMER_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DUTY_CHNG_END_CH(0-5)` reader - The interrupt enable bit for the DUTY_CHNG_END_CH%s interrupt.
pub type DUTY_CHNG_END_CH_R = crate::BitReader;
///Field `DUTY_CHNG_END_CH(0-5)` writer - The interrupt enable bit for the DUTY_CHNG_END_CH%s interrupt.
pub type DUTY_CHNG_END_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVF_CNT_CH(0-5)` reader - The interrupt enable bit for the OVF_CNT_CH%s interrupt.
pub type OVF_CNT_CH_R = crate::BitReader;
///Field `OVF_CNT_CH(0-5)` writer - The interrupt enable bit for the OVF_CNT_CH%s interrupt.
pub type OVF_CNT_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///The interrupt enable bit for the TIMER(0-3)_OVF interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `TIMER0_OVF` field
    #[inline(always)]
    pub fn timer_ovf(&self, n: u8) -> TIMER_OVF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TIMER_OVF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///The interrupt enable bit for the TIMER(0-3)_OVF interrupt.
    #[inline(always)]
    pub fn timer_ovf_iter(&self) -> impl Iterator<Item = TIMER_OVF_R> + '_ {
        (0..4).map(move |n| TIMER_OVF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - The interrupt enable bit for the TIMER0_OVF interrupt.
    #[inline(always)]
    pub fn timer0_ovf(&self) -> TIMER_OVF_R {
        TIMER_OVF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The interrupt enable bit for the TIMER1_OVF interrupt.
    #[inline(always)]
    pub fn timer1_ovf(&self) -> TIMER_OVF_R {
        TIMER_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The interrupt enable bit for the TIMER2_OVF interrupt.
    #[inline(always)]
    pub fn timer2_ovf(&self) -> TIMER_OVF_R {
        TIMER_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The interrupt enable bit for the TIMER3_OVF interrupt.
    #[inline(always)]
    pub fn timer3_ovf(&self) -> TIMER_OVF_R {
        TIMER_OVF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///The interrupt enable bit for the DUTY_CHNG_END_CH(0-5) interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_CH0` field
    #[inline(always)]
    pub fn duty_chng_end_ch(&self, n: u8) -> DUTY_CHNG_END_CH_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        DUTY_CHNG_END_CH_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    ///Iterator for array of:
    ///The interrupt enable bit for the DUTY_CHNG_END_CH(0-5) interrupt.
    #[inline(always)]
    pub fn duty_chng_end_ch_iter(&self) -> impl Iterator<Item = DUTY_CHNG_END_CH_R> + '_ {
        (0..6).map(move |n| DUTY_CHNG_END_CH_R::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    ///Bit 4 - The interrupt enable bit for the DUTY_CHNG_END_CH0 interrupt.
    #[inline(always)]
    pub fn duty_chng_end_ch0(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The interrupt enable bit for the DUTY_CHNG_END_CH1 interrupt.
    #[inline(always)]
    pub fn duty_chng_end_ch1(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The interrupt enable bit for the DUTY_CHNG_END_CH2 interrupt.
    #[inline(always)]
    pub fn duty_chng_end_ch2(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The interrupt enable bit for the DUTY_CHNG_END_CH3 interrupt.
    #[inline(always)]
    pub fn duty_chng_end_ch3(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - The interrupt enable bit for the DUTY_CHNG_END_CH4 interrupt.
    #[inline(always)]
    pub fn duty_chng_end_ch4(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - The interrupt enable bit for the DUTY_CHNG_END_CH5 interrupt.
    #[inline(always)]
    pub fn duty_chng_end_ch5(&self) -> DUTY_CHNG_END_CH_R {
        DUTY_CHNG_END_CH_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///The interrupt enable bit for the OVF_CNT_CH(0-5) interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `OVF_CNT_CH0` field
    #[inline(always)]
    pub fn ovf_cnt_ch(&self, n: u8) -> OVF_CNT_CH_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OVF_CNT_CH_R::new(((self.bits >> (n + 10)) & 1) != 0)
    }
    ///Iterator for array of:
    ///The interrupt enable bit for the OVF_CNT_CH(0-5) interrupt.
    #[inline(always)]
    pub fn ovf_cnt_ch_iter(&self) -> impl Iterator<Item = OVF_CNT_CH_R> + '_ {
        (0..6).map(move |n| OVF_CNT_CH_R::new(((self.bits >> (n + 10)) & 1) != 0))
    }
    ///Bit 10 - The interrupt enable bit for the OVF_CNT_CH0 interrupt.
    #[inline(always)]
    pub fn ovf_cnt_ch0(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - The interrupt enable bit for the OVF_CNT_CH1 interrupt.
    #[inline(always)]
    pub fn ovf_cnt_ch1(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - The interrupt enable bit for the OVF_CNT_CH2 interrupt.
    #[inline(always)]
    pub fn ovf_cnt_ch2(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - The interrupt enable bit for the OVF_CNT_CH3 interrupt.
    #[inline(always)]
    pub fn ovf_cnt_ch3(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - The interrupt enable bit for the OVF_CNT_CH4 interrupt.
    #[inline(always)]
    pub fn ovf_cnt_ch4(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - The interrupt enable bit for the OVF_CNT_CH5 interrupt.
    #[inline(always)]
    pub fn ovf_cnt_ch5(&self) -> OVF_CNT_CH_R {
        OVF_CNT_CH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("timer0_ovf", &self.timer0_ovf())
            .field("timer1_ovf", &self.timer1_ovf())
            .field("timer2_ovf", &self.timer2_ovf())
            .field("timer3_ovf", &self.timer3_ovf())
            .field("duty_chng_end_ch0", &self.duty_chng_end_ch0())
            .field("duty_chng_end_ch1", &self.duty_chng_end_ch1())
            .field("duty_chng_end_ch2", &self.duty_chng_end_ch2())
            .field("duty_chng_end_ch3", &self.duty_chng_end_ch3())
            .field("duty_chng_end_ch4", &self.duty_chng_end_ch4())
            .field("duty_chng_end_ch5", &self.duty_chng_end_ch5())
            .field("ovf_cnt_ch0", &self.ovf_cnt_ch0())
            .field("ovf_cnt_ch1", &self.ovf_cnt_ch1())
            .field("ovf_cnt_ch2", &self.ovf_cnt_ch2())
            .field("ovf_cnt_ch3", &self.ovf_cnt_ch3())
            .field("ovf_cnt_ch4", &self.ovf_cnt_ch4())
            .field("ovf_cnt_ch5", &self.ovf_cnt_ch5())
            .finish()
    }
}
impl W {
    ///The interrupt enable bit for the TIMER(0-3)_OVF interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `TIMER0_OVF` field
    #[inline(always)]
    #[must_use]
    pub fn timer_ovf(&mut self, n: u8) -> TIMER_OVF_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TIMER_OVF_W::new(self, n)
    }
    ///Bit 0 - The interrupt enable bit for the TIMER0_OVF interrupt.
    #[inline(always)]
    #[must_use]
    pub fn timer0_ovf(&mut self) -> TIMER_OVF_W<INT_ENA_SPEC> {
        TIMER_OVF_W::new(self, 0)
    }
    ///Bit 1 - The interrupt enable bit for the TIMER1_OVF interrupt.
    #[inline(always)]
    #[must_use]
    pub fn timer1_ovf(&mut self) -> TIMER_OVF_W<INT_ENA_SPEC> {
        TIMER_OVF_W::new(self, 1)
    }
    ///Bit 2 - The interrupt enable bit for the TIMER2_OVF interrupt.
    #[inline(always)]
    #[must_use]
    pub fn timer2_ovf(&mut self) -> TIMER_OVF_W<INT_ENA_SPEC> {
        TIMER_OVF_W::new(self, 2)
    }
    ///Bit 3 - The interrupt enable bit for the TIMER3_OVF interrupt.
    #[inline(always)]
    #[must_use]
    pub fn timer3_ovf(&mut self) -> TIMER_OVF_W<INT_ENA_SPEC> {
        TIMER_OVF_W::new(self, 3)
    }
    ///The interrupt enable bit for the DUTY_CHNG_END_CH(0-5) interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_CH0` field
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch(&mut self, n: u8) -> DUTY_CHNG_END_CH_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        DUTY_CHNG_END_CH_W::new(self, n + 4)
    }
    ///Bit 4 - The interrupt enable bit for the DUTY_CHNG_END_CH0 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch0(&mut self) -> DUTY_CHNG_END_CH_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH_W::new(self, 4)
    }
    ///Bit 5 - The interrupt enable bit for the DUTY_CHNG_END_CH1 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch1(&mut self) -> DUTY_CHNG_END_CH_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH_W::new(self, 5)
    }
    ///Bit 6 - The interrupt enable bit for the DUTY_CHNG_END_CH2 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch2(&mut self) -> DUTY_CHNG_END_CH_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH_W::new(self, 6)
    }
    ///Bit 7 - The interrupt enable bit for the DUTY_CHNG_END_CH3 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch3(&mut self) -> DUTY_CHNG_END_CH_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH_W::new(self, 7)
    }
    ///Bit 8 - The interrupt enable bit for the DUTY_CHNG_END_CH4 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch4(&mut self) -> DUTY_CHNG_END_CH_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH_W::new(self, 8)
    }
    ///Bit 9 - The interrupt enable bit for the DUTY_CHNG_END_CH5 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch5(&mut self) -> DUTY_CHNG_END_CH_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH_W::new(self, 9)
    }
    ///The interrupt enable bit for the OVF_CNT_CH(0-5) interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `OVF_CNT_CH0` field
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch(&mut self, n: u8) -> OVF_CNT_CH_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OVF_CNT_CH_W::new(self, n + 10)
    }
    ///Bit 10 - The interrupt enable bit for the OVF_CNT_CH0 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch0(&mut self) -> OVF_CNT_CH_W<INT_ENA_SPEC> {
        OVF_CNT_CH_W::new(self, 10)
    }
    ///Bit 11 - The interrupt enable bit for the OVF_CNT_CH1 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch1(&mut self) -> OVF_CNT_CH_W<INT_ENA_SPEC> {
        OVF_CNT_CH_W::new(self, 11)
    }
    ///Bit 12 - The interrupt enable bit for the OVF_CNT_CH2 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch2(&mut self) -> OVF_CNT_CH_W<INT_ENA_SPEC> {
        OVF_CNT_CH_W::new(self, 12)
    }
    ///Bit 13 - The interrupt enable bit for the OVF_CNT_CH3 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch3(&mut self) -> OVF_CNT_CH_W<INT_ENA_SPEC> {
        OVF_CNT_CH_W::new(self, 13)
    }
    ///Bit 14 - The interrupt enable bit for the OVF_CNT_CH4 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch4(&mut self) -> OVF_CNT_CH_W<INT_ENA_SPEC> {
        OVF_CNT_CH_W::new(self, 14)
    }
    ///Bit 15 - The interrupt enable bit for the OVF_CNT_CH5 interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch5(&mut self) -> OVF_CNT_CH_W<INT_ENA_SPEC> {
        OVF_CNT_CH_W::new(self, 15)
    }
}
/**Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
