#[doc = "Register `USRID` reader"]
pub struct R(crate::R<USRID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USRID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USRID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USRID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USRID` writer"]
pub struct W(crate::W<USRID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USRID_SPEC>;
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
impl From<crate::W<USRID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USRID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USRID` reader - User identification register, value set by user. Can also be used as a scratchpad register by user."]
pub type USRID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USRID` writer - User identification register, value set by user. Can also be used as a scratchpad register by user."]
pub type USRID_W<'a, const O: u8> = crate::FieldWriter<'a, USRID_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - User identification register, value set by user. Can also be used as a scratchpad register by user."]
    #[inline(always)]
    pub fn usrid(&self) -> USRID_R {
        USRID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USRID")
            .field("usrid", &format_args!("{}", self.usrid().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USRID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - User identification register, value set by user. Can also be used as a scratchpad register by user."]
    #[inline(always)]
    #[must_use]
    pub fn usrid(&mut self) -> USRID_W<0> {
        USRID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User ID (scratchpad) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usrid](index.html) module"]
pub struct USRID_SPEC;
impl crate::RegisterSpec for USRID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usrid::R](R) reader structure"]
impl crate::Readable for USRID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usrid::W](W) writer structure"]
impl crate::Writable for USRID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USRID to value 0"]
impl crate::Resettable for USRID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
