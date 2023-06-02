#[doc = "Register `CONFIG5` reader"]
pub struct R(crate::R<CONFIG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG5` writer"]
pub struct W(crate::W<CONFIG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG5_SPEC>;
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
impl From<crate::W<CONFIG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHIP_RESET_TARGET` reader - need_des"]
pub type CHIP_RESET_TARGET_R = crate::FieldReader;
#[doc = "Field `CHIP_RESET_TARGET` writer - need_des"]
pub type CHIP_RESET_TARGET_W<'a, const O: u8> = crate::FieldWriter<'a, CONFIG5_SPEC, 8, O>;
#[doc = "Field `CHIP_RESET_EN` reader - need_des"]
pub type CHIP_RESET_EN_R = crate::BitReader;
#[doc = "Field `CHIP_RESET_EN` writer - need_des"]
pub type CHIP_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONFIG5_SPEC, O>;
#[doc = "Field `CHIP_RESET_KEY` reader - need_des"]
pub type CHIP_RESET_KEY_R = crate::FieldReader;
#[doc = "Field `CHIP_RESET_KEY` writer - need_des"]
pub type CHIP_RESET_KEY_W<'a, const O: u8> = crate::FieldWriter<'a, CONFIG5_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn chip_reset_target(&self) -> CHIP_RESET_TARGET_R {
        CHIP_RESET_TARGET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn chip_reset_en(&self) -> CHIP_RESET_EN_R {
        CHIP_RESET_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - need_des"]
    #[inline(always)]
    pub fn chip_reset_key(&self) -> CHIP_RESET_KEY_R {
        CHIP_RESET_KEY_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG5")
            .field(
                "chip_reset_target",
                &format_args!("{}", self.chip_reset_target().bits()),
            )
            .field(
                "chip_reset_en",
                &format_args!("{}", self.chip_reset_en().bit()),
            )
            .field(
                "chip_reset_key",
                &format_args!("{}", self.chip_reset_key().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONFIG5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn chip_reset_target(&mut self) -> CHIP_RESET_TARGET_W<0> {
        CHIP_RESET_TARGET_W::new(self)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn chip_reset_en(&mut self) -> CHIP_RESET_EN_W<8> {
        CHIP_RESET_EN_W::new(self)
    }
    #[doc = "Bits 9:16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn chip_reset_key(&mut self) -> CHIP_RESET_KEY_W<9> {
        CHIP_RESET_KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config5](index.html) module"]
pub struct CONFIG5_SPEC;
impl crate::RegisterSpec for CONFIG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config5::R](R) reader structure"]
impl crate::Readable for CONFIG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config5::W](W) writer structure"]
impl crate::Writable for CONFIG5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG5 to value 0xff"]
impl crate::Resettable for CONFIG5_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
