#[doc = "Register `LP_CORE_DBUS_TIMEOUT` reader"]
pub type R = crate::R<LP_CORE_DBUS_TIMEOUT_SPEC>;
#[doc = "Register `LP_CORE_DBUS_TIMEOUT` writer"]
pub type W = crate::W<LP_CORE_DBUS_TIMEOUT_SPEC>;
#[doc = "Field `EN` reader - set this field to 1 to enable lp core dbus timeout handle"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - set this field to 1 to enable lp core dbus timeout handle"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES` reader - This field used to set lp core dbus timeout threshold"]
pub type THRES_R = crate::FieldReader<u16>;
#[doc = "Field `THRES` writer - This field used to set lp core dbus timeout threshold"]
pub type THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - set this field to 1 to enable lp core dbus timeout handle"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - This field used to set lp core dbus timeout threshold"]
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CORE_DBUS_TIMEOUT")
            .field("en", &self.en())
            .field("thres", &self.thres())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - set this field to 1 to enable lp core dbus timeout handle"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<LP_CORE_DBUS_TIMEOUT_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:16 - This field used to set lp core dbus timeout threshold"]
    #[inline(always)]
    pub fn thres(&mut self) -> THRES_W<LP_CORE_DBUS_TIMEOUT_SPEC> {
        THRES_W::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_dbus_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_dbus_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CORE_DBUS_TIMEOUT_SPEC;
impl crate::RegisterSpec for LP_CORE_DBUS_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_core_dbus_timeout::R`](R) reader structure"]
impl crate::Readable for LP_CORE_DBUS_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_core_dbus_timeout::W`](W) writer structure"]
impl crate::Writable for LP_CORE_DBUS_TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_CORE_DBUS_TIMEOUT to value 0x0001_ffff"]
impl crate::Resettable for LP_CORE_DBUS_TIMEOUT_SPEC {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
