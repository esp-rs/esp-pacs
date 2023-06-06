#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub struct R(crate::R<FUNC_OUT_SEL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_OUT_SEL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_OUT_SEL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub struct W(crate::W<FUNC_OUT_SEL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
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
impl From<crate::W<FUNC_OUT_SEL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_OUT_SEL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_SEL` reader - The value of the bits: 0&lt;=s&lt;=256. Set the value to select output signal. s=0-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
pub type OUT_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `OUT_SEL` writer - The value of the bits: 0&lt;=s&lt;=256. Set the value to select output signal. s=0-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
pub type OUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, FUNC_OUT_SEL_CFG_SPEC, 9, O, u16>;
#[doc = "Field `INV_SEL` reader - set this bit to invert output signal.1:invert.0:not invert."]
pub type INV_SEL_R = crate::BitReader;
#[doc = "Field `INV_SEL` writer - set this bit to invert output signal.1:invert.0:not invert."]
pub type INV_SEL_W<'a, const O: u8> = crate::BitWriter<'a, FUNC_OUT_SEL_CFG_SPEC, O>;
#[doc = "Field `OEN_SEL` reader - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
pub type OEN_SEL_R = crate::BitReader;
#[doc = "Field `OEN_SEL` writer - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
pub type OEN_SEL_W<'a, const O: u8> = crate::BitWriter<'a, FUNC_OUT_SEL_CFG_SPEC, O>;
#[doc = "Field `OEN_INV_SEL` reader - set this bit to invert output enable signal.1:invert.0:not invert."]
pub type OEN_INV_SEL_R = crate::BitReader;
#[doc = "Field `OEN_INV_SEL` writer - set this bit to invert output enable signal.1:invert.0:not invert."]
pub type OEN_INV_SEL_W<'a, const O: u8> = crate::BitWriter<'a, FUNC_OUT_SEL_CFG_SPEC, O>;
impl R {
    #[doc = "Bits 0:8 - The value of the bits: 0&lt;=s&lt;=256. Set the value to select output signal. s=0-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - set this bit to invert output signal.1:invert.0:not invert."]
    #[inline(always)]
    pub fn inv_sel(&self) -> INV_SEL_R {
        INV_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
    #[inline(always)]
    pub fn oen_sel(&self) -> OEN_SEL_R {
        OEN_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - set this bit to invert output enable signal.1:invert.0:not invert."]
    #[inline(always)]
    pub fn oen_inv_sel(&self) -> OEN_INV_SEL_R {
        OEN_INV_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_OUT_SEL_CFG")
            .field("out_sel", &format_args!("{}", self.out_sel().bits()))
            .field("inv_sel", &format_args!("{}", self.inv_sel().bit()))
            .field("oen_sel", &format_args!("{}", self.oen_sel().bit()))
            .field("oen_inv_sel", &format_args!("{}", self.oen_inv_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC_OUT_SEL_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - The value of the bits: 0&lt;=s&lt;=256. Set the value to select output signal. s=0-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OUT_SEL_W<0> {
        OUT_SEL_W::new(self)
    }
    #[doc = "Bit 9 - set this bit to invert output signal.1:invert.0:not invert."]
    #[inline(always)]
    #[must_use]
    pub fn inv_sel(&mut self) -> INV_SEL_W<9> {
        INV_SEL_W::new(self)
    }
    #[doc = "Bit 10 - set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn oen_sel(&mut self) -> OEN_SEL_W<10> {
        OEN_SEL_W::new(self)
    }
    #[doc = "Bit 11 - set this bit to invert output enable signal.1:invert.0:not invert."]
    #[inline(always)]
    #[must_use]
    pub fn oen_inv_sel(&mut self) -> OEN_INV_SEL_W<11> {
        OEN_INV_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO output function select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_out_sel_cfg](index.html) module"]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_out_sel_cfg::R](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_out_sel_cfg::W](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
