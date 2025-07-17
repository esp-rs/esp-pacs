#[doc = "Register `MONTR_ENA` reader"]
pub type R = crate::R<MONTR_ENA_SPEC>;
#[doc = "Register `MONTR_ENA` writer"]
pub type W = crate::W<MONTR_ENA_SPEC>;
#[doc = "Field `SP_SPILL_MIN_ENA` reader - enbale sp underlow monitor"]
pub type SP_SPILL_MIN_ENA_R = crate::BitReader;
#[doc = "Field `SP_SPILL_MIN_ENA` writer - enbale sp underlow monitor"]
pub type SP_SPILL_MIN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP_SPILL_MAX_ENA` reader - enbale sp overflow monitor"]
pub type SP_SPILL_MAX_ENA_R = crate::BitReader;
#[doc = "Field `SP_SPILL_MAX_ENA` writer - enbale sp overflow monitor"]
pub type SP_SPILL_MAX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enbale sp underlow monitor"]
    #[inline(always)]
    pub fn sp_spill_min_ena(&self) -> SP_SPILL_MIN_ENA_R {
        SP_SPILL_MIN_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor"]
    #[inline(always)]
    pub fn sp_spill_max_ena(&self) -> SP_SPILL_MAX_ENA_R {
        SP_SPILL_MAX_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MONTR_ENA")
            .field("sp_spill_min_ena", &self.sp_spill_min_ena())
            .field("sp_spill_max_ena", &self.sp_spill_max_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enbale sp underlow monitor"]
    #[inline(always)]
    pub fn sp_spill_min_ena(&mut self) -> SP_SPILL_MIN_ENA_W<MONTR_ENA_SPEC> {
        SP_SPILL_MIN_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor"]
    #[inline(always)]
    pub fn sp_spill_max_ena(&mut self) -> SP_SPILL_MAX_ENA_W<MONTR_ENA_SPEC> {
        SP_SPILL_MAX_ENA_W::new(self, 1)
    }
}
#[doc = "core0 monitor enable configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`montr_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`montr_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MONTR_ENA_SPEC;
impl crate::RegisterSpec for MONTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`montr_ena::R`](R) reader structure"]
impl crate::Readable for MONTR_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`montr_ena::W`](W) writer structure"]
impl crate::Writable for MONTR_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MONTR_ENA to value 0"]
impl crate::Resettable for MONTR_ENA_SPEC {}
