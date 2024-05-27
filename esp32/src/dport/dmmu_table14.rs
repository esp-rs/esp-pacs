///Register `DMMU_TABLE14` reader
pub type R = crate::R<DMMU_TABLE14_SPEC>;
///Register `DMMU_TABLE14` writer
pub type W = crate::W<DMMU_TABLE14_SPEC>;
///Field `DMMU_TABLE14` reader -
pub type DMMU_TABLE14_R = crate::FieldReader;
///Field `DMMU_TABLE14` writer -
pub type DMMU_TABLE14_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6
    #[inline(always)]
    pub fn dmmu_table14(&self) -> DMMU_TABLE14_R {
        DMMU_TABLE14_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE14")
            .field("dmmu_table14", &self.dmmu_table14())
            .finish()
    }
}
impl W {
    ///Bits 0:6
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table14(&mut self) -> DMMU_TABLE14_W<DMMU_TABLE14_SPEC> {
        DMMU_TABLE14_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMMU_TABLE14_SPEC;
impl crate::RegisterSpec for DMMU_TABLE14_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dmmu_table14::R`](R) reader structure
impl crate::Readable for DMMU_TABLE14_SPEC {}
///`write(|w| ..)` method takes [`dmmu_table14::W`](W) writer structure
impl crate::Writable for DMMU_TABLE14_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMMU_TABLE14 to value 0x0e
impl crate::Resettable for DMMU_TABLE14_SPEC {
    const RESET_VALUE: u32 = 0x0e;
}
