#[doc = "Register `HP_MODEM_XTAL` reader"]
pub type R = crate::R<HP_MODEM_XTAL_SPEC>;
#[doc = "Register `HP_MODEM_XTAL` writer"]
pub type W = crate::W<HP_MODEM_XTAL_SPEC>;
#[doc = "Field `HP_MODEM_XPD_XTAL` reader - need_des"]
pub type HP_MODEM_XPD_XTAL_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_XTAL` writer - need_des"]
pub type HP_MODEM_XPD_XTAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn hp_modem_xpd_xtal(&mut self) -> HP_MODEM_XPD_XTAL_W<HP_MODEM_XTAL_SPEC, 31> {
        HP_MODEM_XPD_XTAL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_modem_xtal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_xtal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_XTAL_SPEC;
impl crate::RegisterSpec for HP_MODEM_XTAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_modem_xtal::R`](R) reader structure"]
impl crate::Readable for HP_MODEM_XTAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_modem_xtal::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_XTAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_XTAL to value 0x8000_0000"]
impl crate::Resettable for HP_MODEM_XTAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
