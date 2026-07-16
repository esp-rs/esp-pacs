#[doc = "Register `TOUCH_AON` reader"]
pub type R = crate::R<TOUCH_AON_SPEC>;
#[doc = "Register `TOUCH_AON` writer"]
pub type W = crate::W<TOUCH_AON_SPEC>;
#[doc = "Field `RST_EN` reader - need_des"]
pub type RST_EN_R = crate::BitReader;
#[doc = "Field `RST_EN` writer - need_des"]
pub type RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_AON")
            .field("rst_en", &self.rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn rst_en(&mut self) -> RST_EN_W<'_, TOUCH_AON_SPEC> {
        RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_aon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_aon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_AON_SPEC;
impl crate::RegisterSpec for TOUCH_AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_aon::R`](R) reader structure"]
impl crate::Readable for TOUCH_AON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_aon::W`](W) writer structure"]
impl crate::Writable for TOUCH_AON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_AON to value 0"]
impl crate::Resettable for TOUCH_AON_SPEC {}
