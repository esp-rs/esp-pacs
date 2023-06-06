#[doc = "Register `EXT0` reader"]
pub struct R(crate::R<EXT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT0` writer"]
pub struct W(crate::W<EXT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT0_SPEC>;
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
impl From<crate::W<EXT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_PP_TIME` reader - page program delay time by system clock."]
pub type T_PP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `T_PP_TIME` writer - page program delay time by system clock."]
pub type T_PP_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, EXT0_SPEC, 12, O, u16>;
#[doc = "Field `T_PP_SHIFT` reader - page program delay time shift ."]
pub type T_PP_SHIFT_R = crate::FieldReader;
#[doc = "Field `T_PP_SHIFT` writer - page program delay time shift ."]
pub type T_PP_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, EXT0_SPEC, 4, O>;
#[doc = "Field `T_PP_ENA` reader - page program delay enable."]
pub type T_PP_ENA_R = crate::BitReader;
#[doc = "Field `T_PP_ENA` writer - page program delay enable."]
pub type T_PP_ENA_W<'a, const O: u8> = crate::BitWriter<'a, EXT0_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - page program delay time by system clock."]
    #[inline(always)]
    pub fn t_pp_time(&self) -> T_PP_TIME_R {
        T_PP_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - page program delay time shift ."]
    #[inline(always)]
    pub fn t_pp_shift(&self) -> T_PP_SHIFT_R {
        T_PP_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - page program delay enable."]
    #[inline(always)]
    pub fn t_pp_ena(&self) -> T_PP_ENA_R {
        T_PP_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT0")
            .field("t_pp_time", &format_args!("{}", self.t_pp_time().bits()))
            .field("t_pp_shift", &format_args!("{}", self.t_pp_shift().bits()))
            .field("t_pp_ena", &format_args!("{}", self.t_pp_ena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - page program delay time by system clock."]
    #[inline(always)]
    #[must_use]
    pub fn t_pp_time(&mut self) -> T_PP_TIME_W<0> {
        T_PP_TIME_W::new(self)
    }
    #[doc = "Bits 16:19 - page program delay time shift ."]
    #[inline(always)]
    #[must_use]
    pub fn t_pp_shift(&mut self) -> T_PP_SHIFT_W<16> {
        T_PP_SHIFT_W::new(self)
    }
    #[doc = "Bit 31 - page program delay enable."]
    #[inline(always)]
    #[must_use]
    pub fn t_pp_ena(&mut self) -> T_PP_ENA_W<31> {
        T_PP_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext0](index.html) module"]
pub struct EXT0_SPEC;
impl crate::RegisterSpec for EXT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext0::R](R) reader structure"]
impl crate::Readable for EXT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext0::W](W) writer structure"]
impl crate::Writable for EXT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT0 to value 0x800a_0050"]
impl crate::Resettable for EXT0_SPEC {
    const RESET_VALUE: Self::Ux = 0x800a_0050;
}
