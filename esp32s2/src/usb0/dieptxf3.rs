#[doc = "Register `DIEPTXF3` reader"]
pub struct R(crate::R<DIEPTXF3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF3` writer"]
pub struct W(crate::W<DIEPTXF3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF3_SPEC>;
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
impl From<crate::W<DIEPTXF3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEP3TXFSTADDR` reader - "]
pub type INEP3TXFSTADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEP3TXFSTADDR` writer - "]
pub type INEP3TXFSTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTXF3_SPEC, 16, O, u16, u16>;
#[doc = "Field `INEP3TXFDEP` reader - "]
pub type INEP3TXFDEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEP3TXFDEP` writer - "]
pub type INEP3TXFDEP_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTXF3_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn inep3txfstaddr(&self) -> INEP3TXFSTADDR_R {
        INEP3TXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn inep3txfdep(&self) -> INEP3TXFDEP_R {
        INEP3TXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF3")
            .field(
                "inep3txfstaddr",
                &format_args!("{}", self.inep3txfstaddr().bits()),
            )
            .field(
                "inep3txfdep",
                &format_args!("{}", self.inep3txfdep().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTXF3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn inep3txfstaddr(&mut self) -> INEP3TXFSTADDR_W<0> {
        INEP3TXFSTADDR_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn inep3txfdep(&mut self) -> INEP3TXFDEP_W<16> {
        INEP3TXFDEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf3](index.html) module"]
pub struct DIEPTXF3_SPEC;
impl crate::RegisterSpec for DIEPTXF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf3::R](R) reader structure"]
impl crate::Readable for DIEPTXF3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf3::W](W) writer structure"]
impl crate::Writable for DIEPTXF3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF3 to value 0x1000_0200"]
impl crate::Resettable for DIEPTXF3_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0200;
}
