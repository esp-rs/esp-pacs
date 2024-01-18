#[doc = "Register `TCM_PARITY_CHECK_CTRL` reader"]
pub type R = crate::R<TCM_PARITY_CHECK_CTRL_SPEC>;
#[doc = "Register `TCM_PARITY_CHECK_CTRL` writer"]
pub type W = crate::W<TCM_PARITY_CHECK_CTRL_SPEC>;
#[doc = "Field `TCM_PARITY_CHECK_EN` reader - Set 1 to turn on tcm parity check"]
pub type TCM_PARITY_CHECK_EN_R = crate::BitReader;
#[doc = "Field `TCM_PARITY_CHECK_EN` writer - Set 1 to turn on tcm parity check"]
pub type TCM_PARITY_CHECK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to turn on tcm parity check"]
    #[inline(always)]
    pub fn tcm_parity_check_en(&self) -> TCM_PARITY_CHECK_EN_R {
        TCM_PARITY_CHECK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_PARITY_CHECK_CTRL")
            .field(
                "tcm_parity_check_en",
                &format_args!("{}", self.tcm_parity_check_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TCM_PARITY_CHECK_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to turn on tcm parity check"]
    #[inline(always)]
    #[must_use]
    pub fn tcm_parity_check_en(&mut self) -> TCM_PARITY_CHECK_EN_W<TCM_PARITY_CHECK_CTRL_SPEC> {
        TCM_PARITY_CHECK_EN_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_parity_check_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_parity_check_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_PARITY_CHECK_CTRL_SPEC;
impl crate::RegisterSpec for TCM_PARITY_CHECK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_parity_check_ctrl::R`](R) reader structure"]
impl crate::Readable for TCM_PARITY_CHECK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcm_parity_check_ctrl::W`](W) writer structure"]
impl crate::Writable for TCM_PARITY_CHECK_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCM_PARITY_CHECK_CTRL to value 0"]
impl crate::Resettable for TCM_PARITY_CHECK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
