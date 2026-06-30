#[doc = "Register `HP_CORE_DBUS_TIMEOUT_CONF` reader"]
pub type R = crate::R<HP_CORE_DBUS_TIMEOUT_CONF_SPEC>;
#[doc = "Register `HP_CORE_DBUS_TIMEOUT_CONF` writer"]
pub type W = crate::W<HP_CORE_DBUS_TIMEOUT_CONF_SPEC>;
#[doc = "Field `HP_CORE_DBUS_TIMEOUT_PROTECT_EN` reader - set this field to 1 to enable hp core0&1 dbus timeout handle"]
pub type HP_CORE_DBUS_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `HP_CORE_DBUS_TIMEOUT_PROTECT_EN` writer - set this field to 1 to enable hp core0&1 dbus timeout handle"]
pub type HP_CORE_DBUS_TIMEOUT_PROTECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE_DBUS_TIMEOUT_THRES` reader - This field used to set hp core0&1 dbus timeout threshold"]
pub type HP_CORE_DBUS_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `HP_CORE_DBUS_TIMEOUT_THRES` writer - This field used to set hp core0&1 dbus timeout threshold"]
pub type HP_CORE_DBUS_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - set this field to 1 to enable hp core0&1 dbus timeout handle"]
    #[inline(always)]
    pub fn hp_core_dbus_timeout_protect_en(&self) -> HP_CORE_DBUS_TIMEOUT_PROTECT_EN_R {
        HP_CORE_DBUS_TIMEOUT_PROTECT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - This field used to set hp core0&1 dbus timeout threshold"]
    #[inline(always)]
    pub fn hp_core_dbus_timeout_thres(&self) -> HP_CORE_DBUS_TIMEOUT_THRES_R {
        HP_CORE_DBUS_TIMEOUT_THRES_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CORE_DBUS_TIMEOUT_CONF")
            .field(
                "hp_core_dbus_timeout_protect_en",
                &self.hp_core_dbus_timeout_protect_en(),
            )
            .field(
                "hp_core_dbus_timeout_thres",
                &self.hp_core_dbus_timeout_thres(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - set this field to 1 to enable hp core0&1 dbus timeout handle"]
    #[inline(always)]
    pub fn hp_core_dbus_timeout_protect_en(
        &mut self,
    ) -> HP_CORE_DBUS_TIMEOUT_PROTECT_EN_W<'_, HP_CORE_DBUS_TIMEOUT_CONF_SPEC> {
        HP_CORE_DBUS_TIMEOUT_PROTECT_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:16 - This field used to set hp core0&1 dbus timeout threshold"]
    #[inline(always)]
    pub fn hp_core_dbus_timeout_thres(
        &mut self,
    ) -> HP_CORE_DBUS_TIMEOUT_THRES_W<'_, HP_CORE_DBUS_TIMEOUT_CONF_SPEC> {
        HP_CORE_DBUS_TIMEOUT_THRES_W::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_dbus_timeout_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_dbus_timeout_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CORE_DBUS_TIMEOUT_CONF_SPEC;
impl crate::RegisterSpec for HP_CORE_DBUS_TIMEOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_core_dbus_timeout_conf::R`](R) reader structure"]
impl crate::Readable for HP_CORE_DBUS_TIMEOUT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_core_dbus_timeout_conf::W`](W) writer structure"]
impl crate::Writable for HP_CORE_DBUS_TIMEOUT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CORE_DBUS_TIMEOUT_CONF to value 0x0001_ffff"]
impl crate::Resettable for HP_CORE_DBUS_TIMEOUT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
