#[doc = "Register `EXTMEM_REJECT_INT_ENA` reader"]
pub type R = crate::R<EXTMEM_REJECT_INT_ENA_SPEC>;
#[doc = "Register `EXTMEM_REJECT_INT_ENA` writer"]
pub type W = crate::W<EXTMEM_REJECT_INT_ENA_SPEC>;
#[doc = "Field `EXTMEM_REJECT_INT_ENA` reader - The interrupt enable bit for the EXTMEM_REJECT_INT interrupt."]
pub type EXTMEM_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `EXTMEM_REJECT_INT_ENA` writer - The interrupt enable bit for the EXTMEM_REJECT_INT interrupt."]
pub type EXTMEM_REJECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the EXTMEM_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn extmem_reject_int_ena(&self) -> EXTMEM_REJECT_INT_ENA_R {
        EXTMEM_REJECT_INT_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM_REJECT_INT_ENA")
            .field("extmem_reject_int_ena", &self.extmem_reject_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the EXTMEM_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn extmem_reject_int_ena(
        &mut self,
    ) -> EXTMEM_REJECT_INT_ENA_W<'_, EXTMEM_REJECT_INT_ENA_SPEC> {
        EXTMEM_REJECT_INT_ENA_W::new(self, 0)
    }
}
#[doc = "Interrupt enable bits of external RAM permission\n\nYou can [`read`](crate::Reg::read) this register and get [`extmem_reject_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extmem_reject_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTMEM_REJECT_INT_ENA_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extmem_reject_int_ena::R`](R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extmem_reject_int_ena::W`](W) writer structure"]
impl crate::Writable for EXTMEM_REJECT_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_ENA to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_ENA_SPEC {}
