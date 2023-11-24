#[doc = "Register `CONF_W12` reader"]
pub type R = crate::R<CONF_W12_SPEC>;
#[doc = "Register `CONF_W12` writer"]
pub type W = crate::W<CONF_W12_SPEC>;
#[doc = "Field `SLCHOST_CONF48` reader - *******Description***********"]
pub type SLCHOST_CONF48_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF48` writer - *******Description***********"]
pub type SLCHOST_CONF48_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF49` reader - *******Description***********"]
pub type SLCHOST_CONF49_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF49` writer - *******Description***********"]
pub type SLCHOST_CONF49_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF50` reader - *******Description***********"]
pub type SLCHOST_CONF50_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF50` writer - *******Description***********"]
pub type SLCHOST_CONF50_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF51` reader - *******Description***********"]
pub type SLCHOST_CONF51_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF51` writer - *******Description***********"]
pub type SLCHOST_CONF51_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf48(&self) -> SLCHOST_CONF48_R {
        SLCHOST_CONF48_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf49(&self) -> SLCHOST_CONF49_R {
        SLCHOST_CONF49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf50(&self) -> SLCHOST_CONF50_R {
        SLCHOST_CONF50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf51(&self) -> SLCHOST_CONF51_R {
        SLCHOST_CONF51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W12")
            .field(
                "slchost_conf48",
                &format_args!("{}", self.slchost_conf48().bits()),
            )
            .field(
                "slchost_conf49",
                &format_args!("{}", self.slchost_conf49().bits()),
            )
            .field(
                "slchost_conf50",
                &format_args!("{}", self.slchost_conf50().bits()),
            )
            .field(
                "slchost_conf51",
                &format_args!("{}", self.slchost_conf51().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf48(&mut self) -> SLCHOST_CONF48_W<CONF_W12_SPEC> {
        SLCHOST_CONF48_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf49(&mut self) -> SLCHOST_CONF49_W<CONF_W12_SPEC> {
        SLCHOST_CONF49_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf50(&mut self) -> SLCHOST_CONF50_W<CONF_W12_SPEC> {
        SLCHOST_CONF50_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf51(&mut self) -> SLCHOST_CONF51_W<CONF_W12_SPEC> {
        SLCHOST_CONF51_W::new(self, 24)
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
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W12_SPEC;
impl crate::RegisterSpec for CONF_W12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w12::R`](R) reader structure"]
impl crate::Readable for CONF_W12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w12::W`](W) writer structure"]
impl crate::Writable for CONF_W12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W12 to value 0"]
impl crate::Resettable for CONF_W12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
