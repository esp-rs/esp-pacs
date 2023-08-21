#[doc = "Register `HOST_SLCHOST_CONF_W9` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W9_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W9` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W9_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF36` reader - "]
pub type HOST_SLCHOST_CONF36_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF36` writer - "]
pub type HOST_SLCHOST_CONF36_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF37` reader - "]
pub type HOST_SLCHOST_CONF37_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF37` writer - "]
pub type HOST_SLCHOST_CONF37_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF38` reader - "]
pub type HOST_SLCHOST_CONF38_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF38` writer - "]
pub type HOST_SLCHOST_CONF38_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF39` reader - "]
pub type HOST_SLCHOST_CONF39_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF39` writer - "]
pub type HOST_SLCHOST_CONF39_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf36(&self) -> HOST_SLCHOST_CONF36_R {
        HOST_SLCHOST_CONF36_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf37(&self) -> HOST_SLCHOST_CONF37_R {
        HOST_SLCHOST_CONF37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf38(&self) -> HOST_SLCHOST_CONF38_R {
        HOST_SLCHOST_CONF38_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf39(&self) -> HOST_SLCHOST_CONF39_R {
        HOST_SLCHOST_CONF39_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W9")
            .field(
                "host_slchost_conf36",
                &format_args!("{}", self.host_slchost_conf36().bits()),
            )
            .field(
                "host_slchost_conf37",
                &format_args!("{}", self.host_slchost_conf37().bits()),
            )
            .field(
                "host_slchost_conf38",
                &format_args!("{}", self.host_slchost_conf38().bits()),
            )
            .field(
                "host_slchost_conf39",
                &format_args!("{}", self.host_slchost_conf39().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf36(&mut self) -> HOST_SLCHOST_CONF36_W<HOST_SLCHOST_CONF_W9_SPEC, 0> {
        HOST_SLCHOST_CONF36_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf37(&mut self) -> HOST_SLCHOST_CONF37_W<HOST_SLCHOST_CONF_W9_SPEC, 8> {
        HOST_SLCHOST_CONF37_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf38(&mut self) -> HOST_SLCHOST_CONF38_W<HOST_SLCHOST_CONF_W9_SPEC, 16> {
        HOST_SLCHOST_CONF38_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf39(&mut self) -> HOST_SLCHOST_CONF39_W<HOST_SLCHOST_CONF_W9_SPEC, 24> {
        HOST_SLCHOST_CONF39_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W9_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w9::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w9::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W9 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
