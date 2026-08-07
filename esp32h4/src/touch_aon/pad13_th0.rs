#[doc = "Register `PAD13_TH0` reader"]
pub type R = crate::R<PAD13_TH0_SPEC>;
#[doc = "Register `PAD13_TH0` writer"]
pub type W = crate::W<PAD13_TH0_SPEC>;
#[doc = "Field `TOUCH_PAD13_TH0` reader - Reserved"]
pub type TOUCH_PAD13_TH0_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_PAD13_TH0` writer - Reserved"]
pub type TOUCH_PAD13_TH0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn touch_pad13_th0(&self) -> TOUCH_PAD13_TH0_R {
        TOUCH_PAD13_TH0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD13_TH0")
            .field("touch_pad13_th0", &self.touch_pad13_th0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn touch_pad13_th0(&mut self) -> TOUCH_PAD13_TH0_W<'_, PAD13_TH0_SPEC> {
        TOUCH_PAD13_TH0_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad13_th0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad13_th0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD13_TH0_SPEC;
impl crate::RegisterSpec for PAD13_TH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad13_th0::R`](R) reader structure"]
impl crate::Readable for PAD13_TH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad13_th0::W`](W) writer structure"]
impl crate::Writable for PAD13_TH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD13_TH0 to value 0"]
impl crate::Resettable for PAD13_TH0_SPEC {}
