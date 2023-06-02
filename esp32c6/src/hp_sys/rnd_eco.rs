#[doc = "Register `RND_ECO` reader"]
pub struct R(crate::R<RND_ECO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RND_ECO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RND_ECO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RND_ECO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RND_ECO` writer"]
pub struct W(crate::W<RND_ECO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RND_ECO_SPEC>;
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
impl From<crate::W<RND_ECO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RND_ECO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDCY_ENA` reader - Only reserved for ECO."]
pub type REDCY_ENA_R = crate::BitReader;
#[doc = "Field `REDCY_ENA` writer - Only reserved for ECO."]
pub type REDCY_ENA_W<'a, const O: u8> = crate::BitWriter<'a, RND_ECO_SPEC, O>;
#[doc = "Field `REDCY_RESULT` reader - Only reserved for ECO."]
pub type REDCY_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Only reserved for ECO."]
    #[inline(always)]
    pub fn redcy_ena(&self) -> REDCY_ENA_R {
        REDCY_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Only reserved for ECO."]
    #[inline(always)]
    pub fn redcy_result(&self) -> REDCY_RESULT_R {
        REDCY_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RND_ECO")
            .field("redcy_ena", &format_args!("{}", self.redcy_ena().bit()))
            .field(
                "redcy_result",
                &format_args!("{}", self.redcy_result().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RND_ECO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Only reserved for ECO."]
    #[inline(always)]
    #[must_use]
    pub fn redcy_ena(&mut self) -> REDCY_ENA_W<0> {
        REDCY_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "redcy eco register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnd_eco](index.html) module"]
pub struct RND_ECO_SPEC;
impl crate::RegisterSpec for RND_ECO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnd_eco::R](R) reader structure"]
impl crate::Readable for RND_ECO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rnd_eco::W](W) writer structure"]
impl crate::Writable for RND_ECO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RND_ECO to value 0"]
impl crate::Resettable for RND_ECO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
