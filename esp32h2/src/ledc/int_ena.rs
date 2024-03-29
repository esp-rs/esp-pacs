#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `TIMER0_OVF` reader - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
pub type TIMER0_OVF_R = crate::BitReader;
#[doc = "Field `TIMER0_OVF` writer - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
pub type TIMER0_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_OVF` reader - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
pub type TIMER1_OVF_R = crate::BitReader;
#[doc = "Field `TIMER1_OVF` writer - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
pub type TIMER1_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_OVF` reader - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
pub type TIMER2_OVF_R = crate::BitReader;
#[doc = "Field `TIMER2_OVF` writer - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
pub type TIMER2_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_OVF` reader - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
pub type TIMER3_OVF_R = crate::BitReader;
#[doc = "Field `TIMER3_OVF` writer - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
pub type TIMER3_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH0` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
pub type DUTY_CHNG_END_CH0_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH0` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
pub type DUTY_CHNG_END_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH1` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
pub type DUTY_CHNG_END_CH1_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH1` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
pub type DUTY_CHNG_END_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH2` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
pub type DUTY_CHNG_END_CH2_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH2` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
pub type DUTY_CHNG_END_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH3` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
pub type DUTY_CHNG_END_CH3_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH3` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
pub type DUTY_CHNG_END_CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH4` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
pub type DUTY_CHNG_END_CH4_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH4` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
pub type DUTY_CHNG_END_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH5` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
pub type DUTY_CHNG_END_CH5_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH5` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
pub type DUTY_CHNG_END_CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH0` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
pub type OVF_CNT_CH0_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH0` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
pub type OVF_CNT_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH1` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
pub type OVF_CNT_CH1_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH1` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
pub type OVF_CNT_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH2` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
pub type OVF_CNT_CH2_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH2` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
pub type OVF_CNT_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH3` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
pub type OVF_CNT_CH3_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH3` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
pub type OVF_CNT_CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH4` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
pub type OVF_CNT_CH4_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH4` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
pub type OVF_CNT_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH5` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
pub type OVF_CNT_CH5_R = crate::BitReader;
#[doc = "Field `OVF_CNT_CH5` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
pub type OVF_CNT_CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer0_ovf(&self) -> TIMER0_OVF_R {
        TIMER0_OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer1_ovf(&self) -> TIMER1_OVF_R {
        TIMER1_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer2_ovf(&self) -> TIMER2_OVF_R {
        TIMER2_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer3_ovf(&self) -> TIMER3_OVF_R {
        TIMER3_OVF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch0(&self) -> DUTY_CHNG_END_CH0_R {
        DUTY_CHNG_END_CH0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch1(&self) -> DUTY_CHNG_END_CH1_R {
        DUTY_CHNG_END_CH1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch2(&self) -> DUTY_CHNG_END_CH2_R {
        DUTY_CHNG_END_CH2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch3(&self) -> DUTY_CHNG_END_CH3_R {
        DUTY_CHNG_END_CH3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch4(&self) -> DUTY_CHNG_END_CH4_R {
        DUTY_CHNG_END_CH4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch5(&self) -> DUTY_CHNG_END_CH5_R {
        DUTY_CHNG_END_CH5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch0(&self) -> OVF_CNT_CH0_R {
        OVF_CNT_CH0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch1(&self) -> OVF_CNT_CH1_R {
        OVF_CNT_CH1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch2(&self) -> OVF_CNT_CH2_R {
        OVF_CNT_CH2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch3(&self) -> OVF_CNT_CH3_R {
        OVF_CNT_CH3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch4(&self) -> OVF_CNT_CH4_R {
        OVF_CNT_CH4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch5(&self) -> OVF_CNT_CH5_R {
        OVF_CNT_CH5_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("timer0_ovf", &format_args!("{}", self.timer0_ovf().bit()))
            .field("timer1_ovf", &format_args!("{}", self.timer1_ovf().bit()))
            .field("timer2_ovf", &format_args!("{}", self.timer2_ovf().bit()))
            .field("timer3_ovf", &format_args!("{}", self.timer3_ovf().bit()))
            .field(
                "duty_chng_end_ch0",
                &format_args!("{}", self.duty_chng_end_ch0().bit()),
            )
            .field(
                "duty_chng_end_ch1",
                &format_args!("{}", self.duty_chng_end_ch1().bit()),
            )
            .field(
                "duty_chng_end_ch2",
                &format_args!("{}", self.duty_chng_end_ch2().bit()),
            )
            .field(
                "duty_chng_end_ch3",
                &format_args!("{}", self.duty_chng_end_ch3().bit()),
            )
            .field(
                "duty_chng_end_ch4",
                &format_args!("{}", self.duty_chng_end_ch4().bit()),
            )
            .field(
                "duty_chng_end_ch5",
                &format_args!("{}", self.duty_chng_end_ch5().bit()),
            )
            .field("ovf_cnt_ch0", &format_args!("{}", self.ovf_cnt_ch0().bit()))
            .field("ovf_cnt_ch1", &format_args!("{}", self.ovf_cnt_ch1().bit()))
            .field("ovf_cnt_ch2", &format_args!("{}", self.ovf_cnt_ch2().bit()))
            .field("ovf_cnt_ch3", &format_args!("{}", self.ovf_cnt_ch3().bit()))
            .field("ovf_cnt_ch4", &format_args!("{}", self.ovf_cnt_ch4().bit()))
            .field("ovf_cnt_ch5", &format_args!("{}", self.ovf_cnt_ch5().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ovf(&mut self) -> TIMER0_OVF_W<INT_ENA_SPEC> {
        TIMER0_OVF_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ovf(&mut self) -> TIMER1_OVF_W<INT_ENA_SPEC> {
        TIMER1_OVF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_ovf(&mut self) -> TIMER2_OVF_W<INT_ENA_SPEC> {
        TIMER2_OVF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn timer3_ovf(&mut self) -> TIMER3_OVF_W<INT_ENA_SPEC> {
        TIMER3_OVF_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch0(&mut self) -> DUTY_CHNG_END_CH0_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH0_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch1(&mut self) -> DUTY_CHNG_END_CH1_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH1_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch2(&mut self) -> DUTY_CHNG_END_CH2_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH2_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch3(&mut self) -> DUTY_CHNG_END_CH3_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH3_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch4(&mut self) -> DUTY_CHNG_END_CH4_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH4_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_ch5(&mut self) -> DUTY_CHNG_END_CH5_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_CH5_W::new(self, 9)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch0(&mut self) -> OVF_CNT_CH0_W<INT_ENA_SPEC> {
        OVF_CNT_CH0_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch1(&mut self) -> OVF_CNT_CH1_W<INT_ENA_SPEC> {
        OVF_CNT_CH1_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch2(&mut self) -> OVF_CNT_CH2_W<INT_ENA_SPEC> {
        OVF_CNT_CH2_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch3(&mut self) -> OVF_CNT_CH3_W<INT_ENA_SPEC> {
        OVF_CNT_CH3_W::new(self, 15)
    }
    #[doc = "Bit 16 - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch4(&mut self) -> OVF_CNT_CH4_W<INT_ENA_SPEC> {
        OVF_CNT_CH4_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_cnt_ch5(&mut self) -> OVF_CNT_CH5_W<INT_ENA_SPEC> {
        OVF_CNT_CH5_W::new(self, 17)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
