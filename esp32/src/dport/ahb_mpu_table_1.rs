#[doc = "Register `AHB_MPU_TABLE_1` reader"]
pub struct R(crate::R<AHB_MPU_TABLE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_MPU_TABLE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_MPU_TABLE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_MPU_TABLE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_MPU_TABLE_1` writer"]
pub struct W(crate::W<AHB_MPU_TABLE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_MPU_TABLE_1_SPEC>;
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
impl From<crate::W<AHB_MPU_TABLE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_MPU_TABLE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHB_ACCESS_GRANT_1` reader - "]
pub type AHB_ACCESS_GRANT_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AHB_ACCESS_GRANT_1` writer - "]
pub type AHB_ACCESS_GRANT_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AHB_MPU_TABLE_1_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn ahb_access_grant_1(&self) -> AHB_ACCESS_GRANT_1_R {
        AHB_ACCESS_GRANT_1_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn ahb_access_grant_1(&mut self) -> AHB_ACCESS_GRANT_1_W<0> {
        AHB_ACCESS_GRANT_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_mpu_table_1](index.html) module"]
pub struct AHB_MPU_TABLE_1_SPEC;
impl crate::RegisterSpec for AHB_MPU_TABLE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_mpu_table_1::R](R) reader structure"]
impl crate::Readable for AHB_MPU_TABLE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_mpu_table_1::W](W) writer structure"]
impl crate::Writable for AHB_MPU_TABLE_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB_MPU_TABLE_1 to value 0x01ff"]
impl crate::Resettable for AHB_MPU_TABLE_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01ff
    }
}
