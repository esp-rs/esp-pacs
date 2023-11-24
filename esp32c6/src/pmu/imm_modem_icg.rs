#[doc = "Register `IMM_MODEM_ICG` writer"]
pub type W = crate::W<IMM_MODEM_ICG_SPEC>;
#[doc = "Field `UPDATE_DIG_ICG_MODEM_EN` writer - need_des"]
pub type UPDATE_DIG_ICG_MODEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn update_dig_icg_modem_en(&mut self) -> UPDATE_DIG_ICG_MODEM_EN_W<IMM_MODEM_ICG_SPEC> {
        UPDATE_DIG_ICG_MODEM_EN_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_modem_icg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMM_MODEM_ICG_SPEC;
impl crate::RegisterSpec for IMM_MODEM_ICG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`imm_modem_icg::W`](W) writer structure"]
impl crate::Writable for IMM_MODEM_ICG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMM_MODEM_ICG to value 0"]
impl crate::Resettable for IMM_MODEM_ICG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
