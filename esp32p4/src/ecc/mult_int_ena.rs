#[doc = "Register `MULT_INT_ENA` reader"]
pub type R = crate::R<MULT_INT_ENA_SPEC>;
#[doc = "Register `MULT_INT_ENA` writer"]
pub type W = crate::W<MULT_INT_ENA_SPEC>;
#[doc = "Field `MULT_CALC_DONE` reader - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
pub type MULT_CALC_DONE_R = crate::BitReader;
#[doc = "Field `MULT_CALC_DONE` writer - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
pub type MULT_CALC_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
    #[inline(always)]
    pub fn mult_calc_done(&self) -> MULT_CALC_DONE_R {
        MULT_CALC_DONE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_INT_ENA")
            .field("mult_calc_done", &self.mult_calc_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
    #[inline(always)]
    pub fn mult_calc_done(&mut self) -> MULT_CALC_DONE_W<'_, MULT_INT_ENA_SPEC> {
        MULT_CALC_DONE_W::new(self, 0)
    }
}
#[doc = "ECC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_INT_ENA_SPEC;
impl crate::RegisterSpec for MULT_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_int_ena::R`](R) reader structure"]
impl crate::Readable for MULT_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mult_int_ena::W`](W) writer structure"]
impl crate::Writable for MULT_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_INT_ENA to value 0"]
impl crate::Resettable for MULT_INT_ENA_SPEC {}
