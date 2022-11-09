#[doc = "Register `DMMU_TABLE11` reader"]
pub struct R(crate::R<DMMU_TABLE11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMMU_TABLE11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMMU_TABLE11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMMU_TABLE11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMMU_TABLE11` writer"]
pub struct W(crate::W<DMMU_TABLE11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMMU_TABLE11_SPEC>;
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
impl From<crate::W<DMMU_TABLE11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMMU_TABLE11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMMU_TABLE11` reader - "]
pub type DMMU_TABLE11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMMU_TABLE11` writer - "]
pub type DMMU_TABLE11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMMU_TABLE11_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table11(&self) -> DMMU_TABLE11_R {
        DMMU_TABLE11_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table11(&mut self) -> DMMU_TABLE11_W<0> {
        DMMU_TABLE11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmmu_table11](index.html) module"]
pub struct DMMU_TABLE11_SPEC;
impl crate::RegisterSpec for DMMU_TABLE11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmmu_table11::R](R) reader structure"]
impl crate::Readable for DMMU_TABLE11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmmu_table11::W](W) writer structure"]
impl crate::Writable for DMMU_TABLE11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMMU_TABLE11 to value 0x0b"]
impl crate::Resettable for DMMU_TABLE11_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
