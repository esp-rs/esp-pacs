///Register `IMMU_TABLE11` reader
pub type R = crate::R<IMMU_TABLE11_SPEC>;
///Register `IMMU_TABLE11` writer
pub type W = crate::W<IMMU_TABLE11_SPEC>;
///Field `IMMU_TABLE11` reader -
pub type IMMU_TABLE11_R = crate::FieldReader;
///Field `IMMU_TABLE11` writer -
pub type IMMU_TABLE11_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6
    #[inline(always)]
    pub fn immu_table11(&self) -> IMMU_TABLE11_R {
        IMMU_TABLE11_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_TABLE11")
            .field("immu_table11", &self.immu_table11())
            .finish()
    }
}
impl W {
    ///Bits 0:6
    #[inline(always)]
    #[must_use]
    pub fn immu_table11(&mut self) -> IMMU_TABLE11_W<IMMU_TABLE11_SPEC> {
        IMMU_TABLE11_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`immu_table11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IMMU_TABLE11_SPEC;
impl crate::RegisterSpec for IMMU_TABLE11_SPEC {
    type Ux = u32;
}
///`read()` method returns [`immu_table11::R`](R) reader structure
impl crate::Readable for IMMU_TABLE11_SPEC {}
///`write(|w| ..)` method takes [`immu_table11::W`](W) writer structure
impl crate::Writable for IMMU_TABLE11_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMMU_TABLE11 to value 0x0b
impl crate::Resettable for IMMU_TABLE11_SPEC {
    const RESET_VALUE: u32 = 0x0b;
}
