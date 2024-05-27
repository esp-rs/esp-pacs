///Register `CORE_1_SP_UNSTABLE` reader
pub type R = crate::R<CORE_1_SP_UNSTABLE_SPEC>;
///Register `CORE_1_SP_UNSTABLE` writer
pub type W = crate::W<CORE_1_SP_UNSTABLE_SPEC>;
///Field `CORE_1_SP_UNSTABLE` reader - unstable period when window change,during this period no check stackpointer
pub type CORE_1_SP_UNSTABLE_R = crate::FieldReader;
///Field `CORE_1_SP_UNSTABLE` writer - unstable period when window change,during this period no check stackpointer
pub type CORE_1_SP_UNSTABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - unstable period when window change,during this period no check stackpointer
    #[inline(always)]
    pub fn core_1_sp_unstable(&self) -> CORE_1_SP_UNSTABLE_R {
        CORE_1_SP_UNSTABLE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_SP_UNSTABLE")
            .field("core_1_sp_unstable", &self.core_1_sp_unstable())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - unstable period when window change,during this period no check stackpointer
    #[inline(always)]
    #[must_use]
    pub fn core_1_sp_unstable(&mut self) -> CORE_1_SP_UNSTABLE_W<CORE_1_SP_UNSTABLE_SPEC> {
        CORE_1_SP_UNSTABLE_W::new(self, 0)
    }
}
/**Core1 sp unstable configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_sp_unstable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_sp_unstable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_SP_UNSTABLE_SPEC;
impl crate::RegisterSpec for CORE_1_SP_UNSTABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_sp_unstable::R`](R) reader structure
impl crate::Readable for CORE_1_SP_UNSTABLE_SPEC {}
///`write(|w| ..)` method takes [`core_1_sp_unstable::W`](W) writer structure
impl crate::Writable for CORE_1_SP_UNSTABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_1_SP_UNSTABLE to value 0x0b
impl crate::Resettable for CORE_1_SP_UNSTABLE_SPEC {
    const RESET_VALUE: u32 = 0x0b;
}
