#[doc = "Register `DIEPTXF1` reader"]
pub struct R(crate::R<DIEPTXF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF1` writer"]
pub struct W(crate::W<DIEPTXF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF1_SPEC>;
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
impl From<crate::W<DIEPTXF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEP1TXFSTADDR` reader - "]
pub type INEP1TXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEP1TXFSTADDR` writer - "]
pub type INEP1TXFSTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTXF1_SPEC, 16, O, u16>;
#[doc = "Field `INEP1TXFDEP` reader - "]
pub type INEP1TXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEP1TXFDEP` writer - "]
pub type INEP1TXFDEP_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPTXF1_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn inep1txfstaddr(&self) -> INEP1TXFSTADDR_R {
        INEP1TXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn inep1txfdep(&self) -> INEP1TXFDEP_R {
        INEP1TXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF1")
            .field(
                "inep1txfstaddr",
                &format_args!("{}", self.inep1txfstaddr().bits()),
            )
            .field(
                "inep1txfdep",
                &format_args!("{}", self.inep1txfdep().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTXF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn inep1txfstaddr(&mut self) -> INEP1TXFSTADDR_W<0> {
        INEP1TXFSTADDR_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn inep1txfdep(&mut self) -> INEP1TXFDEP_W<16> {
        INEP1TXFDEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf1](index.html) module"]
pub struct DIEPTXF1_SPEC;
impl crate::RegisterSpec for DIEPTXF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf1::R](R) reader structure"]
impl crate::Readable for DIEPTXF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf1::W](W) writer structure"]
impl crate::Writable for DIEPTXF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF1 to value 0x1000_0200"]
impl crate::Resettable for DIEPTXF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0200;
}
