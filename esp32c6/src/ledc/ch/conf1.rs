///Register `CONF1` reader
pub type R = crate::R<CONF1_SPEC>;
///Register `CONF1` writer
pub type W = crate::W<CONF1_SPEC>;
///Field `DUTY_START` reader - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1.
pub type DUTY_START_R = crate::BitReader;
///Field `DUTY_START` writer - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1.
pub type DUTY_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1.
    #[inline(always)]
    pub fn duty_start(&self) -> DUTY_START_R {
        DUTY_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("duty_start", &self.duty_start())
            .finish()
    }
}
impl W {
    ///Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1.
    #[inline(always)]
    #[must_use]
    pub fn duty_start(&mut self) -> DUTY_START_W<CONF1_SPEC> {
        DUTY_START_W::new(self, 31)
    }
}
/**Configuration register 1 for channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf1::R`](R) reader structure
impl crate::Readable for CONF1_SPEC {}
///`write(|w| ..)` method takes [`conf1::W`](W) writer structure
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF1 to value 0
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
