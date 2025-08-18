#[doc = "Register `HOST_SLCHOST_CONF_W5` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W5_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W5` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W5_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF20` reader - "]
pub type HOST_SLCHOST_CONF20_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF20` writer - "]
pub type HOST_SLCHOST_CONF20_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF21` reader - "]
pub type HOST_SLCHOST_CONF21_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF21` writer - "]
pub type HOST_SLCHOST_CONF21_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF22` reader - "]
pub type HOST_SLCHOST_CONF22_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF22` writer - "]
pub type HOST_SLCHOST_CONF22_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF23` reader - "]
pub type HOST_SLCHOST_CONF23_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF23` writer - "]
pub type HOST_SLCHOST_CONF23_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf20(&self) -> HOST_SLCHOST_CONF20_R {
        HOST_SLCHOST_CONF20_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf21(&self) -> HOST_SLCHOST_CONF21_R {
        HOST_SLCHOST_CONF21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf22(&self) -> HOST_SLCHOST_CONF22_R {
        HOST_SLCHOST_CONF22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf23(&self) -> HOST_SLCHOST_CONF23_R {
        HOST_SLCHOST_CONF23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W5")
            .field("host_slchost_conf20", &self.host_slchost_conf20())
            .field("host_slchost_conf21", &self.host_slchost_conf21())
            .field("host_slchost_conf22", &self.host_slchost_conf22())
            .field("host_slchost_conf23", &self.host_slchost_conf23())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf20(&mut self) -> HOST_SLCHOST_CONF20_W<'_, HOST_SLCHOST_CONF_W5_SPEC> {
        HOST_SLCHOST_CONF20_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf21(&mut self) -> HOST_SLCHOST_CONF21_W<'_, HOST_SLCHOST_CONF_W5_SPEC> {
        HOST_SLCHOST_CONF21_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf22(&mut self) -> HOST_SLCHOST_CONF22_W<'_, HOST_SLCHOST_CONF_W5_SPEC> {
        HOST_SLCHOST_CONF22_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf23(&mut self) -> HOST_SLCHOST_CONF23_W<'_, HOST_SLCHOST_CONF_W5_SPEC> {
        HOST_SLCHOST_CONF23_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slchost_conf_w5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_slchost_conf_w5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W5_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w5::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w5::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W5 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W5_SPEC {}
