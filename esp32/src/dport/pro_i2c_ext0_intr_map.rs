#[doc = "Register `PRO_I2C_EXT0_INTR_MAP` reader"]
pub struct R(crate::R<PRO_I2C_EXT0_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_I2C_EXT0_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_I2C_EXT0_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_I2C_EXT0_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_I2C_EXT0_INTR_MAP` writer"]
pub struct W(crate::W<PRO_I2C_EXT0_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_I2C_EXT0_INTR_MAP_SPEC>;
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
impl From<crate::W<PRO_I2C_EXT0_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_I2C_EXT0_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_I2C_EXT0_INTR_MAP` reader - "]
pub type PRO_I2C_EXT0_INTR_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRO_I2C_EXT0_INTR_MAP` writer - "]
pub type PRO_I2C_EXT0_INTR_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_I2C_EXT0_INTR_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pro_i2c_ext0_intr_map(&self) -> PRO_I2C_EXT0_INTR_MAP_R {
        PRO_I2C_EXT0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pro_i2c_ext0_intr_map(&mut self) -> PRO_I2C_EXT0_INTR_MAP_W<0> {
        PRO_I2C_EXT0_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_i2c_ext0_intr_map](index.html) module"]
pub struct PRO_I2C_EXT0_INTR_MAP_SPEC;
impl crate::RegisterSpec for PRO_I2C_EXT0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_i2c_ext0_intr_map::R](R) reader structure"]
impl crate::Readable for PRO_I2C_EXT0_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_i2c_ext0_intr_map::W](W) writer structure"]
impl crate::Writable for PRO_I2C_EXT0_INTR_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_I2C_EXT0_INTR_MAP to value 0x10"]
impl crate::Resettable for PRO_I2C_EXT0_INTR_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
