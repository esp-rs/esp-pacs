///Register `LP_PAD_HOLD` reader
pub type R = crate::R<LP_PAD_HOLD_SPEC>;
///Register `LP_PAD_HOLD` writer
pub type W = crate::W<LP_PAD_HOLD_SPEC>;
///Field `REG_LP_GPIO_HOLD` reader - Reserved
pub type REG_LP_GPIO_HOLD_R = crate::FieldReader<u16>;
///Field `REG_LP_GPIO_HOLD` writer - Reserved
pub type REG_LP_GPIO_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Reserved
    #[inline(always)]
    pub fn reg_lp_gpio_hold(&self) -> REG_LP_GPIO_HOLD_R {
        REG_LP_GPIO_HOLD_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PAD_HOLD")
            .field("reg_lp_gpio_hold", &self.reg_lp_gpio_hold())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_lp_gpio_hold(&mut self) -> REG_LP_GPIO_HOLD_W<LP_PAD_HOLD_SPEC> {
        REG_LP_GPIO_HOLD_W::new(self, 0)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`lp_pad_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_pad_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_PAD_HOLD_SPEC;
impl crate::RegisterSpec for LP_PAD_HOLD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_pad_hold::R`](R) reader structure
impl crate::Readable for LP_PAD_HOLD_SPEC {}
///`write(|w| ..)` method takes [`lp_pad_hold::W`](W) writer structure
impl crate::Writable for LP_PAD_HOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_PAD_HOLD to value 0
impl crate::Resettable for LP_PAD_HOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
