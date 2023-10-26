#[doc = "Register `CONF_W3` reader"]
pub type R = crate::R<CONF_W3_SPEC>;
#[doc = "Register `CONF_W3` writer"]
pub type W = crate::W<CONF_W3_SPEC>;
#[doc = "Field `SLCHOST_CONF12` reader - *******Description***********"]
pub type SLCHOST_CONF12_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF12` writer - *******Description***********"]
pub type SLCHOST_CONF12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF13` reader - *******Description***********"]
pub type SLCHOST_CONF13_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF13` writer - *******Description***********"]
pub type SLCHOST_CONF13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF14` reader - *******Description***********"]
pub type SLCHOST_CONF14_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF14` writer - *******Description***********"]
pub type SLCHOST_CONF14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF15` reader - *******Description***********"]
pub type SLCHOST_CONF15_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF15` writer - *******Description***********"]
pub type SLCHOST_CONF15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf12(&self) -> SLCHOST_CONF12_R {
        SLCHOST_CONF12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf13(&self) -> SLCHOST_CONF13_R {
        SLCHOST_CONF13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf14(&self) -> SLCHOST_CONF14_R {
        SLCHOST_CONF14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf15(&self) -> SLCHOST_CONF15_R {
        SLCHOST_CONF15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W3")
            .field(
                "slchost_conf12",
                &format_args!("{}", self.slchost_conf12().bits()),
            )
            .field(
                "slchost_conf13",
                &format_args!("{}", self.slchost_conf13().bits()),
            )
            .field(
                "slchost_conf14",
                &format_args!("{}", self.slchost_conf14().bits()),
            )
            .field(
                "slchost_conf15",
                &format_args!("{}", self.slchost_conf15().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf12(&mut self) -> SLCHOST_CONF12_W<CONF_W3_SPEC, 0> {
        SLCHOST_CONF12_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf13(&mut self) -> SLCHOST_CONF13_W<CONF_W3_SPEC, 8> {
        SLCHOST_CONF13_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf14(&mut self) -> SLCHOST_CONF14_W<CONF_W3_SPEC, 16> {
        SLCHOST_CONF14_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf15(&mut self) -> SLCHOST_CONF15_W<CONF_W3_SPEC, 24> {
        SLCHOST_CONF15_W::new(self)
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
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W3_SPEC;
impl crate::RegisterSpec for CONF_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w3::R`](R) reader structure"]
impl crate::Readable for CONF_W3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w3::W`](W) writer structure"]
impl crate::Writable for CONF_W3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W3 to value 0xc0"]
impl crate::Resettable for CONF_W3_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
