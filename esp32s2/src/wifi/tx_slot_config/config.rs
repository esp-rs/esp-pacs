#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `TIMEOUT` reader - "]
pub type TIMEOUT_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT` writer - "]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BACKOFF_TIME` reader - "]
pub type BACKOFF_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `BACKOFF_TIME` writer - "]
pub type BACKOFF_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AIFSN` reader - "]
pub type AIFSN_R = crate::FieldReader;
#[doc = "Field `AIFSN` writer - "]
pub type AIFSN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn backoff_time(&self) -> BACKOFF_TIME_R {
        BACKOFF_TIME_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn aifsn(&self) -> AIFSN_R {
        AIFSN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("timeout", &self.timeout())
            .field("backoff_time", &self.backoff_time())
            .field("aifsn", &self.aifsn())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<CONFIG_SPEC> {
        TIMEOUT_W::new(self, 0)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn backoff_time(&mut self) -> BACKOFF_TIME_W<CONFIG_SPEC> {
        BACKOFF_TIME_W::new(self, 12)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn aifsn(&mut self) -> AIFSN_W<CONFIG_SPEC> {
        AIFSN_W::new(self, 24)
    }
}
#[doc = "Config\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
