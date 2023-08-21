#[doc = "Register `CORE_0_MONTR_ENA` reader"]
pub type R = crate::R<CORE_0_MONTR_ENA_SPEC>;
#[doc = "Register `CORE_0_MONTR_ENA` writer"]
pub type W = crate::W<CORE_0_MONTR_ENA_SPEC>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_ENA` reader - enbale sp underlow monitor"]
pub type CORE_0_SP_SPILL_MIN_ENA_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MIN_ENA` writer - enbale sp underlow monitor"]
pub type CORE_0_SP_SPILL_MIN_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_ENA` reader - enbale sp overflow monitor"]
pub type CORE_0_SP_SPILL_MAX_ENA_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MAX_ENA` writer - enbale sp overflow monitor"]
pub type CORE_0_SP_SPILL_MAX_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - enbale sp underlow monitor"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_ena(&self) -> CORE_0_SP_SPILL_MIN_ENA_R {
        CORE_0_SP_SPILL_MIN_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_ena(&self) -> CORE_0_SP_SPILL_MAX_ENA_R {
        CORE_0_SP_SPILL_MAX_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_MONTR_ENA")
            .field(
                "core_0_sp_spill_min_ena",
                &format_args!("{}", self.core_0_sp_spill_min_ena().bit()),
            )
            .field(
                "core_0_sp_spill_max_ena",
                &format_args!("{}", self.core_0_sp_spill_max_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_MONTR_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - enbale sp underlow monitor"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_min_ena(
        &mut self,
    ) -> CORE_0_SP_SPILL_MIN_ENA_W<CORE_0_MONTR_ENA_SPEC, 0> {
        CORE_0_SP_SPILL_MIN_ENA_W::new(self)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_max_ena(
        &mut self,
    ) -> CORE_0_SP_SPILL_MAX_ENA_W<CORE_0_MONTR_ENA_SPEC, 1> {
        CORE_0_SP_SPILL_MAX_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "core0 monitor enable configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_montr_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_montr_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_MONTR_ENA_SPEC;
impl crate::RegisterSpec for CORE_0_MONTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_montr_ena::R`](R) reader structure"]
impl crate::Readable for CORE_0_MONTR_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_montr_ena::W`](W) writer structure"]
impl crate::Writable for CORE_0_MONTR_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_MONTR_ENA to value 0"]
impl crate::Resettable for CORE_0_MONTR_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
