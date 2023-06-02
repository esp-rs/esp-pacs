#[doc = "Register `CFG_UHS1_INT_MODE` reader"]
pub struct R(crate::R<CFG_UHS1_INT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_UHS1_INT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_UHS1_INT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_UHS1_INT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_UHS1_INT_MODE` writer"]
pub struct W(crate::W<CFG_UHS1_INT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_UHS1_INT_MODE_SPEC>;
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
impl From<crate::W<CFG_UHS1_INT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_UHS1_INT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTOE_END_AHEAD_MODE` reader - intoe on dat1 end ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
pub type INTOE_END_AHEAD_MODE_R = crate::FieldReader;
#[doc = "Field `INTOE_END_AHEAD_MODE` writer - intoe on dat1 end ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
pub type INTOE_END_AHEAD_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CFG_UHS1_INT_MODE_SPEC, 2, O>;
#[doc = "Field `INT_END_AHEAD_MODE` reader - int on dat1 end ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
pub type INT_END_AHEAD_MODE_R = crate::FieldReader;
#[doc = "Field `INT_END_AHEAD_MODE` writer - int on dat1 end ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
pub type INT_END_AHEAD_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CFG_UHS1_INT_MODE_SPEC, 2, O>;
#[doc = "Field `INTOE_ST_AHEAD_MODE` reader - intoe on dat1 start ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
pub type INTOE_ST_AHEAD_MODE_R = crate::FieldReader;
#[doc = "Field `INTOE_ST_AHEAD_MODE` writer - intoe on dat1 start ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
pub type INTOE_ST_AHEAD_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CFG_UHS1_INT_MODE_SPEC, 2, O>;
#[doc = "Field `INT_ST_AHEAD_MODE` reader - int on dat1 start ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
pub type INT_ST_AHEAD_MODE_R = crate::FieldReader;
#[doc = "Field `INT_ST_AHEAD_MODE` writer - int on dat1 start ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
pub type INT_ST_AHEAD_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CFG_UHS1_INT_MODE_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - intoe on dat1 end ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
    #[inline(always)]
    pub fn intoe_end_ahead_mode(&self) -> INTOE_END_AHEAD_MODE_R {
        INTOE_END_AHEAD_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - int on dat1 end ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
    #[inline(always)]
    pub fn int_end_ahead_mode(&self) -> INT_END_AHEAD_MODE_R {
        INT_END_AHEAD_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - intoe on dat1 start ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
    #[inline(always)]
    pub fn intoe_st_ahead_mode(&self) -> INTOE_ST_AHEAD_MODE_R {
        INTOE_ST_AHEAD_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - int on dat1 start ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
    #[inline(always)]
    pub fn int_st_ahead_mode(&self) -> INT_ST_AHEAD_MODE_R {
        INT_ST_AHEAD_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_UHS1_INT_MODE")
            .field(
                "intoe_end_ahead_mode",
                &format_args!("{}", self.intoe_end_ahead_mode().bits()),
            )
            .field(
                "int_end_ahead_mode",
                &format_args!("{}", self.int_end_ahead_mode().bits()),
            )
            .field(
                "intoe_st_ahead_mode",
                &format_args!("{}", self.intoe_st_ahead_mode().bits()),
            )
            .field(
                "int_st_ahead_mode",
                &format_args!("{}", self.int_st_ahead_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CFG_UHS1_INT_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - intoe on dat1 end ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
    #[inline(always)]
    #[must_use]
    pub fn intoe_end_ahead_mode(&mut self) -> INTOE_END_AHEAD_MODE_W<0> {
        INTOE_END_AHEAD_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - int on dat1 end ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
    #[inline(always)]
    #[must_use]
    pub fn int_end_ahead_mode(&mut self) -> INT_END_AHEAD_MODE_W<2> {
        INT_END_AHEAD_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - intoe on dat1 start ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
    #[inline(always)]
    #[must_use]
    pub fn intoe_st_ahead_mode(&mut self) -> INTOE_ST_AHEAD_MODE_W<4> {
        INTOE_ST_AHEAD_MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - int on dat1 start ahead of time: 0/3-no, 1-ahead 1sdclk, 2-ahead 2sdclk"]
    #[inline(always)]
    #[must_use]
    pub fn int_st_ahead_mode(&mut self) -> INT_ST_AHEAD_MODE_W<6> {
        INT_ST_AHEAD_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure int to start and end ahead of time in uhs1 mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_uhs1_int_mode](index.html) module"]
pub struct CFG_UHS1_INT_MODE_SPEC;
impl crate::RegisterSpec for CFG_UHS1_INT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_uhs1_int_mode::R](R) reader structure"]
impl crate::Readable for CFG_UHS1_INT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_uhs1_int_mode::W](W) writer structure"]
impl crate::Writable for CFG_UHS1_INT_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_UHS1_INT_MODE to value 0"]
impl crate::Resettable for CFG_UHS1_INT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
