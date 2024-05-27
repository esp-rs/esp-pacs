///Register `OUT` reader
pub type R = crate::R<OUT_SPEC>;
///Register `OUT` writer
pub type W = crate::W<OUT_SPEC>;
///Field `REG_GPIO_OUT_DATA` reader - Reserved
pub type REG_GPIO_OUT_DATA_R = crate::FieldReader<u16>;
///Field `REG_GPIO_OUT_DATA` writer - Reserved
pub type REG_GPIO_OUT_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Reserved
    #[inline(always)]
    pub fn reg_gpio_out_data(&self) -> REG_GPIO_OUT_DATA_R {
        REG_GPIO_OUT_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT")
            .field("reg_gpio_out_data", &self.reg_gpio_out_data())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_out_data(&mut self) -> REG_GPIO_OUT_DATA_W<OUT_SPEC> {
        REG_GPIO_OUT_DATA_W::new(self, 0)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out::R`](R) reader structure
impl crate::Readable for OUT_SPEC {}
///`write(|w| ..)` method takes [`out::W`](W) writer structure
impl crate::Writable for OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OUT to value 0
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
