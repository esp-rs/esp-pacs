///Register `SCL_START_HOLD` reader
pub type R = crate::R<SCL_START_HOLD_SPEC>;
///Register `SCL_START_HOLD` writer
pub type W = crate::W<SCL_START_HOLD_SPEC>;
///Field `REG_SCL_START_HOLD_TIME` reader - I2C_SCL_START_HOLD_TIME
pub type REG_SCL_START_HOLD_TIME_R = crate::FieldReader<u16>;
///Field `REG_SCL_START_HOLD_TIME` writer - I2C_SCL_START_HOLD_TIME
pub type REG_SCL_START_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `REG_START_DET_HOLD_TIME` reader - NA
pub type REG_START_DET_HOLD_TIME_R = crate::FieldReader;
///Field `REG_START_DET_HOLD_TIME` writer - NA
pub type REG_START_DET_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:8 - I2C_SCL_START_HOLD_TIME
    #[inline(always)]
    pub fn reg_scl_start_hold_time(&self) -> REG_SCL_START_HOLD_TIME_R {
        REG_SCL_START_HOLD_TIME_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:10 - NA
    #[inline(always)]
    pub fn reg_start_det_hold_time(&self) -> REG_START_DET_HOLD_TIME_R {
        REG_START_DET_HOLD_TIME_R::new(((self.bits >> 9) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_START_HOLD")
            .field("reg_scl_start_hold_time", &self.reg_scl_start_hold_time())
            .field("reg_start_det_hold_time", &self.reg_start_det_hold_time())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - I2C_SCL_START_HOLD_TIME
    #[inline(always)]
    #[must_use]
    pub fn reg_scl_start_hold_time(&mut self) -> REG_SCL_START_HOLD_TIME_W<SCL_START_HOLD_SPEC> {
        REG_SCL_START_HOLD_TIME_W::new(self, 0)
    }
    ///Bits 9:10 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_start_det_hold_time(&mut self) -> REG_START_DET_HOLD_TIME_W<SCL_START_HOLD_SPEC> {
        REG_START_DET_HOLD_TIME_W::new(self, 9)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`scl_start_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_start_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCL_START_HOLD_SPEC;
impl crate::RegisterSpec for SCL_START_HOLD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`scl_start_hold::R`](R) reader structure
impl crate::Readable for SCL_START_HOLD_SPEC {}
///`write(|w| ..)` method takes [`scl_start_hold::W`](W) writer structure
impl crate::Writable for SCL_START_HOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCL_START_HOLD to value 0x08
impl crate::Resettable for SCL_START_HOLD_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
