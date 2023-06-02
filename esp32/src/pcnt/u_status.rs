#[doc = "Register `U%s_STATUS` reader"]
pub struct R(crate::R<U_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U%s_STATUS` writer"]
pub struct W(crate::W<U_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U_STATUS_SPEC>;
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
impl From<crate::W<U_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_STATUS_U0` reader - "]
pub type CORE_STATUS_U0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ZERO_MODE` reader - "]
pub type ZERO_MODE_R = crate::FieldReader;
#[doc = "Field `ZERO_MODE` writer - "]
pub type ZERO_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, U_STATUS_SPEC, 2, O>;
#[doc = "Field `THRES1` reader - "]
pub type THRES1_R = crate::BitReader;
#[doc = "Field `THRES1` writer - "]
pub type THRES1_W<'a, const O: u8> = crate::BitWriter<'a, U_STATUS_SPEC, O>;
#[doc = "Field `THRES0` reader - "]
pub type THRES0_R = crate::BitReader;
#[doc = "Field `THRES0` writer - "]
pub type THRES0_W<'a, const O: u8> = crate::BitWriter<'a, U_STATUS_SPEC, O>;
#[doc = "Field `L_LIM` reader - "]
pub type L_LIM_R = crate::BitReader;
#[doc = "Field `L_LIM` writer - "]
pub type L_LIM_W<'a, const O: u8> = crate::BitWriter<'a, U_STATUS_SPEC, O>;
#[doc = "Field `H_LIM` reader - "]
pub type H_LIM_R = crate::BitReader;
#[doc = "Field `H_LIM` writer - "]
pub type H_LIM_W<'a, const O: u8> = crate::BitWriter<'a, U_STATUS_SPEC, O>;
#[doc = "Field `ZERO` reader - "]
pub type ZERO_R = crate::BitReader;
#[doc = "Field `ZERO` writer - "]
pub type ZERO_W<'a, const O: u8> = crate::BitWriter<'a, U_STATUS_SPEC, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_status_u0(&self) -> CORE_STATUS_U0_R {
        CORE_STATUS_U0_R::new(self.bits)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn zero_mode(&self) -> ZERO_MODE_R {
        ZERO_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn thres1(&self) -> THRES1_R {
        THRES1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn thres0(&self) -> THRES0_R {
        THRES0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn l_lim(&self) -> L_LIM_R {
        L_LIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_lim(&self) -> H_LIM_R {
        H_LIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_STATUS")
            .field(
                "core_status_u0",
                &format_args!("{}", self.core_status_u0().bits()),
            )
            .field("zero_mode", &format_args!("{}", self.zero_mode().bits()))
            .field("thres1", &format_args!("{}", self.thres1().bit()))
            .field("thres0", &format_args!("{}", self.thres0().bit()))
            .field("l_lim", &format_args!("{}", self.l_lim().bit()))
            .field("h_lim", &format_args!("{}", self.h_lim().bit()))
            .field("zero", &format_args!("{}", self.zero().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<U_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn zero_mode(&mut self) -> ZERO_MODE_W<0> {
        ZERO_MODE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn thres1(&mut self) -> THRES1_W<2> {
        THRES1_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn thres0(&mut self) -> THRES0_W<3> {
        THRES0_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn l_lim(&mut self) -> L_LIM_W<4> {
        L_LIM_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_lim(&mut self) -> H_LIM_W<5> {
        H_LIM_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZERO_W<6> {
        ZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u_status](index.html) module"]
pub struct U_STATUS_SPEC;
impl crate::RegisterSpec for U_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u_status::R](R) reader structure"]
impl crate::Readable for U_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u_status::W](W) writer structure"]
impl crate::Writable for U_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets U%s_STATUS to value 0"]
impl crate::Resettable for U_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
