///Register `IDPARTNO` reader
pub type R = crate::R<IDPARTNO_SPEC>;
///Register `IDPARTNO` writer
pub type W = crate::W<IDPARTNO_SPEC>;
///Field `PARTNO` reader - NA
pub type PARTNO_R = crate::FieldReader<u32>;
///Field `PARTNO` writer - NA
pub type PARTNO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - NA
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDPARTNO")
            .field("partno", &self.partno())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - NA
    #[inline(always)]
    #[must_use]
    pub fn partno(&mut self) -> PARTNO_W<IDPARTNO_SPEC> {
        PARTNO_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`idpartno::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idpartno::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IDPARTNO_SPEC;
impl crate::RegisterSpec for IDPARTNO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`idpartno::R`](R) reader structure
impl crate::Readable for IDPARTNO_SPEC {}
///`write(|w| ..)` method takes [`idpartno::W`](W) writer structure
impl crate::Writable for IDPARTNO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IDPARTNO to value 0
impl crate::Resettable for IDPARTNO_SPEC {
    const RESET_VALUE: u32 = 0;
}
