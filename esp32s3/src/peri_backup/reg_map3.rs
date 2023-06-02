#[doc = "Register `REG_MAP3` reader"]
pub struct R(crate::R<REG_MAP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_MAP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_MAP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_MAP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_MAP3` writer"]
pub struct W(crate::W<REG_MAP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_MAP3_SPEC>;
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
impl From<crate::W<REG_MAP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_MAP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAP3` reader - x"]
pub type MAP3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MAP3` writer - x"]
pub type MAP3_W<'a, const O: u8> = crate::FieldWriter<'a, REG_MAP3_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map3(&self) -> MAP3_R {
        MAP3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_MAP3")
            .field("map3", &format_args!("{}", self.map3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_MAP3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    #[must_use]
    pub fn map3(&mut self) -> MAP3_W<0> {
        MAP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_map3](index.html) module"]
pub struct REG_MAP3_SPEC;
impl crate::RegisterSpec for REG_MAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_map3::R](R) reader structure"]
impl crate::Readable for REG_MAP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_map3::W](W) writer structure"]
impl crate::Writable for REG_MAP3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_MAP3 to value 0"]
impl crate::Resettable for REG_MAP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
