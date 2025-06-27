#[doc = "Register `MASK_HIGH%s` reader"]
pub type R = crate::R<MASK_HIGH_SPEC>;
#[doc = "Register `MASK_HIGH%s` writer"]
pub type W = crate::W<MASK_HIGH_SPEC>;
#[doc = "Field `MASK` reader - "]
pub type MASK_R = crate::FieldReader<u16>;
#[doc = "Field `MASK` writer - "]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENABLED` reader - "]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - "]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK_HIGH")
            .field("mask", &self.mask())
            .field("enabled", &self.enabled())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W<MASK_HIGH_SPEC> {
        MASK_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<MASK_HIGH_SPEC> {
        ENABLED_W::new(self, 16)
    }
}
#[doc = "last 2 bytes of BSSID MAC address filter mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASK_HIGH_SPEC;
impl crate::RegisterSpec for MASK_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_high::R`](R) reader structure"]
impl crate::Readable for MASK_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mask_high::W`](W) writer structure"]
impl crate::Writable for MASK_HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASK_HIGH%s to value 0"]
impl crate::Resettable for MASK_HIGH_SPEC {}
