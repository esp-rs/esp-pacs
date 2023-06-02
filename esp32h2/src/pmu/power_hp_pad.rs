#[doc = "Register `POWER_HP_PAD` reader"]
pub struct R(crate::R<POWER_HP_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_HP_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_HP_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_HP_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_HP_PAD` writer"]
pub struct W(crate::W<POWER_HP_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_HP_PAD_SPEC>;
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
impl From<crate::W<POWER_HP_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_HP_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_HP_PAD_NO_ISO_ALL` reader - need_des"]
pub type FORCE_HP_PAD_NO_ISO_ALL_R = crate::BitReader;
#[doc = "Field `FORCE_HP_PAD_NO_ISO_ALL` writer - need_des"]
pub type FORCE_HP_PAD_NO_ISO_ALL_W<'a, const O: u8> = crate::BitWriter<'a, POWER_HP_PAD_SPEC, O>;
#[doc = "Field `FORCE_HP_PAD_ISO_ALL` reader - need_des"]
pub type FORCE_HP_PAD_ISO_ALL_R = crate::BitReader;
#[doc = "Field `FORCE_HP_PAD_ISO_ALL` writer - need_des"]
pub type FORCE_HP_PAD_ISO_ALL_W<'a, const O: u8> = crate::BitWriter<'a, POWER_HP_PAD_SPEC, O>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_hp_pad_no_iso_all(&self) -> FORCE_HP_PAD_NO_ISO_ALL_R {
        FORCE_HP_PAD_NO_ISO_ALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_hp_pad_iso_all(&self) -> FORCE_HP_PAD_ISO_ALL_R {
        FORCE_HP_PAD_ISO_ALL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_HP_PAD")
            .field(
                "force_hp_pad_no_iso_all",
                &format_args!("{}", self.force_hp_pad_no_iso_all().bit()),
            )
            .field(
                "force_hp_pad_iso_all",
                &format_args!("{}", self.force_hp_pad_iso_all().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_HP_PAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_pad_no_iso_all(&mut self) -> FORCE_HP_PAD_NO_ISO_ALL_W<0> {
        FORCE_HP_PAD_NO_ISO_ALL_W::new(self)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_pad_iso_all(&mut self) -> FORCE_HP_PAD_ISO_ALL_W<1> {
        FORCE_HP_PAD_ISO_ALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_hp_pad](index.html) module"]
pub struct POWER_HP_PAD_SPEC;
impl crate::RegisterSpec for POWER_HP_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_hp_pad::R](R) reader structure"]
impl crate::Readable for POWER_HP_PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_hp_pad::W](W) writer structure"]
impl crate::Writable for POWER_HP_PAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_HP_PAD to value 0"]
impl crate::Resettable for POWER_HP_PAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
