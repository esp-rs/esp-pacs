#[doc = "Register `HOST_SLCHOST_CONF_W8` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W8_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W8` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W8_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF32` reader - "]
pub type HOST_SLCHOST_CONF32_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF32` writer - "]
pub type HOST_SLCHOST_CONF32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF33` reader - "]
pub type HOST_SLCHOST_CONF33_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF33` writer - "]
pub type HOST_SLCHOST_CONF33_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF34` reader - "]
pub type HOST_SLCHOST_CONF34_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF34` writer - "]
pub type HOST_SLCHOST_CONF34_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF35` reader - "]
pub type HOST_SLCHOST_CONF35_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF35` writer - "]
pub type HOST_SLCHOST_CONF35_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf32(&self) -> HOST_SLCHOST_CONF32_R {
        HOST_SLCHOST_CONF32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf33(&self) -> HOST_SLCHOST_CONF33_R {
        HOST_SLCHOST_CONF33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf34(&self) -> HOST_SLCHOST_CONF34_R {
        HOST_SLCHOST_CONF34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf35(&self) -> HOST_SLCHOST_CONF35_R {
        HOST_SLCHOST_CONF35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W8")
            .field(
                "host_slchost_conf32",
                &format_args!("{}", self.host_slchost_conf32().bits()),
            )
            .field(
                "host_slchost_conf33",
                &format_args!("{}", self.host_slchost_conf33().bits()),
            )
            .field(
                "host_slchost_conf34",
                &format_args!("{}", self.host_slchost_conf34().bits()),
            )
            .field(
                "host_slchost_conf35",
                &format_args!("{}", self.host_slchost_conf35().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf32(&mut self) -> HOST_SLCHOST_CONF32_W<HOST_SLCHOST_CONF_W8_SPEC> {
        HOST_SLCHOST_CONF32_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf33(&mut self) -> HOST_SLCHOST_CONF33_W<HOST_SLCHOST_CONF_W8_SPEC> {
        HOST_SLCHOST_CONF33_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf34(&mut self) -> HOST_SLCHOST_CONF34_W<HOST_SLCHOST_CONF_W8_SPEC> {
        HOST_SLCHOST_CONF34_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf35(&mut self) -> HOST_SLCHOST_CONF35_W<HOST_SLCHOST_CONF_W8_SPEC> {
        HOST_SLCHOST_CONF35_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W8_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w8::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w8::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W8 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
