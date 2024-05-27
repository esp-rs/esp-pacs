///Register `STORE0` reader
pub type R = crate::R<STORE0_SPEC>;
///Register `STORE0` writer
pub type W = crate::W<STORE0_SPEC>;
///Field `SCRATCH0` reader - reserved register
pub type SCRATCH0_R = crate::FieldReader<u32>;
///Field `SCRATCH0` writer - reserved register
pub type SCRATCH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - reserved register
    #[inline(always)]
    pub fn scratch0(&self) -> SCRATCH0_R {
        SCRATCH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE0")
            .field("scratch0", &self.scratch0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - reserved register
    #[inline(always)]
    #[must_use]
    pub fn scratch0(&mut self) -> SCRATCH0_W<STORE0_SPEC> {
        SCRATCH0_W::new(self, 0)
    }
}
/**rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STORE0_SPEC;
impl crate::RegisterSpec for STORE0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`store0::R`](R) reader structure
impl crate::Readable for STORE0_SPEC {}
///`write(|w| ..)` method takes [`store0::W`](W) writer structure
impl crate::Writable for STORE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STORE0 to value 0
impl crate::Resettable for STORE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
