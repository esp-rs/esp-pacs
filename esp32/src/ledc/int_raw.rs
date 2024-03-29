#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `HSTIMER0_OVF` reader - The interrupt raw bit for high speed channel0 counter overflow."]
pub type HSTIMER0_OVF_R = crate::BitReader;
#[doc = "Field `HSTIMER1_OVF` reader - The interrupt raw bit for high speed channel1 counter overflow."]
pub type HSTIMER1_OVF_R = crate::BitReader;
#[doc = "Field `HSTIMER2_OVF` reader - The interrupt raw bit for high speed channel2 counter overflow."]
pub type HSTIMER2_OVF_R = crate::BitReader;
#[doc = "Field `HSTIMER3_OVF` reader - The interrupt raw bit for high speed channel3 counter overflow."]
pub type HSTIMER3_OVF_R = crate::BitReader;
#[doc = "Field `LSTIMER0_OVF` reader - The interrupt raw bit for low speed channel0 counter overflow."]
pub type LSTIMER0_OVF_R = crate::BitReader;
#[doc = "Field `LSTIMER1_OVF` reader - The interrupt raw bit for low speed channel1 counter overflow."]
pub type LSTIMER1_OVF_R = crate::BitReader;
#[doc = "Field `LSTIMER2_OVF` reader - The interrupt raw bit for low speed channel2 counter overflow."]
pub type LSTIMER2_OVF_R = crate::BitReader;
#[doc = "Field `LSTIMER3_OVF` reader - The interrupt raw bit for low speed channel3 counter overflow."]
pub type LSTIMER3_OVF_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH0` reader - The interrupt raw bit for high speed channel 0 duty change done."]
pub type DUTY_CHNG_END_HSCH0_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH1` reader - The interrupt raw bit for high speed channel 1 duty change done."]
pub type DUTY_CHNG_END_HSCH1_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH2` reader - The interrupt raw bit for high speed channel 2 duty change done."]
pub type DUTY_CHNG_END_HSCH2_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH3` reader - The interrupt raw bit for high speed channel 3 duty change done."]
pub type DUTY_CHNG_END_HSCH3_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH4` reader - The interrupt raw bit for high speed channel 4 duty change done."]
pub type DUTY_CHNG_END_HSCH4_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH5` reader - The interrupt raw bit for high speed channel 5 duty change done."]
pub type DUTY_CHNG_END_HSCH5_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH6` reader - The interrupt raw bit for high speed channel 6 duty change done."]
pub type DUTY_CHNG_END_HSCH6_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH7` reader - The interrupt raw bit for high speed channel 7 duty change done."]
pub type DUTY_CHNG_END_HSCH7_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH0` reader - The interrupt raw bit for low speed channel 0 duty change done."]
pub type DUTY_CHNG_END_LSCH0_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH1` reader - The interrupt raw bit for low speed channel 1 duty change done."]
pub type DUTY_CHNG_END_LSCH1_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH2` reader - The interrupt raw bit for low speed channel 2 duty change done."]
pub type DUTY_CHNG_END_LSCH2_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH3` reader - The interrupt raw bit for low speed channel 3 duty change done."]
pub type DUTY_CHNG_END_LSCH3_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH4` reader - The interrupt raw bit for low speed channel 4 duty change done."]
pub type DUTY_CHNG_END_LSCH4_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH5` reader - The interrupt raw bit for low speed channel 5 duty change done."]
pub type DUTY_CHNG_END_LSCH5_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH6` reader - The interrupt raw bit for low speed channel 6 duty change done."]
pub type DUTY_CHNG_END_LSCH6_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH7` reader - The interrupt raw bit for low speed channel 7 duty change done."]
pub type DUTY_CHNG_END_LSCH7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for high speed channel0 counter overflow."]
    #[inline(always)]
    pub fn hstimer0_ovf(&self) -> HSTIMER0_OVF_R {
        HSTIMER0_OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for high speed channel1 counter overflow."]
    #[inline(always)]
    pub fn hstimer1_ovf(&self) -> HSTIMER1_OVF_R {
        HSTIMER1_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for high speed channel2 counter overflow."]
    #[inline(always)]
    pub fn hstimer2_ovf(&self) -> HSTIMER2_OVF_R {
        HSTIMER2_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for high speed channel3 counter overflow."]
    #[inline(always)]
    pub fn hstimer3_ovf(&self) -> HSTIMER3_OVF_R {
        HSTIMER3_OVF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for low speed channel0 counter overflow."]
    #[inline(always)]
    pub fn lstimer0_ovf(&self) -> LSTIMER0_OVF_R {
        LSTIMER0_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for low speed channel1 counter overflow."]
    #[inline(always)]
    pub fn lstimer1_ovf(&self) -> LSTIMER1_OVF_R {
        LSTIMER1_OVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for low speed channel2 counter overflow."]
    #[inline(always)]
    pub fn lstimer2_ovf(&self) -> LSTIMER2_OVF_R {
        LSTIMER2_OVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for low speed channel3 counter overflow."]
    #[inline(always)]
    pub fn lstimer3_ovf(&self) -> LSTIMER3_OVF_R {
        LSTIMER3_OVF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for high speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0(&self) -> DUTY_CHNG_END_HSCH0_R {
        DUTY_CHNG_END_HSCH0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for high speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1(&self) -> DUTY_CHNG_END_HSCH1_R {
        DUTY_CHNG_END_HSCH1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for high speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2(&self) -> DUTY_CHNG_END_HSCH2_R {
        DUTY_CHNG_END_HSCH2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for high speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3(&self) -> DUTY_CHNG_END_HSCH3_R {
        DUTY_CHNG_END_HSCH3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for high speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4(&self) -> DUTY_CHNG_END_HSCH4_R {
        DUTY_CHNG_END_HSCH4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for high speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5(&self) -> DUTY_CHNG_END_HSCH5_R {
        DUTY_CHNG_END_HSCH5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for high speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6(&self) -> DUTY_CHNG_END_HSCH6_R {
        DUTY_CHNG_END_HSCH6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for high speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7(&self) -> DUTY_CHNG_END_HSCH7_R {
        DUTY_CHNG_END_HSCH7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for low speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0(&self) -> DUTY_CHNG_END_LSCH0_R {
        DUTY_CHNG_END_LSCH0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for low speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1(&self) -> DUTY_CHNG_END_LSCH1_R {
        DUTY_CHNG_END_LSCH1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for low speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2(&self) -> DUTY_CHNG_END_LSCH2_R {
        DUTY_CHNG_END_LSCH2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for low speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3(&self) -> DUTY_CHNG_END_LSCH3_R {
        DUTY_CHNG_END_LSCH3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for low speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4(&self) -> DUTY_CHNG_END_LSCH4_R {
        DUTY_CHNG_END_LSCH4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for low speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5(&self) -> DUTY_CHNG_END_LSCH5_R {
        DUTY_CHNG_END_LSCH5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for low speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6(&self) -> DUTY_CHNG_END_LSCH6_R {
        DUTY_CHNG_END_LSCH6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for low speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7(&self) -> DUTY_CHNG_END_LSCH7_R {
        DUTY_CHNG_END_LSCH7_R::new(((self.bits >> 23) & 1) != 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
