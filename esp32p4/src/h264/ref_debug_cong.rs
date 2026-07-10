#[doc = "Register `REF_DEBUG_CONG` reader"]
pub type R = crate::R<REF_DEBUG_CONG_SPEC>;
#[doc = "Register `REF_DEBUG_CONG` writer"]
pub type W = crate::W<REF_DEBUG_CONG_SPEC>;
#[doc = "Field `DBG_REPLACE_REF_DATA_EN` reader - Configure whether to replace reference picture pixels.\\\\0: not replace\\\\1: replace"]
pub type DBG_REPLACE_REF_DATA_EN_R = crate::BitReader;
#[doc = "Field `DBG_REPLACE_REF_DATA_EN` writer - Configure whether to replace reference picture pixels.\\\\0: not replace\\\\1: replace"]
pub type DBG_REPLACE_REF_DATA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_REPLACE_REF_DATA` reader - Configure reference picture pixels to be replaced.byte0~2 is VUY"]
pub type DBG_REPLACE_REF_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DBG_REPLACE_REF_DATA` writer - Configure reference picture pixels to be replaced.byte0~2 is VUY"]
pub type DBG_REPLACE_REF_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Configure whether to replace reference picture pixels.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_ref_data_en(&self) -> DBG_REPLACE_REF_DATA_EN_R {
        DBG_REPLACE_REF_DATA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:24 - Configure reference picture pixels to be replaced.byte0~2 is VUY"]
    #[inline(always)]
    pub fn dbg_replace_ref_data(&self) -> DBG_REPLACE_REF_DATA_R {
        DBG_REPLACE_REF_DATA_R::new((self.bits >> 1) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_DEBUG_CONG")
            .field("dbg_replace_ref_data_en", &self.dbg_replace_ref_data_en())
            .field("dbg_replace_ref_data", &self.dbg_replace_ref_data())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether to replace reference picture pixels.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_ref_data_en(
        &mut self,
    ) -> DBG_REPLACE_REF_DATA_EN_W<'_, REF_DEBUG_CONG_SPEC> {
        DBG_REPLACE_REF_DATA_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:24 - Configure reference picture pixels to be replaced.byte0~2 is VUY"]
    #[inline(always)]
    pub fn dbg_replace_ref_data(&mut self) -> DBG_REPLACE_REF_DATA_W<'_, REF_DEBUG_CONG_SPEC> {
        DBG_REPLACE_REF_DATA_W::new(self, 1)
    }
}
#[doc = "Deblocking filter final data debug configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_debug_cong::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_debug_cong::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_DEBUG_CONG_SPEC;
impl crate::RegisterSpec for REF_DEBUG_CONG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_debug_cong::R`](R) reader structure"]
impl crate::Readable for REF_DEBUG_CONG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_debug_cong::W`](W) writer structure"]
impl crate::Writable for REF_DEBUG_CONG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_DEBUG_CONG to value 0"]
impl crate::Resettable for REF_DEBUG_CONG_SPEC {}
