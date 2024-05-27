///Register `SCL_RSTART_SETUP` reader
pub type R = crate::R<SCL_RSTART_SETUP_SPEC>;
///Register `SCL_RSTART_SETUP` writer
pub type W = crate::W<SCL_RSTART_SETUP_SPEC>;
///Field `TIME` reader - reg_scl_rstart_setup_time
pub type TIME_R = crate::FieldReader<u16>;
///Field `TIME` writer - reg_scl_rstart_setup_time
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - reg_scl_rstart_setup_time
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_RSTART_SETUP")
            .field("time", &self.time())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - reg_scl_rstart_setup_time
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<SCL_RSTART_SETUP_SPEC> {
        TIME_W::new(self, 0)
    }
}
/**I2C_SCL_RSTART_SETUP_REG

You can [`read`](crate::generic::Reg::read) this register and get [`scl_rstart_setup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_rstart_setup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCL_RSTART_SETUP_SPEC;
impl crate::RegisterSpec for SCL_RSTART_SETUP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`scl_rstart_setup::R`](R) reader structure
impl crate::Readable for SCL_RSTART_SETUP_SPEC {}
///`write(|w| ..)` method takes [`scl_rstart_setup::W`](W) writer structure
impl crate::Writable for SCL_RSTART_SETUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCL_RSTART_SETUP to value 0x08
impl crate::Resettable for SCL_RSTART_SETUP_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
