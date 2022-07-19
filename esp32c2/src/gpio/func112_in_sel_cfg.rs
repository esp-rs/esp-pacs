#[doc = "Register `FUNC112_IN_SEL_CFG` reader"]
pub struct R(crate::R<FUNC112_IN_SEL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC112_IN_SEL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC112_IN_SEL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC112_IN_SEL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC112_IN_SEL_CFG` writer"]
pub struct W(crate::W<FUNC112_IN_SEL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC112_IN_SEL_CFG_SPEC>;
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
impl From<crate::W<FUNC112_IN_SEL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC112_IN_SEL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC112_IN_SEL` reader - set this value: s=0-53: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
pub type FUNC112_IN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNC112_IN_SEL` writer - set this value: s=0-53: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
pub type FUNC112_IN_SEL_W<'a> = crate::FieldWriter<'a, u32, FUNC112_IN_SEL_CFG_SPEC, u8, u8, 5, 0>;
#[doc = "Field `FUNC112_IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC112_IN_INV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `FUNC112_IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC112_IN_INV_SEL_W<'a> = crate::BitWriter<'a, u32, FUNC112_IN_SEL_CFG_SPEC, bool, 5>;
#[doc = "Field `SIG112_IN_SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG112_IN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SIG112_IN_SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG112_IN_SEL_W<'a> = crate::BitWriter<'a, u32, FUNC112_IN_SEL_CFG_SPEC, bool, 6>;
impl R {
    #[doc = "Bits 0:4 - set this value: s=0-53: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
    #[inline(always)]
    pub fn func112_in_sel(&self) -> FUNC112_IN_SEL_R {
        FUNC112_IN_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func112_in_inv_sel(&self) -> FUNC112_IN_INV_SEL_R {
        FUNC112_IN_INV_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig112_in_sel(&self) -> SIG112_IN_SEL_R {
        SIG112_IN_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - set this value: s=0-53: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
    #[inline(always)]
    pub fn func112_in_sel(&mut self) -> FUNC112_IN_SEL_W {
        FUNC112_IN_SEL_W::new(self)
    }
    #[doc = "Bit 5 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func112_in_inv_sel(&mut self) -> FUNC112_IN_INV_SEL_W {
        FUNC112_IN_INV_SEL_W::new(self)
    }
    #[doc = "Bit 6 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig112_in_sel(&mut self) -> SIG112_IN_SEL_W {
        SIG112_IN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO input function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func112_in_sel_cfg](index.html) module"]
pub struct FUNC112_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC112_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func112_in_sel_cfg::R](R) reader structure"]
impl crate::Readable for FUNC112_IN_SEL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func112_in_sel_cfg::W](W) writer structure"]
impl crate::Writable for FUNC112_IN_SEL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC112_IN_SEL_CFG to value 0"]
impl crate::Resettable for FUNC112_IN_SEL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
