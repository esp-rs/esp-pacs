#[doc = "Register `MODEM_PERI_TIMEOUT_CONF` reader"]
pub type R = crate::R<MODEM_PERI_TIMEOUT_CONF_SPEC>;
#[doc = "Register `MODEM_PERI_TIMEOUT_CONF` writer"]
pub type W = crate::W<MODEM_PERI_TIMEOUT_CONF_SPEC>;
#[doc = "Field `MODEM_PERI_TIMEOUT_THRES` reader - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain."]
pub type MODEM_PERI_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `MODEM_PERI_TIMEOUT_THRES` writer - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain."]
pub type MODEM_PERI_TIMEOUT_THRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `MODEM_PERI_TIMEOUT_INT_CLEAR` writer - Set this bit as 1 to clear timeout interrupt"]
pub type MODEM_PERI_TIMEOUT_INT_CLEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODEM_PERI_TIMEOUT_PROTECT_EN` reader - Set this bit as 1 to enable timeout protection for accessing modem registers"]
pub type MODEM_PERI_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `MODEM_PERI_TIMEOUT_PROTECT_EN` writer - Set this bit as 1 to enable timeout protection for accessing modem registers"]
pub type MODEM_PERI_TIMEOUT_PROTECT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn modem_peri_timeout_thres(
        &mut self,
    ) -> MODEM_PERI_TIMEOUT_THRES_W<MODEM_PERI_TIMEOUT_CONF_SPEC, 0> {
        MODEM_PERI_TIMEOUT_THRES_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit as 1 to clear timeout interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn modem_peri_timeout_int_clear(
        &mut self,
    ) -> MODEM_PERI_TIMEOUT_INT_CLEAR_W<MODEM_PERI_TIMEOUT_CONF_SPEC, 16> {
        MODEM_PERI_TIMEOUT_INT_CLEAR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit as 1 to enable timeout protection for accessing modem registers"]
    #[inline(always)]
    #[must_use]
    pub fn modem_peri_timeout_protect_en(
        &mut self,
    ) -> MODEM_PERI_TIMEOUT_PROTECT_EN_W<MODEM_PERI_TIMEOUT_CONF_SPEC, 17> {
        MODEM_PERI_TIMEOUT_PROTECT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MODEM_PERI_TIMEOUT configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modem_peri_timeout_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_peri_timeout_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_PERI_TIMEOUT_CONF_SPEC;
impl crate::RegisterSpec for MODEM_PERI_TIMEOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_peri_timeout_conf::R`](R) reader structure"]
impl crate::Readable for MODEM_PERI_TIMEOUT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_peri_timeout_conf::W`](W) writer structure"]
impl crate::Writable for MODEM_PERI_TIMEOUT_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEM_PERI_TIMEOUT_CONF to value 0x0002_ffff"]
impl crate::Resettable for MODEM_PERI_TIMEOUT_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_ffff;
}
