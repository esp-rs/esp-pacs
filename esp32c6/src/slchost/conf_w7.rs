///Register `CONF_W7` reader
pub type R = crate::R<CONF_W7_SPEC>;
///Register `CONF_W7` writer
pub type W = crate::W<CONF_W7_SPEC>;
///Field `SLCHOST_CONF28` reader - *******Description***********
pub type SLCHOST_CONF28_R = crate::FieldReader;
///Field `SLCHOST_CONF28` writer - *******Description***********
pub type SLCHOST_CONF28_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SLCHOST_CONF29` reader - *******Description***********
pub type SLCHOST_CONF29_R = crate::FieldReader;
///Field `SLCHOST_CONF29` writer - *******Description***********
pub type SLCHOST_CONF29_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SLCHOST_CONF30` reader - *******Description***********
pub type SLCHOST_CONF30_R = crate::FieldReader;
///Field `SLCHOST_CONF30` writer - *******Description***********
pub type SLCHOST_CONF30_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SLCHOST_CONF31` reader - *******Description***********
pub type SLCHOST_CONF31_R = crate::FieldReader;
///Field `SLCHOST_CONF31` writer - *******Description***********
pub type SLCHOST_CONF31_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - *******Description***********
    #[inline(always)]
    pub fn slchost_conf28(&self) -> SLCHOST_CONF28_R {
        SLCHOST_CONF28_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - *******Description***********
    #[inline(always)]
    pub fn slchost_conf29(&self) -> SLCHOST_CONF29_R {
        SLCHOST_CONF29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - *******Description***********
    #[inline(always)]
    pub fn slchost_conf30(&self) -> SLCHOST_CONF30_R {
        SLCHOST_CONF30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - *******Description***********
    #[inline(always)]
    pub fn slchost_conf31(&self) -> SLCHOST_CONF31_R {
        SLCHOST_CONF31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W7")
            .field("slchost_conf28", &self.slchost_conf28())
            .field("slchost_conf29", &self.slchost_conf29())
            .field("slchost_conf30", &self.slchost_conf30())
            .field("slchost_conf31", &self.slchost_conf31())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - *******Description***********
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf28(&mut self) -> SLCHOST_CONF28_W<CONF_W7_SPEC> {
        SLCHOST_CONF28_W::new(self, 0)
    }
    ///Bits 8:15 - *******Description***********
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf29(&mut self) -> SLCHOST_CONF29_W<CONF_W7_SPEC> {
        SLCHOST_CONF29_W::new(self, 8)
    }
    ///Bits 16:23 - *******Description***********
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf30(&mut self) -> SLCHOST_CONF30_W<CONF_W7_SPEC> {
        SLCHOST_CONF30_W::new(self, 16)
    }
    ///Bits 24:31 - *******Description***********
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf31(&mut self) -> SLCHOST_CONF31_W<CONF_W7_SPEC> {
        SLCHOST_CONF31_W::new(self, 24)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W7_SPEC;
impl crate::RegisterSpec for CONF_W7_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf_w7::R`](R) reader structure
impl crate::Readable for CONF_W7_SPEC {}
///`write(|w| ..)` method takes [`conf_w7::W`](W) writer structure
impl crate::Writable for CONF_W7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF_W7 to value 0
impl crate::Resettable for CONF_W7_SPEC {
    const RESET_VALUE: u32 = 0;
}
