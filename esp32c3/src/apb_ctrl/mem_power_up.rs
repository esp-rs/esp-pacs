#[doc = "Register `MEM_POWER_UP` reader"]
pub struct R(crate::R<MEM_POWER_UP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_POWER_UP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_POWER_UP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_POWER_UP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_POWER_UP` writer"]
pub struct W(crate::W<MEM_POWER_UP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_POWER_UP_SPEC>;
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
impl From<crate::W<MEM_POWER_UP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_POWER_UP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_POWER_UP` reader - reg_rom_power_up"]
pub type ROM_POWER_UP_R = crate::FieldReader;
#[doc = "Field `ROM_POWER_UP` writer - reg_rom_power_up"]
pub type ROM_POWER_UP_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_POWER_UP_SPEC, 2, O>;
#[doc = "Field `SRAM_POWER_UP` reader - reg_sram_power_up"]
pub type SRAM_POWER_UP_R = crate::FieldReader;
#[doc = "Field `SRAM_POWER_UP` writer - reg_sram_power_up"]
pub type SRAM_POWER_UP_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_POWER_UP_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:1 - reg_rom_power_up"]
    #[inline(always)]
    pub fn rom_power_up(&self) -> ROM_POWER_UP_R {
        ROM_POWER_UP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - reg_sram_power_up"]
    #[inline(always)]
    pub fn sram_power_up(&self) -> SRAM_POWER_UP_R {
        SRAM_POWER_UP_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_POWER_UP")
            .field(
                "rom_power_up",
                &format_args!("{}", self.rom_power_up().bits()),
            )
            .field(
                "sram_power_up",
                &format_args!("{}", self.sram_power_up().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_POWER_UP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_rom_power_up"]
    #[inline(always)]
    #[must_use]
    pub fn rom_power_up(&mut self) -> ROM_POWER_UP_W<0> {
        ROM_POWER_UP_W::new(self)
    }
    #[doc = "Bits 2:5 - reg_sram_power_up"]
    #[inline(always)]
    #[must_use]
    pub fn sram_power_up(&mut self) -> SRAM_POWER_UP_W<2> {
        SRAM_POWER_UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_MEM_POWER_UP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_power_up](index.html) module"]
pub struct MEM_POWER_UP_SPEC;
impl crate::RegisterSpec for MEM_POWER_UP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_power_up::R](R) reader structure"]
impl crate::Readable for MEM_POWER_UP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_power_up::W](W) writer structure"]
impl crate::Writable for MEM_POWER_UP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_POWER_UP to value 0x3f"]
impl crate::Resettable for MEM_POWER_UP_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
