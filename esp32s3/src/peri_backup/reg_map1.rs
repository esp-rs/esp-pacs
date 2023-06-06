#[doc = "Register `REG_MAP1` reader"]
pub struct R(crate::R<REG_MAP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_MAP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_MAP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_MAP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_MAP1` writer"]
pub struct W(crate::W<REG_MAP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_MAP1_SPEC>;
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
impl From<crate::W<REG_MAP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_MAP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAP1` reader - x"]
pub type MAP1_R = crate::FieldReader<u32>;
#[doc = "Field `MAP1` writer - x"]
pub type MAP1_W<'a, const O: u8> = crate::FieldWriter<'a, REG_MAP1_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map1(&self) -> MAP1_R {
        MAP1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_MAP1")
            .field("map1", &format_args!("{}", self.map1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_MAP1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    #[must_use]
    pub fn map1(&mut self) -> MAP1_W<0> {
        MAP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_map1](index.html) module"]
pub struct REG_MAP1_SPEC;
impl crate::RegisterSpec for REG_MAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_map1::R](R) reader structure"]
impl crate::Readable for REG_MAP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_map1::W](W) writer structure"]
impl crate::Writable for REG_MAP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_MAP1 to value 0"]
impl crate::Resettable for REG_MAP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
