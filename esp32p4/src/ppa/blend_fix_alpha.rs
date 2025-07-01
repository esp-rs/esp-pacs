#[doc = "Register `BLEND_FIX_ALPHA` reader"]
pub type R = crate::R<BLEND_FIX_ALPHA_SPEC>;
#[doc = "Register `BLEND_FIX_ALPHA` writer"]
pub type W = crate::W<BLEND_FIX_ALPHA_SPEC>;
#[doc = "Field `BLEND0_RX_FIX_ALPHA` reader - The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled."]
pub type BLEND0_RX_FIX_ALPHA_R = crate::FieldReader;
#[doc = "Field `BLEND0_RX_FIX_ALPHA` writer - The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled."]
pub type BLEND0_RX_FIX_ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLEND1_RX_FIX_ALPHA` reader - The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled."]
pub type BLEND1_RX_FIX_ALPHA_R = crate::FieldReader;
#[doc = "Field `BLEND1_RX_FIX_ALPHA` writer - The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled."]
pub type BLEND1_RX_FIX_ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLEND0_RX_ALPHA_MOD` reader - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
pub type BLEND0_RX_ALPHA_MOD_R = crate::FieldReader;
#[doc = "Field `BLEND0_RX_ALPHA_MOD` writer - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
pub type BLEND0_RX_ALPHA_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLEND1_RX_ALPHA_MOD` reader - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
pub type BLEND1_RX_ALPHA_MOD_R = crate::FieldReader;
#[doc = "Field `BLEND1_RX_ALPHA_MOD` writer - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
pub type BLEND1_RX_ALPHA_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLEND0_RX_ALPHA_INV` reader - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
pub type BLEND0_RX_ALPHA_INV_R = crate::BitReader;
#[doc = "Field `BLEND0_RX_ALPHA_INV` writer - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
pub type BLEND0_RX_ALPHA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND1_RX_ALPHA_INV` reader - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
pub type BLEND1_RX_ALPHA_INV_R = crate::BitReader;
#[doc = "Field `BLEND1_RX_ALPHA_INV` writer - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
pub type BLEND1_RX_ALPHA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled."]
    #[inline(always)]
    pub fn blend0_rx_fix_alpha(&self) -> BLEND0_RX_FIX_ALPHA_R {
        BLEND0_RX_FIX_ALPHA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled."]
    #[inline(always)]
    pub fn blend1_rx_fix_alpha(&self) -> BLEND1_RX_FIX_ALPHA_R {
        BLEND1_RX_FIX_ALPHA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
    #[inline(always)]
    pub fn blend0_rx_alpha_mod(&self) -> BLEND0_RX_ALPHA_MOD_R {
        BLEND0_RX_ALPHA_MOD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
    #[inline(always)]
    pub fn blend1_rx_alpha_mod(&self) -> BLEND1_RX_ALPHA_MOD_R {
        BLEND1_RX_ALPHA_MOD_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
    #[inline(always)]
    pub fn blend0_rx_alpha_inv(&self) -> BLEND0_RX_ALPHA_INV_R {
        BLEND0_RX_ALPHA_INV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
    #[inline(always)]
    pub fn blend1_rx_alpha_inv(&self) -> BLEND1_RX_ALPHA_INV_R {
        BLEND1_RX_ALPHA_INV_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND_FIX_ALPHA")
            .field("blend0_rx_fix_alpha", &self.blend0_rx_fix_alpha())
            .field("blend1_rx_fix_alpha", &self.blend1_rx_fix_alpha())
            .field("blend0_rx_alpha_mod", &self.blend0_rx_alpha_mod())
            .field("blend1_rx_alpha_mod", &self.blend1_rx_alpha_mod())
            .field("blend0_rx_alpha_inv", &self.blend0_rx_alpha_inv())
            .field("blend1_rx_alpha_inv", &self.blend1_rx_alpha_inv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled."]
    #[inline(always)]
    pub fn blend0_rx_fix_alpha(&mut self) -> BLEND0_RX_FIX_ALPHA_W<BLEND_FIX_ALPHA_SPEC> {
        BLEND0_RX_FIX_ALPHA_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled."]
    #[inline(always)]
    pub fn blend1_rx_fix_alpha(&mut self) -> BLEND1_RX_FIX_ALPHA_W<BLEND_FIX_ALPHA_SPEC> {
        BLEND1_RX_FIX_ALPHA_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
    #[inline(always)]
    pub fn blend0_rx_alpha_mod(&mut self) -> BLEND0_RX_ALPHA_MOD_W<BLEND_FIX_ALPHA_SPEC> {
        BLEND0_RX_ALPHA_MOD_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256."]
    #[inline(always)]
    pub fn blend1_rx_alpha_mod(&mut self) -> BLEND1_RX_ALPHA_MOD_W<BLEND_FIX_ALPHA_SPEC> {
        BLEND1_RX_ALPHA_MOD_W::new(self, 18)
    }
    #[doc = "Bit 20 - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
    #[inline(always)]
    pub fn blend0_rx_alpha_inv(&mut self) -> BLEND0_RX_ALPHA_INV_W<BLEND_FIX_ALPHA_SPEC> {
        BLEND0_RX_ALPHA_INV_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
    #[inline(always)]
    pub fn blend1_rx_alpha_inv(&mut self) -> BLEND1_RX_ALPHA_INV_W<BLEND_FIX_ALPHA_SPEC> {
        BLEND1_RX_ALPHA_INV_W::new(self, 21)
    }
}
#[doc = "Blending engine alpha override register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_fix_alpha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_fix_alpha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEND_FIX_ALPHA_SPEC;
impl crate::RegisterSpec for BLEND_FIX_ALPHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_fix_alpha::R`](R) reader structure"]
impl crate::Readable for BLEND_FIX_ALPHA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blend_fix_alpha::W`](W) writer structure"]
impl crate::Writable for BLEND_FIX_ALPHA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_FIX_ALPHA to value 0x8080"]
impl crate::Resettable for BLEND_FIX_ALPHA_SPEC {
    const RESET_VALUE: u32 = 0x8080;
}
