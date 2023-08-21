#[doc = "Register `CONF_W13` reader"]
pub type R = crate::R<CONF_W13_SPEC>;
#[doc = "Register `CONF_W13` writer"]
pub type W = crate::W<CONF_W13_SPEC>;
#[doc = "Field `SLCHOST_CONF52` reader - *******Description***********"]
pub type SLCHOST_CONF52_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF52` writer - *******Description***********"]
pub type SLCHOST_CONF52_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF53` reader - *******Description***********"]
pub type SLCHOST_CONF53_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF53` writer - *******Description***********"]
pub type SLCHOST_CONF53_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF54` reader - *******Description***********"]
pub type SLCHOST_CONF54_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF54` writer - *******Description***********"]
pub type SLCHOST_CONF54_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF55` reader - *******Description***********"]
pub type SLCHOST_CONF55_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF55` writer - *******Description***********"]
pub type SLCHOST_CONF55_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf52(&self) -> SLCHOST_CONF52_R {
        SLCHOST_CONF52_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf53(&self) -> SLCHOST_CONF53_R {
        SLCHOST_CONF53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf54(&self) -> SLCHOST_CONF54_R {
        SLCHOST_CONF54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf55(&self) -> SLCHOST_CONF55_R {
        SLCHOST_CONF55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W13")
            .field(
                "slchost_conf52",
                &format_args!("{}", self.slchost_conf52().bits()),
            )
            .field(
                "slchost_conf53",
                &format_args!("{}", self.slchost_conf53().bits()),
            )
            .field(
                "slchost_conf54",
                &format_args!("{}", self.slchost_conf54().bits()),
            )
            .field(
                "slchost_conf55",
                &format_args!("{}", self.slchost_conf55().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf52(&mut self) -> SLCHOST_CONF52_W<CONF_W13_SPEC, 0> {
        SLCHOST_CONF52_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf53(&mut self) -> SLCHOST_CONF53_W<CONF_W13_SPEC, 8> {
        SLCHOST_CONF53_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf54(&mut self) -> SLCHOST_CONF54_W<CONF_W13_SPEC, 16> {
        SLCHOST_CONF54_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf55(&mut self) -> SLCHOST_CONF55_W<CONF_W13_SPEC, 24> {
        SLCHOST_CONF55_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W13_SPEC;
impl crate::RegisterSpec for CONF_W13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w13::R`](R) reader structure"]
impl crate::Readable for CONF_W13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w13::W`](W) writer structure"]
impl crate::Writable for CONF_W13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W13 to value 0"]
impl crate::Resettable for CONF_W13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
