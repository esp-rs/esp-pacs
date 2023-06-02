#[doc = "Register `ROM_CTRL_1` reader"]
pub struct R(crate::R<ROM_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_CTRL_1` writer"]
pub struct W(crate::W<ROM_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_CTRL_1_SPEC>;
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
impl From<crate::W<ROM_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_FORCE_PD` reader - This field is used to power down internal ROM."]
pub type ROM_FORCE_PD_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PD` writer - This field is used to power down internal ROM."]
pub type ROM_FORCE_PD_W<'a, const O: u8> = crate::FieldWriter<'a, ROM_CTRL_1_SPEC, 2, O>;
#[doc = "Field `ROM_FORCE_PU` reader - This field is used to power up internal ROM."]
pub type ROM_FORCE_PU_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PU` writer - This field is used to power up internal ROM."]
pub type ROM_FORCE_PU_W<'a, const O: u8> = crate::FieldWriter<'a, ROM_CTRL_1_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - This field is used to power down internal ROM."]
    #[inline(always)]
    pub fn rom_force_pd(&self) -> ROM_FORCE_PD_R {
        ROM_FORCE_PD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - This field is used to power up internal ROM."]
    #[inline(always)]
    pub fn rom_force_pu(&self) -> ROM_FORCE_PU_R {
        ROM_FORCE_PU_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_CTRL_1")
            .field(
                "rom_force_pd",
                &format_args!("{}", self.rom_force_pd().bits()),
            )
            .field(
                "rom_force_pu",
                &format_args!("{}", self.rom_force_pu().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ROM_CTRL_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to power down internal ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom_force_pd(&mut self) -> ROM_FORCE_PD_W<0> {
        ROM_FORCE_PD_W::new(self)
    }
    #[doc = "Bits 2:3 - This field is used to power up internal ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom_force_pu(&mut self) -> ROM_FORCE_PU_W<2> {
        ROM_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System ROM configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_ctrl_1](index.html) module"]
pub struct ROM_CTRL_1_SPEC;
impl crate::RegisterSpec for ROM_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_ctrl_1::R](R) reader structure"]
impl crate::Readable for ROM_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_ctrl_1::W](W) writer structure"]
impl crate::Writable for ROM_CTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROM_CTRL_1 to value 0x0c"]
impl crate::Resettable for ROM_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
