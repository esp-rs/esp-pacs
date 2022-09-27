#[doc = "Register `Z_MEM[%s]` reader"]
pub struct R(crate::R<Z_MEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<Z_MEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<Z_MEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<Z_MEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Z_MEM[%s]` writer"]
pub struct W(crate::W<Z_MEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<Z_MEM_SPEC>;
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
impl From<crate::W<Z_MEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<Z_MEM_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "memory that stores Z\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [z_mem](index.html) module"]
pub struct Z_MEM_SPEC;
impl crate::RegisterSpec for Z_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [z_mem::R](R) reader structure"]
impl crate::Readable for Z_MEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [z_mem::W](W) writer structure"]
impl crate::Writable for Z_MEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Z_MEM[%s] to value 0"]
impl crate::Resettable for Z_MEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
