#[doc = "Register `HOST_SLCHOST_CONF_W4` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W4_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W4` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W4_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF16` reader - SLC timeout value"]
pub type HOST_SLCHOST_CONF16_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF16` writer - SLC timeout value"]
pub type HOST_SLCHOST_CONF16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF17` reader - SLC timeout enable"]
pub type HOST_SLCHOST_CONF17_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF17` writer - SLC timeout enable"]
pub type HOST_SLCHOST_CONF17_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF18` reader - "]
pub type HOST_SLCHOST_CONF18_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF18` writer - "]
pub type HOST_SLCHOST_CONF18_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF19` reader - Interrupt to target CPU"]
pub type HOST_SLCHOST_CONF19_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF19` writer - Interrupt to target CPU"]
pub type HOST_SLCHOST_CONF19_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SLC timeout value"]
    #[inline(always)]
    pub fn host_slchost_conf16(&self) -> HOST_SLCHOST_CONF16_R {
        HOST_SLCHOST_CONF16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SLC timeout enable"]
    #[inline(always)]
    pub fn host_slchost_conf17(&self) -> HOST_SLCHOST_CONF17_R {
        HOST_SLCHOST_CONF17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf18(&self) -> HOST_SLCHOST_CONF18_R {
        HOST_SLCHOST_CONF18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt to target CPU"]
    #[inline(always)]
    pub fn host_slchost_conf19(&self) -> HOST_SLCHOST_CONF19_R {
        HOST_SLCHOST_CONF19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W4")
            .field("host_slchost_conf16", &self.host_slchost_conf16())
            .field("host_slchost_conf17", &self.host_slchost_conf17())
            .field("host_slchost_conf18", &self.host_slchost_conf18())
            .field("host_slchost_conf19", &self.host_slchost_conf19())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - SLC timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf16(&mut self) -> HOST_SLCHOST_CONF16_W<HOST_SLCHOST_CONF_W4_SPEC> {
        HOST_SLCHOST_CONF16_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SLC timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf17(&mut self) -> HOST_SLCHOST_CONF17_W<HOST_SLCHOST_CONF_W4_SPEC> {
        HOST_SLCHOST_CONF17_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf18(&mut self) -> HOST_SLCHOST_CONF18_W<HOST_SLCHOST_CONF_W4_SPEC> {
        HOST_SLCHOST_CONF18_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt to target CPU"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf19(&mut self) -> HOST_SLCHOST_CONF19_W<HOST_SLCHOST_CONF_W4_SPEC> {
        HOST_SLCHOST_CONF19_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W4_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w4::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w4::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W4 to value 0x01ff"]
impl crate::Resettable for HOST_SLCHOST_CONF_W4_SPEC {
    const RESET_VALUE: u32 = 0x01ff;
}
