#[doc = "Register `CH_ENA_AD0` reader"]
pub struct R(crate::R<CH_ENA_AD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_ENA_AD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_ENA_AD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_ENA_AD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH_ENA_AD0` writer"]
pub struct W(crate::W<CH_ENA_AD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_ENA_AD0_SPEC>;
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
impl From<crate::W<CH_ENA_AD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_ENA_AD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_ENA0` reader - ch0 enable"]
pub type CH_ENA0_R = crate::BitReader;
#[doc = "Field `CH_ENA0` writer - ch0 enable"]
pub type CH_ENA0_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA1` reader - ch1 enable"]
pub type CH_ENA1_R = crate::BitReader;
#[doc = "Field `CH_ENA1` writer - ch1 enable"]
pub type CH_ENA1_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA2` reader - ch2 enable"]
pub type CH_ENA2_R = crate::BitReader;
#[doc = "Field `CH_ENA2` writer - ch2 enable"]
pub type CH_ENA2_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA3` reader - ch3 enable"]
pub type CH_ENA3_R = crate::BitReader;
#[doc = "Field `CH_ENA3` writer - ch3 enable"]
pub type CH_ENA3_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA4` reader - ch4 enable"]
pub type CH_ENA4_R = crate::BitReader;
#[doc = "Field `CH_ENA4` writer - ch4 enable"]
pub type CH_ENA4_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA5` reader - ch5 enable"]
pub type CH_ENA5_R = crate::BitReader;
#[doc = "Field `CH_ENA5` writer - ch5 enable"]
pub type CH_ENA5_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA6` reader - ch6 enable"]
pub type CH_ENA6_R = crate::BitReader;
#[doc = "Field `CH_ENA6` writer - ch6 enable"]
pub type CH_ENA6_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA7` reader - ch7 enable"]
pub type CH_ENA7_R = crate::BitReader;
#[doc = "Field `CH_ENA7` writer - ch7 enable"]
pub type CH_ENA7_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA8` reader - ch8 enable"]
pub type CH_ENA8_R = crate::BitReader;
#[doc = "Field `CH_ENA8` writer - ch8 enable"]
pub type CH_ENA8_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA9` reader - ch9 enable"]
pub type CH_ENA9_R = crate::BitReader;
#[doc = "Field `CH_ENA9` writer - ch9 enable"]
pub type CH_ENA9_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA10` reader - ch10 enable"]
pub type CH_ENA10_R = crate::BitReader;
#[doc = "Field `CH_ENA10` writer - ch10 enable"]
pub type CH_ENA10_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA11` reader - ch11 enable"]
pub type CH_ENA11_R = crate::BitReader;
#[doc = "Field `CH_ENA11` writer - ch11 enable"]
pub type CH_ENA11_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA12` reader - ch12 enable"]
pub type CH_ENA12_R = crate::BitReader;
#[doc = "Field `CH_ENA12` writer - ch12 enable"]
pub type CH_ENA12_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA13` reader - ch13 enable"]
pub type CH_ENA13_R = crate::BitReader;
#[doc = "Field `CH_ENA13` writer - ch13 enable"]
pub type CH_ENA13_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA14` reader - ch14 enable"]
pub type CH_ENA14_R = crate::BitReader;
#[doc = "Field `CH_ENA14` writer - ch14 enable"]
pub type CH_ENA14_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA15` reader - ch15 enable"]
pub type CH_ENA15_R = crate::BitReader;
#[doc = "Field `CH_ENA15` writer - ch15 enable"]
pub type CH_ENA15_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA16` reader - ch16 enable"]
pub type CH_ENA16_R = crate::BitReader;
#[doc = "Field `CH_ENA16` writer - ch16 enable"]
pub type CH_ENA16_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA17` reader - ch17 enable"]
pub type CH_ENA17_R = crate::BitReader;
#[doc = "Field `CH_ENA17` writer - ch17 enable"]
pub type CH_ENA17_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA18` reader - ch18 enable"]
pub type CH_ENA18_R = crate::BitReader;
#[doc = "Field `CH_ENA18` writer - ch18 enable"]
pub type CH_ENA18_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA19` reader - ch19 enable"]
pub type CH_ENA19_R = crate::BitReader;
#[doc = "Field `CH_ENA19` writer - ch19 enable"]
pub type CH_ENA19_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA20` reader - ch20 enable"]
pub type CH_ENA20_R = crate::BitReader;
#[doc = "Field `CH_ENA20` writer - ch20 enable"]
pub type CH_ENA20_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA21` reader - ch21 enable"]
pub type CH_ENA21_R = crate::BitReader;
#[doc = "Field `CH_ENA21` writer - ch21 enable"]
pub type CH_ENA21_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA22` reader - ch22 enable"]
pub type CH_ENA22_R = crate::BitReader;
#[doc = "Field `CH_ENA22` writer - ch22 enable"]
pub type CH_ENA22_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA23` reader - ch23 enable"]
pub type CH_ENA23_R = crate::BitReader;
#[doc = "Field `CH_ENA23` writer - ch23 enable"]
pub type CH_ENA23_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA24` reader - ch24 enable"]
pub type CH_ENA24_R = crate::BitReader;
#[doc = "Field `CH_ENA24` writer - ch24 enable"]
pub type CH_ENA24_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA25` reader - ch25 enable"]
pub type CH_ENA25_R = crate::BitReader;
#[doc = "Field `CH_ENA25` writer - ch25 enable"]
pub type CH_ENA25_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA26` reader - ch26 enable"]
pub type CH_ENA26_R = crate::BitReader;
#[doc = "Field `CH_ENA26` writer - ch26 enable"]
pub type CH_ENA26_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA27` reader - ch27 enable"]
pub type CH_ENA27_R = crate::BitReader;
#[doc = "Field `CH_ENA27` writer - ch27 enable"]
pub type CH_ENA27_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA28` reader - ch28 enable"]
pub type CH_ENA28_R = crate::BitReader;
#[doc = "Field `CH_ENA28` writer - ch28 enable"]
pub type CH_ENA28_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA29` reader - ch29 enable"]
pub type CH_ENA29_R = crate::BitReader;
#[doc = "Field `CH_ENA29` writer - ch29 enable"]
pub type CH_ENA29_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA30` reader - ch30 enable"]
pub type CH_ENA30_R = crate::BitReader;
#[doc = "Field `CH_ENA30` writer - ch30 enable"]
pub type CH_ENA30_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
#[doc = "Field `CH_ENA31` reader - ch31 enable"]
pub type CH_ENA31_R = crate::BitReader;
#[doc = "Field `CH_ENA31` writer - ch31 enable"]
pub type CH_ENA31_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ch0 enable"]
    #[inline(always)]
    pub fn ch_ena0(&self) -> CH_ENA0_R {
        CH_ENA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ch1 enable"]
    #[inline(always)]
    pub fn ch_ena1(&self) -> CH_ENA1_R {
        CH_ENA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ch2 enable"]
    #[inline(always)]
    pub fn ch_ena2(&self) -> CH_ENA2_R {
        CH_ENA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ch3 enable"]
    #[inline(always)]
    pub fn ch_ena3(&self) -> CH_ENA3_R {
        CH_ENA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ch4 enable"]
    #[inline(always)]
    pub fn ch_ena4(&self) -> CH_ENA4_R {
        CH_ENA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ch5 enable"]
    #[inline(always)]
    pub fn ch_ena5(&self) -> CH_ENA5_R {
        CH_ENA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ch6 enable"]
    #[inline(always)]
    pub fn ch_ena6(&self) -> CH_ENA6_R {
        CH_ENA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ch7 enable"]
    #[inline(always)]
    pub fn ch_ena7(&self) -> CH_ENA7_R {
        CH_ENA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ch8 enable"]
    #[inline(always)]
    pub fn ch_ena8(&self) -> CH_ENA8_R {
        CH_ENA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ch9 enable"]
    #[inline(always)]
    pub fn ch_ena9(&self) -> CH_ENA9_R {
        CH_ENA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ch10 enable"]
    #[inline(always)]
    pub fn ch_ena10(&self) -> CH_ENA10_R {
        CH_ENA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ch11 enable"]
    #[inline(always)]
    pub fn ch_ena11(&self) -> CH_ENA11_R {
        CH_ENA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ch12 enable"]
    #[inline(always)]
    pub fn ch_ena12(&self) -> CH_ENA12_R {
        CH_ENA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ch13 enable"]
    #[inline(always)]
    pub fn ch_ena13(&self) -> CH_ENA13_R {
        CH_ENA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ch14 enable"]
    #[inline(always)]
    pub fn ch_ena14(&self) -> CH_ENA14_R {
        CH_ENA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ch15 enable"]
    #[inline(always)]
    pub fn ch_ena15(&self) -> CH_ENA15_R {
        CH_ENA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ch16 enable"]
    #[inline(always)]
    pub fn ch_ena16(&self) -> CH_ENA16_R {
        CH_ENA16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ch17 enable"]
    #[inline(always)]
    pub fn ch_ena17(&self) -> CH_ENA17_R {
        CH_ENA17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ch18 enable"]
    #[inline(always)]
    pub fn ch_ena18(&self) -> CH_ENA18_R {
        CH_ENA18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ch19 enable"]
    #[inline(always)]
    pub fn ch_ena19(&self) -> CH_ENA19_R {
        CH_ENA19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ch20 enable"]
    #[inline(always)]
    pub fn ch_ena20(&self) -> CH_ENA20_R {
        CH_ENA20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ch21 enable"]
    #[inline(always)]
    pub fn ch_ena21(&self) -> CH_ENA21_R {
        CH_ENA21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ch22 enable"]
    #[inline(always)]
    pub fn ch_ena22(&self) -> CH_ENA22_R {
        CH_ENA22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ch23 enable"]
    #[inline(always)]
    pub fn ch_ena23(&self) -> CH_ENA23_R {
        CH_ENA23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ch24 enable"]
    #[inline(always)]
    pub fn ch_ena24(&self) -> CH_ENA24_R {
        CH_ENA24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ch25 enable"]
    #[inline(always)]
    pub fn ch_ena25(&self) -> CH_ENA25_R {
        CH_ENA25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ch26 enable"]
    #[inline(always)]
    pub fn ch_ena26(&self) -> CH_ENA26_R {
        CH_ENA26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ch27 enable"]
    #[inline(always)]
    pub fn ch_ena27(&self) -> CH_ENA27_R {
        CH_ENA27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ch28 enable"]
    #[inline(always)]
    pub fn ch_ena28(&self) -> CH_ENA28_R {
        CH_ENA28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ch29 enable"]
    #[inline(always)]
    pub fn ch_ena29(&self) -> CH_ENA29_R {
        CH_ENA29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ch30 enable"]
    #[inline(always)]
    pub fn ch_ena30(&self) -> CH_ENA30_R {
        CH_ENA30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ch31 enable"]
    #[inline(always)]
    pub fn ch_ena31(&self) -> CH_ENA31_R {
        CH_ENA31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_ENA_AD0")
            .field("ch_ena0", &format_args!("{}", self.ch_ena0().bit()))
            .field("ch_ena1", &format_args!("{}", self.ch_ena1().bit()))
            .field("ch_ena2", &format_args!("{}", self.ch_ena2().bit()))
            .field("ch_ena3", &format_args!("{}", self.ch_ena3().bit()))
            .field("ch_ena4", &format_args!("{}", self.ch_ena4().bit()))
            .field("ch_ena5", &format_args!("{}", self.ch_ena5().bit()))
            .field("ch_ena6", &format_args!("{}", self.ch_ena6().bit()))
            .field("ch_ena7", &format_args!("{}", self.ch_ena7().bit()))
            .field("ch_ena8", &format_args!("{}", self.ch_ena8().bit()))
            .field("ch_ena9", &format_args!("{}", self.ch_ena9().bit()))
            .field("ch_ena10", &format_args!("{}", self.ch_ena10().bit()))
            .field("ch_ena11", &format_args!("{}", self.ch_ena11().bit()))
            .field("ch_ena12", &format_args!("{}", self.ch_ena12().bit()))
            .field("ch_ena13", &format_args!("{}", self.ch_ena13().bit()))
            .field("ch_ena14", &format_args!("{}", self.ch_ena14().bit()))
            .field("ch_ena15", &format_args!("{}", self.ch_ena15().bit()))
            .field("ch_ena16", &format_args!("{}", self.ch_ena16().bit()))
            .field("ch_ena17", &format_args!("{}", self.ch_ena17().bit()))
            .field("ch_ena18", &format_args!("{}", self.ch_ena18().bit()))
            .field("ch_ena19", &format_args!("{}", self.ch_ena19().bit()))
            .field("ch_ena20", &format_args!("{}", self.ch_ena20().bit()))
            .field("ch_ena21", &format_args!("{}", self.ch_ena21().bit()))
            .field("ch_ena22", &format_args!("{}", self.ch_ena22().bit()))
            .field("ch_ena23", &format_args!("{}", self.ch_ena23().bit()))
            .field("ch_ena24", &format_args!("{}", self.ch_ena24().bit()))
            .field("ch_ena25", &format_args!("{}", self.ch_ena25().bit()))
            .field("ch_ena26", &format_args!("{}", self.ch_ena26().bit()))
            .field("ch_ena27", &format_args!("{}", self.ch_ena27().bit()))
            .field("ch_ena28", &format_args!("{}", self.ch_ena28().bit()))
            .field("ch_ena29", &format_args!("{}", self.ch_ena29().bit()))
            .field("ch_ena30", &format_args!("{}", self.ch_ena30().bit()))
            .field("ch_ena31", &format_args!("{}", self.ch_ena31().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - ch0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena0(&mut self) -> CH_ENA0_W<0> {
        CH_ENA0_W::new(self)
    }
    #[doc = "Bit 1 - ch1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena1(&mut self) -> CH_ENA1_W<1> {
        CH_ENA1_W::new(self)
    }
    #[doc = "Bit 2 - ch2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena2(&mut self) -> CH_ENA2_W<2> {
        CH_ENA2_W::new(self)
    }
    #[doc = "Bit 3 - ch3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena3(&mut self) -> CH_ENA3_W<3> {
        CH_ENA3_W::new(self)
    }
    #[doc = "Bit 4 - ch4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena4(&mut self) -> CH_ENA4_W<4> {
        CH_ENA4_W::new(self)
    }
    #[doc = "Bit 5 - ch5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena5(&mut self) -> CH_ENA5_W<5> {
        CH_ENA5_W::new(self)
    }
    #[doc = "Bit 6 - ch6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena6(&mut self) -> CH_ENA6_W<6> {
        CH_ENA6_W::new(self)
    }
    #[doc = "Bit 7 - ch7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena7(&mut self) -> CH_ENA7_W<7> {
        CH_ENA7_W::new(self)
    }
    #[doc = "Bit 8 - ch8 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena8(&mut self) -> CH_ENA8_W<8> {
        CH_ENA8_W::new(self)
    }
    #[doc = "Bit 9 - ch9 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena9(&mut self) -> CH_ENA9_W<9> {
        CH_ENA9_W::new(self)
    }
    #[doc = "Bit 10 - ch10 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena10(&mut self) -> CH_ENA10_W<10> {
        CH_ENA10_W::new(self)
    }
    #[doc = "Bit 11 - ch11 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena11(&mut self) -> CH_ENA11_W<11> {
        CH_ENA11_W::new(self)
    }
    #[doc = "Bit 12 - ch12 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena12(&mut self) -> CH_ENA12_W<12> {
        CH_ENA12_W::new(self)
    }
    #[doc = "Bit 13 - ch13 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena13(&mut self) -> CH_ENA13_W<13> {
        CH_ENA13_W::new(self)
    }
    #[doc = "Bit 14 - ch14 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena14(&mut self) -> CH_ENA14_W<14> {
        CH_ENA14_W::new(self)
    }
    #[doc = "Bit 15 - ch15 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena15(&mut self) -> CH_ENA15_W<15> {
        CH_ENA15_W::new(self)
    }
    #[doc = "Bit 16 - ch16 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena16(&mut self) -> CH_ENA16_W<16> {
        CH_ENA16_W::new(self)
    }
    #[doc = "Bit 17 - ch17 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena17(&mut self) -> CH_ENA17_W<17> {
        CH_ENA17_W::new(self)
    }
    #[doc = "Bit 18 - ch18 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena18(&mut self) -> CH_ENA18_W<18> {
        CH_ENA18_W::new(self)
    }
    #[doc = "Bit 19 - ch19 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena19(&mut self) -> CH_ENA19_W<19> {
        CH_ENA19_W::new(self)
    }
    #[doc = "Bit 20 - ch20 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena20(&mut self) -> CH_ENA20_W<20> {
        CH_ENA20_W::new(self)
    }
    #[doc = "Bit 21 - ch21 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena21(&mut self) -> CH_ENA21_W<21> {
        CH_ENA21_W::new(self)
    }
    #[doc = "Bit 22 - ch22 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena22(&mut self) -> CH_ENA22_W<22> {
        CH_ENA22_W::new(self)
    }
    #[doc = "Bit 23 - ch23 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena23(&mut self) -> CH_ENA23_W<23> {
        CH_ENA23_W::new(self)
    }
    #[doc = "Bit 24 - ch24 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena24(&mut self) -> CH_ENA24_W<24> {
        CH_ENA24_W::new(self)
    }
    #[doc = "Bit 25 - ch25 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena25(&mut self) -> CH_ENA25_W<25> {
        CH_ENA25_W::new(self)
    }
    #[doc = "Bit 26 - ch26 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena26(&mut self) -> CH_ENA26_W<26> {
        CH_ENA26_W::new(self)
    }
    #[doc = "Bit 27 - ch27 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena27(&mut self) -> CH_ENA27_W<27> {
        CH_ENA27_W::new(self)
    }
    #[doc = "Bit 28 - ch28 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena28(&mut self) -> CH_ENA28_W<28> {
        CH_ENA28_W::new(self)
    }
    #[doc = "Bit 29 - ch29 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena29(&mut self) -> CH_ENA29_W<29> {
        CH_ENA29_W::new(self)
    }
    #[doc = "Bit 30 - ch30 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena30(&mut self) -> CH_ENA30_W<30> {
        CH_ENA30_W::new(self)
    }
    #[doc = "Bit 31 - ch31 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena31(&mut self) -> CH_ENA31_W<31> {
        CH_ENA31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ena_ad0](index.html) module"]
pub struct CH_ENA_AD0_SPEC;
impl crate::RegisterSpec for CH_ENA_AD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_ena_ad0::R](R) reader structure"]
impl crate::Readable for CH_ENA_AD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_ena_ad0::W](W) writer structure"]
impl crate::Writable for CH_ENA_AD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH_ENA_AD0 to value 0"]
impl crate::Resettable for CH_ENA_AD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
