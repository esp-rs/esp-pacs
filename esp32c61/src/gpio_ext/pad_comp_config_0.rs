#[doc = "Register `PAD_COMP_CONFIG_0` reader"]
pub type R = crate::R<PAD_COMP_CONFIG_0_SPEC>;
#[doc = "Register `PAD_COMP_CONFIG_0` writer"]
pub type W = crate::W<PAD_COMP_CONFIG_0_SPEC>;
#[doc = "Field `XPD_COMP_0` reader - Pad compare enable bit."]
pub type XPD_COMP_0_R = crate::BitReader;
#[doc = "Field `XPD_COMP_0` writer - Pad compare enable bit."]
pub type XPD_COMP_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE_COMP_0` reader - 1 to enable external reference from PAD\\[x\\]. 0 to enable internal reference, meanwhile PAD\\[x\\] can be used as a regular GPIO."]
pub type MODE_COMP_0_R = crate::BitReader;
#[doc = "Field `MODE_COMP_0` writer - 1 to enable external reference from PAD\\[x\\]. 0 to enable internal reference, meanwhile PAD\\[x\\] can be used as a regular GPIO."]
pub type MODE_COMP_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREF_COMP_0` reader - internal reference voltage tuning bit. 0V to 0.7*VDDPST step 0.1*VDDPST."]
pub type DREF_COMP_0_R = crate::FieldReader;
#[doc = "Field `DREF_COMP_0` writer - internal reference voltage tuning bit. 0V to 0.7*VDDPST step 0.1*VDDPST."]
pub type DREF_COMP_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Pad compare enable bit."]
    #[inline(always)]
    pub fn xpd_comp_0(&self) -> XPD_COMP_0_R {
        XPD_COMP_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 to enable external reference from PAD\\[x\\]. 0 to enable internal reference, meanwhile PAD\\[x\\] can be used as a regular GPIO."]
    #[inline(always)]
    pub fn mode_comp_0(&self) -> MODE_COMP_0_R {
        MODE_COMP_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - internal reference voltage tuning bit. 0V to 0.7*VDDPST step 0.1*VDDPST."]
    #[inline(always)]
    pub fn dref_comp_0(&self) -> DREF_COMP_0_R {
        DREF_COMP_0_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_COMP_CONFIG_0")
            .field("xpd_comp_0", &self.xpd_comp_0())
            .field("mode_comp_0", &self.mode_comp_0())
            .field("dref_comp_0", &self.dref_comp_0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Pad compare enable bit."]
    #[inline(always)]
    pub fn xpd_comp_0(&mut self) -> XPD_COMP_0_W<PAD_COMP_CONFIG_0_SPEC> {
        XPD_COMP_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1 to enable external reference from PAD\\[x\\]. 0 to enable internal reference, meanwhile PAD\\[x\\] can be used as a regular GPIO."]
    #[inline(always)]
    pub fn mode_comp_0(&mut self) -> MODE_COMP_0_W<PAD_COMP_CONFIG_0_SPEC> {
        MODE_COMP_0_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - internal reference voltage tuning bit. 0V to 0.7*VDDPST step 0.1*VDDPST."]
    #[inline(always)]
    pub fn dref_comp_0(&mut self) -> DREF_COMP_0_W<PAD_COMP_CONFIG_0_SPEC> {
        DREF_COMP_0_W::new(self, 2)
    }
}
#[doc = "PAD Compare configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_COMP_CONFIG_0_SPEC;
impl crate::RegisterSpec for PAD_COMP_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_comp_config_0::R`](R) reader structure"]
impl crate::Readable for PAD_COMP_CONFIG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_comp_config_0::W`](W) writer structure"]
impl crate::Writable for PAD_COMP_CONFIG_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_COMP_CONFIG_0 to value 0"]
impl crate::Resettable for PAD_COMP_CONFIG_0_SPEC {}
