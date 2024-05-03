#[doc = "Register `DHT_INFO` reader"]
pub type R = crate::R<DHT_INFO_SPEC>;
#[doc = "Register `DHT_INFO` writer"]
pub type W = crate::W<DHT_INFO_SPEC>;
#[doc = "Field `DC0_DHT_ID` reader - configure dht dc table 0 id"]
pub type DC0_DHT_ID_R = crate::FieldReader;
#[doc = "Field `DC0_DHT_ID` writer - configure dht dc table 0 id"]
pub type DC0_DHT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DC1_DHT_ID` reader - configure dht dc table 1 id"]
pub type DC1_DHT_ID_R = crate::FieldReader;
#[doc = "Field `DC1_DHT_ID` writer - configure dht dc table 1 id"]
pub type DC1_DHT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AC0_DHT_ID` reader - configure dht ac table 0 id"]
pub type AC0_DHT_ID_R = crate::FieldReader;
#[doc = "Field `AC0_DHT_ID` writer - configure dht ac table 0 id"]
pub type AC0_DHT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AC1_DHT_ID` reader - configure dht ac table 1 id"]
pub type AC1_DHT_ID_R = crate::FieldReader;
#[doc = "Field `AC1_DHT_ID` writer - configure dht ac table 1 id"]
pub type AC1_DHT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - configure dht dc table 0 id"]
    #[inline(always)]
    pub fn dc0_dht_id(&self) -> DC0_DHT_ID_R {
        DC0_DHT_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - configure dht dc table 1 id"]
    #[inline(always)]
    pub fn dc1_dht_id(&self) -> DC1_DHT_ID_R {
        DC1_DHT_ID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - configure dht ac table 0 id"]
    #[inline(always)]
    pub fn ac0_dht_id(&self) -> AC0_DHT_ID_R {
        AC0_DHT_ID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - configure dht ac table 1 id"]
    #[inline(always)]
    pub fn ac1_dht_id(&self) -> AC1_DHT_ID_R {
        AC1_DHT_ID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHT_INFO")
            .field("dc0_dht_id", &self.dc0_dht_id().bits())
            .field("dc1_dht_id", &self.dc1_dht_id().bits())
            .field("ac0_dht_id", &self.ac0_dht_id().bits())
            .field("ac1_dht_id", &self.ac1_dht_id().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DHT_INFO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - configure dht dc table 0 id"]
    #[inline(always)]
    #[must_use]
    pub fn dc0_dht_id(&mut self) -> DC0_DHT_ID_W<DHT_INFO_SPEC> {
        DC0_DHT_ID_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - configure dht dc table 1 id"]
    #[inline(always)]
    #[must_use]
    pub fn dc1_dht_id(&mut self) -> DC1_DHT_ID_W<DHT_INFO_SPEC> {
        DC1_DHT_ID_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - configure dht ac table 0 id"]
    #[inline(always)]
    #[must_use]
    pub fn ac0_dht_id(&mut self) -> AC0_DHT_ID_W<DHT_INFO_SPEC> {
        AC0_DHT_ID_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - configure dht ac table 1 id"]
    #[inline(always)]
    #[must_use]
    pub fn ac1_dht_id(&mut self) -> AC1_DHT_ID_W<DHT_INFO_SPEC> {
        AC1_DHT_ID_W::new(self, 12)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dht_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHT_INFO_SPEC;
impl crate::RegisterSpec for DHT_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_info::R`](R) reader structure"]
impl crate::Readable for DHT_INFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dht_info::W`](W) writer structure"]
impl crate::Writable for DHT_INFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHT_INFO to value 0x1010"]
impl crate::Resettable for DHT_INFO_SPEC {
    const RESET_VALUE: u32 = 0x1010;
}
