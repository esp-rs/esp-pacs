#[doc = "Register `PHYSICAL_ADDRESS` reader"]
pub struct R(crate::R<PHYSICAL_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYSICAL_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYSICAL_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYSICAL_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHYSICAL_ADDRESS` writer"]
pub struct W(crate::W<PHYSICAL_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHYSICAL_ADDRESS_SPEC>;
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
impl From<crate::W<PHYSICAL_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHYSICAL_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHYSICAL_ADDRESS` reader - Physical address."]
pub type PHYSICAL_ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `PHYSICAL_ADDRESS` writer - Physical address."]
pub type PHYSICAL_ADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, PHYSICAL_ADDRESS_SPEC, 30, O, u32>;
impl R {
    #[doc = "Bits 0:29 - Physical address."]
    #[inline(always)]
    pub fn physical_address(&self) -> PHYSICAL_ADDRESS_R {
        PHYSICAL_ADDRESS_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHYSICAL_ADDRESS")
            .field(
                "physical_address",
                &format_args!("{}", self.physical_address().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHYSICAL_ADDRESS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:29 - Physical address."]
    #[inline(always)]
    #[must_use]
    pub fn physical_address(&mut self) -> PHYSICAL_ADDRESS_W<0> {
        PHYSICAL_ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Physical address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [physical_address](index.html) module"]
pub struct PHYSICAL_ADDRESS_SPEC;
impl crate::RegisterSpec for PHYSICAL_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [physical_address::R](R) reader structure"]
impl crate::Readable for PHYSICAL_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [physical_address::W](W) writer structure"]
impl crate::Writable for PHYSICAL_ADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHYSICAL_ADDRESS to value 0"]
impl crate::Resettable for PHYSICAL_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
