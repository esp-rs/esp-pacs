#[doc = "Register `CORE_0_MONTR_ENA` reader"]
pub type R = crate::R<CORE_0_MONTR_ENA_SPEC>;
#[doc = "Register `CORE_0_MONTR_ENA` writer"]
pub type W = crate::W<CORE_0_MONTR_ENA_SPEC>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_ENA` reader - enbale sp underlow monitor"]
pub type CORE_0_SP_SPILL_MIN_ENA_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MIN_ENA` writer - enbale sp underlow monitor"]
pub type CORE_0_SP_SPILL_MIN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_ENA` reader - enbale sp overflow monitor"]
pub type CORE_0_SP_SPILL_MAX_ENA_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MAX_ENA` writer - enbale sp overflow monitor"]
pub type CORE_0_SP_SPILL_MAX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("core_0_sp_spill_min_ena", &self.core_0_sp_spill_min_ena())
            .field("core_0_sp_spill_max_ena", &self.core_0_sp_spill_max_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enbale sp underlow monitor"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_ena(&mut self) -> CORE_0_SP_SPILL_MIN_ENA_W<CORE_0_MONTR_ENA_SPEC> {
        CORE_0_SP_SPILL_MIN_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_ena(&mut self) -> CORE_0_SP_SPILL_MAX_ENA_W<CORE_0_MONTR_ENA_SPEC> {
        CORE_0_SP_SPILL_MAX_ENA_W::new(self, 1)
    }
}
#[doc = "core0 monitor enable configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_montr_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_montr_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_MONTR_ENA_SPEC;
impl crate::RegisterSpec for CORE_0_MONTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_montr_ena::R`](R) reader structure"]
impl crate::Readable for CORE_0_MONTR_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_montr_ena::W`](W) writer structure"]
impl crate::Writable for CORE_0_MONTR_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_MONTR_ENA to value 0"]
impl crate::Resettable for CORE_0_MONTR_ENA_SPEC {}
