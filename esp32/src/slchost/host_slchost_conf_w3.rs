#[doc = "Register `HOST_SLCHOST_CONF_W3` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W3_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W3` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W3_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF12` reader - "]
pub type HOST_SLCHOST_CONF12_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF12` writer - "]
pub type HOST_SLCHOST_CONF12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF13` reader - "]
pub type HOST_SLCHOST_CONF13_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF13` writer - "]
pub type HOST_SLCHOST_CONF13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF14` reader - "]
pub type HOST_SLCHOST_CONF14_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF14` writer - "]
pub type HOST_SLCHOST_CONF14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF15` reader - "]
pub type HOST_SLCHOST_CONF15_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF15` writer - "]
pub type HOST_SLCHOST_CONF15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf12(&self) -> HOST_SLCHOST_CONF12_R {
        HOST_SLCHOST_CONF12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf13(&self) -> HOST_SLCHOST_CONF13_R {
        HOST_SLCHOST_CONF13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf14(&self) -> HOST_SLCHOST_CONF14_R {
        HOST_SLCHOST_CONF14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf15(&self) -> HOST_SLCHOST_CONF15_R {
        HOST_SLCHOST_CONF15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W3")
            .field("host_slchost_conf12", &self.host_slchost_conf12())
            .field("host_slchost_conf13", &self.host_slchost_conf13())
            .field("host_slchost_conf14", &self.host_slchost_conf14())
            .field("host_slchost_conf15", &self.host_slchost_conf15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf12(&mut self) -> HOST_SLCHOST_CONF12_W<HOST_SLCHOST_CONF_W3_SPEC> {
        HOST_SLCHOST_CONF12_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf13(&mut self) -> HOST_SLCHOST_CONF13_W<HOST_SLCHOST_CONF_W3_SPEC> {
        HOST_SLCHOST_CONF13_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf14(&mut self) -> HOST_SLCHOST_CONF14_W<HOST_SLCHOST_CONF_W3_SPEC> {
        HOST_SLCHOST_CONF14_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf15(&mut self) -> HOST_SLCHOST_CONF15_W<HOST_SLCHOST_CONF_W3_SPEC> {
        HOST_SLCHOST_CONF15_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slchost_conf_w3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_slchost_conf_w3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W3_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w3::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w3::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W3 to value 0xc0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W3_SPEC {
    const RESET_VALUE: u32 = 0xc0;
}
