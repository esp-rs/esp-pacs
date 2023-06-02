#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTIMER0_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
pub type HSTIMER0_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `HSTIMER0_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
pub type HSTIMER0_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `HSTIMER1_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
pub type HSTIMER1_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `HSTIMER1_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
pub type HSTIMER1_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `HSTIMER2_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
pub type HSTIMER2_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `HSTIMER2_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
pub type HSTIMER2_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `HSTIMER3_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
pub type HSTIMER3_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `HSTIMER3_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
pub type HSTIMER3_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `LSTIMER0_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
pub type LSTIMER0_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `LSTIMER0_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
pub type LSTIMER0_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `LSTIMER1_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
pub type LSTIMER1_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `LSTIMER1_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
pub type LSTIMER1_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `LSTIMER2_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
pub type LSTIMER2_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `LSTIMER2_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
pub type LSTIMER2_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `LSTIMER3_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
pub type LSTIMER3_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `LSTIMER3_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
pub type LSTIMER3_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH0_INT_ENA` reader - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH0_INT_ENA` writer - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH1_INT_ENA` reader - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH1_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH1_INT_ENA` writer - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH2_INT_ENA` reader - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH2_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH2_INT_ENA` writer - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH2_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH3_INT_ENA` reader - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH3_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH3_INT_ENA` writer - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH3_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH4_INT_ENA` reader - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH4_INT_ENA` writer - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH4_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH5_INT_ENA` reader - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH5_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH5_INT_ENA` writer - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH5_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH6_INT_ENA` reader - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH6_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH6_INT_ENA` writer - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH6_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_HSCH7_INT_ENA` reader - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH7_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH7_INT_ENA` writer - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH7_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ENA` reader - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ENA` writer - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ENA` reader - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH1_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ENA` writer - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ENA` reader - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH2_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ENA` writer - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH2_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ENA` reader - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH3_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ENA` writer - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH3_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ENA` reader - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ENA` writer - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH4_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ENA` reader - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH5_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ENA` writer - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH5_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH6_INT_ENA` reader - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH6_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH6_INT_ENA` writer - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH6_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DUTY_CHNG_END_LSCH7_INT_ENA` reader - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH7_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH7_INT_ENA` writer - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH7_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer0_ovf_int_ena(&self) -> HSTIMER0_OVF_INT_ENA_R {
        HSTIMER0_OVF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer1_ovf_int_ena(&self) -> HSTIMER1_OVF_INT_ENA_R {
        HSTIMER1_OVF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer2_ovf_int_ena(&self) -> HSTIMER2_OVF_INT_ENA_R {
        HSTIMER2_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer3_ovf_int_ena(&self) -> HSTIMER3_OVF_INT_ENA_R {
        HSTIMER3_OVF_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_ena(&self) -> LSTIMER0_OVF_INT_ENA_R {
        LSTIMER0_OVF_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_ena(&self) -> LSTIMER1_OVF_INT_ENA_R {
        LSTIMER1_OVF_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_ena(&self) -> LSTIMER2_OVF_INT_ENA_R {
        LSTIMER2_OVF_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_ena(&self) -> LSTIMER3_OVF_INT_ENA_R {
        LSTIMER3_OVF_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0_int_ena(&self) -> DUTY_CHNG_END_HSCH0_INT_ENA_R {
        DUTY_CHNG_END_HSCH0_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1_int_ena(&self) -> DUTY_CHNG_END_HSCH1_INT_ENA_R {
        DUTY_CHNG_END_HSCH1_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2_int_ena(&self) -> DUTY_CHNG_END_HSCH2_INT_ENA_R {
        DUTY_CHNG_END_HSCH2_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3_int_ena(&self) -> DUTY_CHNG_END_HSCH3_INT_ENA_R {
        DUTY_CHNG_END_HSCH3_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4_int_ena(&self) -> DUTY_CHNG_END_HSCH4_INT_ENA_R {
        DUTY_CHNG_END_HSCH4_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5_int_ena(&self) -> DUTY_CHNG_END_HSCH5_INT_ENA_R {
        DUTY_CHNG_END_HSCH5_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6_int_ena(&self) -> DUTY_CHNG_END_HSCH6_INT_ENA_R {
        DUTY_CHNG_END_HSCH6_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7_int_ena(&self) -> DUTY_CHNG_END_HSCH7_INT_ENA_R {
        DUTY_CHNG_END_HSCH7_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_ena(&self) -> DUTY_CHNG_END_LSCH0_INT_ENA_R {
        DUTY_CHNG_END_LSCH0_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_ena(&self) -> DUTY_CHNG_END_LSCH1_INT_ENA_R {
        DUTY_CHNG_END_LSCH1_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_ena(&self) -> DUTY_CHNG_END_LSCH2_INT_ENA_R {
        DUTY_CHNG_END_LSCH2_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_ena(&self) -> DUTY_CHNG_END_LSCH3_INT_ENA_R {
        DUTY_CHNG_END_LSCH3_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_ena(&self) -> DUTY_CHNG_END_LSCH4_INT_ENA_R {
        DUTY_CHNG_END_LSCH4_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_ena(&self) -> DUTY_CHNG_END_LSCH5_INT_ENA_R {
        DUTY_CHNG_END_LSCH5_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6_int_ena(&self) -> DUTY_CHNG_END_LSCH6_INT_ENA_R {
        DUTY_CHNG_END_LSCH6_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7_int_ena(&self) -> DUTY_CHNG_END_LSCH7_INT_ENA_R {
        DUTY_CHNG_END_LSCH7_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "hstimer0_ovf_int_ena",
                &format_args!("{}", self.hstimer0_ovf_int_ena().bit()),
            )
            .field(
                "hstimer1_ovf_int_ena",
                &format_args!("{}", self.hstimer1_ovf_int_ena().bit()),
            )
            .field(
                "hstimer2_ovf_int_ena",
                &format_args!("{}", self.hstimer2_ovf_int_ena().bit()),
            )
            .field(
                "hstimer3_ovf_int_ena",
                &format_args!("{}", self.hstimer3_ovf_int_ena().bit()),
            )
            .field(
                "lstimer0_ovf_int_ena",
                &format_args!("{}", self.lstimer0_ovf_int_ena().bit()),
            )
            .field(
                "lstimer1_ovf_int_ena",
                &format_args!("{}", self.lstimer1_ovf_int_ena().bit()),
            )
            .field(
                "lstimer2_ovf_int_ena",
                &format_args!("{}", self.lstimer2_ovf_int_ena().bit()),
            )
            .field(
                "lstimer3_ovf_int_ena",
                &format_args!("{}", self.lstimer3_ovf_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch0_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch0_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch1_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch1_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch2_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch2_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch3_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch3_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch4_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch4_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch5_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch5_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch6_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch6_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch7_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch7_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch0_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch0_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch1_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch1_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch2_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch2_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch3_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch3_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch4_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch4_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch5_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch5_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch6_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch6_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch7_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch7_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer0_ovf_int_ena(&mut self) -> HSTIMER0_OVF_INT_ENA_W<0> {
        HSTIMER0_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer1_ovf_int_ena(&mut self) -> HSTIMER1_OVF_INT_ENA_W<1> {
        HSTIMER1_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer2_ovf_int_ena(&mut self) -> HSTIMER2_OVF_INT_ENA_W<2> {
        HSTIMER2_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer3_ovf_int_ena(&mut self) -> HSTIMER3_OVF_INT_ENA_W<3> {
        HSTIMER3_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer0_ovf_int_ena(&mut self) -> LSTIMER0_OVF_INT_ENA_W<4> {
        LSTIMER0_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer1_ovf_int_ena(&mut self) -> LSTIMER1_OVF_INT_ENA_W<5> {
        LSTIMER1_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer2_ovf_int_ena(&mut self) -> LSTIMER2_OVF_INT_ENA_W<6> {
        LSTIMER2_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer3_ovf_int_ena(&mut self) -> LSTIMER3_OVF_INT_ENA_W<7> {
        LSTIMER3_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch0_int_ena(&mut self) -> DUTY_CHNG_END_HSCH0_INT_ENA_W<8> {
        DUTY_CHNG_END_HSCH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch1_int_ena(&mut self) -> DUTY_CHNG_END_HSCH1_INT_ENA_W<9> {
        DUTY_CHNG_END_HSCH1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch2_int_ena(&mut self) -> DUTY_CHNG_END_HSCH2_INT_ENA_W<10> {
        DUTY_CHNG_END_HSCH2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch3_int_ena(&mut self) -> DUTY_CHNG_END_HSCH3_INT_ENA_W<11> {
        DUTY_CHNG_END_HSCH3_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch4_int_ena(&mut self) -> DUTY_CHNG_END_HSCH4_INT_ENA_W<12> {
        DUTY_CHNG_END_HSCH4_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch5_int_ena(&mut self) -> DUTY_CHNG_END_HSCH5_INT_ENA_W<13> {
        DUTY_CHNG_END_HSCH5_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch6_int_ena(&mut self) -> DUTY_CHNG_END_HSCH6_INT_ENA_W<14> {
        DUTY_CHNG_END_HSCH6_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch7_int_ena(&mut self) -> DUTY_CHNG_END_HSCH7_INT_ENA_W<15> {
        DUTY_CHNG_END_HSCH7_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch0_int_ena(&mut self) -> DUTY_CHNG_END_LSCH0_INT_ENA_W<16> {
        DUTY_CHNG_END_LSCH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch1_int_ena(&mut self) -> DUTY_CHNG_END_LSCH1_INT_ENA_W<17> {
        DUTY_CHNG_END_LSCH1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch2_int_ena(&mut self) -> DUTY_CHNG_END_LSCH2_INT_ENA_W<18> {
        DUTY_CHNG_END_LSCH2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch3_int_ena(&mut self) -> DUTY_CHNG_END_LSCH3_INT_ENA_W<19> {
        DUTY_CHNG_END_LSCH3_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20 - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch4_int_ena(&mut self) -> DUTY_CHNG_END_LSCH4_INT_ENA_W<20> {
        DUTY_CHNG_END_LSCH4_INT_ENA_W::new(self)
    }
    #[doc = "Bit 21 - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch5_int_ena(&mut self) -> DUTY_CHNG_END_LSCH5_INT_ENA_W<21> {
        DUTY_CHNG_END_LSCH5_INT_ENA_W::new(self)
    }
    #[doc = "Bit 22 - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch6_int_ena(&mut self) -> DUTY_CHNG_END_LSCH6_INT_ENA_W<22> {
        DUTY_CHNG_END_LSCH6_INT_ENA_W::new(self)
    }
    #[doc = "Bit 23 - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch7_int_ena(&mut self) -> DUTY_CHNG_END_LSCH7_INT_ENA_W<23> {
        DUTY_CHNG_END_LSCH7_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
