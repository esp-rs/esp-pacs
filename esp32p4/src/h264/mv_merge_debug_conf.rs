#[doc = "Register `MV_MERGE_DEBUG_CONF` reader"]
pub type R = crate::R<MV_MERGE_DEBUG_CONF_SPEC>;
#[doc = "Register `MV_MERGE_DEBUG_CONF` writer"]
pub type W = crate::W<MV_MERGE_DEBUG_CONF_SPEC>;
#[doc = "Field `DBG_REPLACE_MV_MERGE_DATA_EN` reader - Configures whether to replace mv merge data.\\\\0: not replace\\\\1: replace"]
pub type DBG_REPLACE_MV_MERGE_DATA_EN_R = crate::BitReader;
#[doc = "Field `DBG_REPLACE_MV_MERGE_DATA_EN` writer - Configures whether to replace mv merge data.\\\\0: not replace\\\\1: replace"]
pub type DBG_REPLACE_MV_MERGE_DATA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_REPLACE_MV_MERGE_DATA` reader - Configures mv merge data to be replaced."]
pub type DBG_REPLACE_MV_MERGE_DATA_R = crate::FieldReader;
#[doc = "Field `DBG_REPLACE_MV_MERGE_DATA` writer - Configures mv merge data to be replaced."]
pub type DBG_REPLACE_MV_MERGE_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Configures whether to replace mv merge data.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_mv_merge_data_en(&self) -> DBG_REPLACE_MV_MERGE_DATA_EN_R {
        DBG_REPLACE_MV_MERGE_DATA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Configures mv merge data to be replaced."]
    #[inline(always)]
    pub fn dbg_replace_mv_merge_data(&self) -> DBG_REPLACE_MV_MERGE_DATA_R {
        DBG_REPLACE_MV_MERGE_DATA_R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MV_MERGE_DEBUG_CONF")
            .field(
                "dbg_replace_mv_merge_data_en",
                &self.dbg_replace_mv_merge_data_en(),
            )
            .field(
                "dbg_replace_mv_merge_data",
                &self.dbg_replace_mv_merge_data(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to replace mv merge data.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_mv_merge_data_en(
        &mut self,
    ) -> DBG_REPLACE_MV_MERGE_DATA_EN_W<'_, MV_MERGE_DEBUG_CONF_SPEC> {
        DBG_REPLACE_MV_MERGE_DATA_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:8 - Configures mv merge data to be replaced."]
    #[inline(always)]
    pub fn dbg_replace_mv_merge_data(
        &mut self,
    ) -> DBG_REPLACE_MV_MERGE_DATA_W<'_, MV_MERGE_DEBUG_CONF_SPEC> {
        DBG_REPLACE_MV_MERGE_DATA_W::new(self, 1)
    }
}
#[doc = "Original picture debug configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mv_merge_debug_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mv_merge_debug_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MV_MERGE_DEBUG_CONF_SPEC;
impl crate::RegisterSpec for MV_MERGE_DEBUG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mv_merge_debug_conf::R`](R) reader structure"]
impl crate::Readable for MV_MERGE_DEBUG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mv_merge_debug_conf::W`](W) writer structure"]
impl crate::Writable for MV_MERGE_DEBUG_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MV_MERGE_DEBUG_CONF to value 0"]
impl crate::Resettable for MV_MERGE_DEBUG_CONF_SPEC {}
