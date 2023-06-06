#[doc = "Register `SLCHOSTDATE` reader"]
pub struct R(crate::R<SLCHOSTDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLCHOSTDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLCHOSTDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLCHOSTDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLCHOSTDATE` writer"]
pub struct W(crate::W<SLCHOSTDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLCHOSTDATE_SPEC>;
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
impl From<crate::W<SLCHOSTDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLCHOSTDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_DATE` reader - *******Description***********"]
pub type SLCHOST_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `SLCHOST_DATE` writer - *******Description***********"]
pub type SLCHOST_DATE_W<'a, const O: u8> = crate::FieldWriter<'a, SLCHOSTDATE_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_date(&self) -> SLCHOST_DATE_R {
        SLCHOST_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLCHOSTDATE")
            .field(
                "slchost_date",
                &format_args!("{}", self.slchost_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLCHOSTDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_date(&mut self) -> SLCHOST_DATE_W<0> {
        SLCHOST_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slchostdate](index.html) module"]
pub struct SLCHOSTDATE_SPEC;
impl crate::RegisterSpec for SLCHOSTDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slchostdate::R](R) reader structure"]
impl crate::Readable for SLCHOSTDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slchostdate::W](W) writer structure"]
impl crate::Writable for SLCHOSTDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLCHOSTDATE to value 0x2106_0700"]
impl crate::Resettable for SLCHOSTDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x2106_0700;
}
