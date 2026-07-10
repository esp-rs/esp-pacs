#[doc = "Register `ORI_DEBUG_CONF` reader"]
pub type R = crate::R<ORI_DEBUG_CONF_SPEC>;
#[doc = "Register `ORI_DEBUG_CONF` writer"]
pub type W = crate::W<ORI_DEBUG_CONF_SPEC>;
#[doc = "Field `DBG_REPLACE_ORI_DATA_EN` reader - Configures whether to replace original picture pixels.\\\\0: not replace\\\\1: replace"]
pub type DBG_REPLACE_ORI_DATA_EN_R = crate::BitReader;
#[doc = "Field `DBG_REPLACE_ORI_DATA_EN` writer - Configures whether to replace original picture pixels.\\\\0: not replace\\\\1: replace"]
pub type DBG_REPLACE_ORI_DATA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_REPLACE_ORI_DATA` reader - Configures original picture pixels to be replaced. When the original picture color space is RGB, byte0~2 is BGR. When the original picture color space is YUV, byte0~2 is VUY. When the original picture color space is GRAY, byte0 is GRAY."]
pub type DBG_REPLACE_ORI_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DBG_REPLACE_ORI_DATA` writer - Configures original picture pixels to be replaced. When the original picture color space is RGB, byte0~2 is BGR. When the original picture color space is YUV, byte0~2 is VUY. When the original picture color space is GRAY, byte0 is GRAY."]
pub type DBG_REPLACE_ORI_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Configures whether to replace original picture pixels.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_ori_data_en(&self) -> DBG_REPLACE_ORI_DATA_EN_R {
        DBG_REPLACE_ORI_DATA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:24 - Configures original picture pixels to be replaced. When the original picture color space is RGB, byte0~2 is BGR. When the original picture color space is YUV, byte0~2 is VUY. When the original picture color space is GRAY, byte0 is GRAY."]
    #[inline(always)]
    pub fn dbg_replace_ori_data(&self) -> DBG_REPLACE_ORI_DATA_R {
        DBG_REPLACE_ORI_DATA_R::new((self.bits >> 1) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ORI_DEBUG_CONF")
            .field("dbg_replace_ori_data_en", &self.dbg_replace_ori_data_en())
            .field("dbg_replace_ori_data", &self.dbg_replace_ori_data())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to replace original picture pixels.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_ori_data_en(
        &mut self,
    ) -> DBG_REPLACE_ORI_DATA_EN_W<'_, ORI_DEBUG_CONF_SPEC> {
        DBG_REPLACE_ORI_DATA_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:24 - Configures original picture pixels to be replaced. When the original picture color space is RGB, byte0~2 is BGR. When the original picture color space is YUV, byte0~2 is VUY. When the original picture color space is GRAY, byte0 is GRAY."]
    #[inline(always)]
    pub fn dbg_replace_ori_data(&mut self) -> DBG_REPLACE_ORI_DATA_W<'_, ORI_DEBUG_CONF_SPEC> {
        DBG_REPLACE_ORI_DATA_W::new(self, 1)
    }
}
#[doc = "Original picture debug configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ori_debug_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ori_debug_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORI_DEBUG_CONF_SPEC;
impl crate::RegisterSpec for ORI_DEBUG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ori_debug_conf::R`](R) reader structure"]
impl crate::Readable for ORI_DEBUG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ori_debug_conf::W`](W) writer structure"]
impl crate::Writable for ORI_DEBUG_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ORI_DEBUG_CONF to value 0"]
impl crate::Resettable for ORI_DEBUG_CONF_SPEC {}
