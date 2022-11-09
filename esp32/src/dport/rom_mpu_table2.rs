#[doc = "Register `ROM_MPU_TABLE2` reader"]
pub struct R(crate::R<ROM_MPU_TABLE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_MPU_TABLE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_MPU_TABLE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_MPU_TABLE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_MPU_TABLE2` writer"]
pub struct W(crate::W<ROM_MPU_TABLE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_MPU_TABLE2_SPEC>;
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
impl From<crate::W<ROM_MPU_TABLE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_MPU_TABLE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_MPU_TABLE2` reader - "]
pub type ROM_MPU_TABLE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROM_MPU_TABLE2` writer - "]
pub type ROM_MPU_TABLE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROM_MPU_TABLE2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_mpu_table2(&self) -> ROM_MPU_TABLE2_R {
        ROM_MPU_TABLE2_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn rom_mpu_table2(&mut self) -> ROM_MPU_TABLE2_W<0> {
        ROM_MPU_TABLE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_mpu_table2](index.html) module"]
pub struct ROM_MPU_TABLE2_SPEC;
impl crate::RegisterSpec for ROM_MPU_TABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_mpu_table2::R](R) reader structure"]
impl crate::Readable for ROM_MPU_TABLE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_mpu_table2::W](W) writer structure"]
impl crate::Writable for ROM_MPU_TABLE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROM_MPU_TABLE2 to value 0x01"]
impl crate::Resettable for ROM_MPU_TABLE2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
