#[doc = "Register `HOST_SLCHOST_CONF_W15` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W15_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W15` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W15_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF60` reader - "]
pub type HOST_SLCHOST_CONF60_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF60` writer - "]
pub type HOST_SLCHOST_CONF60_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF61` reader - "]
pub type HOST_SLCHOST_CONF61_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF61` writer - "]
pub type HOST_SLCHOST_CONF61_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF62` reader - "]
pub type HOST_SLCHOST_CONF62_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF62` writer - "]
pub type HOST_SLCHOST_CONF62_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF63` reader - "]
pub type HOST_SLCHOST_CONF63_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF63` writer - "]
pub type HOST_SLCHOST_CONF63_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf60(&self) -> HOST_SLCHOST_CONF60_R {
        HOST_SLCHOST_CONF60_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf61(&self) -> HOST_SLCHOST_CONF61_R {
        HOST_SLCHOST_CONF61_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf62(&self) -> HOST_SLCHOST_CONF62_R {
        HOST_SLCHOST_CONF62_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf63(&self) -> HOST_SLCHOST_CONF63_R {
        HOST_SLCHOST_CONF63_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W15")
            .field("host_slchost_conf60", &self.host_slchost_conf60())
            .field("host_slchost_conf61", &self.host_slchost_conf61())
            .field("host_slchost_conf62", &self.host_slchost_conf62())
            .field("host_slchost_conf63", &self.host_slchost_conf63())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf60(&mut self) -> HOST_SLCHOST_CONF60_W<HOST_SLCHOST_CONF_W15_SPEC> {
        HOST_SLCHOST_CONF60_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf61(&mut self) -> HOST_SLCHOST_CONF61_W<HOST_SLCHOST_CONF_W15_SPEC> {
        HOST_SLCHOST_CONF61_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf62(&mut self) -> HOST_SLCHOST_CONF62_W<HOST_SLCHOST_CONF_W15_SPEC> {
        HOST_SLCHOST_CONF62_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf63(&mut self) -> HOST_SLCHOST_CONF63_W<HOST_SLCHOST_CONF_W15_SPEC> {
        HOST_SLCHOST_CONF63_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slchost_conf_w15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_slchost_conf_w15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W15_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w15::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w15::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W15_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W15 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W15_SPEC {
    const RESET_VALUE: u32 = 0;
}
