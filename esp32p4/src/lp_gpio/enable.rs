///Register `ENABLE` reader
pub type R = crate::R<ENABLE_SPEC>;
///Register `ENABLE` writer
pub type W = crate::W<ENABLE_SPEC>;
///Field `REG_GPIO_ENABLE_DATA` reader - Reserved
pub type REG_GPIO_ENABLE_DATA_R = crate::FieldReader<u16>;
///Field `REG_GPIO_ENABLE_DATA` writer - Reserved
pub type REG_GPIO_ENABLE_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Reserved
    #[inline(always)]
    pub fn reg_gpio_enable_data(&self) -> REG_GPIO_ENABLE_DATA_R {
        REG_GPIO_ENABLE_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE")
            .field("reg_gpio_enable_data", &self.reg_gpio_enable_data())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_enable_data(&mut self) -> REG_GPIO_ENABLE_DATA_W<ENABLE_SPEC> {
        REG_GPIO_ENABLE_DATA_W::new(self, 0)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`enable::R`](R) reader structure
impl crate::Readable for ENABLE_SPEC {}
///`write(|w| ..)` method takes [`enable::W`](W) writer structure
impl crate::Writable for ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ENABLE to value 0
impl crate::Resettable for ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
