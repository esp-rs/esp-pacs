#[doc = "Register `CORE_0_INTR_ENA` reader"]
pub type R = crate::R<CORE_0_INTR_ENA_SPEC>;
#[doc = "Register `CORE_0_INTR_ENA` writer"]
pub type W = crate::W<CORE_0_INTR_ENA_SPEC>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_INTR_ENA` reader - enbale sp underlow monitor interrupt"]
pub type CORE_0_SP_SPILL_MIN_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MIN_INTR_ENA` writer - enbale sp underlow monitor interrupt"]
pub type CORE_0_SP_SPILL_MIN_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_INTR_ENA` reader - enbale sp overflow monitor interrupt"]
pub type CORE_0_SP_SPILL_MAX_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MAX_INTR_ENA` writer - enbale sp overflow monitor interrupt"]
pub type CORE_0_SP_SPILL_MAX_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enbale sp underlow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_intr_ena(&self) -> CORE_0_SP_SPILL_MIN_INTR_ENA_R {
        CORE_0_SP_SPILL_MIN_INTR_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_intr_ena(&self) -> CORE_0_SP_SPILL_MAX_INTR_ENA_R {
        CORE_0_SP_SPILL_MAX_INTR_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_ENA")
            .field(
                "core_0_sp_spill_min_intr_ena",
                &self.core_0_sp_spill_min_intr_ena(),
            )
            .field(
                "core_0_sp_spill_max_intr_ena",
                &self.core_0_sp_spill_max_intr_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enbale sp underlow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_intr_ena(
        &mut self,
    ) -> CORE_0_SP_SPILL_MIN_INTR_ENA_W<CORE_0_INTR_ENA_SPEC> {
        CORE_0_SP_SPILL_MIN_INTR_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_intr_ena(
        &mut self,
    ) -> CORE_0_SP_SPILL_MAX_INTR_ENA_W<CORE_0_INTR_ENA_SPEC> {
        CORE_0_SP_SPILL_MAX_INTR_ENA_W::new(self, 1)
    }
}
#[doc = "core0 monitor interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_ENA_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_ena::R`](R) reader structure"]
impl crate::Readable for CORE_0_INTR_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_intr_ena::W`](W) writer structure"]
impl crate::Writable for CORE_0_INTR_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_INTR_ENA to value 0"]
impl crate::Resettable for CORE_0_INTR_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
