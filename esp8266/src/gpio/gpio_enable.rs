#[doc = "Register `GPIO_ENABLE` reader"]
pub type R = crate::R<GPIO_ENABLE_SPEC>;
#[doc = "Register `GPIO_ENABLE` writer"]
pub type W = crate::W<GPIO_ENABLE_SPEC>;
#[doc = "Field `GPIO_ENABLE_DATA` reader - The output enable register."]
pub type GPIO_ENABLE_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO_ENABLE_DATA` writer - The output enable register."]
pub type GPIO_ENABLE_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `GPIO_SDIO_SEL` reader - SDIO-dis selection register"]
pub type GPIO_SDIO_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_SDIO_SEL` writer - SDIO-dis selection register"]
pub type GPIO_SDIO_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:15 - The output enable register."]
    #[inline(always)]
    pub fn gpio_enable_data(&self) -> GPIO_ENABLE_DATA_R {
        GPIO_ENABLE_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - SDIO-dis selection register"]
    #[inline(always)]
    pub fn gpio_sdio_sel(&self) -> GPIO_SDIO_SEL_R {
        GPIO_SDIO_SEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_ENABLE")
            .field(
                "gpio_sdio_sel",
                &format_args!("{}", self.gpio_sdio_sel().bits()),
            )
            .field(
                "gpio_enable_data",
                &format_args!("{}", self.gpio_enable_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The output enable register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_enable_data(&mut self) -> GPIO_ENABLE_DATA_W<GPIO_ENABLE_SPEC, 0> {
        GPIO_ENABLE_DATA_W::new(self)
    }
    #[doc = "Bits 16:21 - SDIO-dis selection register"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_sdio_sel(&mut self) -> GPIO_SDIO_SEL_W<GPIO_ENABLE_SPEC, 16> {
        GPIO_SDIO_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO_ENABLE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_ENABLE_SPEC;
impl crate::RegisterSpec for GPIO_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_enable::R`](R) reader structure"]
impl crate::Readable for GPIO_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_enable::W`](W) writer structure"]
impl crate::Writable for GPIO_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_ENABLE to value 0"]
impl crate::Resettable for GPIO_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
