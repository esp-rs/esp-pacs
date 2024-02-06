#[doc = "Register `RS485_CONF_SYNC` reader"]
pub type R = crate::R<RS485_CONF_SYNC_SPEC>;
#[doc = "Register `RS485_CONF_SYNC` writer"]
pub type W = crate::W<RS485_CONF_SYNC_SPEC>;
#[doc = "Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_R = crate::BitReader;
#[doc = "Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_R = crate::BitReader;
#[doc = "Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&self) -> DL0_EN_R {
        DL0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&self) -> DL1_EN_R {
        DL1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RS485_CONF_SYNC")
            .field("dl0_en", &format_args!("{}", self.dl0_en().bit()))
            .field("dl1_en", &format_args!("{}", self.dl1_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RS485_CONF_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn dl0_en(&mut self) -> DL0_EN_W<RS485_CONF_SYNC_SPEC> {
        DL0_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn dl1_en(&mut self) -> DL1_EN_W<RS485_CONF_SYNC_SPEC> {
        DL1_EN_W::new(self, 2)
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
#[doc = "RS485 mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485_conf_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_conf_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RS485_CONF_SYNC_SPEC;
impl crate::RegisterSpec for RS485_CONF_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485_conf_sync::R`](R) reader structure"]
impl crate::Readable for RS485_CONF_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rs485_conf_sync::W`](W) writer structure"]
impl crate::Writable for RS485_CONF_SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RS485_CONF_SYNC to value 0"]
impl crate::Resettable for RS485_CONF_SYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
