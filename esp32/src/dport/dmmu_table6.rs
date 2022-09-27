#[doc = "Register `DMMU_TABLE6` reader"]
pub struct R(crate::R<DMMU_TABLE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMMU_TABLE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMMU_TABLE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMMU_TABLE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMMU_TABLE6` writer"]
pub struct W(crate::W<DMMU_TABLE6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMMU_TABLE6_SPEC>;
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
impl From<crate::W<DMMU_TABLE6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMMU_TABLE6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMMU_TABLE6` reader - "]
pub type DMMU_TABLE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMMU_TABLE6` writer - "]
pub type DMMU_TABLE6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMMU_TABLE6_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table6(&self) -> DMMU_TABLE6_R {
        DMMU_TABLE6_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table6(&mut self) -> DMMU_TABLE6_W<0> {
        DMMU_TABLE6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmmu_table6](index.html) module"]
pub struct DMMU_TABLE6_SPEC;
impl crate::RegisterSpec for DMMU_TABLE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmmu_table6::R](R) reader structure"]
impl crate::Readable for DMMU_TABLE6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmmu_table6::W](W) writer structure"]
impl crate::Writable for DMMU_TABLE6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMMU_TABLE6 to value 0x06"]
impl crate::Resettable for DMMU_TABLE6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
