#[doc = "Register `BLEND_TRANS_MODE` reader"]
pub type R = crate::R<BLEND_TRANS_MODE_SPEC>;
#[doc = "Register `BLEND_TRANS_MODE` writer"]
pub type W = crate::W<BLEND_TRANS_MODE_SPEC>;
#[doc = "Field `BLEND_EN` reader - Set this bit to enable alpha blending."]
pub type BLEND_EN_R = crate::BitReader;
#[doc = "Field `BLEND_EN` writer - Set this bit to enable alpha blending."]
pub type BLEND_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_BYPASS` reader - Set this bit to bypass blender. Then background date would be output."]
pub type BLEND_BYPASS_R = crate::BitReader;
#[doc = "Field `BLEND_BYPASS` writer - Set this bit to bypass blender. Then background date would be output."]
pub type BLEND_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_FIX_PIXEL_FILL_EN` reader - This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL."]
pub type BLEND_FIX_PIXEL_FILL_EN_R = crate::BitReader;
#[doc = "Field `BLEND_FIX_PIXEL_FILL_EN` writer - This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL."]
pub type BLEND_FIX_PIXEL_FILL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDATE` writer - Set this bit to update the transfer mode. Only the bit is set the transfer mode is valid."]
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_RST` reader - write 1 then write 0 to reset blending engine."]
pub type BLEND_RST_R = crate::BitReader;
#[doc = "Field `BLEND_RST` writer - write 1 then write 0 to reset blending engine."]
pub type BLEND_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable alpha blending."]
    #[inline(always)]
    pub fn blend_en(&self) -> BLEND_EN_R {
        BLEND_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to bypass blender. Then background date would be output."]
    #[inline(always)]
    pub fn blend_bypass(&self) -> BLEND_BYPASS_R {
        BLEND_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL."]
    #[inline(always)]
    pub fn blend_fix_pixel_fill_en(&self) -> BLEND_FIX_PIXEL_FILL_EN_R {
        BLEND_FIX_PIXEL_FILL_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - write 1 then write 0 to reset blending engine."]
    #[inline(always)]
    pub fn blend_rst(&self) -> BLEND_RST_R {
        BLEND_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND_TRANS_MODE")
            .field("blend_en", &self.blend_en())
            .field("blend_bypass", &self.blend_bypass())
            .field("blend_fix_pixel_fill_en", &self.blend_fix_pixel_fill_en())
            .field("blend_rst", &self.blend_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable alpha blending."]
    #[inline(always)]
    pub fn blend_en(&mut self) -> BLEND_EN_W<'_, BLEND_TRANS_MODE_SPEC> {
        BLEND_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to bypass blender. Then background date would be output."]
    #[inline(always)]
    pub fn blend_bypass(&mut self) -> BLEND_BYPASS_W<'_, BLEND_TRANS_MODE_SPEC> {
        BLEND_BYPASS_W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL."]
    #[inline(always)]
    pub fn blend_fix_pixel_fill_en(
        &mut self,
    ) -> BLEND_FIX_PIXEL_FILL_EN_W<'_, BLEND_TRANS_MODE_SPEC> {
        BLEND_FIX_PIXEL_FILL_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to update the transfer mode. Only the bit is set the transfer mode is valid."]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<'_, BLEND_TRANS_MODE_SPEC> {
        UPDATE_W::new(self, 3)
    }
    #[doc = "Bit 4 - write 1 then write 0 to reset blending engine."]
    #[inline(always)]
    pub fn blend_rst(&mut self) -> BLEND_RST_W<'_, BLEND_TRANS_MODE_SPEC> {
        BLEND_RST_W::new(self, 4)
    }
}
#[doc = "Blending engine mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_trans_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_trans_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEND_TRANS_MODE_SPEC;
impl crate::RegisterSpec for BLEND_TRANS_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_trans_mode::R`](R) reader structure"]
impl crate::Readable for BLEND_TRANS_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blend_trans_mode::W`](W) writer structure"]
impl crate::Writable for BLEND_TRANS_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_TRANS_MODE to value 0"]
impl crate::Resettable for BLEND_TRANS_MODE_SPEC {}
