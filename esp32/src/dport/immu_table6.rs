///Register `IMMU_TABLE6` reader
pub type R = crate::R<IMMU_TABLE6_SPEC>;
///Register `IMMU_TABLE6` writer
pub type W = crate::W<IMMU_TABLE6_SPEC>;
///Field `IMMU_TABLE6` reader -
pub type IMMU_TABLE6_R = crate::FieldReader;
///Field `IMMU_TABLE6` writer -
pub type IMMU_TABLE6_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6
    #[inline(always)]
    pub fn immu_table6(&self) -> IMMU_TABLE6_R {
        IMMU_TABLE6_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_TABLE6")
            .field("immu_table6", &self.immu_table6())
            .finish()
    }
}
impl W {
    ///Bits 0:6
    #[inline(always)]
    #[must_use]
    pub fn immu_table6(&mut self) -> IMMU_TABLE6_W<IMMU_TABLE6_SPEC> {
        IMMU_TABLE6_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`immu_table6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IMMU_TABLE6_SPEC;
impl crate::RegisterSpec for IMMU_TABLE6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`immu_table6::R`](R) reader structure
impl crate::Readable for IMMU_TABLE6_SPEC {}
///`write(|w| ..)` method takes [`immu_table6::W`](W) writer structure
impl crate::Writable for IMMU_TABLE6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMMU_TABLE6 to value 0x06
impl crate::Resettable for IMMU_TABLE6_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
