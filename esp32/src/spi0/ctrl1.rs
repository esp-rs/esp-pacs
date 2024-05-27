#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CS_HOLD_DELAY_RES` reader - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
pub type CS_HOLD_DELAY_RES_R = crate::FieldReader<u16>;
#[doc = "Field `CS_HOLD_DELAY_RES` writer - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
pub type CS_HOLD_DELAY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CS_HOLD_DELAY` reader - SPI cs signal is delayed by spi clock cycles"]
pub type CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_DELAY` writer - SPI cs signal is delayed by spi clock cycles"]
pub type CS_HOLD_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 16:27 - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
    #[inline(always)]
    pub fn cs_hold_delay_res(&self) -> CS_HOLD_DELAY_RES_R {
        CS_HOLD_DELAY_RES_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - SPI cs signal is delayed by spi clock cycles"]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CS_HOLD_DELAY_R {
        CS_HOLD_DELAY_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("cs_hold_delay_res", &self.cs_hold_delay_res())
            .field("cs_hold_delay", &self.cs_hold_delay())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:27 - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_delay_res(&mut self) -> CS_HOLD_DELAY_RES_W<CTRL1_SPEC> {
        CS_HOLD_DELAY_RES_W::new(self, 16)
    }
    #[doc = "Bits 28:31 - SPI cs signal is delayed by spi clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_delay(&mut self) -> CS_HOLD_DELAY_W<CTRL1_SPEC> {
        CS_HOLD_DELAY_W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0x5fff_0000"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x5fff_0000;
}
