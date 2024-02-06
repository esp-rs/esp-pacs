#[doc = "Register `CONF_W2` reader"]
pub type R = crate::R<CONF_W2_SPEC>;
#[doc = "Register `CONF_W2` writer"]
pub type W = crate::W<CONF_W2_SPEC>;
#[doc = "Field `SLCHOST_CONF8` reader - *******Description***********"]
pub type SLCHOST_CONF8_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF8` writer - *******Description***********"]
pub type SLCHOST_CONF8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF9` reader - *******Description***********"]
pub type SLCHOST_CONF9_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF9` writer - *******Description***********"]
pub type SLCHOST_CONF9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF10` reader - *******Description***********"]
pub type SLCHOST_CONF10_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF10` writer - *******Description***********"]
pub type SLCHOST_CONF10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF11` reader - *******Description***********"]
pub type SLCHOST_CONF11_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF11` writer - *******Description***********"]
pub type SLCHOST_CONF11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf8(&self) -> SLCHOST_CONF8_R {
        SLCHOST_CONF8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf9(&self) -> SLCHOST_CONF9_R {
        SLCHOST_CONF9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf10(&self) -> SLCHOST_CONF10_R {
        SLCHOST_CONF10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf11(&self) -> SLCHOST_CONF11_R {
        SLCHOST_CONF11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W2")
            .field(
                "slchost_conf8",
                &format_args!("{}", self.slchost_conf8().bits()),
            )
            .field(
                "slchost_conf9",
                &format_args!("{}", self.slchost_conf9().bits()),
            )
            .field(
                "slchost_conf10",
                &format_args!("{}", self.slchost_conf10().bits()),
            )
            .field(
                "slchost_conf11",
                &format_args!("{}", self.slchost_conf11().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf8(&mut self) -> SLCHOST_CONF8_W<CONF_W2_SPEC> {
        SLCHOST_CONF8_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf9(&mut self) -> SLCHOST_CONF9_W<CONF_W2_SPEC> {
        SLCHOST_CONF9_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf10(&mut self) -> SLCHOST_CONF10_W<CONF_W2_SPEC> {
        SLCHOST_CONF10_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf11(&mut self) -> SLCHOST_CONF11_W<CONF_W2_SPEC> {
        SLCHOST_CONF11_W::new(self, 24)
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
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W2_SPEC;
impl crate::RegisterSpec for CONF_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w2::R`](R) reader structure"]
impl crate::Readable for CONF_W2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w2::W`](W) writer structure"]
impl crate::Writable for CONF_W2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W2 to value 0"]
impl crate::Resettable for CONF_W2_SPEC {
    const RESET_VALUE: u32 = 0;
}
