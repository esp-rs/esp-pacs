#[doc = "Register `SRM_FIX_ALPHA` reader"]
pub type R = crate::R<SRM_FIX_ALPHA_SPEC>;
#[doc = "Register `SRM_FIX_ALPHA` writer"]
pub type W = crate::W<SRM_FIX_ALPHA_SPEC>;
#[doc = "Field `SRM_RX_FIX_ALPHA` reader - The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SRM_RX_ALPHA_CONF_EN is enabled."]
pub type SRM_RX_FIX_ALPHA_R = crate::FieldReader;
#[doc = "Field `SRM_RX_FIX_ALPHA` writer - The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SRM_RX_ALPHA_CONF_EN is enabled."]
pub type SRM_RX_FIX_ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRM_RX_ALPHA_MOD` reader - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SRM_FIX_ALPHA. 2: Original alpha multiply with PPA_SRM_FIX_ALPHA/256."]
pub type SRM_RX_ALPHA_MOD_R = crate::FieldReader;
#[doc = "Field `SRM_RX_ALPHA_MOD` writer - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SRM_FIX_ALPHA. 2: Original alpha multiply with PPA_SRM_FIX_ALPHA/256."]
pub type SRM_RX_ALPHA_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRM_RX_ALPHA_INV` reader - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
pub type SRM_RX_ALPHA_INV_R = crate::BitReader;
#[doc = "Field `SRM_RX_ALPHA_INV` writer - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
pub type SRM_RX_ALPHA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SRM_RX_ALPHA_CONF_EN is enabled."]
    #[inline(always)]
    pub fn srm_rx_fix_alpha(&self) -> SRM_RX_FIX_ALPHA_R {
        SRM_RX_FIX_ALPHA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SRM_FIX_ALPHA. 2: Original alpha multiply with PPA_SRM_FIX_ALPHA/256."]
    #[inline(always)]
    pub fn srm_rx_alpha_mod(&self) -> SRM_RX_ALPHA_MOD_R {
        SRM_RX_ALPHA_MOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
    #[inline(always)]
    pub fn srm_rx_alpha_inv(&self) -> SRM_RX_ALPHA_INV_R {
        SRM_RX_ALPHA_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRM_FIX_ALPHA")
            .field("srm_rx_fix_alpha", &self.srm_rx_fix_alpha())
            .field("srm_rx_alpha_mod", &self.srm_rx_alpha_mod())
            .field("srm_rx_alpha_inv", &self.srm_rx_alpha_inv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SRM_RX_ALPHA_CONF_EN is enabled."]
    #[inline(always)]
    pub fn srm_rx_fix_alpha(&mut self) -> SRM_RX_FIX_ALPHA_W<'_, SRM_FIX_ALPHA_SPEC> {
        SRM_RX_FIX_ALPHA_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SRM_FIX_ALPHA. 2: Original alpha multiply with PPA_SRM_FIX_ALPHA/256."]
    #[inline(always)]
    pub fn srm_rx_alpha_mod(&mut self) -> SRM_RX_ALPHA_MOD_W<'_, SRM_FIX_ALPHA_SPEC> {
        SRM_RX_ALPHA_MOD_W::new(self, 8)
    }
    #[doc = "Bit 10 - Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255."]
    #[inline(always)]
    pub fn srm_rx_alpha_inv(&mut self) -> SRM_RX_ALPHA_INV_W<'_, SRM_FIX_ALPHA_SPEC> {
        SRM_RX_ALPHA_INV_W::new(self, 10)
    }
}
#[doc = "Scaling and rotating engine alpha override register\n\nYou can [`read`](crate::Reg::read) this register and get [`srm_fix_alpha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srm_fix_alpha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRM_FIX_ALPHA_SPEC;
impl crate::RegisterSpec for SRM_FIX_ALPHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srm_fix_alpha::R`](R) reader structure"]
impl crate::Readable for SRM_FIX_ALPHA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srm_fix_alpha::W`](W) writer structure"]
impl crate::Writable for SRM_FIX_ALPHA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRM_FIX_ALPHA to value 0x80"]
impl crate::Resettable for SRM_FIX_ALPHA_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
