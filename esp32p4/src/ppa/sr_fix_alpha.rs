#[doc = "Register `SR_FIX_ALPHA` reader"]
pub type R = crate::R<SR_FIX_ALPHA_SPEC>;
#[doc = "Register `SR_FIX_ALPHA` writer"]
pub type W = crate::W<SR_FIX_ALPHA_SPEC>;
#[doc = "Field `SR_RX_FIX_ALPHA` reader - The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled."]
pub type SR_RX_FIX_ALPHA_R = crate::FieldReader;
#[doc = "Field `SR_RX_FIX_ALPHA` writer - The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled."]
pub type SR_RX_FIX_ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SR_RX_ALPHA_MOD` reader - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
pub type SR_RX_ALPHA_MOD_R = crate::FieldReader;
#[doc = "Field `SR_RX_ALPHA_MOD` writer - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
pub type SR_RX_ALPHA_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SR_RX_ALPHA_INV` reader - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
pub type SR_RX_ALPHA_INV_R = crate::BitReader;
#[doc = "Field `SR_RX_ALPHA_INV` writer - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
pub type SR_RX_ALPHA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled."]
    #[inline(always)]
    pub fn sr_rx_fix_alpha(&self) -> SR_RX_FIX_ALPHA_R {
        SR_RX_FIX_ALPHA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
    #[inline(always)]
    pub fn sr_rx_alpha_mod(&self) -> SR_RX_ALPHA_MOD_R {
        SR_RX_ALPHA_MOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
    #[inline(always)]
    pub fn sr_rx_alpha_inv(&self) -> SR_RX_ALPHA_INV_R {
        SR_RX_ALPHA_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_FIX_ALPHA")
            .field("sr_rx_fix_alpha", &self.sr_rx_fix_alpha())
            .field("sr_rx_alpha_mod", &self.sr_rx_alpha_mod())
            .field("sr_rx_alpha_inv", &self.sr_rx_alpha_inv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sr_rx_fix_alpha(&mut self) -> SR_RX_FIX_ALPHA_W<SR_FIX_ALPHA_SPEC> {
        SR_RX_FIX_ALPHA_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
    #[inline(always)]
    #[must_use]
    pub fn sr_rx_alpha_mod(&mut self) -> SR_RX_ALPHA_MOD_W<SR_FIX_ALPHA_SPEC> {
        SR_RX_ALPHA_MOD_W::new(self, 8)
    }
    #[doc = "Bit 10 - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
    #[inline(always)]
    #[must_use]
    pub fn sr_rx_alpha_inv(&mut self) -> SR_RX_ALPHA_INV_W<SR_FIX_ALPHA_SPEC> {
        SR_RX_ALPHA_INV_W::new(self, 10)
    }
}
#[doc = "Scaling and rotating engine alpha override register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_fix_alpha::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_fix_alpha::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_FIX_ALPHA_SPEC;
impl crate::RegisterSpec for SR_FIX_ALPHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_fix_alpha::R`](R) reader structure"]
impl crate::Readable for SR_FIX_ALPHA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr_fix_alpha::W`](W) writer structure"]
impl crate::Writable for SR_FIX_ALPHA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR_FIX_ALPHA to value 0x80"]
impl crate::Resettable for SR_FIX_ALPHA_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
