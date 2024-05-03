#[doc = "Register `BLEND_RGB` reader"]
pub type R = crate::R<BLEND_RGB_SPEC>;
#[doc = "Register `BLEND_RGB` writer"]
pub type W = crate::W<BLEND_RGB_SPEC>;
#[doc = "Field `BLEND1_RX_B` reader - blue color for A4/A8 mode."]
pub type BLEND1_RX_B_R = crate::FieldReader;
#[doc = "Field `BLEND1_RX_B` writer - blue color for A4/A8 mode."]
pub type BLEND1_RX_B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLEND1_RX_G` reader - green color for A4/A8 mode."]
pub type BLEND1_RX_G_R = crate::FieldReader;
#[doc = "Field `BLEND1_RX_G` writer - green color for A4/A8 mode."]
pub type BLEND1_RX_G_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLEND1_RX_R` reader - red color for A4/A8 mode."]
pub type BLEND1_RX_R_R = crate::FieldReader;
#[doc = "Field `BLEND1_RX_R` writer - red color for A4/A8 mode."]
pub type BLEND1_RX_R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - blue color for A4/A8 mode."]
    #[inline(always)]
    pub fn blend1_rx_b(&self) -> BLEND1_RX_B_R {
        BLEND1_RX_B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - green color for A4/A8 mode."]
    #[inline(always)]
    pub fn blend1_rx_g(&self) -> BLEND1_RX_G_R {
        BLEND1_RX_G_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - red color for A4/A8 mode."]
    #[inline(always)]
    pub fn blend1_rx_r(&self) -> BLEND1_RX_R_R {
        BLEND1_RX_R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND_RGB")
            .field("blend1_rx_b", &self.blend1_rx_b().bits())
            .field("blend1_rx_g", &self.blend1_rx_g().bits())
            .field("blend1_rx_r", &self.blend1_rx_r().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLEND_RGB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - blue color for A4/A8 mode."]
    #[inline(always)]
    #[must_use]
    pub fn blend1_rx_b(&mut self) -> BLEND1_RX_B_W<BLEND_RGB_SPEC> {
        BLEND1_RX_B_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - green color for A4/A8 mode."]
    #[inline(always)]
    #[must_use]
    pub fn blend1_rx_g(&mut self) -> BLEND1_RX_G_W<BLEND_RGB_SPEC> {
        BLEND1_RX_G_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - red color for A4/A8 mode."]
    #[inline(always)]
    #[must_use]
    pub fn blend1_rx_r(&mut self) -> BLEND1_RX_R_W<BLEND_RGB_SPEC> {
        BLEND1_RX_R_W::new(self, 16)
    }
}
#[doc = "RGB color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blend_rgb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_rgb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEND_RGB_SPEC;
impl crate::RegisterSpec for BLEND_RGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_rgb::R`](R) reader structure"]
impl crate::Readable for BLEND_RGB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blend_rgb::W`](W) writer structure"]
impl crate::Writable for BLEND_RGB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLEND_RGB to value 0x0080_8080"]
impl crate::Resettable for BLEND_RGB_SPEC {
    const RESET_VALUE: u32 = 0x0080_8080;
}
