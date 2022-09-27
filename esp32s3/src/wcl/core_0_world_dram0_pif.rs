#[doc = "Register `Core_0_World_DRam0_PIF` reader"]
pub struct R(crate::R<CORE_0_WORLD_DRAM0_PIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_WORLD_DRAM0_PIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_WORLD_DRAM0_PIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_WORLD_DRAM0_PIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_0_World_DRam0_PIF` writer"]
pub struct W(crate::W<CORE_0_WORLD_DRAM0_PIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_WORLD_DRAM0_PIF_SPEC>;
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
impl From<crate::W<CORE_0_WORLD_DRAM0_PIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_WORLD_DRAM0_PIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_WORLD_DRAM0_PIF` reader - this field is used to read current world of Dram0 bus and PIF bus"]
pub type CORE_0_WORLD_DRAM0_PIF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_WORLD_DRAM0_PIF` writer - this field is used to read current world of Dram0 bus and PIF bus"]
pub type CORE_0_WORLD_DRAM0_PIF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_WORLD_DRAM0_PIF_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - this field is used to read current world of Dram0 bus and PIF bus"]
    #[inline(always)]
    pub fn core_0_world_dram0_pif(&self) -> CORE_0_WORLD_DRAM0_PIF_R {
        CORE_0_WORLD_DRAM0_PIF_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - this field is used to read current world of Dram0 bus and PIF bus"]
    #[inline(always)]
    pub fn core_0_world_dram0_pif(&mut self) -> CORE_0_WORLD_DRAM0_PIF_W<0> {
        CORE_0_WORLD_DRAM0_PIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_0 dram0 and PIF world register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_world_dram0_pif](index.html) module"]
pub struct CORE_0_WORLD_DRAM0_PIF_SPEC;
impl crate::RegisterSpec for CORE_0_WORLD_DRAM0_PIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_world_dram0_pif::R](R) reader structure"]
impl crate::Readable for CORE_0_WORLD_DRAM0_PIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_world_dram0_pif::W](W) writer structure"]
impl crate::Writable for CORE_0_WORLD_DRAM0_PIF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_0_World_DRam0_PIF to value 0"]
impl crate::Resettable for CORE_0_WORLD_DRAM0_PIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
