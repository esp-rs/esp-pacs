#[doc = "Register `DSI_BRG_VER_DATE` reader"]
pub type R = crate::R<DSI_BRG_VER_DATE_SPEC>;
#[doc = "Register `DSI_BRG_VER_DATE` writer"]
pub type W = crate::W<DSI_BRG_VER_DATE_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_BRG_VER_DATE")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, DSI_BRG_VER_DATE_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_brg_ver_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_brg_ver_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_BRG_VER_DATE_SPEC;
impl crate::RegisterSpec for DSI_BRG_VER_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_brg_ver_date::R`](R) reader structure"]
impl crate::Readable for DSI_BRG_VER_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_brg_ver_date::W`](W) writer structure"]
impl crate::Writable for DSI_BRG_VER_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_BRG_VER_DATE to value 0"]
impl crate::Resettable for DSI_BRG_VER_DATE_SPEC {}
