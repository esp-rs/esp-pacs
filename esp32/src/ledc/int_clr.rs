#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `HSTIMER0_OVF` writer - Set this bit to clear high speed channel0 counter overflow interrupt."]
pub type HSTIMER0_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HSTIMER1_OVF` writer - Set this bit to clear high speed channel1 counter overflow interrupt."]
pub type HSTIMER1_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HSTIMER2_OVF` writer - Set this bit to clear high speed channel2 counter overflow interrupt."]
pub type HSTIMER2_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HSTIMER3_OVF` writer - Set this bit to clear high speed channel3 counter overflow interrupt."]
pub type HSTIMER3_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LSTIMER0_OVF` writer - Set this bit to clear low speed channel0 counter overflow interrupt."]
pub type LSTIMER0_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LSTIMER1_OVF` writer - Set this bit to clear low speed channel1 counter overflow interrupt."]
pub type LSTIMER1_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LSTIMER2_OVF` writer - Set this bit to clear low speed channel2 counter overflow interrupt."]
pub type LSTIMER2_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LSTIMER3_OVF` writer - Set this bit to clear low speed channel3 counter overflow interrupt."]
pub type LSTIMER3_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH0` writer - Set this bit to clear high speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH1` writer - Set this bit to clear high speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH2` writer - Set this bit to clear high speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH3` writer - Set this bit to clear high speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH4` writer - Set this bit to clear high speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH4_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH5` writer - Set this bit to clear high speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH5_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH6` writer - Set this bit to clear high speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH6_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH7` writer - Set this bit to clear high speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH7_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH0` writer - Set this bit to clear low speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH1` writer - Set this bit to clear low speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH2` writer - Set this bit to clear low speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH3` writer - Set this bit to clear low speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH4` writer - Set this bit to clear low speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH4_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH5` writer - Set this bit to clear low speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH5_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH6` writer - Set this bit to clear low speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH6_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH7` writer - Set this bit to clear low speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH7_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear high speed channel0 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer0_ovf(&mut self) -> HSTIMER0_OVF_W<INT_CLR_SPEC> {
        HSTIMER0_OVF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer1_ovf(&mut self) -> HSTIMER1_OVF_W<INT_CLR_SPEC> {
        HSTIMER1_OVF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer2_ovf(&mut self) -> HSTIMER2_OVF_W<INT_CLR_SPEC> {
        HSTIMER2_OVF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer3_ovf(&mut self) -> HSTIMER3_OVF_W<INT_CLR_SPEC> {
        HSTIMER3_OVF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer0_ovf(&mut self) -> LSTIMER0_OVF_W<INT_CLR_SPEC> {
        LSTIMER0_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer1_ovf(&mut self) -> LSTIMER1_OVF_W<INT_CLR_SPEC> {
        LSTIMER1_OVF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer2_ovf(&mut self) -> LSTIMER2_OVF_W<INT_CLR_SPEC> {
        LSTIMER2_OVF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer3_ovf(&mut self) -> LSTIMER3_OVF_W<INT_CLR_SPEC> {
        LSTIMER3_OVF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch0(&mut self) -> DUTY_CHNG_END_HSCH0_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch1(&mut self) -> DUTY_CHNG_END_HSCH1_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch2(&mut self) -> DUTY_CHNG_END_HSCH2_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to clear high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch3(&mut self) -> DUTY_CHNG_END_HSCH3_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to clear high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch4(&mut self) -> DUTY_CHNG_END_HSCH4_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to clear high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch5(&mut self) -> DUTY_CHNG_END_HSCH5_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to clear high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch6(&mut self) -> DUTY_CHNG_END_HSCH6_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to clear high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch7(&mut self) -> DUTY_CHNG_END_HSCH7_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_HSCH7_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to clear low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch0(&mut self) -> DUTY_CHNG_END_LSCH0_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to clear low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch1(&mut self) -> DUTY_CHNG_END_LSCH1_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to clear low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch2(&mut self) -> DUTY_CHNG_END_LSCH2_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to clear low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch3(&mut self) -> DUTY_CHNG_END_LSCH3_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set this bit to clear low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch4(&mut self) -> DUTY_CHNG_END_LSCH4_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to clear low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch5(&mut self) -> DUTY_CHNG_END_LSCH5_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to clear low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch6(&mut self) -> DUTY_CHNG_END_LSCH6_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH6_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set this bit to clear low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch7(&mut self) -> DUTY_CHNG_END_LSCH7_W<INT_CLR_SPEC> {
        DUTY_CHNG_END_LSCH7_W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00ff_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
