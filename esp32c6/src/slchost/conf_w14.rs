#[doc = "Register `CONF_W14` reader"]
pub type R = crate::R<CONF_W14_SPEC>;
#[doc = "Register `CONF_W14` writer"]
pub type W = crate::W<CONF_W14_SPEC>;
#[doc = "Field `SLCHOST_CONF56` reader - *******Description***********"]
pub type SLCHOST_CONF56_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF56` writer - *******Description***********"]
pub type SLCHOST_CONF56_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF57` reader - *******Description***********"]
pub type SLCHOST_CONF57_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF57` writer - *******Description***********"]
pub type SLCHOST_CONF57_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF58` reader - *******Description***********"]
pub type SLCHOST_CONF58_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF58` writer - *******Description***********"]
pub type SLCHOST_CONF58_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF59` reader - *******Description***********"]
pub type SLCHOST_CONF59_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF59` writer - *******Description***********"]
pub type SLCHOST_CONF59_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf56(&self) -> SLCHOST_CONF56_R {
        SLCHOST_CONF56_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf57(&self) -> SLCHOST_CONF57_R {
        SLCHOST_CONF57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf58(&self) -> SLCHOST_CONF58_R {
        SLCHOST_CONF58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf59(&self) -> SLCHOST_CONF59_R {
        SLCHOST_CONF59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W14")
            .field(
                "slchost_conf56",
                &format_args!("{}", self.slchost_conf56().bits()),
            )
            .field(
                "slchost_conf57",
                &format_args!("{}", self.slchost_conf57().bits()),
            )
            .field(
                "slchost_conf58",
                &format_args!("{}", self.slchost_conf58().bits()),
            )
            .field(
                "slchost_conf59",
                &format_args!("{}", self.slchost_conf59().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf56(&mut self) -> SLCHOST_CONF56_W<CONF_W14_SPEC> {
        SLCHOST_CONF56_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf57(&mut self) -> SLCHOST_CONF57_W<CONF_W14_SPEC> {
        SLCHOST_CONF57_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf58(&mut self) -> SLCHOST_CONF58_W<CONF_W14_SPEC> {
        SLCHOST_CONF58_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf59(&mut self) -> SLCHOST_CONF59_W<CONF_W14_SPEC> {
        SLCHOST_CONF59_W::new(self, 24)
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
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W14_SPEC;
impl crate::RegisterSpec for CONF_W14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w14::R`](R) reader structure"]
impl crate::Readable for CONF_W14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w14::W`](W) writer structure"]
impl crate::Writable for CONF_W14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W14 to value 0"]
impl crate::Resettable for CONF_W14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
