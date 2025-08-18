#[doc = "Register `HOST_SLCHOST_CONF_W13` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W13_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W13` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W13_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF52` reader - "]
pub type HOST_SLCHOST_CONF52_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF52` writer - "]
pub type HOST_SLCHOST_CONF52_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF53` reader - "]
pub type HOST_SLCHOST_CONF53_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF53` writer - "]
pub type HOST_SLCHOST_CONF53_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF54` reader - "]
pub type HOST_SLCHOST_CONF54_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF54` writer - "]
pub type HOST_SLCHOST_CONF54_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF55` reader - "]
pub type HOST_SLCHOST_CONF55_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF55` writer - "]
pub type HOST_SLCHOST_CONF55_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf52(&self) -> HOST_SLCHOST_CONF52_R {
        HOST_SLCHOST_CONF52_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf53(&self) -> HOST_SLCHOST_CONF53_R {
        HOST_SLCHOST_CONF53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf54(&self) -> HOST_SLCHOST_CONF54_R {
        HOST_SLCHOST_CONF54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf55(&self) -> HOST_SLCHOST_CONF55_R {
        HOST_SLCHOST_CONF55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W13")
            .field("host_slchost_conf52", &self.host_slchost_conf52())
            .field("host_slchost_conf53", &self.host_slchost_conf53())
            .field("host_slchost_conf54", &self.host_slchost_conf54())
            .field("host_slchost_conf55", &self.host_slchost_conf55())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf52(&mut self) -> HOST_SLCHOST_CONF52_W<'_, HOST_SLCHOST_CONF_W13_SPEC> {
        HOST_SLCHOST_CONF52_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf53(&mut self) -> HOST_SLCHOST_CONF53_W<'_, HOST_SLCHOST_CONF_W13_SPEC> {
        HOST_SLCHOST_CONF53_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf54(&mut self) -> HOST_SLCHOST_CONF54_W<'_, HOST_SLCHOST_CONF_W13_SPEC> {
        HOST_SLCHOST_CONF54_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf55(&mut self) -> HOST_SLCHOST_CONF55_W<'_, HOST_SLCHOST_CONF_W13_SPEC> {
        HOST_SLCHOST_CONF55_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slchost_conf_w13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_slchost_conf_w13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W13_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w13::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w13::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W13_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W13 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W13_SPEC {}
