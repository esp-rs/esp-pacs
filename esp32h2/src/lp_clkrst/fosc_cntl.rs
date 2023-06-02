#[doc = "Register `FOSC_CNTL` reader"]
pub struct R(crate::R<FOSC_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FOSC_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FOSC_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FOSC_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FOSC_CNTL` writer"]
pub struct W(crate::W<FOSC_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FOSC_CNTL_SPEC>;
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
impl From<crate::W<FOSC_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FOSC_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FOSC_DFREQ` reader - need_des"]
pub type FOSC_DFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FOSC_DFREQ` writer - need_des"]
pub type FOSC_DFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, FOSC_CNTL_SPEC, 10, O, u16, u16>;
impl R {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn fosc_dfreq(&self) -> FOSC_DFREQ_R {
        FOSC_DFREQ_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FOSC_CNTL")
            .field("fosc_dfreq", &format_args!("{}", self.fosc_dfreq().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FOSC_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn fosc_dfreq(&mut self) -> FOSC_DFREQ_W<22> {
        FOSC_DFREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fosc_cntl](index.html) module"]
pub struct FOSC_CNTL_SPEC;
impl crate::RegisterSpec for FOSC_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fosc_cntl::R](R) reader structure"]
impl crate::Readable for FOSC_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fosc_cntl::W](W) writer structure"]
impl crate::Writable for FOSC_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FOSC_CNTL to value 0x9600_0000"]
impl crate::Resettable for FOSC_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x9600_0000;
}
