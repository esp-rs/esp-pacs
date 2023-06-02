#[doc = "Register `CLKGATE_FORCE_ON` reader"]
pub struct R(crate::R<CLKGATE_FORCE_ON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKGATE_FORCE_ON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKGATE_FORCE_ON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKGATE_FORCE_ON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKGATE_FORCE_ON` writer"]
pub struct W(crate::W<CLKGATE_FORCE_ON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKGATE_FORCE_ON_SPEC>;
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
impl From<crate::W<CLKGATE_FORCE_ON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKGATE_FORCE_ON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_CLKGATE_FORCE_ON` reader - reg_rom_clkgate_force_on"]
pub type ROM_CLKGATE_FORCE_ON_R = crate::FieldReader;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` writer - reg_rom_clkgate_force_on"]
pub type ROM_CLKGATE_FORCE_ON_W<'a, const O: u8> =
    crate::FieldWriter<'a, CLKGATE_FORCE_ON_SPEC, 2, O>;
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` reader - reg_sram_clkgate_force_on"]
pub type SRAM_CLKGATE_FORCE_ON_R = crate::FieldReader;
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` writer - reg_sram_clkgate_force_on"]
pub type SRAM_CLKGATE_FORCE_ON_W<'a, const O: u8> =
    crate::FieldWriter<'a, CLKGATE_FORCE_ON_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:1 - reg_rom_clkgate_force_on"]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - reg_sram_clkgate_force_on"]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&self) -> SRAM_CLKGATE_FORCE_ON_R {
        SRAM_CLKGATE_FORCE_ON_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKGATE_FORCE_ON")
            .field(
                "rom_clkgate_force_on",
                &format_args!("{}", self.rom_clkgate_force_on().bits()),
            )
            .field(
                "sram_clkgate_force_on",
                &format_args!("{}", self.sram_clkgate_force_on().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKGATE_FORCE_ON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_rom_clkgate_force_on"]
    #[inline(always)]
    #[must_use]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W<0> {
        ROM_CLKGATE_FORCE_ON_W::new(self)
    }
    #[doc = "Bits 2:5 - reg_sram_clkgate_force_on"]
    #[inline(always)]
    #[must_use]
    pub fn sram_clkgate_force_on(&mut self) -> SRAM_CLKGATE_FORCE_ON_W<2> {
        SRAM_CLKGATE_FORCE_ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_CLKGATE_FORCE_ON_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkgate_force_on](index.html) module"]
pub struct CLKGATE_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLKGATE_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkgate_force_on::R](R) reader structure"]
impl crate::Readable for CLKGATE_FORCE_ON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkgate_force_on::W](W) writer structure"]
impl crate::Writable for CLKGATE_FORCE_ON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKGATE_FORCE_ON to value 0x3f"]
impl crate::Resettable for CLKGATE_FORCE_ON_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
