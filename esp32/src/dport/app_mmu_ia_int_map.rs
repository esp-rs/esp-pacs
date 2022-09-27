#[doc = "Register `APP_MMU_IA_INT_MAP` reader"]
pub struct R(crate::R<APP_MMU_IA_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_MMU_IA_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_MMU_IA_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_MMU_IA_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_MMU_IA_INT_MAP` writer"]
pub struct W(crate::W<APP_MMU_IA_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_MMU_IA_INT_MAP_SPEC>;
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
impl From<crate::W<APP_MMU_IA_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_MMU_IA_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_MMU_IA_INT_MAP` reader - "]
pub type APP_MMU_IA_INT_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APP_MMU_IA_INT_MAP` writer - "]
pub type APP_MMU_IA_INT_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APP_MMU_IA_INT_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_mmu_ia_int_map(&self) -> APP_MMU_IA_INT_MAP_R {
        APP_MMU_IA_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_mmu_ia_int_map(&mut self) -> APP_MMU_IA_INT_MAP_W<0> {
        APP_MMU_IA_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_mmu_ia_int_map](index.html) module"]
pub struct APP_MMU_IA_INT_MAP_SPEC;
impl crate::RegisterSpec for APP_MMU_IA_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_mmu_ia_int_map::R](R) reader structure"]
impl crate::Readable for APP_MMU_IA_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_mmu_ia_int_map::W](W) writer structure"]
impl crate::Writable for APP_MMU_IA_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_MMU_IA_INT_MAP to value 0x10"]
impl crate::Resettable for APP_MMU_IA_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
