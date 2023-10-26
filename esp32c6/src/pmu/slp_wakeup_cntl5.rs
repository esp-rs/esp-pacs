#[doc = "Register `SLP_WAKEUP_CNTL5` reader"]
pub type R = crate::R<SLP_WAKEUP_CNTL5_SPEC>;
#[doc = "Register `SLP_WAKEUP_CNTL5` writer"]
pub type W = crate::W<SLP_WAKEUP_CNTL5_SPEC>;
#[doc = "Field `MODEM_WAIT_TARGET` reader - need_des"]
pub type MODEM_WAIT_TARGET_R = crate::FieldReader<u32>;
#[doc = "Field `MODEM_WAIT_TARGET` writer - need_des"]
pub type MODEM_WAIT_TARGET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
#[doc = "Field `LP_ANA_WAIT_TARGET` reader - need_des"]
pub type LP_ANA_WAIT_TARGET_R = crate::FieldReader;
#[doc = "Field `LP_ANA_WAIT_TARGET` writer - need_des"]
pub type LP_ANA_WAIT_TARGET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:19 - need_des"]
    #[inline(always)]
    pub fn modem_wait_target(&self) -> MODEM_WAIT_TARGET_R {
        MODEM_WAIT_TARGET_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_wait_target(&self) -> LP_ANA_WAIT_TARGET_R {
        LP_ANA_WAIT_TARGET_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL5")
            .field(
                "modem_wait_target",
                &format_args!("{}", self.modem_wait_target().bits()),
            )
            .field(
                "lp_ana_wait_target",
                &format_args!("{}", self.lp_ana_wait_target().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CNTL5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_wait_target(&mut self) -> MODEM_WAIT_TARGET_W<SLP_WAKEUP_CNTL5_SPEC, 0> {
        MODEM_WAIT_TARGET_W::new(self)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_wait_target(&mut self) -> LP_ANA_WAIT_TARGET_W<SLP_WAKEUP_CNTL5_SPEC, 24> {
        LP_ANA_WAIT_TARGET_W::new(self)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_CNTL5_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cntl5::R`](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl5::W`](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL5 to value 0x0100_0080"]
impl crate::Resettable for SLP_WAKEUP_CNTL5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0080;
}
