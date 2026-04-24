#[doc = "Register `ICACHE2_PRELOCK_CONF` reader"]
pub type R = crate::R<ICACHE2_PRELOCK_CONF_SPEC>;
#[doc = "Register `ICACHE2_PRELOCK_CONF` writer"]
pub type W = crate::W<ICACHE2_PRELOCK_CONF_SPEC>;
#[doc = "Field `ICACHE2_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on ICache2."]
pub type ICACHE2_PRELOCK_SCT0_EN_R = crate::BitReader;
#[doc = "Field `ICACHE2_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function on ICache2."]
pub type ICACHE2_PRELOCK_SCT0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE2_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on ICache2."]
pub type ICACHE2_PRELOCK_SCT1_EN_R = crate::BitReader;
#[doc = "Field `ICACHE2_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function on ICache2."]
pub type ICACHE2_PRELOCK_SCT1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE2_PRELOCK_RGID` reader - The bit is used to set the gid of icache2 prelock."]
pub type ICACHE2_PRELOCK_RGID_R = crate::FieldReader;
#[doc = "Field `ICACHE2_PRELOCK_RGID` writer - The bit is used to set the gid of icache2 prelock."]
pub type ICACHE2_PRELOCK_RGID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on ICache2."]
    #[inline(always)]
    pub fn icache2_prelock_sct0_en(&self) -> ICACHE2_PRELOCK_SCT0_EN_R {
        ICACHE2_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on ICache2."]
    #[inline(always)]
    pub fn icache2_prelock_sct1_en(&self) -> ICACHE2_PRELOCK_SCT1_EN_R {
        ICACHE2_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of icache2 prelock."]
    #[inline(always)]
    pub fn icache2_prelock_rgid(&self) -> ICACHE2_PRELOCK_RGID_R {
        ICACHE2_PRELOCK_RGID_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_PRELOCK_CONF")
            .field("icache2_prelock_sct0_en", &self.icache2_prelock_sct0_en())
            .field("icache2_prelock_sct1_en", &self.icache2_prelock_sct1_en())
            .field("icache2_prelock_rgid", &self.icache2_prelock_rgid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on ICache2."]
    #[inline(always)]
    pub fn icache2_prelock_sct0_en(
        &mut self,
    ) -> ICACHE2_PRELOCK_SCT0_EN_W<'_, ICACHE2_PRELOCK_CONF_SPEC> {
        ICACHE2_PRELOCK_SCT0_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on ICache2."]
    #[inline(always)]
    pub fn icache2_prelock_sct1_en(
        &mut self,
    ) -> ICACHE2_PRELOCK_SCT1_EN_W<'_, ICACHE2_PRELOCK_CONF_SPEC> {
        ICACHE2_PRELOCK_SCT1_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of icache2 prelock."]
    #[inline(always)]
    pub fn icache2_prelock_rgid(
        &mut self,
    ) -> ICACHE2_PRELOCK_RGID_W<'_, ICACHE2_PRELOCK_CONF_SPEC> {
        ICACHE2_PRELOCK_RGID_W::new(self, 2)
    }
}
#[doc = "Instruction Cache 2 prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_prelock_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_PRELOCK_CONF_SPEC;
impl crate::RegisterSpec for ICACHE2_PRELOCK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_prelock_conf::R`](R) reader structure"]
impl crate::Readable for ICACHE2_PRELOCK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache2_prelock_conf::W`](W) writer structure"]
impl crate::Writable for ICACHE2_PRELOCK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICACHE2_PRELOCK_CONF to value 0"]
impl crate::Resettable for ICACHE2_PRELOCK_CONF_SPEC {}
