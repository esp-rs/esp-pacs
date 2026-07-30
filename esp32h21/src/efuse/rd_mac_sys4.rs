#[doc = "Register `RD_MAC_SYS4` reader"]
pub type R = crate::R<RD_MAC_SYS4_SPEC>;
#[doc = "Field `PVT_LIMIT` reader - Represents the threshold of power glitch monitor.\\\\"]
pub type PVT_LIMIT_R = crate::FieldReader<u16>;
#[doc = "Field `PVT_GLITCH_CHARGE_RESET` reader - Represents whether to trigger reset or charge pump when PVT power glitch happened.\\\\ 1:Trigger charge pump. \\\\ 0:Trigger reset\\\\"]
pub type PVT_GLITCH_CHARGE_RESET_R = crate::BitReader;
#[doc = "Field `PVT_GLITCH_MODE` reader - Represents the configuration of glitch mode.\\\\"]
pub type PVT_GLITCH_MODE_R = crate::FieldReader;
#[doc = "Field `PVT_PUMP_LIMIT` reader - Represents the configuration voltage monitor limit for charge pump.\\\\"]
pub type PVT_PUMP_LIMIT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 5:20 - Represents the threshold of power glitch monitor.\\\\"]
    #[inline(always)]
    pub fn pvt_limit(&self) -> PVT_LIMIT_R {
        PVT_LIMIT_R::new(((self.bits >> 5) & 0xffff) as u16)
    }
    #[doc = "Bit 21 - Represents whether to trigger reset or charge pump when PVT power glitch happened.\\\\ 1:Trigger charge pump. \\\\ 0:Trigger reset\\\\"]
    #[inline(always)]
    pub fn pvt_glitch_charge_reset(&self) -> PVT_GLITCH_CHARGE_RESET_R {
        PVT_GLITCH_CHARGE_RESET_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Represents the configuration of glitch mode.\\\\"]
    #[inline(always)]
    pub fn pvt_glitch_mode(&self) -> PVT_GLITCH_MODE_R {
        PVT_GLITCH_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Represents the configuration voltage monitor limit for charge pump.\\\\"]
    #[inline(always)]
    pub fn pvt_pump_limit(&self) -> PVT_PUMP_LIMIT_R {
        PVT_PUMP_LIMIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS4")
            .field("pvt_limit", &self.pvt_limit())
            .field("pvt_glitch_charge_reset", &self.pvt_glitch_charge_reset())
            .field("pvt_glitch_mode", &self.pvt_glitch_mode())
            .field("pvt_pump_limit", &self.pvt_pump_limit())
            .finish()
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS4_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys4::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS4_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SYS4 to value 0"]
impl crate::Resettable for RD_MAC_SYS4_SPEC {}
