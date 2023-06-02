#[doc = "Register `IDLE_INTR_CNT` reader"]
pub struct R(crate::R<IDLE_INTR_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLE_INTR_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLE_INTR_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLE_INTR_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLE_INTR_CNT` writer"]
pub struct W(crate::W<IDLE_INTR_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLE_INTR_CNT_SPEC>;
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
impl From<crate::W<IDLE_INTR_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLE_INTR_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDLE_INTR_CNT` reader - Configure the number of cycles before triggering idle interrupt."]
pub type IDLE_INTR_CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDLE_INTR_CNT` writer - Configure the number of cycles before triggering idle interrupt."]
pub type IDLE_INTR_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, IDLE_INTR_CNT_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure the number of cycles before triggering idle interrupt."]
    #[inline(always)]
    pub fn idle_intr_cnt(&self) -> IDLE_INTR_CNT_R {
        IDLE_INTR_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDLE_INTR_CNT")
            .field(
                "idle_intr_cnt",
                &format_args!("{}", self.idle_intr_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IDLE_INTR_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure the number of cycles before triggering idle interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn idle_intr_cnt(&mut self) -> IDLE_INTR_CNT_W<0> {
        IDLE_INTR_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure idle interrupt counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle_intr_cnt](index.html) module"]
pub struct IDLE_INTR_CNT_SPEC;
impl crate::RegisterSpec for IDLE_INTR_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idle_intr_cnt::R](R) reader structure"]
impl crate::Readable for IDLE_INTR_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idle_intr_cnt::W](W) writer structure"]
impl crate::Writable for IDLE_INTR_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDLE_INTR_CNT to value 0x01"]
impl crate::Resettable for IDLE_INTR_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
