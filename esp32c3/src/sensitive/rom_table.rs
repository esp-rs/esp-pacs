#[doc = "Register `ROM_TABLE` reader"]
pub struct R(crate::R<ROM_TABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_TABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_TABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_TABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_TABLE` writer"]
pub struct W(crate::W<ROM_TABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_TABLE_SPEC>;
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
impl From<crate::W<ROM_TABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_TABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_TABLE` reader - rom_table"]
pub type ROM_TABLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ROM_TABLE` writer - rom_table"]
pub type ROM_TABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROM_TABLE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - rom_table"]
    #[inline(always)]
    pub fn rom_table(&self) -> ROM_TABLE_R {
        ROM_TABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - rom_table"]
    #[inline(always)]
    pub fn rom_table(&mut self) -> ROM_TABLE_W<0> {
        ROM_TABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_ROM_TABLE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_table](index.html) module"]
pub struct ROM_TABLE_SPEC;
impl crate::RegisterSpec for ROM_TABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_table::R](R) reader structure"]
impl crate::Readable for ROM_TABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_table::W](W) writer structure"]
impl crate::Writable for ROM_TABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_TABLE to value 0"]
impl crate::Resettable for ROM_TABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
