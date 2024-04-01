#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `HSTIMER_OVF(0-3)` reader - The interrupt raw bit for high speed channel%s counter overflow."]
pub type HSTIMER_OVF_R = crate::BitReader;
#[doc = "Field `HSTIMER_OVF(0-3)` writer - The interrupt raw bit for high speed channel%s counter overflow."]
pub type HSTIMER_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER_OVF(0-3)` reader - The interrupt raw bit for low speed channel%s counter overflow."]
pub type LSTIMER_OVF_R = crate::BitReader;
#[doc = "Field `LSTIMER_OVF(0-3)` writer - The interrupt raw bit for low speed channel%s counter overflow."]
pub type LSTIMER_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH(0-7)` reader - The interrupt raw bit for high speed channel %s duty change done."]
pub type DUTY_CHNG_END_HSCH_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH(0-7)` writer - The interrupt raw bit for high speed channel %s duty change done."]
pub type DUTY_CHNG_END_HSCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH(0-7)` reader - The interrupt raw bit for low speed channel %s duty change done."]
pub type DUTY_CHNG_END_LSCH_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH(0-7)` writer - The interrupt raw bit for low speed channel %s duty change done."]
pub type DUTY_CHNG_END_LSCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "The interrupt raw bit for high speed channel(0-3) counter overflow."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `HSTIMER0_OVF` field"]
    #[inline(always)]
    pub fn hstimer_ovf(&self, n: u8) -> HSTIMER_OVF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        HSTIMER_OVF_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for high speed channel(0-3) counter overflow."]
    #[inline(always)]
    pub fn hstimer_ovf_iter(&self) -> impl Iterator<Item = HSTIMER_OVF_R> + '_ {
        (0..4).map(move |n| HSTIMER_OVF_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The interrupt raw bit for high speed channel0 counter overflow."]
    #[inline(always)]
    pub fn hstimer0_ovf(&self) -> HSTIMER_OVF_R {
        HSTIMER_OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for high speed channel1 counter overflow."]
    #[inline(always)]
    pub fn hstimer1_ovf(&self) -> HSTIMER_OVF_R {
        HSTIMER_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for high speed channel2 counter overflow."]
    #[inline(always)]
    pub fn hstimer2_ovf(&self) -> HSTIMER_OVF_R {
        HSTIMER_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for high speed channel3 counter overflow."]
    #[inline(always)]
    pub fn hstimer3_ovf(&self) -> HSTIMER_OVF_R {
        HSTIMER_OVF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for low speed channel(0-3) counter overflow."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `LSTIMER0_OVF` field"]
    #[inline(always)]
    pub fn lstimer_ovf(&self, n: u8) -> LSTIMER_OVF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        LSTIMER_OVF_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for low speed channel(0-3) counter overflow."]
    #[inline(always)]
    pub fn lstimer_ovf_iter(&self) -> impl Iterator<Item = LSTIMER_OVF_R> + '_ {
        (0..4).map(move |n| LSTIMER_OVF_R::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - The interrupt raw bit for low speed channel0 counter overflow."]
    #[inline(always)]
    pub fn lstimer0_ovf(&self) -> LSTIMER_OVF_R {
        LSTIMER_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for low speed channel1 counter overflow."]
    #[inline(always)]
    pub fn lstimer1_ovf(&self) -> LSTIMER_OVF_R {
        LSTIMER_OVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for low speed channel2 counter overflow."]
    #[inline(always)]
    pub fn lstimer2_ovf(&self) -> LSTIMER_OVF_R {
        LSTIMER_OVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for low speed channel3 counter overflow."]
    #[inline(always)]
    pub fn lstimer3_ovf(&self) -> LSTIMER_OVF_R {
        LSTIMER_OVF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for high speed channel (0-7) duty change done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_HSCH0` field"]
    #[inline(always)]
    pub fn duty_chng_end_hsch(&self, n: u8) -> DUTY_CHNG_END_HSCH_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DUTY_CHNG_END_HSCH_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for high speed channel (0-7) duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch_iter(&self) -> impl Iterator<Item = DUTY_CHNG_END_HSCH_R> + '_ {
        (0..8).map(move |n| DUTY_CHNG_END_HSCH_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - The interrupt raw bit for high speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0(&self) -> DUTY_CHNG_END_HSCH_R {
        DUTY_CHNG_END_HSCH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for high speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1(&self) -> DUTY_CHNG_END_HSCH_R {
        DUTY_CHNG_END_HSCH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for high speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2(&self) -> DUTY_CHNG_END_HSCH_R {
        DUTY_CHNG_END_HSCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for high speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3(&self) -> DUTY_CHNG_END_HSCH_R {
        DUTY_CHNG_END_HSCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for high speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4(&self) -> DUTY_CHNG_END_HSCH_R {
        DUTY_CHNG_END_HSCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for high speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5(&self) -> DUTY_CHNG_END_HSCH_R {
        DUTY_CHNG_END_HSCH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for high speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6(&self) -> DUTY_CHNG_END_HSCH_R {
        DUTY_CHNG_END_HSCH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for high speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7(&self) -> DUTY_CHNG_END_HSCH_R {
        DUTY_CHNG_END_HSCH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for low speed channel (0-7) duty change done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_LSCH0` field"]
    #[inline(always)]
    pub fn duty_chng_end_lsch(&self, n: u8) -> DUTY_CHNG_END_LSCH_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DUTY_CHNG_END_LSCH_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for low speed channel (0-7) duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch_iter(&self) -> impl Iterator<Item = DUTY_CHNG_END_LSCH_R> + '_ {
        (0..8).map(move |n| DUTY_CHNG_END_LSCH_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - The interrupt raw bit for low speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0(&self) -> DUTY_CHNG_END_LSCH_R {
        DUTY_CHNG_END_LSCH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for low speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1(&self) -> DUTY_CHNG_END_LSCH_R {
        DUTY_CHNG_END_LSCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for low speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2(&self) -> DUTY_CHNG_END_LSCH_R {
        DUTY_CHNG_END_LSCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for low speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3(&self) -> DUTY_CHNG_END_LSCH_R {
        DUTY_CHNG_END_LSCH_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for low speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4(&self) -> DUTY_CHNG_END_LSCH_R {
        DUTY_CHNG_END_LSCH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for low speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5(&self) -> DUTY_CHNG_END_LSCH_R {
        DUTY_CHNG_END_LSCH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for low speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6(&self) -> DUTY_CHNG_END_LSCH_R {
        DUTY_CHNG_END_LSCH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for low speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7(&self) -> DUTY_CHNG_END_LSCH_R {
        DUTY_CHNG_END_LSCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
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
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "The interrupt raw bit for high speed channel(0-3) counter overflow."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `HSTIMER0_OVF` field"]
    #[inline(always)]
    #[must_use]
    pub fn hstimer_ovf(&mut self, n: u8) -> HSTIMER_OVF_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        HSTIMER_OVF_W::new(self, n)
    }
    #[doc = "Bit 0 - The interrupt raw bit for high speed channel0 counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer0_ovf(&mut self) -> HSTIMER_OVF_W<INT_RAW_SPEC> {
        HSTIMER_OVF_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for high speed channel1 counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer1_ovf(&mut self) -> HSTIMER_OVF_W<INT_RAW_SPEC> {
        HSTIMER_OVF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt raw bit for high speed channel2 counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer2_ovf(&mut self) -> HSTIMER_OVF_W<INT_RAW_SPEC> {
        HSTIMER_OVF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt raw bit for high speed channel3 counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer3_ovf(&mut self) -> HSTIMER_OVF_W<INT_RAW_SPEC> {
        HSTIMER_OVF_W::new(self, 3)
    }
    #[doc = "The interrupt raw bit for low speed channel(0-3) counter overflow."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `LSTIMER0_OVF` field"]
    #[inline(always)]
    #[must_use]
    pub fn lstimer_ovf(&mut self, n: u8) -> LSTIMER_OVF_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        LSTIMER_OVF_W::new(self, n + 4)
    }
    #[doc = "Bit 4 - The interrupt raw bit for low speed channel0 counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer0_ovf(&mut self) -> LSTIMER_OVF_W<INT_RAW_SPEC> {
        LSTIMER_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt raw bit for low speed channel1 counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer1_ovf(&mut self) -> LSTIMER_OVF_W<INT_RAW_SPEC> {
        LSTIMER_OVF_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt raw bit for low speed channel2 counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer2_ovf(&mut self) -> LSTIMER_OVF_W<INT_RAW_SPEC> {
        LSTIMER_OVF_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt raw bit for low speed channel3 counter overflow."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer3_ovf(&mut self) -> LSTIMER_OVF_W<INT_RAW_SPEC> {
        LSTIMER_OVF_W::new(self, 7)
    }
    #[doc = "The interrupt raw bit for high speed channel (0-7) duty change done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_HSCH0` field"]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch(&mut self, n: u8) -> DUTY_CHNG_END_HSCH_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DUTY_CHNG_END_HSCH_W::new(self, n + 8)
    }
    #[doc = "Bit 8 - The interrupt raw bit for high speed channel 0 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch0(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt raw bit for high speed channel 1 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch1(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt raw bit for high speed channel 2 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch2(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt raw bit for high speed channel 3 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch3(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt raw bit for high speed channel 4 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch4(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt raw bit for high speed channel 5 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch5(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt raw bit for high speed channel 6 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch6(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt raw bit for high speed channel 7 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch7(&mut self) -> DUTY_CHNG_END_HSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_HSCH_W::new(self, 15)
    }
    #[doc = "The interrupt raw bit for low speed channel (0-7) duty change done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_LSCH0` field"]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch(&mut self, n: u8) -> DUTY_CHNG_END_LSCH_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DUTY_CHNG_END_LSCH_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - The interrupt raw bit for low speed channel 0 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch0(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt raw bit for low speed channel 1 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch1(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt raw bit for low speed channel 2 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch2(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt raw bit for low speed channel 3 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch3(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 19)
    }
    #[doc = "Bit 20 - The interrupt raw bit for low speed channel 4 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch4(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 20)
    }
    #[doc = "Bit 21 - The interrupt raw bit for low speed channel 5 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch5(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt raw bit for low speed channel 6 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch6(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt raw bit for low speed channel 7 duty change done."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch7(&mut self) -> DUTY_CHNG_END_LSCH_W<INT_RAW_SPEC> {
        DUTY_CHNG_END_LSCH_W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
