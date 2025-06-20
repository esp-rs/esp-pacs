#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `CALC_DONE_INT_ENA` reader - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
pub type CALC_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `CALC_DONE_INT_ENA` writer - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
pub type CALC_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
    #[inline(always)]
    pub fn calc_done_int_ena(&self) -> CALC_DONE_INT_ENA_R {
        CALC_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("calc_done_int_ena", &self.calc_done_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable the ECC_CALC_DONE_INT interrupt."]
    #[inline(always)]
    pub fn calc_done_int_ena(&mut self) -> CALC_DONE_INT_ENA_W<INT_ENA_SPEC> {
        CALC_DONE_INT_ENA_W::new(self, 0)
    }
}
#[doc = "ECC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
