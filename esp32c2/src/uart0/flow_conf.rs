#[doc = "Register `FLOW_CONF` reader"]
pub type R = crate::R<FLOW_CONF_SPEC>;
#[doc = "Register `FLOW_CONF` writer"]
pub type W = crate::W<FLOW_CONF_SPEC>;
#[doc = "Field `SW_FLOW_CON_EN` reader - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
pub type SW_FLOW_CON_EN_R = crate::BitReader;
#[doc = "Field `SW_FLOW_CON_EN` writer - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
pub type SW_FLOW_CON_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XONOFF_DEL` reader - Set this bit to remove flow control char from the received data."]
pub type XONOFF_DEL_R = crate::BitReader;
#[doc = "Field `XONOFF_DEL` writer - Set this bit to remove flow control char from the received data."]
pub type XONOFF_DEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_XON` reader - Set this bit to enable the transmitter to go on sending data."]
pub type FORCE_XON_R = crate::BitReader;
#[doc = "Field `FORCE_XON` writer - Set this bit to enable the transmitter to go on sending data."]
pub type FORCE_XON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_XOFF` reader - Set this bit to stop the transmitter from sending data."]
pub type FORCE_XOFF_R = crate::BitReader;
#[doc = "Field `FORCE_XOFF` writer - Set this bit to stop the transmitter from sending data."]
pub type FORCE_XOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_XON` reader - Set this bit to send Xon char. It is cleared by hardware automatically."]
pub type SEND_XON_R = crate::BitReader;
#[doc = "Field `SEND_XON` writer - Set this bit to send Xon char. It is cleared by hardware automatically."]
pub type SEND_XON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_XOFF` reader - Set this bit to send Xoff char. It is cleared by hardware automatically."]
pub type SEND_XOFF_R = crate::BitReader;
#[doc = "Field `SEND_XOFF` writer - Set this bit to send Xoff char. It is cleared by hardware automatically."]
pub type SEND_XOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
    #[inline(always)]
    pub fn sw_flow_con_en(&self) -> SW_FLOW_CON_EN_R {
        SW_FLOW_CON_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    pub fn xonoff_del(&self) -> XONOFF_DEL_R {
        XONOFF_DEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable the transmitter to go on sending data."]
    #[inline(always)]
    pub fn force_xon(&self) -> FORCE_XON_R {
        FORCE_XON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to stop the transmitter from sending data."]
    #[inline(always)]
    pub fn force_xoff(&self) -> FORCE_XOFF_R {
        FORCE_XOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to send Xon char. It is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xon(&self) -> SEND_XON_R {
        SEND_XON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to send Xoff char. It is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xoff(&self) -> SEND_XOFF_R {
        SEND_XOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLOW_CONF")
            .field(
                "sw_flow_con_en",
                &format_args!("{}", self.sw_flow_con_en().bit()),
            )
            .field("xonoff_del", &format_args!("{}", self.xonoff_del().bit()))
            .field("force_xon", &format_args!("{}", self.force_xon().bit()))
            .field("force_xoff", &format_args!("{}", self.force_xoff().bit()))
            .field("send_xon", &format_args!("{}", self.send_xon().bit()))
            .field("send_xoff", &format_args!("{}", self.send_xoff().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FLOW_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
    #[inline(always)]
    #[must_use]
    pub fn sw_flow_con_en(&mut self) -> SW_FLOW_CON_EN_W<FLOW_CONF_SPEC> {
        SW_FLOW_CON_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    #[must_use]
    pub fn xonoff_del(&mut self) -> XONOFF_DEL_W<FLOW_CONF_SPEC> {
        XONOFF_DEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable the transmitter to go on sending data."]
    #[inline(always)]
    #[must_use]
    pub fn force_xon(&mut self) -> FORCE_XON_W<FLOW_CONF_SPEC> {
        FORCE_XON_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to stop the transmitter from sending data."]
    #[inline(always)]
    #[must_use]
    pub fn force_xoff(&mut self) -> FORCE_XOFF_W<FLOW_CONF_SPEC> {
        FORCE_XOFF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to send Xon char. It is cleared by hardware automatically."]
    #[inline(always)]
    #[must_use]
    pub fn send_xon(&mut self) -> SEND_XON_W<FLOW_CONF_SPEC> {
        SEND_XON_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to send Xoff char. It is cleared by hardware automatically."]
    #[inline(always)]
    #[must_use]
    pub fn send_xoff(&mut self) -> SEND_XOFF_W<FLOW_CONF_SPEC> {
        SEND_XOFF_W::new(self, 5)
    }
}
#[doc = "Software flow-control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLOW_CONF_SPEC;
impl crate::RegisterSpec for FLOW_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flow_conf::R`](R) reader structure"]
impl crate::Readable for FLOW_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flow_conf::W`](W) writer structure"]
impl crate::Writable for FLOW_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLOW_CONF to value 0"]
impl crate::Resettable for FLOW_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
