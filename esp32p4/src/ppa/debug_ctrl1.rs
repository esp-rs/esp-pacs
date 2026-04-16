#[doc = "Register `DEBUG_CTRL1` reader"]
pub type R = crate::R<DEBUG_CTRL1_SPEC>;
#[doc = "Register `DEBUG_CTRL1` writer"]
pub type W = crate::W<DEBUG_CTRL1_SPEC>;
#[doc = "Field `DBG_REPLACE_DATA` reader - Configures the replace data"]
pub type DBG_REPLACE_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DBG_REPLACE_DATA` writer - Configures the replace data"]
pub type DBG_REPLACE_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the replace data"]
    #[inline(always)]
    pub fn dbg_replace_data(&self) -> DBG_REPLACE_DATA_R {
        DBG_REPLACE_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_CTRL1")
            .field("dbg_replace_data", &self.dbg_replace_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the replace data"]
    #[inline(always)]
    pub fn dbg_replace_data(&mut self) -> DBG_REPLACE_DATA_W<'_, DEBUG_CTRL1_SPEC> {
        DBG_REPLACE_DATA_W::new(self, 0)
    }
}
#[doc = "debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_CTRL1_SPEC;
impl crate::RegisterSpec for DEBUG_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_ctrl1::R`](R) reader structure"]
impl crate::Readable for DEBUG_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_ctrl1::W`](W) writer structure"]
impl crate::Writable for DEBUG_CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_CTRL1 to value 0"]
impl crate::Resettable for DEBUG_CTRL1_SPEC {}
