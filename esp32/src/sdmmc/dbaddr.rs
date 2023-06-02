#[doc = "Register `DBADDR` reader"]
pub struct R(crate::R<DBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBADDR` writer"]
pub struct W(crate::W<DBADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBADDR_SPEC>;
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
impl From<crate::W<DBADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBADDR` reader - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[1:0\\] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only."]
pub type DBADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DBADDR` writer - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[1:0\\] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only."]
pub type DBADDR_W<'a, const O: u8> = crate::FieldWriter<'a, DBADDR_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[1:0\\] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only."]
    #[inline(always)]
    pub fn dbaddr(&self) -> DBADDR_R {
        DBADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBADDR")
            .field("dbaddr", &format_args!("{}", self.dbaddr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[1:0\\] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only."]
    #[inline(always)]
    #[must_use]
    pub fn dbaddr(&mut self) -> DBADDR_W<0> {
        DBADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbaddr](index.html) module"]
pub struct DBADDR_SPEC;
impl crate::RegisterSpec for DBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbaddr::R](R) reader structure"]
impl crate::Readable for DBADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbaddr::W](W) writer structure"]
impl crate::Writable for DBADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBADDR to value 0"]
impl crate::Resettable for DBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
