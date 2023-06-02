#[doc = "Register `PRO_VECBASE_SET` reader"]
pub struct R(crate::R<PRO_VECBASE_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_VECBASE_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_VECBASE_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_VECBASE_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_VECBASE_SET` writer"]
pub struct W(crate::W<PRO_VECBASE_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_VECBASE_SET_SPEC>;
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
impl From<crate::W<PRO_VECBASE_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_VECBASE_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_OUT_VECBASE` reader - "]
pub type PRO_OUT_VECBASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_OUT_VECBASE` writer - "]
pub type PRO_OUT_VECBASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_VECBASE_SET_SPEC, 22, O, u32, u32>;
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn pro_out_vecbase(&self) -> PRO_OUT_VECBASE_R {
        PRO_OUT_VECBASE_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_VECBASE_SET")
            .field(
                "pro_out_vecbase",
                &format_args!("{}", self.pro_out_vecbase().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_VECBASE_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    #[must_use]
    pub fn pro_out_vecbase(&mut self) -> PRO_OUT_VECBASE_W<0> {
        PRO_OUT_VECBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_vecbase_set](index.html) module"]
pub struct PRO_VECBASE_SET_SPEC;
impl crate::RegisterSpec for PRO_VECBASE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_vecbase_set::R](R) reader structure"]
impl crate::Readable for PRO_VECBASE_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_vecbase_set::W](W) writer structure"]
impl crate::Writable for PRO_VECBASE_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_VECBASE_SET to value 0"]
impl crate::Resettable for PRO_VECBASE_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
