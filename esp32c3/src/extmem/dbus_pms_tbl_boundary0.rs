///Register `DBUS_PMS_TBL_BOUNDARY0` reader
pub type R = crate::R<DBUS_PMS_TBL_BOUNDARY0_SPEC>;
///Register `DBUS_PMS_TBL_BOUNDARY0` writer
pub type W = crate::W<DBUS_PMS_TBL_BOUNDARY0_SPEC>;
///Field `DBUS_PMS_BOUNDARY0` reader - The bit is used to configure the dbus permission control section boundary0
pub type DBUS_PMS_BOUNDARY0_R = crate::FieldReader<u16>;
///Field `DBUS_PMS_BOUNDARY0` writer - The bit is used to configure the dbus permission control section boundary0
pub type DBUS_PMS_BOUNDARY0_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - The bit is used to configure the dbus permission control section boundary0
    #[inline(always)]
    pub fn dbus_pms_boundary0(&self) -> DBUS_PMS_BOUNDARY0_R {
        DBUS_PMS_BOUNDARY0_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS_PMS_TBL_BOUNDARY0")
            .field("dbus_pms_boundary0", &self.dbus_pms_boundary0())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - The bit is used to configure the dbus permission control section boundary0
    #[inline(always)]
    #[must_use]
    pub fn dbus_pms_boundary0(&mut self) -> DBUS_PMS_BOUNDARY0_W<DBUS_PMS_TBL_BOUNDARY0_SPEC> {
        DBUS_PMS_BOUNDARY0_W::new(self, 0)
    }
}
/**This description will be updated in the near future.

You can [`read`](crate::generic::Reg::read) this register and get [`dbus_pms_tbl_boundary0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbus_pms_tbl_boundary0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DBUS_PMS_TBL_BOUNDARY0_SPEC;
impl crate::RegisterSpec for DBUS_PMS_TBL_BOUNDARY0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dbus_pms_tbl_boundary0::R`](R) reader structure
impl crate::Readable for DBUS_PMS_TBL_BOUNDARY0_SPEC {}
///`write(|w| ..)` method takes [`dbus_pms_tbl_boundary0::W`](W) writer structure
impl crate::Writable for DBUS_PMS_TBL_BOUNDARY0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBUS_PMS_TBL_BOUNDARY0 to value 0
impl crate::Resettable for DBUS_PMS_TBL_BOUNDARY0_SPEC {
    const RESET_VALUE: u32 = 0;
}
