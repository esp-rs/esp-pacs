#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `HSTIMER0_OVF_INT_CLR` writer - Set this bit to clear high speed channel0 counter overflow interrupt."]
pub type HSTIMER0_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSTIMER1_OVF_INT_CLR` writer - Set this bit to clear high speed channel1 counter overflow interrupt."]
pub type HSTIMER1_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSTIMER2_OVF_INT_CLR` writer - Set this bit to clear high speed channel2 counter overflow interrupt."]
pub type HSTIMER2_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSTIMER3_OVF_INT_CLR` writer - Set this bit to clear high speed channel3 counter overflow interrupt."]
pub type HSTIMER3_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSTIMER0_OVF_INT_CLR` writer - Set this bit to clear low speed channel0 counter overflow interrupt."]
pub type LSTIMER0_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSTIMER1_OVF_INT_CLR` writer - Set this bit to clear low speed channel1 counter overflow interrupt."]
pub type LSTIMER1_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSTIMER2_OVF_INT_CLR` writer - Set this bit to clear low speed channel2 counter overflow interrupt."]
pub type LSTIMER2_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSTIMER3_OVF_INT_CLR` writer - Set this bit to clear low speed channel3 counter overflow interrupt."]
pub type LSTIMER3_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH0_INT_CLR` writer - Set this bit to clear high speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH1_INT_CLR` writer - Set this bit to clear high speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH1_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH2_INT_CLR` writer - Set this bit to clear high speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH2_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH3_INT_CLR` writer - Set this bit to clear high speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH3_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH4_INT_CLR` writer - Set this bit to clear high speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH4_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH5_INT_CLR` writer - Set this bit to clear high speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH5_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH6_INT_CLR` writer - Set this bit to clear high speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH6_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH7_INT_CLR` writer - Set this bit to clear high speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH7_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_CLR` writer - Set this bit to clear low speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_CLR` writer - Set this bit to clear low speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH1_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_CLR` writer - Set this bit to clear low speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH2_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_CLR` writer - Set this bit to clear low speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH3_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_CLR` writer - Set this bit to clear low speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH4_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_CLR` writer - Set this bit to clear low speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH5_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH6_INT_CLR` writer - Set this bit to clear low speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH6_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH7_INT_CLR` writer - Set this bit to clear low speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH7_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn hstimer0_ovf_int_clr(&mut self) -> HSTIMER0_OVF_INT_CLR_W<INT_CLR_SPEC, 0> {
        HSTIMER0_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer1_ovf_int_clr(&mut self) -> HSTIMER1_OVF_INT_CLR_W<INT_CLR_SPEC, 1> {
        HSTIMER1_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer2_ovf_int_clr(&mut self) -> HSTIMER2_OVF_INT_CLR_W<INT_CLR_SPEC, 2> {
        HSTIMER2_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer3_ovf_int_clr(&mut self) -> HSTIMER3_OVF_INT_CLR_W<INT_CLR_SPEC, 3> {
        HSTIMER3_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer0_ovf_int_clr(&mut self) -> LSTIMER0_OVF_INT_CLR_W<INT_CLR_SPEC, 4> {
        LSTIMER0_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer1_ovf_int_clr(&mut self) -> LSTIMER1_OVF_INT_CLR_W<INT_CLR_SPEC, 5> {
        LSTIMER1_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer2_ovf_int_clr(&mut self) -> LSTIMER2_OVF_INT_CLR_W<INT_CLR_SPEC, 6> {
        LSTIMER2_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer3_ovf_int_clr(&mut self) -> LSTIMER3_OVF_INT_CLR_W<INT_CLR_SPEC, 7> {
        LSTIMER3_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch0_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_HSCH0_INT_CLR_W<INT_CLR_SPEC, 8> {
        DUTY_CHNG_END_HSCH0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch1_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_HSCH1_INT_CLR_W<INT_CLR_SPEC, 9> {
        DUTY_CHNG_END_HSCH1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch2_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_HSCH2_INT_CLR_W<INT_CLR_SPEC, 10> {
        DUTY_CHNG_END_HSCH2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch3_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_HSCH3_INT_CLR_W<INT_CLR_SPEC, 11> {
        DUTY_CHNG_END_HSCH3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch4_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_HSCH4_INT_CLR_W<INT_CLR_SPEC, 12> {
        DUTY_CHNG_END_HSCH4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch5_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_HSCH5_INT_CLR_W<INT_CLR_SPEC, 13> {
        DUTY_CHNG_END_HSCH5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch6_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_HSCH6_INT_CLR_W<INT_CLR_SPEC, 14> {
        DUTY_CHNG_END_HSCH6_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch7_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_HSCH7_INT_CLR_W<INT_CLR_SPEC, 15> {
        DUTY_CHNG_END_HSCH7_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to clear low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch0_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_LSCH0_INT_CLR_W<INT_CLR_SPEC, 16> {
        DUTY_CHNG_END_LSCH0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to clear low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch1_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_LSCH1_INT_CLR_W<INT_CLR_SPEC, 17> {
        DUTY_CHNG_END_LSCH1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to clear low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch2_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_LSCH2_INT_CLR_W<INT_CLR_SPEC, 18> {
        DUTY_CHNG_END_LSCH2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to clear low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch3_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_LSCH3_INT_CLR_W<INT_CLR_SPEC, 19> {
        DUTY_CHNG_END_LSCH3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to clear low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch4_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_LSCH4_INT_CLR_W<INT_CLR_SPEC, 20> {
        DUTY_CHNG_END_LSCH4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to clear low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch5_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_LSCH5_INT_CLR_W<INT_CLR_SPEC, 21> {
        DUTY_CHNG_END_LSCH5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to clear low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch6_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_LSCH6_INT_CLR_W<INT_CLR_SPEC, 22> {
        DUTY_CHNG_END_LSCH6_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to clear low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch7_int_clr(
        &mut self,
    ) -> DUTY_CHNG_END_LSCH7_INT_CLR_W<INT_CLR_SPEC, 23> {
        DUTY_CHNG_END_LSCH7_INT_CLR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
