#[doc = "Register `REF_CNT_RST` reader"]
pub struct R(crate::R<REF_CNT_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REF_CNT_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REF_CNT_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REF_CNT_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REF_CNT_RST` writer"]
pub struct W(crate::W<REF_CNT_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REF_CNT_RST_SPEC>;
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
impl From<crate::W<REF_CNT_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REF_CNT_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - This register is used to reset the clock divider of CHANNEL0."]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - This register is used to reset the clock divider of CHANNEL0."]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, REF_CNT_RST_SPEC, O>;
#[doc = "Field `CH1` reader - This register is used to reset the clock divider of CHANNEL1."]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - This register is used to reset the clock divider of CHANNEL1."]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, REF_CNT_RST_SPEC, O>;
#[doc = "Field `CH2` reader - This register is used to reset the clock divider of CHANNEL2."]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - This register is used to reset the clock divider of CHANNEL2."]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, REF_CNT_RST_SPEC, O>;
#[doc = "Field `CH3` reader - This register is used to reset the clock divider of CHANNEL3."]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH3` writer - This register is used to reset the clock divider of CHANNEL3."]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, REF_CNT_RST_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This register is used to reset the clock divider of CHANNEL1."]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This register is used to reset the clock divider of CHANNEL2."]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This register is used to reset the clock divider of CHANNEL3."]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_CNT_RST")
            .field("ch0", &format_args!("{}", self.ch0().bit()))
            .field("ch1", &format_args!("{}", self.ch1().bit()))
            .field("ch2", &format_args!("{}", self.ch2().bit()))
            .field("ch3", &format_args!("{}", self.ch3().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CNT_RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - This register is used to reset the clock divider of CHANNEL1."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - This register is used to reset the clock divider of CHANNEL2."]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - This register is used to reset the clock divider of CHANNEL3."]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT clock divider reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_cnt_rst](index.html) module"]
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ref_cnt_rst::R](R) reader structure"]
impl crate::Readable for REF_CNT_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ref_cnt_rst::W](W) writer structure"]
impl crate::Writable for REF_CNT_RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_CNT_RST to value 0"]
impl crate::Resettable for REF_CNT_RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
