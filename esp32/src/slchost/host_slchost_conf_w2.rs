#[doc = "Register `HOST_SLCHOST_CONF_W2` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W2_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W2` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W2_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF8` reader - "]
pub type HOST_SLCHOST_CONF8_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF8` writer - "]
pub type HOST_SLCHOST_CONF8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF9` reader - "]
pub type HOST_SLCHOST_CONF9_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF9` writer - "]
pub type HOST_SLCHOST_CONF9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF10` reader - "]
pub type HOST_SLCHOST_CONF10_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF10` writer - "]
pub type HOST_SLCHOST_CONF10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF11` reader - "]
pub type HOST_SLCHOST_CONF11_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF11` writer - "]
pub type HOST_SLCHOST_CONF11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf8(&self) -> HOST_SLCHOST_CONF8_R {
        HOST_SLCHOST_CONF8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf9(&self) -> HOST_SLCHOST_CONF9_R {
        HOST_SLCHOST_CONF9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf10(&self) -> HOST_SLCHOST_CONF10_R {
        HOST_SLCHOST_CONF10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf11(&self) -> HOST_SLCHOST_CONF11_R {
        HOST_SLCHOST_CONF11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W2")
            .field("host_slchost_conf8", &self.host_slchost_conf8())
            .field("host_slchost_conf9", &self.host_slchost_conf9())
            .field("host_slchost_conf10", &self.host_slchost_conf10())
            .field("host_slchost_conf11", &self.host_slchost_conf11())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf8(&mut self) -> HOST_SLCHOST_CONF8_W<HOST_SLCHOST_CONF_W2_SPEC> {
        HOST_SLCHOST_CONF8_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf9(&mut self) -> HOST_SLCHOST_CONF9_W<HOST_SLCHOST_CONF_W2_SPEC> {
        HOST_SLCHOST_CONF9_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf10(&mut self) -> HOST_SLCHOST_CONF10_W<HOST_SLCHOST_CONF_W2_SPEC> {
        HOST_SLCHOST_CONF10_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf11(&mut self) -> HOST_SLCHOST_CONF11_W<HOST_SLCHOST_CONF_W2_SPEC> {
        HOST_SLCHOST_CONF11_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W2_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w2::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w2::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W2 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W2_SPEC {
    const RESET_VALUE: u32 = 0;
}
