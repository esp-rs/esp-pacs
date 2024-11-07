#[doc = "Register `HOST_SLCHOST_CONF_W1` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W1_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W1` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W1_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF4` reader - "]
pub type HOST_SLCHOST_CONF4_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF4` writer - "]
pub type HOST_SLCHOST_CONF4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF5` reader - "]
pub type HOST_SLCHOST_CONF5_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF5` writer - "]
pub type HOST_SLCHOST_CONF5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF6` reader - "]
pub type HOST_SLCHOST_CONF6_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF6` writer - "]
pub type HOST_SLCHOST_CONF6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF7` reader - "]
pub type HOST_SLCHOST_CONF7_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF7` writer - "]
pub type HOST_SLCHOST_CONF7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf4(&self) -> HOST_SLCHOST_CONF4_R {
        HOST_SLCHOST_CONF4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf5(&self) -> HOST_SLCHOST_CONF5_R {
        HOST_SLCHOST_CONF5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf6(&self) -> HOST_SLCHOST_CONF6_R {
        HOST_SLCHOST_CONF6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf7(&self) -> HOST_SLCHOST_CONF7_R {
        HOST_SLCHOST_CONF7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W1")
            .field("host_slchost_conf4", &self.host_slchost_conf4())
            .field("host_slchost_conf5", &self.host_slchost_conf5())
            .field("host_slchost_conf6", &self.host_slchost_conf6())
            .field("host_slchost_conf7", &self.host_slchost_conf7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf4(&mut self) -> HOST_SLCHOST_CONF4_W<HOST_SLCHOST_CONF_W1_SPEC> {
        HOST_SLCHOST_CONF4_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf5(&mut self) -> HOST_SLCHOST_CONF5_W<HOST_SLCHOST_CONF_W1_SPEC> {
        HOST_SLCHOST_CONF5_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf6(&mut self) -> HOST_SLCHOST_CONF6_W<HOST_SLCHOST_CONF_W1_SPEC> {
        HOST_SLCHOST_CONF6_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf7(&mut self) -> HOST_SLCHOST_CONF7_W<HOST_SLCHOST_CONF_W1_SPEC> {
        HOST_SLCHOST_CONF7_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slchost_conf_w1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_slchost_conf_w1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w1::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w1::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W1 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
