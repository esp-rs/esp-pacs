#[doc = "Register `SCL_RSTART_SETUP` reader"]
pub type R = crate::R<SCL_RSTART_SETUP_SPEC>;
#[doc = "Register `SCL_RSTART_SETUP` writer"]
pub type W = crate::W<SCL_RSTART_SETUP_SPEC>;
#[doc = "Field `REG_SCL_RSTART_SETUP_TIME` reader - I2C_SCL_RSTART_SETUP_TIME"]
pub type REG_SCL_RSTART_SETUP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `REG_SCL_RSTART_SETUP_TIME` writer - I2C_SCL_RSTART_SETUP_TIME"]
pub type REG_SCL_RSTART_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - I2C_SCL_RSTART_SETUP_TIME"]
    #[inline(always)]
    pub fn reg_scl_rstart_setup_time(&self) -> REG_SCL_RSTART_SETUP_TIME_R {
        REG_SCL_RSTART_SETUP_TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_RSTART_SETUP")
            .field(
                "reg_scl_rstart_setup_time",
                &self.reg_scl_rstart_setup_time(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - I2C_SCL_RSTART_SETUP_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn reg_scl_rstart_setup_time(
        &mut self,
    ) -> REG_SCL_RSTART_SETUP_TIME_W<SCL_RSTART_SETUP_SPEC> {
        REG_SCL_RSTART_SETUP_TIME_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_rstart_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_rstart_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_RSTART_SETUP_SPEC;
impl crate::RegisterSpec for SCL_RSTART_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_rstart_setup::R`](R) reader structure"]
impl crate::Readable for SCL_RSTART_SETUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_rstart_setup::W`](W) writer structure"]
impl crate::Writable for SCL_RSTART_SETUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL_RSTART_SETUP to value 0x08"]
impl crate::Resettable for SCL_RSTART_SETUP_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
