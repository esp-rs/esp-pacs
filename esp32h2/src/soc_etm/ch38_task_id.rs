#[doc = "Register `CH38_TASK_ID` reader"]
pub struct R(crate::R<CH38_TASK_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH38_TASK_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH38_TASK_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH38_TASK_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH38_TASK_ID` writer"]
pub struct W(crate::W<CH38_TASK_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH38_TASK_ID_SPEC>;
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
impl From<crate::W<CH38_TASK_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH38_TASK_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH38_TASK_ID` reader - ch38_task_id"]
pub type CH38_TASK_ID_R = crate::FieldReader;
#[doc = "Field `CH38_TASK_ID` writer - ch38_task_id"]
pub type CH38_TASK_ID_W<'a, const O: u8> = crate::FieldWriter<'a, CH38_TASK_ID_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ch38_task_id"]
    #[inline(always)]
    pub fn ch38_task_id(&self) -> CH38_TASK_ID_R {
        CH38_TASK_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH38_TASK_ID")
            .field(
                "ch38_task_id",
                &format_args!("{}", self.ch38_task_id().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH38_TASK_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - ch38_task_id"]
    #[inline(always)]
    #[must_use]
    pub fn ch38_task_id(&mut self) -> CH38_TASK_ID_W<0> {
        CH38_TASK_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel38 task id register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch38_task_id](index.html) module"]
pub struct CH38_TASK_ID_SPEC;
impl crate::RegisterSpec for CH38_TASK_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch38_task_id::R](R) reader structure"]
impl crate::Readable for CH38_TASK_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch38_task_id::W](W) writer structure"]
impl crate::Writable for CH38_TASK_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH38_TASK_ID to value 0"]
impl crate::Resettable for CH38_TASK_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
