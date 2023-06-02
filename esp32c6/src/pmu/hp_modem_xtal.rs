#[doc = "Register `HP_MODEM_XTAL` reader"]
pub struct R(crate::R<HP_MODEM_XTAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_MODEM_XTAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_MODEM_XTAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_MODEM_XTAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_MODEM_XTAL` writer"]
pub struct W(crate::W<HP_MODEM_XTAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_MODEM_XTAL_SPEC>;
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
impl From<crate::W<HP_MODEM_XTAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_MODEM_XTAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_MODEM_XPD_XTAL` reader - need_des"]
pub type HP_MODEM_XPD_XTAL_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_XTAL` writer - need_des"]
pub type HP_MODEM_XPD_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, HP_MODEM_XTAL_SPEC, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_xtal(&self) -> HP_MODEM_XPD_XTAL_R {
        HP_MODEM_XPD_XTAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_XTAL")
            .field(
                "hp_modem_xpd_xtal",
                &format_args!("{}", self.hp_modem_xpd_xtal().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_XTAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_xpd_xtal(&mut self) -> HP_MODEM_XPD_XTAL_W<31> {
        HP_MODEM_XPD_XTAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_modem_xtal](index.html) module"]
pub struct HP_MODEM_XTAL_SPEC;
impl crate::RegisterSpec for HP_MODEM_XTAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_modem_xtal::R](R) reader structure"]
impl crate::Readable for HP_MODEM_XTAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_modem_xtal::W](W) writer structure"]
impl crate::Writable for HP_MODEM_XTAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_XTAL to value 0x8000_0000"]
impl crate::Resettable for HP_MODEM_XTAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
