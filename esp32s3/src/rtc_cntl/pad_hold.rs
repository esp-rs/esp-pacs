#[doc = "Register `PAD_HOLD` reader"]
pub struct R(crate::R<PAD_HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_HOLD` writer"]
pub struct W(crate::W<PAD_HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_HOLD_SPEC>;
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
impl From<crate::W<PAD_HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_PAD0_HOLD` reader - hold rtc pad0"]
pub type TOUCH_PAD0_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_HOLD` writer - hold rtc pad0"]
pub type TOUCH_PAD0_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD1_HOLD` reader - hold rtc pad-1"]
pub type TOUCH_PAD1_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD1_HOLD` writer - hold rtc pad-1"]
pub type TOUCH_PAD1_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD2_HOLD` reader - hold rtc pad-2"]
pub type TOUCH_PAD2_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD2_HOLD` writer - hold rtc pad-2"]
pub type TOUCH_PAD2_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD3_HOLD` reader - hold rtc pad-3"]
pub type TOUCH_PAD3_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD3_HOLD` writer - hold rtc pad-3"]
pub type TOUCH_PAD3_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD4_HOLD` reader - hold rtc pad-4"]
pub type TOUCH_PAD4_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD4_HOLD` writer - hold rtc pad-4"]
pub type TOUCH_PAD4_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD5_HOLD` reader - hold rtc pad-5"]
pub type TOUCH_PAD5_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD5_HOLD` writer - hold rtc pad-5"]
pub type TOUCH_PAD5_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD6_HOLD` reader - hold rtc pad-6"]
pub type TOUCH_PAD6_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD6_HOLD` writer - hold rtc pad-6"]
pub type TOUCH_PAD6_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD7_HOLD` reader - hold rtc pad-7"]
pub type TOUCH_PAD7_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD7_HOLD` writer - hold rtc pad-7"]
pub type TOUCH_PAD7_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD8_HOLD` reader - hold rtc pad-8"]
pub type TOUCH_PAD8_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD8_HOLD` writer - hold rtc pad-8"]
pub type TOUCH_PAD8_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD9_HOLD` reader - hold rtc pad-9"]
pub type TOUCH_PAD9_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD9_HOLD` writer - hold rtc pad-9"]
pub type TOUCH_PAD9_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD10_HOLD` reader - hold rtc pad-10"]
pub type TOUCH_PAD10_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD10_HOLD` writer - hold rtc pad-10"]
pub type TOUCH_PAD10_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD11_HOLD` reader - hold rtc pad-11"]
pub type TOUCH_PAD11_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD11_HOLD` writer - hold rtc pad-11"]
pub type TOUCH_PAD11_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD12_HOLD` reader - hold rtc pad-12"]
pub type TOUCH_PAD12_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD12_HOLD` writer - hold rtc pad-12"]
pub type TOUCH_PAD12_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD13_HOLD` reader - hold rtc pad-13"]
pub type TOUCH_PAD13_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD13_HOLD` writer - hold rtc pad-13"]
pub type TOUCH_PAD13_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `TOUCH_PAD14_HOLD` reader - hold rtc pad-14"]
pub type TOUCH_PAD14_HOLD_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD14_HOLD` writer - hold rtc pad-14"]
pub type TOUCH_PAD14_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `X32P_HOLD` reader - hold rtc pad-15"]
pub type X32P_HOLD_R = crate::BitReader;
#[doc = "Field `X32P_HOLD` writer - hold rtc pad-15"]
pub type X32P_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `X32N_HOLD` reader - hold rtc pad-16"]
pub type X32N_HOLD_R = crate::BitReader;
#[doc = "Field `X32N_HOLD` writer - hold rtc pad-16"]
pub type X32N_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `PDAC1_HOLD` reader - hold rtc pad-17"]
pub type PDAC1_HOLD_R = crate::BitReader;
#[doc = "Field `PDAC1_HOLD` writer - hold rtc pad-17"]
pub type PDAC1_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `PDAC2_HOLD` reader - hold rtc pad-18"]
pub type PDAC2_HOLD_R = crate::BitReader;
#[doc = "Field `PDAC2_HOLD` writer - hold rtc pad-18"]
pub type PDAC2_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `PAD19_HOLD` reader - hold rtc pad-19"]
pub type PAD19_HOLD_R = crate::BitReader;
#[doc = "Field `PAD19_HOLD` writer - hold rtc pad-19"]
pub type PAD19_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `PAD20_HOLD` reader - hold rtc pad-20"]
pub type PAD20_HOLD_R = crate::BitReader;
#[doc = "Field `PAD20_HOLD` writer - hold rtc pad-20"]
pub type PAD20_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
#[doc = "Field `PAD21_HOLD` reader - hold rtc pad-21"]
pub type PAD21_HOLD_R = crate::BitReader;
#[doc = "Field `PAD21_HOLD` writer - hold rtc pad-21"]
pub type PAD21_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PAD_HOLD_SPEC, O>;
impl R {
    #[doc = "Bit 0 - hold rtc pad0"]
    #[inline(always)]
    pub fn touch_pad0_hold(&self) -> TOUCH_PAD0_HOLD_R {
        TOUCH_PAD0_HOLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - hold rtc pad-1"]
    #[inline(always)]
    pub fn touch_pad1_hold(&self) -> TOUCH_PAD1_HOLD_R {
        TOUCH_PAD1_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hold rtc pad-2"]
    #[inline(always)]
    pub fn touch_pad2_hold(&self) -> TOUCH_PAD2_HOLD_R {
        TOUCH_PAD2_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hold rtc pad-3"]
    #[inline(always)]
    pub fn touch_pad3_hold(&self) -> TOUCH_PAD3_HOLD_R {
        TOUCH_PAD3_HOLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hold rtc pad-4"]
    #[inline(always)]
    pub fn touch_pad4_hold(&self) -> TOUCH_PAD4_HOLD_R {
        TOUCH_PAD4_HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hold rtc pad-5"]
    #[inline(always)]
    pub fn touch_pad5_hold(&self) -> TOUCH_PAD5_HOLD_R {
        TOUCH_PAD5_HOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - hold rtc pad-6"]
    #[inline(always)]
    pub fn touch_pad6_hold(&self) -> TOUCH_PAD6_HOLD_R {
        TOUCH_PAD6_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - hold rtc pad-7"]
    #[inline(always)]
    pub fn touch_pad7_hold(&self) -> TOUCH_PAD7_HOLD_R {
        TOUCH_PAD7_HOLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - hold rtc pad-8"]
    #[inline(always)]
    pub fn touch_pad8_hold(&self) -> TOUCH_PAD8_HOLD_R {
        TOUCH_PAD8_HOLD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hold rtc pad-9"]
    #[inline(always)]
    pub fn touch_pad9_hold(&self) -> TOUCH_PAD9_HOLD_R {
        TOUCH_PAD9_HOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hold rtc pad-10"]
    #[inline(always)]
    pub fn touch_pad10_hold(&self) -> TOUCH_PAD10_HOLD_R {
        TOUCH_PAD10_HOLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hold rtc pad-11"]
    #[inline(always)]
    pub fn touch_pad11_hold(&self) -> TOUCH_PAD11_HOLD_R {
        TOUCH_PAD11_HOLD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - hold rtc pad-12"]
    #[inline(always)]
    pub fn touch_pad12_hold(&self) -> TOUCH_PAD12_HOLD_R {
        TOUCH_PAD12_HOLD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hold rtc pad-13"]
    #[inline(always)]
    pub fn touch_pad13_hold(&self) -> TOUCH_PAD13_HOLD_R {
        TOUCH_PAD13_HOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - hold rtc pad-14"]
    #[inline(always)]
    pub fn touch_pad14_hold(&self) -> TOUCH_PAD14_HOLD_R {
        TOUCH_PAD14_HOLD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - hold rtc pad-15"]
    #[inline(always)]
    pub fn x32p_hold(&self) -> X32P_HOLD_R {
        X32P_HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - hold rtc pad-16"]
    #[inline(always)]
    pub fn x32n_hold(&self) -> X32N_HOLD_R {
        X32N_HOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - hold rtc pad-17"]
    #[inline(always)]
    pub fn pdac1_hold(&self) -> PDAC1_HOLD_R {
        PDAC1_HOLD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - hold rtc pad-18"]
    #[inline(always)]
    pub fn pdac2_hold(&self) -> PDAC2_HOLD_R {
        PDAC2_HOLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - hold rtc pad-19"]
    #[inline(always)]
    pub fn pad19_hold(&self) -> PAD19_HOLD_R {
        PAD19_HOLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - hold rtc pad-20"]
    #[inline(always)]
    pub fn pad20_hold(&self) -> PAD20_HOLD_R {
        PAD20_HOLD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - hold rtc pad-21"]
    #[inline(always)]
    pub fn pad21_hold(&self) -> PAD21_HOLD_R {
        PAD21_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_HOLD")
            .field(
                "touch_pad0_hold",
                &format_args!("{}", self.touch_pad0_hold().bit()),
            )
            .field(
                "touch_pad1_hold",
                &format_args!("{}", self.touch_pad1_hold().bit()),
            )
            .field(
                "touch_pad2_hold",
                &format_args!("{}", self.touch_pad2_hold().bit()),
            )
            .field(
                "touch_pad3_hold",
                &format_args!("{}", self.touch_pad3_hold().bit()),
            )
            .field(
                "touch_pad4_hold",
                &format_args!("{}", self.touch_pad4_hold().bit()),
            )
            .field(
                "touch_pad5_hold",
                &format_args!("{}", self.touch_pad5_hold().bit()),
            )
            .field(
                "touch_pad6_hold",
                &format_args!("{}", self.touch_pad6_hold().bit()),
            )
            .field(
                "touch_pad7_hold",
                &format_args!("{}", self.touch_pad7_hold().bit()),
            )
            .field(
                "touch_pad8_hold",
                &format_args!("{}", self.touch_pad8_hold().bit()),
            )
            .field(
                "touch_pad9_hold",
                &format_args!("{}", self.touch_pad9_hold().bit()),
            )
            .field(
                "touch_pad10_hold",
                &format_args!("{}", self.touch_pad10_hold().bit()),
            )
            .field(
                "touch_pad11_hold",
                &format_args!("{}", self.touch_pad11_hold().bit()),
            )
            .field(
                "touch_pad12_hold",
                &format_args!("{}", self.touch_pad12_hold().bit()),
            )
            .field(
                "touch_pad13_hold",
                &format_args!("{}", self.touch_pad13_hold().bit()),
            )
            .field(
                "touch_pad14_hold",
                &format_args!("{}", self.touch_pad14_hold().bit()),
            )
            .field("x32p_hold", &format_args!("{}", self.x32p_hold().bit()))
            .field("x32n_hold", &format_args!("{}", self.x32n_hold().bit()))
            .field("pdac1_hold", &format_args!("{}", self.pdac1_hold().bit()))
            .field("pdac2_hold", &format_args!("{}", self.pdac2_hold().bit()))
            .field("pad19_hold", &format_args!("{}", self.pad19_hold().bit()))
            .field("pad20_hold", &format_args!("{}", self.pad20_hold().bit()))
            .field("pad21_hold", &format_args!("{}", self.pad21_hold().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PAD_HOLD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - hold rtc pad0"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_hold(&mut self) -> TOUCH_PAD0_HOLD_W<0> {
        TOUCH_PAD0_HOLD_W::new(self)
    }
    #[doc = "Bit 1 - hold rtc pad-1"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad1_hold(&mut self) -> TOUCH_PAD1_HOLD_W<1> {
        TOUCH_PAD1_HOLD_W::new(self)
    }
    #[doc = "Bit 2 - hold rtc pad-2"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad2_hold(&mut self) -> TOUCH_PAD2_HOLD_W<2> {
        TOUCH_PAD2_HOLD_W::new(self)
    }
    #[doc = "Bit 3 - hold rtc pad-3"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad3_hold(&mut self) -> TOUCH_PAD3_HOLD_W<3> {
        TOUCH_PAD3_HOLD_W::new(self)
    }
    #[doc = "Bit 4 - hold rtc pad-4"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad4_hold(&mut self) -> TOUCH_PAD4_HOLD_W<4> {
        TOUCH_PAD4_HOLD_W::new(self)
    }
    #[doc = "Bit 5 - hold rtc pad-5"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad5_hold(&mut self) -> TOUCH_PAD5_HOLD_W<5> {
        TOUCH_PAD5_HOLD_W::new(self)
    }
    #[doc = "Bit 6 - hold rtc pad-6"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad6_hold(&mut self) -> TOUCH_PAD6_HOLD_W<6> {
        TOUCH_PAD6_HOLD_W::new(self)
    }
    #[doc = "Bit 7 - hold rtc pad-7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad7_hold(&mut self) -> TOUCH_PAD7_HOLD_W<7> {
        TOUCH_PAD7_HOLD_W::new(self)
    }
    #[doc = "Bit 8 - hold rtc pad-8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad8_hold(&mut self) -> TOUCH_PAD8_HOLD_W<8> {
        TOUCH_PAD8_HOLD_W::new(self)
    }
    #[doc = "Bit 9 - hold rtc pad-9"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad9_hold(&mut self) -> TOUCH_PAD9_HOLD_W<9> {
        TOUCH_PAD9_HOLD_W::new(self)
    }
    #[doc = "Bit 10 - hold rtc pad-10"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad10_hold(&mut self) -> TOUCH_PAD10_HOLD_W<10> {
        TOUCH_PAD10_HOLD_W::new(self)
    }
    #[doc = "Bit 11 - hold rtc pad-11"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad11_hold(&mut self) -> TOUCH_PAD11_HOLD_W<11> {
        TOUCH_PAD11_HOLD_W::new(self)
    }
    #[doc = "Bit 12 - hold rtc pad-12"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad12_hold(&mut self) -> TOUCH_PAD12_HOLD_W<12> {
        TOUCH_PAD12_HOLD_W::new(self)
    }
    #[doc = "Bit 13 - hold rtc pad-13"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad13_hold(&mut self) -> TOUCH_PAD13_HOLD_W<13> {
        TOUCH_PAD13_HOLD_W::new(self)
    }
    #[doc = "Bit 14 - hold rtc pad-14"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad14_hold(&mut self) -> TOUCH_PAD14_HOLD_W<14> {
        TOUCH_PAD14_HOLD_W::new(self)
    }
    #[doc = "Bit 15 - hold rtc pad-15"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_hold(&mut self) -> X32P_HOLD_W<15> {
        X32P_HOLD_W::new(self)
    }
    #[doc = "Bit 16 - hold rtc pad-16"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_hold(&mut self) -> X32N_HOLD_W<16> {
        X32N_HOLD_W::new(self)
    }
    #[doc = "Bit 17 - hold rtc pad-17"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_hold(&mut self) -> PDAC1_HOLD_W<17> {
        PDAC1_HOLD_W::new(self)
    }
    #[doc = "Bit 18 - hold rtc pad-18"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_hold(&mut self) -> PDAC2_HOLD_W<18> {
        PDAC2_HOLD_W::new(self)
    }
    #[doc = "Bit 19 - hold rtc pad-19"]
    #[inline(always)]
    #[must_use]
    pub fn pad19_hold(&mut self) -> PAD19_HOLD_W<19> {
        PAD19_HOLD_W::new(self)
    }
    #[doc = "Bit 20 - hold rtc pad-20"]
    #[inline(always)]
    #[must_use]
    pub fn pad20_hold(&mut self) -> PAD20_HOLD_W<20> {
        PAD20_HOLD_W::new(self)
    }
    #[doc = "Bit 21 - hold rtc pad-21"]
    #[inline(always)]
    #[must_use]
    pub fn pad21_hold(&mut self) -> PAD21_HOLD_W<21> {
        PAD21_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc pad hold configure\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_hold](index.html) module"]
pub struct PAD_HOLD_SPEC;
impl crate::RegisterSpec for PAD_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_hold::R](R) reader structure"]
impl crate::Readable for PAD_HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_hold::W](W) writer structure"]
impl crate::Writable for PAD_HOLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_HOLD to value 0"]
impl crate::Resettable for PAD_HOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
