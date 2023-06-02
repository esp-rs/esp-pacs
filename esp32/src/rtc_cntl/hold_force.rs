#[doc = "Register `HOLD_FORCE` reader"]
pub struct R(crate::R<HOLD_FORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOLD_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOLD_FORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOLD_FORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOLD_FORCE` writer"]
pub struct W(crate::W<HOLD_FORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOLD_FORCE_SPEC>;
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
impl From<crate::W<HOLD_FORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOLD_FORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC1_HOLD_FORCE` reader - "]
pub type ADC1_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `ADC1_HOLD_FORCE` writer - "]
pub type ADC1_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `ADC2_HOLD_FORCE` reader - "]
pub type ADC2_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `ADC2_HOLD_FORCE` writer - "]
pub type ADC2_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `PDAC1_HOLD_FORCE` reader - "]
pub type PDAC1_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `PDAC1_HOLD_FORCE` writer - "]
pub type PDAC1_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `PDAC2_HOLD_FORCE` reader - "]
pub type PDAC2_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `PDAC2_HOLD_FORCE` writer - "]
pub type PDAC2_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `SENSE1_HOLD_FORCE` reader - "]
pub type SENSE1_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `SENSE1_HOLD_FORCE` writer - "]
pub type SENSE1_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `SENSE2_HOLD_FORCE` reader - "]
pub type SENSE2_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `SENSE2_HOLD_FORCE` writer - "]
pub type SENSE2_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `SENSE3_HOLD_FORCE` reader - "]
pub type SENSE3_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `SENSE3_HOLD_FORCE` writer - "]
pub type SENSE3_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `SENSE4_HOLD_FORCE` reader - "]
pub type SENSE4_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `SENSE4_HOLD_FORCE` writer - "]
pub type SENSE4_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `TOUCH_PAD0_HOLD_FORCE` reader - "]
pub type TOUCH_PAD0_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_HOLD_FORCE` writer - "]
pub type TOUCH_PAD0_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `TOUCH_PAD1_HOLD_FORCE` reader - "]
pub type TOUCH_PAD1_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD1_HOLD_FORCE` writer - "]
pub type TOUCH_PAD1_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `TOUCH_PAD2_HOLD_FORCE` reader - "]
pub type TOUCH_PAD2_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD2_HOLD_FORCE` writer - "]
pub type TOUCH_PAD2_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `TOUCH_PAD3_HOLD_FORCE` reader - "]
pub type TOUCH_PAD3_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD3_HOLD_FORCE` writer - "]
pub type TOUCH_PAD3_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `TOUCH_PAD4_HOLD_FORCE` reader - "]
pub type TOUCH_PAD4_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD4_HOLD_FORCE` writer - "]
pub type TOUCH_PAD4_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `TOUCH_PAD5_HOLD_FORCE` reader - "]
pub type TOUCH_PAD5_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD5_HOLD_FORCE` writer - "]
pub type TOUCH_PAD5_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `TOUCH_PAD6_HOLD_FORCE` reader - "]
pub type TOUCH_PAD6_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD6_HOLD_FORCE` writer - "]
pub type TOUCH_PAD6_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `TOUCH_PAD7_HOLD_FORCE` reader - "]
pub type TOUCH_PAD7_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD7_HOLD_FORCE` writer - "]
pub type TOUCH_PAD7_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `X32P_HOLD_FORCE` reader - "]
pub type X32P_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `X32P_HOLD_FORCE` writer - "]
pub type X32P_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
#[doc = "Field `X32N_HOLD_FORCE` reader - "]
pub type X32N_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `X32N_HOLD_FORCE` writer - "]
pub type X32N_HOLD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, HOLD_FORCE_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1_hold_force(&self) -> ADC1_HOLD_FORCE_R {
        ADC1_HOLD_FORCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2_hold_force(&self) -> ADC2_HOLD_FORCE_R {
        ADC2_HOLD_FORCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdac1_hold_force(&self) -> PDAC1_HOLD_FORCE_R {
        PDAC1_HOLD_FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdac2_hold_force(&self) -> PDAC2_HOLD_FORCE_R {
        PDAC2_HOLD_FORCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sense1_hold_force(&self) -> SENSE1_HOLD_FORCE_R {
        SENSE1_HOLD_FORCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sense2_hold_force(&self) -> SENSE2_HOLD_FORCE_R {
        SENSE2_HOLD_FORCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sense3_hold_force(&self) -> SENSE3_HOLD_FORCE_R {
        SENSE3_HOLD_FORCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sense4_hold_force(&self) -> SENSE4_HOLD_FORCE_R {
        SENSE4_HOLD_FORCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_pad0_hold_force(&self) -> TOUCH_PAD0_HOLD_FORCE_R {
        TOUCH_PAD0_HOLD_FORCE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_pad1_hold_force(&self) -> TOUCH_PAD1_HOLD_FORCE_R {
        TOUCH_PAD1_HOLD_FORCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_pad2_hold_force(&self) -> TOUCH_PAD2_HOLD_FORCE_R {
        TOUCH_PAD2_HOLD_FORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_pad3_hold_force(&self) -> TOUCH_PAD3_HOLD_FORCE_R {
        TOUCH_PAD3_HOLD_FORCE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_pad4_hold_force(&self) -> TOUCH_PAD4_HOLD_FORCE_R {
        TOUCH_PAD4_HOLD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_pad5_hold_force(&self) -> TOUCH_PAD5_HOLD_FORCE_R {
        TOUCH_PAD5_HOLD_FORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_pad6_hold_force(&self) -> TOUCH_PAD6_HOLD_FORCE_R {
        TOUCH_PAD6_HOLD_FORCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_pad7_hold_force(&self) -> TOUCH_PAD7_HOLD_FORCE_R {
        TOUCH_PAD7_HOLD_FORCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn x32p_hold_force(&self) -> X32P_HOLD_FORCE_R {
        X32P_HOLD_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn x32n_hold_force(&self) -> X32N_HOLD_FORCE_R {
        X32N_HOLD_FORCE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOLD_FORCE")
            .field(
                "adc1_hold_force",
                &format_args!("{}", self.adc1_hold_force().bit()),
            )
            .field(
                "adc2_hold_force",
                &format_args!("{}", self.adc2_hold_force().bit()),
            )
            .field(
                "pdac1_hold_force",
                &format_args!("{}", self.pdac1_hold_force().bit()),
            )
            .field(
                "pdac2_hold_force",
                &format_args!("{}", self.pdac2_hold_force().bit()),
            )
            .field(
                "sense1_hold_force",
                &format_args!("{}", self.sense1_hold_force().bit()),
            )
            .field(
                "sense2_hold_force",
                &format_args!("{}", self.sense2_hold_force().bit()),
            )
            .field(
                "sense3_hold_force",
                &format_args!("{}", self.sense3_hold_force().bit()),
            )
            .field(
                "sense4_hold_force",
                &format_args!("{}", self.sense4_hold_force().bit()),
            )
            .field(
                "touch_pad0_hold_force",
                &format_args!("{}", self.touch_pad0_hold_force().bit()),
            )
            .field(
                "touch_pad1_hold_force",
                &format_args!("{}", self.touch_pad1_hold_force().bit()),
            )
            .field(
                "touch_pad2_hold_force",
                &format_args!("{}", self.touch_pad2_hold_force().bit()),
            )
            .field(
                "touch_pad3_hold_force",
                &format_args!("{}", self.touch_pad3_hold_force().bit()),
            )
            .field(
                "touch_pad4_hold_force",
                &format_args!("{}", self.touch_pad4_hold_force().bit()),
            )
            .field(
                "touch_pad5_hold_force",
                &format_args!("{}", self.touch_pad5_hold_force().bit()),
            )
            .field(
                "touch_pad6_hold_force",
                &format_args!("{}", self.touch_pad6_hold_force().bit()),
            )
            .field(
                "touch_pad7_hold_force",
                &format_args!("{}", self.touch_pad7_hold_force().bit()),
            )
            .field(
                "x32p_hold_force",
                &format_args!("{}", self.x32p_hold_force().bit()),
            )
            .field(
                "x32n_hold_force",
                &format_args!("{}", self.x32n_hold_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOLD_FORCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_hold_force(&mut self) -> ADC1_HOLD_FORCE_W<0> {
        ADC1_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_hold_force(&mut self) -> ADC2_HOLD_FORCE_W<1> {
        ADC2_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_hold_force(&mut self) -> PDAC1_HOLD_FORCE_W<2> {
        PDAC1_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_hold_force(&mut self) -> PDAC2_HOLD_FORCE_W<3> {
        PDAC2_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sense1_hold_force(&mut self) -> SENSE1_HOLD_FORCE_W<4> {
        SENSE1_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sense2_hold_force(&mut self) -> SENSE2_HOLD_FORCE_W<5> {
        SENSE2_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sense3_hold_force(&mut self) -> SENSE3_HOLD_FORCE_W<6> {
        SENSE3_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sense4_hold_force(&mut self) -> SENSE4_HOLD_FORCE_W<7> {
        SENSE4_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_hold_force(&mut self) -> TOUCH_PAD0_HOLD_FORCE_W<8> {
        TOUCH_PAD0_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad1_hold_force(&mut self) -> TOUCH_PAD1_HOLD_FORCE_W<9> {
        TOUCH_PAD1_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad2_hold_force(&mut self) -> TOUCH_PAD2_HOLD_FORCE_W<10> {
        TOUCH_PAD2_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad3_hold_force(&mut self) -> TOUCH_PAD3_HOLD_FORCE_W<11> {
        TOUCH_PAD3_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad4_hold_force(&mut self) -> TOUCH_PAD4_HOLD_FORCE_W<12> {
        TOUCH_PAD4_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad5_hold_force(&mut self) -> TOUCH_PAD5_HOLD_FORCE_W<13> {
        TOUCH_PAD5_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad6_hold_force(&mut self) -> TOUCH_PAD6_HOLD_FORCE_W<14> {
        TOUCH_PAD6_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad7_hold_force(&mut self) -> TOUCH_PAD7_HOLD_FORCE_W<15> {
        TOUCH_PAD7_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_hold_force(&mut self) -> X32P_HOLD_FORCE_W<16> {
        X32P_HOLD_FORCE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_hold_force(&mut self) -> X32N_HOLD_FORCE_W<17> {
        X32N_HOLD_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hold_force](index.html) module"]
pub struct HOLD_FORCE_SPEC;
impl crate::RegisterSpec for HOLD_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hold_force::R](R) reader structure"]
impl crate::Readable for HOLD_FORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hold_force::W](W) writer structure"]
impl crate::Writable for HOLD_FORCE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOLD_FORCE to value 0"]
impl crate::Resettable for HOLD_FORCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
