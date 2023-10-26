#[doc = "Register `CH_ENA_AD0_SET` writer"]
pub type W = crate::W<CH_ENA_AD0_SET_SPEC>;
#[doc = "Field `CH_SET0` writer - ch0 set"]
pub type CH_SET0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET1` writer - ch1 set"]
pub type CH_SET1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET2` writer - ch2 set"]
pub type CH_SET2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET3` writer - ch3 set"]
pub type CH_SET3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET4` writer - ch4 set"]
pub type CH_SET4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET5` writer - ch5 set"]
pub type CH_SET5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET6` writer - ch6 set"]
pub type CH_SET6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET7` writer - ch7 set"]
pub type CH_SET7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET8` writer - ch8 set"]
pub type CH_SET8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET9` writer - ch9 set"]
pub type CH_SET9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET10` writer - ch10 set"]
pub type CH_SET10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET11` writer - ch11 set"]
pub type CH_SET11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET12` writer - ch12 set"]
pub type CH_SET12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET13` writer - ch13 set"]
pub type CH_SET13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET14` writer - ch14 set"]
pub type CH_SET14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET15` writer - ch15 set"]
pub type CH_SET15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET16` writer - ch16 set"]
pub type CH_SET16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET17` writer - ch17 set"]
pub type CH_SET17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET18` writer - ch18 set"]
pub type CH_SET18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET19` writer - ch19 set"]
pub type CH_SET19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET20` writer - ch20 set"]
pub type CH_SET20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET21` writer - ch21 set"]
pub type CH_SET21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET22` writer - ch22 set"]
pub type CH_SET22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET23` writer - ch23 set"]
pub type CH_SET23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET24` writer - ch24 set"]
pub type CH_SET24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET25` writer - ch25 set"]
pub type CH_SET25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET26` writer - ch26 set"]
pub type CH_SET26_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET27` writer - ch27 set"]
pub type CH_SET27_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET28` writer - ch28 set"]
pub type CH_SET28_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET29` writer - ch29 set"]
pub type CH_SET29_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET30` writer - ch30 set"]
pub type CH_SET30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH_SET31` writer - ch31 set"]
pub type CH_SET31_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD0_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - ch0 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set0(&mut self) -> CH_SET0_W<CH_ENA_AD0_SET_SPEC, 0> {
        CH_SET0_W::new(self)
    }
    #[doc = "Bit 1 - ch1 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set1(&mut self) -> CH_SET1_W<CH_ENA_AD0_SET_SPEC, 1> {
        CH_SET1_W::new(self)
    }
    #[doc = "Bit 2 - ch2 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set2(&mut self) -> CH_SET2_W<CH_ENA_AD0_SET_SPEC, 2> {
        CH_SET2_W::new(self)
    }
    #[doc = "Bit 3 - ch3 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set3(&mut self) -> CH_SET3_W<CH_ENA_AD0_SET_SPEC, 3> {
        CH_SET3_W::new(self)
    }
    #[doc = "Bit 4 - ch4 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set4(&mut self) -> CH_SET4_W<CH_ENA_AD0_SET_SPEC, 4> {
        CH_SET4_W::new(self)
    }
    #[doc = "Bit 5 - ch5 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set5(&mut self) -> CH_SET5_W<CH_ENA_AD0_SET_SPEC, 5> {
        CH_SET5_W::new(self)
    }
    #[doc = "Bit 6 - ch6 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set6(&mut self) -> CH_SET6_W<CH_ENA_AD0_SET_SPEC, 6> {
        CH_SET6_W::new(self)
    }
    #[doc = "Bit 7 - ch7 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set7(&mut self) -> CH_SET7_W<CH_ENA_AD0_SET_SPEC, 7> {
        CH_SET7_W::new(self)
    }
    #[doc = "Bit 8 - ch8 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set8(&mut self) -> CH_SET8_W<CH_ENA_AD0_SET_SPEC, 8> {
        CH_SET8_W::new(self)
    }
    #[doc = "Bit 9 - ch9 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set9(&mut self) -> CH_SET9_W<CH_ENA_AD0_SET_SPEC, 9> {
        CH_SET9_W::new(self)
    }
    #[doc = "Bit 10 - ch10 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set10(&mut self) -> CH_SET10_W<CH_ENA_AD0_SET_SPEC, 10> {
        CH_SET10_W::new(self)
    }
    #[doc = "Bit 11 - ch11 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set11(&mut self) -> CH_SET11_W<CH_ENA_AD0_SET_SPEC, 11> {
        CH_SET11_W::new(self)
    }
    #[doc = "Bit 12 - ch12 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set12(&mut self) -> CH_SET12_W<CH_ENA_AD0_SET_SPEC, 12> {
        CH_SET12_W::new(self)
    }
    #[doc = "Bit 13 - ch13 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set13(&mut self) -> CH_SET13_W<CH_ENA_AD0_SET_SPEC, 13> {
        CH_SET13_W::new(self)
    }
    #[doc = "Bit 14 - ch14 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set14(&mut self) -> CH_SET14_W<CH_ENA_AD0_SET_SPEC, 14> {
        CH_SET14_W::new(self)
    }
    #[doc = "Bit 15 - ch15 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set15(&mut self) -> CH_SET15_W<CH_ENA_AD0_SET_SPEC, 15> {
        CH_SET15_W::new(self)
    }
    #[doc = "Bit 16 - ch16 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set16(&mut self) -> CH_SET16_W<CH_ENA_AD0_SET_SPEC, 16> {
        CH_SET16_W::new(self)
    }
    #[doc = "Bit 17 - ch17 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set17(&mut self) -> CH_SET17_W<CH_ENA_AD0_SET_SPEC, 17> {
        CH_SET17_W::new(self)
    }
    #[doc = "Bit 18 - ch18 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set18(&mut self) -> CH_SET18_W<CH_ENA_AD0_SET_SPEC, 18> {
        CH_SET18_W::new(self)
    }
    #[doc = "Bit 19 - ch19 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set19(&mut self) -> CH_SET19_W<CH_ENA_AD0_SET_SPEC, 19> {
        CH_SET19_W::new(self)
    }
    #[doc = "Bit 20 - ch20 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set20(&mut self) -> CH_SET20_W<CH_ENA_AD0_SET_SPEC, 20> {
        CH_SET20_W::new(self)
    }
    #[doc = "Bit 21 - ch21 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set21(&mut self) -> CH_SET21_W<CH_ENA_AD0_SET_SPEC, 21> {
        CH_SET21_W::new(self)
    }
    #[doc = "Bit 22 - ch22 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set22(&mut self) -> CH_SET22_W<CH_ENA_AD0_SET_SPEC, 22> {
        CH_SET22_W::new(self)
    }
    #[doc = "Bit 23 - ch23 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set23(&mut self) -> CH_SET23_W<CH_ENA_AD0_SET_SPEC, 23> {
        CH_SET23_W::new(self)
    }
    #[doc = "Bit 24 - ch24 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set24(&mut self) -> CH_SET24_W<CH_ENA_AD0_SET_SPEC, 24> {
        CH_SET24_W::new(self)
    }
    #[doc = "Bit 25 - ch25 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set25(&mut self) -> CH_SET25_W<CH_ENA_AD0_SET_SPEC, 25> {
        CH_SET25_W::new(self)
    }
    #[doc = "Bit 26 - ch26 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set26(&mut self) -> CH_SET26_W<CH_ENA_AD0_SET_SPEC, 26> {
        CH_SET26_W::new(self)
    }
    #[doc = "Bit 27 - ch27 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set27(&mut self) -> CH_SET27_W<CH_ENA_AD0_SET_SPEC, 27> {
        CH_SET27_W::new(self)
    }
    #[doc = "Bit 28 - ch28 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set28(&mut self) -> CH_SET28_W<CH_ENA_AD0_SET_SPEC, 28> {
        CH_SET28_W::new(self)
    }
    #[doc = "Bit 29 - ch29 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set29(&mut self) -> CH_SET29_W<CH_ENA_AD0_SET_SPEC, 29> {
        CH_SET29_W::new(self)
    }
    #[doc = "Bit 30 - ch30 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set30(&mut self) -> CH_SET30_W<CH_ENA_AD0_SET_SPEC, 30> {
        CH_SET30_W::new(self)
    }
    #[doc = "Bit 31 - ch31 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set31(&mut self) -> CH_SET31_W<CH_ENA_AD0_SET_SPEC, 31> {
        CH_SET31_W::new(self)
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
#[doc = "channel enable set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD0_SET_SPEC;
impl crate::RegisterSpec for CH_ENA_AD0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad0_set::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD0_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH_ENA_AD0_SET to value 0"]
impl crate::Resettable for CH_ENA_AD0_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
