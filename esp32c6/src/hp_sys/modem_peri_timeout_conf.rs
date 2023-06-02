#[doc = "Register `MODEM_PERI_TIMEOUT_CONF` reader"]
pub struct R(crate::R<MODEM_PERI_TIMEOUT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODEM_PERI_TIMEOUT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODEM_PERI_TIMEOUT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODEM_PERI_TIMEOUT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODEM_PERI_TIMEOUT_CONF` writer"]
pub struct W(crate::W<MODEM_PERI_TIMEOUT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODEM_PERI_TIMEOUT_CONF_SPEC>;
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
impl From<crate::W<MODEM_PERI_TIMEOUT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODEM_PERI_TIMEOUT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODEM_PERI_TIMEOUT_THRES` reader - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain."]
pub type MODEM_PERI_TIMEOUT_THRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MODEM_PERI_TIMEOUT_THRES` writer - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain."]
pub type MODEM_PERI_TIMEOUT_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, MODEM_PERI_TIMEOUT_CONF_SPEC, 16, O, u16, u16>;
#[doc = "Field `MODEM_PERI_TIMEOUT_INT_CLEAR` writer - Set this bit as 1 to clear timeout interrupt"]
pub type MODEM_PERI_TIMEOUT_INT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, MODEM_PERI_TIMEOUT_CONF_SPEC, O>;
#[doc = "Field `MODEM_PERI_TIMEOUT_PROTECT_EN` reader - Set this bit as 1 to enable timeout protection for accessing modem registers"]
pub type MODEM_PERI_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `MODEM_PERI_TIMEOUT_PROTECT_EN` writer - Set this bit as 1 to enable timeout protection for accessing modem registers"]
pub type MODEM_PERI_TIMEOUT_PROTECT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, MODEM_PERI_TIMEOUT_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain."]
    #[inline(always)]
    pub fn modem_peri_timeout_thres(&self) -> MODEM_PERI_TIMEOUT_THRES_R {
        MODEM_PERI_TIMEOUT_THRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 17 - Set this bit as 1 to enable timeout protection for accessing modem registers"]
    #[inline(always)]
    pub fn modem_peri_timeout_protect_en(&self) -> MODEM_PERI_TIMEOUT_PROTECT_EN_R {
        MODEM_PERI_TIMEOUT_PROTECT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_PERI_TIMEOUT_CONF")
            .field(
                "modem_peri_timeout_thres",
                &format_args!("{}", self.modem_peri_timeout_thres().bits()),
            )
            .field(
                "modem_peri_timeout_protect_en",
                &format_args!("{}", self.modem_peri_timeout_protect_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEM_PERI_TIMEOUT_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn modem_peri_timeout_thres(&mut self) -> MODEM_PERI_TIMEOUT_THRES_W<0> {
        MODEM_PERI_TIMEOUT_THRES_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit as 1 to clear timeout interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn modem_peri_timeout_int_clear(&mut self) -> MODEM_PERI_TIMEOUT_INT_CLEAR_W<16> {
        MODEM_PERI_TIMEOUT_INT_CLEAR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit as 1 to enable timeout protection for accessing modem registers"]
    #[inline(always)]
    #[must_use]
    pub fn modem_peri_timeout_protect_en(&mut self) -> MODEM_PERI_TIMEOUT_PROTECT_EN_W<17> {
        MODEM_PERI_TIMEOUT_PROTECT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODEM_PERI_TIMEOUT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_peri_timeout_conf](index.html) module"]
pub struct MODEM_PERI_TIMEOUT_CONF_SPEC;
impl crate::RegisterSpec for MODEM_PERI_TIMEOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modem_peri_timeout_conf::R](R) reader structure"]
impl crate::Readable for MODEM_PERI_TIMEOUT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modem_peri_timeout_conf::W](W) writer structure"]
impl crate::Writable for MODEM_PERI_TIMEOUT_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEM_PERI_TIMEOUT_CONF to value 0x0002_ffff"]
impl crate::Resettable for MODEM_PERI_TIMEOUT_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_ffff;
}
