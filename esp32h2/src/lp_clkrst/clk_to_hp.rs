#[doc = "Register `CLK_TO_HP` reader"]
pub struct R(crate::R<CLK_TO_HP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TO_HP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TO_HP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TO_HP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TO_HP` writer"]
pub struct W(crate::W<CLK_TO_HP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TO_HP_SPEC>;
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
impl From<crate::W<CLK_TO_HP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TO_HP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICG_HP_XTAL32K` reader - need_des"]
pub type ICG_HP_XTAL32K_R = crate::BitReader;
#[doc = "Field `ICG_HP_XTAL32K` writer - need_des"]
pub type ICG_HP_XTAL32K_W<'a, const O: u8> = crate::BitWriter<'a, CLK_TO_HP_SPEC, O>;
#[doc = "Field `ICG_HP_SOSC` reader - need_des"]
pub type ICG_HP_SOSC_R = crate::BitReader;
#[doc = "Field `ICG_HP_SOSC` writer - need_des"]
pub type ICG_HP_SOSC_W<'a, const O: u8> = crate::BitWriter<'a, CLK_TO_HP_SPEC, O>;
#[doc = "Field `ICG_HP_OSC32K` reader - need_des"]
pub type ICG_HP_OSC32K_R = crate::BitReader;
#[doc = "Field `ICG_HP_OSC32K` writer - need_des"]
pub type ICG_HP_OSC32K_W<'a, const O: u8> = crate::BitWriter<'a, CLK_TO_HP_SPEC, O>;
#[doc = "Field `ICG_HP_FOSC` reader - need_des"]
pub type ICG_HP_FOSC_R = crate::BitReader;
#[doc = "Field `ICG_HP_FOSC` writer - need_des"]
pub type ICG_HP_FOSC_W<'a, const O: u8> = crate::BitWriter<'a, CLK_TO_HP_SPEC, O>;
impl R {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn icg_hp_xtal32k(&self) -> ICG_HP_XTAL32K_R {
        ICG_HP_XTAL32K_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn icg_hp_sosc(&self) -> ICG_HP_SOSC_R {
        ICG_HP_SOSC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn icg_hp_osc32k(&self) -> ICG_HP_OSC32K_R {
        ICG_HP_OSC32K_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn icg_hp_fosc(&self) -> ICG_HP_FOSC_R {
        ICG_HP_FOSC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_TO_HP")
            .field(
                "icg_hp_xtal32k",
                &format_args!("{}", self.icg_hp_xtal32k().bit()),
            )
            .field("icg_hp_sosc", &format_args!("{}", self.icg_hp_sosc().bit()))
            .field(
                "icg_hp_osc32k",
                &format_args!("{}", self.icg_hp_osc32k().bit()),
            )
            .field("icg_hp_fosc", &format_args!("{}", self.icg_hp_fosc().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_TO_HP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn icg_hp_xtal32k(&mut self) -> ICG_HP_XTAL32K_W<28> {
        ICG_HP_XTAL32K_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn icg_hp_sosc(&mut self) -> ICG_HP_SOSC_W<29> {
        ICG_HP_SOSC_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn icg_hp_osc32k(&mut self) -> ICG_HP_OSC32K_W<30> {
        ICG_HP_OSC32K_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn icg_hp_fosc(&mut self) -> ICG_HP_FOSC_W<31> {
        ICG_HP_FOSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_to_hp](index.html) module"]
pub struct CLK_TO_HP_SPEC;
impl crate::RegisterSpec for CLK_TO_HP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_to_hp::R](R) reader structure"]
impl crate::Readable for CLK_TO_HP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_to_hp::W](W) writer structure"]
impl crate::Writable for CLK_TO_HP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TO_HP to value 0xf000_0000"]
impl crate::Resettable for CLK_TO_HP_SPEC {
    const RESET_VALUE: Self::Ux = 0xf000_0000;
}
