#[doc = "Register `IMM_MODEM_ICG` writer"]
pub struct W(crate::W<IMM_MODEM_ICG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMM_MODEM_ICG_SPEC>;
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
impl From<crate::W<IMM_MODEM_ICG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMM_MODEM_ICG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDATE_DIG_ICG_MODEM_EN` writer - need_des"]
pub type UPDATE_DIG_ICG_MODEM_EN_W<'a, const O: u8> = crate::BitWriter<'a, IMM_MODEM_ICG_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMM_MODEM_ICG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn update_dig_icg_modem_en(&mut self) -> UPDATE_DIG_ICG_MODEM_EN_W<31> {
        UPDATE_DIG_ICG_MODEM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imm_modem_icg](index.html) module"]
pub struct IMM_MODEM_ICG_SPEC;
impl crate::RegisterSpec for IMM_MODEM_ICG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [imm_modem_icg::W](W) writer structure"]
impl crate::Writable for IMM_MODEM_ICG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMM_MODEM_ICG to value 0"]
impl crate::Resettable for IMM_MODEM_ICG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
