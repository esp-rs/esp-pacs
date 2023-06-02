#[doc = "Register `Core_1_World_DRam0_PIF` reader"]
pub struct R(crate::R<CORE_1_WORLD_DRAM0_PIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_WORLD_DRAM0_PIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_WORLD_DRAM0_PIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_WORLD_DRAM0_PIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_1_World_DRam0_PIF` writer"]
pub struct W(crate::W<CORE_1_WORLD_DRAM0_PIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_WORLD_DRAM0_PIF_SPEC>;
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
impl From<crate::W<CORE_1_WORLD_DRAM0_PIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_WORLD_DRAM0_PIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_WORLD_DRAM0_PIF` reader - this field is used to read current world of Dram0 bus and PIF bus"]
pub type CORE_1_WORLD_DRAM0_PIF_R = crate::FieldReader;
#[doc = "Field `CORE_1_WORLD_DRAM0_PIF` writer - this field is used to read current world of Dram0 bus and PIF bus"]
pub type CORE_1_WORLD_DRAM0_PIF_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_WORLD_DRAM0_PIF_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - this field is used to read current world of Dram0 bus and PIF bus"]
    #[inline(always)]
    pub fn core_1_world_dram0_pif(&self) -> CORE_1_WORLD_DRAM0_PIF_R {
        CORE_1_WORLD_DRAM0_PIF_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_World_DRam0_PIF")
            .field(
                "core_1_world_dram0_pif",
                &format_args!("{}", self.core_1_world_dram0_pif().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_WORLD_DRAM0_PIF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - this field is used to read current world of Dram0 bus and PIF bus"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_world_dram0_pif(&mut self) -> CORE_1_WORLD_DRAM0_PIF_W<0> {
        CORE_1_WORLD_DRAM0_PIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_1 dram0 and PIF world register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_world_dram0_pif](index.html) module"]
pub struct CORE_1_WORLD_DRAM0_PIF_SPEC;
impl crate::RegisterSpec for CORE_1_WORLD_DRAM0_PIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_world_dram0_pif::R](R) reader structure"]
impl crate::Readable for CORE_1_WORLD_DRAM0_PIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_world_dram0_pif::W](W) writer structure"]
impl crate::Writable for CORE_1_WORLD_DRAM0_PIF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_1_World_DRam0_PIF to value 0"]
impl crate::Resettable for CORE_1_WORLD_DRAM0_PIF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
