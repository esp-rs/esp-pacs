#[doc = "Register `GPIO_HOLD0` reader"]
pub type R = crate::R<GPIO_HOLD0_SPEC>;
#[doc = "Register `GPIO_HOLD0` writer"]
pub type W = crate::W<GPIO_HOLD0_SPEC>;
#[doc = "Field `GPIO_HOLD0` reader - need_des"]
pub type GPIO_HOLD0_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_HOLD0` writer - need_des"]
pub type GPIO_HOLD0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn gpio_hold0(&self) -> GPIO_HOLD0_R {
        GPIO_HOLD0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HOLD0")
            .field("gpio_hold0", &format_args!("{}", self.gpio_hold0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_HOLD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hold0(&mut self) -> GPIO_HOLD0_W<GPIO_HOLD0_SPEC, 0> {
        GPIO_HOLD0_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_hold0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hold0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_HOLD0_SPEC;
impl crate::RegisterSpec for GPIO_HOLD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_hold0::R`](R) reader structure"]
impl crate::Readable for GPIO_HOLD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_hold0::W`](W) writer structure"]
impl crate::Writable for GPIO_HOLD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_HOLD0 to value 0"]
impl crate::Resettable for GPIO_HOLD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
