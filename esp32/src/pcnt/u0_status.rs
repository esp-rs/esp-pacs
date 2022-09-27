#[doc = "Register `U0_STATUS` reader"]
pub struct R(crate::R<U0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U0_STATUS` writer"]
pub struct W(crate::W<U0_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U0_STATUS_SPEC>;
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
impl From<crate::W<U0_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U0_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_STATUS_U0` reader - "]
pub type CORE_STATUS_U0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STATUS_CNT_MODE` reader - "]
pub type STATUS_CNT_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STATUS_CNT_MODE` writer - "]
pub type STATUS_CNT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U0_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `STATUS_THRES1` reader - "]
pub type STATUS_THRES1_R = crate::BitReader<bool>;
#[doc = "Field `STATUS_THRES1` writer - "]
pub type STATUS_THRES1_W<'a, const O: u8> = crate::BitWriter<'a, u32, U0_STATUS_SPEC, bool, O>;
#[doc = "Field `STATUS_THRES0` reader - "]
pub type STATUS_THRES0_R = crate::BitReader<bool>;
#[doc = "Field `STATUS_THRES0` writer - "]
pub type STATUS_THRES0_W<'a, const O: u8> = crate::BitWriter<'a, u32, U0_STATUS_SPEC, bool, O>;
#[doc = "Field `STATUS_L_LIM` reader - "]
pub type STATUS_L_LIM_R = crate::BitReader<bool>;
#[doc = "Field `STATUS_L_LIM` writer - "]
pub type STATUS_L_LIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, U0_STATUS_SPEC, bool, O>;
#[doc = "Field `STATUS_H_LIM` reader - "]
pub type STATUS_H_LIM_R = crate::BitReader<bool>;
#[doc = "Field `STATUS_H_LIM` writer - "]
pub type STATUS_H_LIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, U0_STATUS_SPEC, bool, O>;
#[doc = "Field `STATUS_ZERO` reader - "]
pub type STATUS_ZERO_R = crate::BitReader<bool>;
#[doc = "Field `STATUS_ZERO` writer - "]
pub type STATUS_ZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, U0_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_status_u0(&self) -> CORE_STATUS_U0_R {
        CORE_STATUS_U0_R::new(self.bits)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn status_cnt_mode(&self) -> STATUS_CNT_MODE_R {
        STATUS_CNT_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn status_thres1(&self) -> STATUS_THRES1_R {
        STATUS_THRES1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn status_thres0(&self) -> STATUS_THRES0_R {
        STATUS_THRES0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn status_l_lim(&self) -> STATUS_L_LIM_R {
        STATUS_L_LIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn status_h_lim(&self) -> STATUS_H_LIM_R {
        STATUS_H_LIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn status_zero(&self) -> STATUS_ZERO_R {
        STATUS_ZERO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn status_cnt_mode(&mut self) -> STATUS_CNT_MODE_W<0> {
        STATUS_CNT_MODE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn status_thres1(&mut self) -> STATUS_THRES1_W<2> {
        STATUS_THRES1_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn status_thres0(&mut self) -> STATUS_THRES0_W<3> {
        STATUS_THRES0_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn status_l_lim(&mut self) -> STATUS_L_LIM_W<4> {
        STATUS_L_LIM_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn status_h_lim(&mut self) -> STATUS_H_LIM_W<5> {
        STATUS_H_LIM_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn status_zero(&mut self) -> STATUS_ZERO_W<6> {
        STATUS_ZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u0_status](index.html) module"]
pub struct U0_STATUS_SPEC;
impl crate::RegisterSpec for U0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u0_status::R](R) reader structure"]
impl crate::Readable for U0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u0_status::W](W) writer structure"]
impl crate::Writable for U0_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U0_STATUS to value 0"]
impl crate::Resettable for U0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
