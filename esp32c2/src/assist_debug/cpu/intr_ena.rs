#[doc = "Register `INTR_ENA` reader"]
pub type R = crate::R<INTR_ENA_SPEC>;
#[doc = "Register `INTR_ENA` writer"]
pub type W = crate::W<INTR_ENA_SPEC>;
#[doc = "Field `SP_SPILL_MIN_INTR_ENA` reader - enbale sp underlow monitor interrupt"]
pub type SP_SPILL_MIN_INTR_ENA_R = crate::BitReader;
#[doc = "Field `SP_SPILL_MIN_INTR_ENA` writer - enbale sp underlow monitor interrupt"]
pub type SP_SPILL_MIN_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP_SPILL_MAX_INTR_ENA` reader - enbale sp overflow monitor interrupt"]
pub type SP_SPILL_MAX_INTR_ENA_R = crate::BitReader;
#[doc = "Field `SP_SPILL_MAX_INTR_ENA` writer - enbale sp overflow monitor interrupt"]
pub type SP_SPILL_MAX_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enbale sp underlow monitor interrupt"]
    #[inline(always)]
    pub fn sp_spill_min_intr_ena(&self) -> SP_SPILL_MIN_INTR_ENA_R {
        SP_SPILL_MIN_INTR_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor interrupt"]
    #[inline(always)]
    pub fn sp_spill_max_intr_ena(&self) -> SP_SPILL_MAX_INTR_ENA_R {
        SP_SPILL_MAX_INTR_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_ENA")
            .field("sp_spill_min_intr_ena", &self.sp_spill_min_intr_ena())
            .field("sp_spill_max_intr_ena", &self.sp_spill_max_intr_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enbale sp underlow monitor interrupt"]
    #[inline(always)]
    pub fn sp_spill_min_intr_ena(&mut self) -> SP_SPILL_MIN_INTR_ENA_W<INTR_ENA_SPEC> {
        SP_SPILL_MIN_INTR_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor interrupt"]
    #[inline(always)]
    pub fn sp_spill_max_intr_ena(&mut self) -> SP_SPILL_MAX_INTR_ENA_W<INTR_ENA_SPEC> {
        SP_SPILL_MAX_INTR_ENA_W::new(self, 1)
    }
}
#[doc = "core0 monitor interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_ENA_SPEC;
impl crate::RegisterSpec for INTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_ena::R`](R) reader structure"]
impl crate::Readable for INTR_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_ena::W`](W) writer structure"]
impl crate::Writable for INTR_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_ENA to value 0"]
impl crate::Resettable for INTR_ENA_SPEC {}
