#[doc = "Register `RD_RESULT_MEM[%s]` reader"]
pub struct R(crate::R<RD_RESULT_MEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_RESULT_MEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_RESULT_MEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_RESULT_MEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_RESULT_MEM[%s]` writer"]
pub struct W(crate::W<RD_RESULT_MEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_RESULT_MEM_SPEC>;
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
impl From<crate::W<RD_RESULT_MEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_RESULT_MEM_SPEC>) -> Self {
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
#[doc = "Result from upstream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_result_mem](index.html) module"]
pub struct RD_RESULT_MEM_SPEC;
impl crate::RegisterSpec for RD_RESULT_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rd_result_mem::R](R) reader structure"]
impl crate::Readable for RD_RESULT_MEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_result_mem::W](W) writer structure"]
impl crate::Writable for RD_RESULT_MEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD_RESULT_MEM[%s] to value 0"]
impl crate::Resettable for RD_RESULT_MEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
