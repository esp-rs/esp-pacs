#[doc = "Register `PRO_DPORT_APB_MASK0` reader"]
pub struct R(crate::R<PRO_DPORT_APB_MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DPORT_APB_MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DPORT_APB_MASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DPORT_APB_MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DPORT_APB_MASK0` writer"]
pub struct W(crate::W<PRO_DPORT_APB_MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DPORT_APB_MASK0_SPEC>;
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
impl From<crate::W<PRO_DPORT_APB_MASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DPORT_APB_MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRODPORT_APB_MASK0` reader - "]
pub type PRODPORT_APB_MASK0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRODPORT_APB_MASK0` writer - "]
pub type PRODPORT_APB_MASK0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_DPORT_APB_MASK0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn prodport_apb_mask0(&self) -> PRODPORT_APB_MASK0_R {
        PRODPORT_APB_MASK0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn prodport_apb_mask0(&mut self) -> PRODPORT_APB_MASK0_W<0> {
        PRODPORT_APB_MASK0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dport_apb_mask0](index.html) module"]
pub struct PRO_DPORT_APB_MASK0_SPEC;
impl crate::RegisterSpec for PRO_DPORT_APB_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dport_apb_mask0::R](R) reader structure"]
impl crate::Readable for PRO_DPORT_APB_MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dport_apb_mask0::W](W) writer structure"]
impl crate::Writable for PRO_DPORT_APB_MASK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_APB_MASK0 to value 0"]
impl crate::Resettable for PRO_DPORT_APB_MASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
