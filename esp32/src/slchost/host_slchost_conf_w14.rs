#[doc = "Register `HOST_SLCHOST_CONF_W14` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W14_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W14` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W14_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF56` reader - "]
pub type HOST_SLCHOST_CONF56_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF56` writer - "]
pub type HOST_SLCHOST_CONF56_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF57` reader - "]
pub type HOST_SLCHOST_CONF57_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF57` writer - "]
pub type HOST_SLCHOST_CONF57_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF58` reader - "]
pub type HOST_SLCHOST_CONF58_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF58` writer - "]
pub type HOST_SLCHOST_CONF58_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF59` reader - "]
pub type HOST_SLCHOST_CONF59_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF59` writer - "]
pub type HOST_SLCHOST_CONF59_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf56(&self) -> HOST_SLCHOST_CONF56_R {
        HOST_SLCHOST_CONF56_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf57(&self) -> HOST_SLCHOST_CONF57_R {
        HOST_SLCHOST_CONF57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf58(&self) -> HOST_SLCHOST_CONF58_R {
        HOST_SLCHOST_CONF58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf59(&self) -> HOST_SLCHOST_CONF59_R {
        HOST_SLCHOST_CONF59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W14")
            .field("host_slchost_conf56", &self.host_slchost_conf56().bits())
            .field("host_slchost_conf57", &self.host_slchost_conf57().bits())
            .field("host_slchost_conf58", &self.host_slchost_conf58().bits())
            .field("host_slchost_conf59", &self.host_slchost_conf59().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf56(&mut self) -> HOST_SLCHOST_CONF56_W<HOST_SLCHOST_CONF_W14_SPEC> {
        HOST_SLCHOST_CONF56_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf57(&mut self) -> HOST_SLCHOST_CONF57_W<HOST_SLCHOST_CONF_W14_SPEC> {
        HOST_SLCHOST_CONF57_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf58(&mut self) -> HOST_SLCHOST_CONF58_W<HOST_SLCHOST_CONF_W14_SPEC> {
        HOST_SLCHOST_CONF58_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf59(&mut self) -> HOST_SLCHOST_CONF59_W<HOST_SLCHOST_CONF_W14_SPEC> {
        HOST_SLCHOST_CONF59_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W14_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w14::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w14::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W14_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W14 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W14_SPEC {
    const RESET_VALUE: u32 = 0;
}
