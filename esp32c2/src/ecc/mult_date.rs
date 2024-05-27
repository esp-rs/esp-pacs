///Register `MULT_DATE` reader
pub type R = crate::R<MULT_DATE_SPEC>;
///Register `MULT_DATE` writer
pub type W = crate::W<MULT_DATE_SPEC>;
///Field `DATE` reader - ECC mult version control register
pub type DATE_R = crate::FieldReader<u32>;
///Field `DATE` writer - ECC mult version control register
pub type DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - ECC mult version control register
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_DATE")
            .field("date", &self.date())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - ECC mult version control register
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<MULT_DATE_SPEC> {
        DATE_W::new(self, 0)
    }
}
/**Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`mult_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MULT_DATE_SPEC;
impl crate::RegisterSpec for MULT_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mult_date::R`](R) reader structure
impl crate::Readable for MULT_DATE_SPEC {}
///`write(|w| ..)` method takes [`mult_date::W`](W) writer structure
impl crate::Writable for MULT_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MULT_DATE to value 0x0201_2230
impl crate::Resettable for MULT_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0201_2230;
}
