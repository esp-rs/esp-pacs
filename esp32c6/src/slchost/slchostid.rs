#[doc = "Register `SLCHOSTID` reader"]
pub struct R(crate::R<SLCHOSTID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLCHOSTID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLCHOSTID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLCHOSTID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLCHOSTID` writer"]
pub struct W(crate::W<SLCHOSTID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLCHOSTID_SPEC>;
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
impl From<crate::W<SLCHOSTID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLCHOSTID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_ID` reader - *******Description***********"]
pub type SLCHOST_ID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLCHOST_ID` writer - *******Description***********"]
pub type SLCHOST_ID_W<'a, const O: u8> = crate::FieldWriter<'a, SLCHOSTID_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_id(&self) -> SLCHOST_ID_R {
        SLCHOST_ID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLCHOSTID")
            .field("slchost_id", &format_args!("{}", self.slchost_id().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLCHOSTID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_id(&mut self) -> SLCHOST_ID_W<0> {
        SLCHOST_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slchostid](index.html) module"]
pub struct SLCHOSTID_SPEC;
impl crate::RegisterSpec for SLCHOSTID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slchostid::R](R) reader structure"]
impl crate::Readable for SLCHOSTID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slchostid::W](W) writer structure"]
impl crate::Writable for SLCHOSTID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLCHOSTID to value 0x0600"]
impl crate::Resettable for SLCHOSTID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
