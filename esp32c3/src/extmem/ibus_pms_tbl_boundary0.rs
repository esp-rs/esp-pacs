///Register `IBUS_PMS_TBL_BOUNDARY0` reader
pub type R = crate::R<IBUS_PMS_TBL_BOUNDARY0_SPEC>;
///Register `IBUS_PMS_TBL_BOUNDARY0` writer
pub type W = crate::W<IBUS_PMS_TBL_BOUNDARY0_SPEC>;
///Field `IBUS_PMS_BOUNDARY0` reader - The bit is used to configure the ibus permission control section boundary0
pub type IBUS_PMS_BOUNDARY0_R = crate::FieldReader<u16>;
///Field `IBUS_PMS_BOUNDARY0` writer - The bit is used to configure the ibus permission control section boundary0
pub type IBUS_PMS_BOUNDARY0_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - The bit is used to configure the ibus permission control section boundary0
    #[inline(always)]
    pub fn ibus_pms_boundary0(&self) -> IBUS_PMS_BOUNDARY0_R {
        IBUS_PMS_BOUNDARY0_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS_PMS_TBL_BOUNDARY0")
            .field("ibus_pms_boundary0", &self.ibus_pms_boundary0())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - The bit is used to configure the ibus permission control section boundary0
    #[inline(always)]
    #[must_use]
    pub fn ibus_pms_boundary0(&mut self) -> IBUS_PMS_BOUNDARY0_W<IBUS_PMS_TBL_BOUNDARY0_SPEC> {
        IBUS_PMS_BOUNDARY0_W::new(self, 0)
    }
}
/**This description will be updated in the near future.

You can [`read`](crate::generic::Reg::read) this register and get [`ibus_pms_tbl_boundary0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibus_pms_tbl_boundary0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IBUS_PMS_TBL_BOUNDARY0_SPEC;
impl crate::RegisterSpec for IBUS_PMS_TBL_BOUNDARY0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ibus_pms_tbl_boundary0::R`](R) reader structure
impl crate::Readable for IBUS_PMS_TBL_BOUNDARY0_SPEC {}
///`write(|w| ..)` method takes [`ibus_pms_tbl_boundary0::W`](W) writer structure
impl crate::Writable for IBUS_PMS_TBL_BOUNDARY0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IBUS_PMS_TBL_BOUNDARY0 to value 0
impl crate::Resettable for IBUS_PMS_TBL_BOUNDARY0_SPEC {
    const RESET_VALUE: u32 = 0;
}
