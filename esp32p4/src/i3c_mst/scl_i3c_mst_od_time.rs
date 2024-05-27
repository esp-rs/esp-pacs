///Register `SCL_I3C_MST_OD_TIME` reader
pub type R = crate::R<SCL_I3C_MST_OD_TIME_SPEC>;
///Register `SCL_I3C_MST_OD_TIME` writer
pub type W = crate::W<SCL_I3C_MST_OD_TIME_SPEC>;
///Field `REG_I3C_MST_OD_LOW_PERIOD` reader - SCL Open-Drain low count for I3C transfers targeted to I3C devices.
pub type REG_I3C_MST_OD_LOW_PERIOD_R = crate::FieldReader<u16>;
///Field `REG_I3C_MST_OD_LOW_PERIOD` writer - SCL Open-Drain low count for I3C transfers targeted to I3C devices.
pub type REG_I3C_MST_OD_LOW_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `REG_I3C_MST_OD_HIGH_PERIOD` reader - SCL Open-Drain High count for I3C transfers targeted to I3C devices.
pub type REG_I3C_MST_OD_HIGH_PERIOD_R = crate::FieldReader<u16>;
///Field `REG_I3C_MST_OD_HIGH_PERIOD` writer - SCL Open-Drain High count for I3C transfers targeted to I3C devices.
pub type REG_I3C_MST_OD_HIGH_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - SCL Open-Drain low count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub fn reg_i3c_mst_od_low_period(&self) -> REG_I3C_MST_OD_LOW_PERIOD_R {
        REG_I3C_MST_OD_LOW_PERIOD_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - SCL Open-Drain High count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub fn reg_i3c_mst_od_high_period(&self) -> REG_I3C_MST_OD_HIGH_PERIOD_R {
        REG_I3C_MST_OD_HIGH_PERIOD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_I3C_MST_OD_TIME")
            .field(
                "reg_i3c_mst_od_low_period",
                &self.reg_i3c_mst_od_low_period(),
            )
            .field(
                "reg_i3c_mst_od_high_period",
                &self.reg_i3c_mst_od_high_period(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:15 - SCL Open-Drain low count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    #[must_use]
    pub fn reg_i3c_mst_od_low_period(
        &mut self,
    ) -> REG_I3C_MST_OD_LOW_PERIOD_W<SCL_I3C_MST_OD_TIME_SPEC> {
        REG_I3C_MST_OD_LOW_PERIOD_W::new(self, 0)
    }
    ///Bits 16:31 - SCL Open-Drain High count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    #[must_use]
    pub fn reg_i3c_mst_od_high_period(
        &mut self,
    ) -> REG_I3C_MST_OD_HIGH_PERIOD_W<SCL_I3C_MST_OD_TIME_SPEC> {
        REG_I3C_MST_OD_HIGH_PERIOD_W::new(self, 16)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`scl_i3c_mst_od_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_i3c_mst_od_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCL_I3C_MST_OD_TIME_SPEC;
impl crate::RegisterSpec for SCL_I3C_MST_OD_TIME_SPEC {
    type Ux = u32;
}
///`read()` method returns [`scl_i3c_mst_od_time::R`](R) reader structure
impl crate::Readable for SCL_I3C_MST_OD_TIME_SPEC {}
///`write(|w| ..)` method takes [`scl_i3c_mst_od_time::W`](W) writer structure
impl crate::Writable for SCL_I3C_MST_OD_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCL_I3C_MST_OD_TIME to value 0x0005_0019
impl crate::Resettable for SCL_I3C_MST_OD_TIME_SPEC {
    const RESET_VALUE: u32 = 0x0005_0019;
}
