///Register `AHB_MPU_TABLE_1` reader
pub type R = crate::R<AHB_MPU_TABLE_1_SPEC>;
///Register `AHB_MPU_TABLE_1` writer
pub type W = crate::W<AHB_MPU_TABLE_1_SPEC>;
///Field `AHB_ACCESS_GRANT_1` reader -
pub type AHB_ACCESS_GRANT_1_R = crate::FieldReader<u16>;
///Field `AHB_ACCESS_GRANT_1` writer -
pub type AHB_ACCESS_GRANT_1_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8
    #[inline(always)]
    pub fn ahb_access_grant_1(&self) -> AHB_ACCESS_GRANT_1_R {
        AHB_ACCESS_GRANT_1_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_MPU_TABLE_1")
            .field("ahb_access_grant_1", &self.ahb_access_grant_1())
            .finish()
    }
}
impl W {
    ///Bits 0:8
    #[inline(always)]
    #[must_use]
    pub fn ahb_access_grant_1(&mut self) -> AHB_ACCESS_GRANT_1_W<AHB_MPU_TABLE_1_SPEC> {
        AHB_ACCESS_GRANT_1_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ahb_mpu_table_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_mpu_table_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AHB_MPU_TABLE_1_SPEC;
impl crate::RegisterSpec for AHB_MPU_TABLE_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ahb_mpu_table_1::R`](R) reader structure
impl crate::Readable for AHB_MPU_TABLE_1_SPEC {}
///`write(|w| ..)` method takes [`ahb_mpu_table_1::W`](W) writer structure
impl crate::Writable for AHB_MPU_TABLE_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB_MPU_TABLE_1 to value 0x01ff
impl crate::Resettable for AHB_MPU_TABLE_1_SPEC {
    const RESET_VALUE: u32 = 0x01ff;
}
