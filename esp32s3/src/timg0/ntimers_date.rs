///Register `NTIMERS_DATE` reader
pub type R = crate::R<NTIMERS_DATE_SPEC>;
///Register `NTIMERS_DATE` writer
pub type W = crate::W<NTIMERS_DATE_SPEC>;
///Field `NTIMERS_DATE` reader - Timer version control register
pub type NTIMERS_DATE_R = crate::FieldReader<u32>;
///Field `NTIMERS_DATE` writer - Timer version control register
pub type NTIMERS_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - Timer version control register
    #[inline(always)]
    pub fn ntimers_date(&self) -> NTIMERS_DATE_R {
        NTIMERS_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NTIMERS_DATE")
            .field("ntimers_date", &self.ntimers_date())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - Timer version control register
    #[inline(always)]
    #[must_use]
    pub fn ntimers_date(&mut self) -> NTIMERS_DATE_W<NTIMERS_DATE_SPEC> {
        NTIMERS_DATE_W::new(self, 0)
    }
}
/**Timer version control register

You can [`read`](crate::generic::Reg::read) this register and get [`ntimers_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimers_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NTIMERS_DATE_SPEC;
impl crate::RegisterSpec for NTIMERS_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ntimers_date::R`](R) reader structure
impl crate::Readable for NTIMERS_DATE_SPEC {}
///`write(|w| ..)` method takes [`ntimers_date::W`](W) writer structure
impl crate::Writable for NTIMERS_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets NTIMERS_DATE to value 0x0200_3071
impl crate::Resettable for NTIMERS_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0200_3071;
}
