#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `HSTIMER0_OVF` reader - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
pub type HSTIMER0_OVF_R = crate::BitReader;
#[doc = "Field `HSTIMER0_OVF` writer - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
pub type HSTIMER0_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTIMER1_OVF` reader - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
pub type HSTIMER1_OVF_R = crate::BitReader;
#[doc = "Field `HSTIMER1_OVF` writer - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
pub type HSTIMER1_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTIMER2_OVF` reader - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
pub type HSTIMER2_OVF_R = crate::BitReader;
#[doc = "Field `HSTIMER2_OVF` writer - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
pub type HSTIMER2_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTIMER3_OVF` reader - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
pub type HSTIMER3_OVF_R = crate::BitReader;
#[doc = "Field `HSTIMER3_OVF` writer - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
pub type HSTIMER3_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER0_OVF` reader - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
pub type LSTIMER0_OVF_R = crate::BitReader;
#[doc = "Field `LSTIMER0_OVF` writer - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
pub type LSTIMER0_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER1_OVF` reader - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
pub type LSTIMER1_OVF_R = crate::BitReader;
#[doc = "Field `LSTIMER1_OVF` writer - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
pub type LSTIMER1_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER2_OVF` reader - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
pub type LSTIMER2_OVF_R = crate::BitReader;
#[doc = "Field `LSTIMER2_OVF` writer - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
pub type LSTIMER2_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER3_OVF` reader - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
pub type LSTIMER3_OVF_R = crate::BitReader;
#[doc = "Field `LSTIMER3_OVF` writer - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
pub type LSTIMER3_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH0` reader - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH0_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH0` writer - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH1` reader - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH1_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH1` writer - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH2` reader - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH2_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH2` writer - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH3` reader - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH3_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH3` writer - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH4` reader - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH4_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH4` writer - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH5` reader - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH5_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH5` writer - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH6` reader - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH6_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH6` writer - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH7` reader - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH7_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH7` writer - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH0` reader - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH0_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH0` writer - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH1` reader - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH1_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH1` writer - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH2` reader - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH2_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH2` writer - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH3` reader - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH3_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH3` writer - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH4` reader - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH4_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH4` writer - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH5` reader - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH5_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH5` writer - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH6` reader - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH6_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH6` writer - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH7` reader - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH7_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH7` writer - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer0_ovf(&self) -> HSTIMER0_OVF_R {
        HSTIMER0_OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer1_ovf(&self) -> HSTIMER1_OVF_R {
        HSTIMER1_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer2_ovf(&self) -> HSTIMER2_OVF_R {
        HSTIMER2_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer3_ovf(&self) -> HSTIMER3_OVF_R {
        HSTIMER3_OVF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer0_ovf(&self) -> LSTIMER0_OVF_R {
        LSTIMER0_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer1_ovf(&self) -> LSTIMER1_OVF_R {
        LSTIMER1_OVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer2_ovf(&self) -> LSTIMER2_OVF_R {
        LSTIMER2_OVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer3_ovf(&self) -> LSTIMER3_OVF_R {
        LSTIMER3_OVF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0(&self) -> DUTY_CHNG_END_HSCH0_R {
        DUTY_CHNG_END_HSCH0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1(&self) -> DUTY_CHNG_END_HSCH1_R {
        DUTY_CHNG_END_HSCH1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2(&self) -> DUTY_CHNG_END_HSCH2_R {
        DUTY_CHNG_END_HSCH2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3(&self) -> DUTY_CHNG_END_HSCH3_R {
        DUTY_CHNG_END_HSCH3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4(&self) -> DUTY_CHNG_END_HSCH4_R {
        DUTY_CHNG_END_HSCH4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5(&self) -> DUTY_CHNG_END_HSCH5_R {
        DUTY_CHNG_END_HSCH5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6(&self) -> DUTY_CHNG_END_HSCH6_R {
        DUTY_CHNG_END_HSCH6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7(&self) -> DUTY_CHNG_END_HSCH7_R {
        DUTY_CHNG_END_HSCH7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0(&self) -> DUTY_CHNG_END_LSCH0_R {
        DUTY_CHNG_END_LSCH0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1(&self) -> DUTY_CHNG_END_LSCH1_R {
        DUTY_CHNG_END_LSCH1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2(&self) -> DUTY_CHNG_END_LSCH2_R {
        DUTY_CHNG_END_LSCH2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3(&self) -> DUTY_CHNG_END_LSCH3_R {
        DUTY_CHNG_END_LSCH3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4(&self) -> DUTY_CHNG_END_LSCH4_R {
        DUTY_CHNG_END_LSCH4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5(&self) -> DUTY_CHNG_END_LSCH5_R {
        DUTY_CHNG_END_LSCH5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6(&self) -> DUTY_CHNG_END_LSCH6_R {
        DUTY_CHNG_END_LSCH6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7(&self) -> DUTY_CHNG_END_LSCH7_R {
        DUTY_CHNG_END_LSCH7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "hstimer0_ovf",
                &format_args!("{}", self.hstimer0_ovf().bit()),
            )
            .field(
                "hstimer1_ovf",
                &format_args!("{}", self.hstimer1_ovf().bit()),
            )
            .field(
                "hstimer2_ovf",
                &format_args!("{}", self.hstimer2_ovf().bit()),
            )
            .field(
                "hstimer3_ovf",
                &format_args!("{}", self.hstimer3_ovf().bit()),
            )
            .field(
                "lstimer0_ovf",
                &format_args!("{}", self.lstimer0_ovf().bit()),
            )
            .field(
                "lstimer1_ovf",
                &format_args!("{}", self.lstimer1_ovf().bit()),
            )
            .field(
                "lstimer2_ovf",
                &format_args!("{}", self.lstimer2_ovf().bit()),
            )
            .field(
                "lstimer3_ovf",
                &format_args!("{}", self.lstimer3_ovf().bit()),
            )
            .field(
                "duty_chng_end_hsch0",
                &format_args!("{}", self.duty_chng_end_hsch0().bit()),
            )
            .field(
                "duty_chng_end_hsch1",
                &format_args!("{}", self.duty_chng_end_hsch1().bit()),
            )
            .field(
                "duty_chng_end_hsch2",
                &format_args!("{}", self.duty_chng_end_hsch2().bit()),
            )
            .field(
                "duty_chng_end_hsch3",
                &format_args!("{}", self.duty_chng_end_hsch3().bit()),
            )
            .field(
                "duty_chng_end_hsch4",
                &format_args!("{}", self.duty_chng_end_hsch4().bit()),
            )
            .field(
                "duty_chng_end_hsch5",
                &format_args!("{}", self.duty_chng_end_hsch5().bit()),
            )
            .field(
                "duty_chng_end_hsch6",
                &format_args!("{}", self.duty_chng_end_hsch6().bit()),
            )
            .field(
                "duty_chng_end_hsch7",
                &format_args!("{}", self.duty_chng_end_hsch7().bit()),
            )
            .field(
                "duty_chng_end_lsch0",
                &format_args!("{}", self.duty_chng_end_lsch0().bit()),
            )
            .field(
                "duty_chng_end_lsch1",
                &format_args!("{}", self.duty_chng_end_lsch1().bit()),
            )
            .field(
                "duty_chng_end_lsch2",
                &format_args!("{}", self.duty_chng_end_lsch2().bit()),
            )
            .field(
                "duty_chng_end_lsch3",
                &format_args!("{}", self.duty_chng_end_lsch3().bit()),
            )
            .field(
                "duty_chng_end_lsch4",
                &format_args!("{}", self.duty_chng_end_lsch4().bit()),
            )
            .field(
                "duty_chng_end_lsch5",
                &format_args!("{}", self.duty_chng_end_lsch5().bit()),
            )
            .field(
                "duty_chng_end_lsch6",
                &format_args!("{}", self.duty_chng_end_lsch6().bit()),
            )
            .field(
                "duty_chng_end_lsch7",
                &format_args!("{}", self.duty_chng_end_lsch7().bit()),
            )
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
    #[doc = "Bit 0 - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer0_ovf(&mut self) -> HSTIMER0_OVF_W<INT_ENA_SPEC> {
        HSTIMER0_OVF_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer1_ovf(&mut self) -> HSTIMER1_OVF_W<INT_ENA_SPEC> {
        HSTIMER1_OVF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer2_ovf(&mut self) -> HSTIMER2_OVF_W<INT_ENA_SPEC> {
        HSTIMER2_OVF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer3_ovf(&mut self) -> HSTIMER3_OVF_W<INT_ENA_SPEC> {
        HSTIMER3_OVF_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer0_ovf(&mut self) -> LSTIMER0_OVF_W<INT_ENA_SPEC> {
        LSTIMER0_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer1_ovf(&mut self) -> LSTIMER1_OVF_W<INT_ENA_SPEC> {
        LSTIMER1_OVF_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer2_ovf(&mut self) -> LSTIMER2_OVF_W<INT_ENA_SPEC> {
        LSTIMER2_OVF_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer3_ovf(&mut self) -> LSTIMER3_OVF_W<INT_ENA_SPEC> {
        LSTIMER3_OVF_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch0(&mut self) -> DUTY_CHNG_END_HSCH0_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH0_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch1(&mut self) -> DUTY_CHNG_END_HSCH1_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH1_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch2(&mut self) -> DUTY_CHNG_END_HSCH2_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH2_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch3(&mut self) -> DUTY_CHNG_END_HSCH3_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH3_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch4(&mut self) -> DUTY_CHNG_END_HSCH4_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH4_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch5(&mut self) -> DUTY_CHNG_END_HSCH5_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH5_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch6(&mut self) -> DUTY_CHNG_END_HSCH6_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH6_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch7(&mut self) -> DUTY_CHNG_END_HSCH7_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH7_W::new(self, 15)
    }
    #[doc = "Bit 16 - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch0(&mut self) -> DUTY_CHNG_END_LSCH0_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH0_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch1(&mut self) -> DUTY_CHNG_END_LSCH1_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH1_W::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch2(&mut self) -> DUTY_CHNG_END_LSCH2_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH2_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch3(&mut self) -> DUTY_CHNG_END_LSCH3_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH3_W::new(self, 19)
    }
    #[doc = "Bit 20 - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch4(&mut self) -> DUTY_CHNG_END_LSCH4_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH4_W::new(self, 20)
    }
    #[doc = "Bit 21 - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch5(&mut self) -> DUTY_CHNG_END_LSCH5_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH5_W::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch6(&mut self) -> DUTY_CHNG_END_LSCH6_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH6_W::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch7(&mut self) -> DUTY_CHNG_END_LSCH7_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH7_W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
