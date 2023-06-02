#[doc = "Register `HP_MODEM_ICG_MODEM` reader"]
pub struct R(crate::R<HP_MODEM_ICG_MODEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_MODEM_ICG_MODEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_MODEM_ICG_MODEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_MODEM_ICG_MODEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_MODEM_ICG_MODEM` writer"]
pub struct W(crate::W<HP_MODEM_ICG_MODEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_MODEM_ICG_MODEM_SPEC>;
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
impl From<crate::W<HP_MODEM_ICG_MODEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_MODEM_ICG_MODEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_MODEM_DIG_ICG_MODEM_CODE` reader - need_des"]
pub type HP_MODEM_DIG_ICG_MODEM_CODE_R = crate::FieldReader;
#[doc = "Field `HP_MODEM_DIG_ICG_MODEM_CODE` writer - need_des"]
pub type HP_MODEM_DIG_ICG_MODEM_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_MODEM_ICG_MODEM_SPEC, 2, O>;
impl R {
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_icg_modem_code(&self) -> HP_MODEM_DIG_ICG_MODEM_CODE_R {
        HP_MODEM_DIG_ICG_MODEM_CODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_ICG_MODEM")
            .field(
                "hp_modem_dig_icg_modem_code",
                &format_args!("{}", self.hp_modem_dig_icg_modem_code().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_ICG_MODEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_dig_icg_modem_code(&mut self) -> HP_MODEM_DIG_ICG_MODEM_CODE_W<30> {
        HP_MODEM_DIG_ICG_MODEM_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_modem_icg_modem](index.html) module"]
pub struct HP_MODEM_ICG_MODEM_SPEC;
impl crate::RegisterSpec for HP_MODEM_ICG_MODEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_modem_icg_modem::R](R) reader structure"]
impl crate::Readable for HP_MODEM_ICG_MODEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_modem_icg_modem::W](W) writer structure"]
impl crate::Writable for HP_MODEM_ICG_MODEM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_ICG_MODEM to value 0"]
impl crate::Resettable for HP_MODEM_ICG_MODEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
