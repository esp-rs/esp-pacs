///Register `CORE_DUMMY_DATA` reader
pub type R = crate::R<CORE_DUMMY_DATA_SPEC>;
///Register `CORE_DUMMY_DATA` writer
pub type W = crate::W<CORE_DUMMY_DATA_SPEC>;
///Field `CORE_DUMMY_DATA` reader -
pub type CORE_DUMMY_DATA_R = crate::FieldReader<u32>;
///Field `CORE_DUMMY_DATA` writer -
pub type CORE_DUMMY_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn core_dummy_data(&self) -> CORE_DUMMY_DATA_R {
        CORE_DUMMY_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_DUMMY_DATA")
            .field("core_dummy_data", &self.core_dummy_data())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn core_dummy_data(&mut self) -> CORE_DUMMY_DATA_W<CORE_DUMMY_DATA_SPEC> {
        CORE_DUMMY_DATA_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`core_dummy_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_dummy_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_DUMMY_DATA_SPEC;
impl crate::RegisterSpec for CORE_DUMMY_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_dummy_data::R`](R) reader structure
impl crate::Readable for CORE_DUMMY_DATA_SPEC {}
///`write(|w| ..)` method takes [`core_dummy_data::W`](W) writer structure
impl crate::Writable for CORE_DUMMY_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_DUMMY_DATA to value 0
impl crate::Resettable for CORE_DUMMY_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
