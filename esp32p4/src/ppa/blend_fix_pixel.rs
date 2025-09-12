#[doc = "Register `BLEND_FIX_PIXEL` reader"]
pub type R = crate::R<BLEND_FIX_PIXEL_SPEC>;
#[doc = "Register `BLEND_FIX_PIXEL` writer"]
pub type W = crate::W<BLEND_FIX_PIXEL_SPEC>;
#[doc = "Field `BLEND_TX_FIX_PIXEL` reader - The configure fix pixel in fix pixel filling mode for blender engine."]
pub type BLEND_TX_FIX_PIXEL_R = crate::FieldReader<u32>;
#[doc = "Field `BLEND_TX_FIX_PIXEL` writer - The configure fix pixel in fix pixel filling mode for blender engine."]
pub type BLEND_TX_FIX_PIXEL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The configure fix pixel in fix pixel filling mode for blender engine."]
    #[inline(always)]
    pub fn blend_tx_fix_pixel(&self) -> BLEND_TX_FIX_PIXEL_R {
        BLEND_TX_FIX_PIXEL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND_FIX_PIXEL")
            .field("blend_tx_fix_pixel", &self.blend_tx_fix_pixel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The configure fix pixel in fix pixel filling mode for blender engine."]
    #[inline(always)]
    pub fn blend_tx_fix_pixel(&mut self) -> BLEND_TX_FIX_PIXEL_W<'_, BLEND_FIX_PIXEL_SPEC> {
        BLEND_TX_FIX_PIXEL_W::new(self, 0)
    }
}
#[doc = "Blending engine fix pixel register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_fix_pixel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_fix_pixel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEND_FIX_PIXEL_SPEC;
impl crate::RegisterSpec for BLEND_FIX_PIXEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_fix_pixel::R`](R) reader structure"]
impl crate::Readable for BLEND_FIX_PIXEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blend_fix_pixel::W`](W) writer structure"]
impl crate::Writable for BLEND_FIX_PIXEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_FIX_PIXEL to value 0"]
impl crate::Resettable for BLEND_FIX_PIXEL_SPEC {}
