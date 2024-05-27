///Register `MULT_INT_ENA` reader
pub type R = crate::R<MULT_INT_ENA_SPEC>;
///Register `MULT_INT_ENA` writer
pub type W = crate::W<MULT_INT_ENA_SPEC>;
///Field `CALC_DONE` reader - The interrupt enable bit for the ecc_calc_done_int interrupt
pub type CALC_DONE_R = crate::BitReader;
///Field `CALC_DONE` writer - The interrupt enable bit for the ecc_calc_done_int interrupt
pub type CALC_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The interrupt enable bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    pub fn calc_done(&self) -> CALC_DONE_R {
        CALC_DONE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_INT_ENA")
            .field("calc_done", &self.calc_done())
            .finish()
    }
}
impl W {
    ///Bit 0 - The interrupt enable bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn calc_done(&mut self) -> CALC_DONE_W<MULT_INT_ENA_SPEC> {
        CALC_DONE_W::new(self, 0)
    }
}
/**ECC interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`mult_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MULT_INT_ENA_SPEC;
impl crate::RegisterSpec for MULT_INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mult_int_ena::R`](R) reader structure
impl crate::Readable for MULT_INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`mult_int_ena::W`](W) writer structure
impl crate::Writable for MULT_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MULT_INT_ENA to value 0
impl crate::Resettable for MULT_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
