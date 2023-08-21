#[doc = "Register `CONF_W5` reader"]
pub type R = crate::R<CONF_W5_SPEC>;
#[doc = "Register `CONF_W5` writer"]
pub type W = crate::W<CONF_W5_SPEC>;
#[doc = "Field `SLCHOST_CONF20` reader - *******Description***********"]
pub type SLCHOST_CONF20_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF20` writer - *******Description***********"]
pub type SLCHOST_CONF20_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF21` reader - *******Description***********"]
pub type SLCHOST_CONF21_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF21` writer - *******Description***********"]
pub type SLCHOST_CONF21_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF22` reader - *******Description***********"]
pub type SLCHOST_CONF22_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF22` writer - *******Description***********"]
pub type SLCHOST_CONF22_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF23` reader - *******Description***********"]
pub type SLCHOST_CONF23_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF23` writer - *******Description***********"]
pub type SLCHOST_CONF23_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf20(&self) -> SLCHOST_CONF20_R {
        SLCHOST_CONF20_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf21(&self) -> SLCHOST_CONF21_R {
        SLCHOST_CONF21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf22(&self) -> SLCHOST_CONF22_R {
        SLCHOST_CONF22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf23(&self) -> SLCHOST_CONF23_R {
        SLCHOST_CONF23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W5")
            .field(
                "slchost_conf20",
                &format_args!("{}", self.slchost_conf20().bits()),
            )
            .field(
                "slchost_conf21",
                &format_args!("{}", self.slchost_conf21().bits()),
            )
            .field(
                "slchost_conf22",
                &format_args!("{}", self.slchost_conf22().bits()),
            )
            .field(
                "slchost_conf23",
                &format_args!("{}", self.slchost_conf23().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf20(&mut self) -> SLCHOST_CONF20_W<CONF_W5_SPEC, 0> {
        SLCHOST_CONF20_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf21(&mut self) -> SLCHOST_CONF21_W<CONF_W5_SPEC, 8> {
        SLCHOST_CONF21_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf22(&mut self) -> SLCHOST_CONF22_W<CONF_W5_SPEC, 16> {
        SLCHOST_CONF22_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf23(&mut self) -> SLCHOST_CONF23_W<CONF_W5_SPEC, 24> {
        SLCHOST_CONF23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W5_SPEC;
impl crate::RegisterSpec for CONF_W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w5::R`](R) reader structure"]
impl crate::Readable for CONF_W5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w5::W`](W) writer structure"]
impl crate::Writable for CONF_W5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W5 to value 0"]
impl crate::Resettable for CONF_W5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
