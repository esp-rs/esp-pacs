#[doc = "Register `BTVERSION` reader"]
pub type R = crate::R<BTVERSION_SPEC>;
#[doc = "Register `BTVERSION` writer"]
pub type W = crate::W<BTVERSION_SPEC>;
#[doc = "Field `BUILD` reader - BR/EDR Core Build number"]
pub type BUILD_R = crate::FieldReader;
#[doc = "Field `BUILD` writer - BR/EDR Core Build number"]
pub type BUILD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UPG` reader - BR/EDR Core Upgrade number"]
pub type UPG_R = crate::FieldReader;
#[doc = "Field `UPG` writer - BR/EDR Core Upgrade number"]
pub type UPG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REL` reader - BR/EDR Core version Major release number"]
pub type REL_R = crate::FieldReader;
#[doc = "Field `REL` writer - BR/EDR Core version Major release number"]
pub type REL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TYP` reader - BR/EDR Core Type (This corresponds to Bluetooth Assigned Numbers document, Core specification versions section)"]
pub type TYP_R = crate::FieldReader;
#[doc = "Field `TYP` writer - BR/EDR Core Type (This corresponds to Bluetooth Assigned Numbers document, Core specification versions section)"]
pub type TYP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - BR/EDR Core Build number"]
    #[inline(always)]
    pub fn build(&self) -> BUILD_R {
        BUILD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BR/EDR Core Upgrade number"]
    #[inline(always)]
    pub fn upg(&self) -> UPG_R {
        UPG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - BR/EDR Core version Major release number"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - BR/EDR Core Type (This corresponds to Bluetooth Assigned Numbers document, Core specification versions section)"]
    #[inline(always)]
    pub fn typ(&self) -> TYP_R {
        TYP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTVERSION")
            .field("build", &self.build())
            .field("upg", &self.upg())
            .field("rel", &self.rel())
            .field("typ", &self.typ())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - BR/EDR Core Build number"]
    #[inline(always)]
    pub fn build(&mut self) -> BUILD_W<'_, BTVERSION_SPEC> {
        BUILD_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - BR/EDR Core Upgrade number"]
    #[inline(always)]
    pub fn upg(&mut self) -> UPG_W<'_, BTVERSION_SPEC> {
        UPG_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - BR/EDR Core version Major release number"]
    #[inline(always)]
    pub fn rel(&mut self) -> REL_W<'_, BTVERSION_SPEC> {
        REL_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - BR/EDR Core Type (This corresponds to Bluetooth Assigned Numbers document, Core specification versions section)"]
    #[inline(always)]
    pub fn typ(&mut self) -> TYP_W<'_, BTVERSION_SPEC> {
        TYP_W::new(self, 24)
    }
}
#[doc = "BR/EDR peripheral version\n\nYou can [`read`](crate::Reg::read) this register and get [`btversion::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btversion::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTVERSION_SPEC;
impl crate::RegisterSpec for BTVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btversion::R`](R) reader structure"]
impl crate::Readable for BTVERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btversion::W`](W) writer structure"]
impl crate::Writable for BTVERSION_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTVERSION to value 0"]
impl crate::Resettable for BTVERSION_SPEC {}
