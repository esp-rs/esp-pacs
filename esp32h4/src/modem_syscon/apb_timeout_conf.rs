#[doc = "Register `APB_TIMEOUT_CONF` reader"]
pub type R = crate::R<APB_TIMEOUT_CONF_SPEC>;
#[doc = "Register `APB_TIMEOUT_CONF` writer"]
pub type W = crate::W<APB_TIMEOUT_CONF_SPEC>;
#[doc = "Field `APB_TIMEOUT_THRES` reader - "]
pub type APB_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `APB_TIMEOUT_THRES` writer - "]
pub type APB_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `APB_TIMEOUT_PROTECT_EN` reader - "]
pub type APB_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `APB_TIMEOUT_PROTECT_EN` writer - "]
pub type APB_TIMEOUT_PROTECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_TIMEOUT_INT_CLR` writer - "]
pub type APB_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_APB_TIMEOUT_EXCEPTION_PID` reader - "]
pub type MODEM_APB_TIMEOUT_EXCEPTION_PID_R = crate::FieldReader;
#[doc = "Field `MODEM_APB_TIMEOUT_INT` reader - "]
pub type MODEM_APB_TIMEOUT_INT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn apb_timeout_thres(&self) -> APB_TIMEOUT_THRES_R {
        APB_TIMEOUT_THRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn apb_timeout_protect_en(&self) -> APB_TIMEOUT_PROTECT_EN_R {
        APB_TIMEOUT_PROTECT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn modem_apb_timeout_exception_pid(&self) -> MODEM_APB_TIMEOUT_EXCEPTION_PID_R {
        MODEM_APB_TIMEOUT_EXCEPTION_PID_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn modem_apb_timeout_int(&self) -> MODEM_APB_TIMEOUT_INT_R {
        MODEM_APB_TIMEOUT_INT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_TIMEOUT_CONF")
            .field("apb_timeout_thres", &self.apb_timeout_thres())
            .field("apb_timeout_protect_en", &self.apb_timeout_protect_en())
            .field(
                "modem_apb_timeout_exception_pid",
                &self.modem_apb_timeout_exception_pid(),
            )
            .field("modem_apb_timeout_int", &self.modem_apb_timeout_int())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn apb_timeout_thres(&mut self) -> APB_TIMEOUT_THRES_W<'_, APB_TIMEOUT_CONF_SPEC> {
        APB_TIMEOUT_THRES_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn apb_timeout_protect_en(
        &mut self,
    ) -> APB_TIMEOUT_PROTECT_EN_W<'_, APB_TIMEOUT_CONF_SPEC> {
        APB_TIMEOUT_PROTECT_EN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn apb_timeout_int_clr(&mut self) -> APB_TIMEOUT_INT_CLR_W<'_, APB_TIMEOUT_CONF_SPEC> {
        APB_TIMEOUT_INT_CLR_W::new(self, 17)
    }
}
#[doc = "APB_TIMEOUT_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_timeout_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_timeout_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_TIMEOUT_CONF_SPEC;
impl crate::RegisterSpec for APB_TIMEOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_timeout_conf::R`](R) reader structure"]
impl crate::Readable for APB_TIMEOUT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_timeout_conf::W`](W) writer structure"]
impl crate::Writable for APB_TIMEOUT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB_TIMEOUT_CONF to value 0x0001_ffff"]
impl crate::Resettable for APB_TIMEOUT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
