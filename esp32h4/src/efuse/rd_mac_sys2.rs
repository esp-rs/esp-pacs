#[doc = "Register `RD_MAC_SYS2` reader"]
pub type R = crate::R<RD_MAC_SYS2_SPEC>;
#[doc = "Field `PVT_LIMIT` reader - Power glitch monitor threthold."]
pub type PVT_LIMIT_R = crate::FieldReader<u16>;
#[doc = "Field `PVT_CELL_SELECT` reader - Power glitch monitor PVT cell select."]
pub type PVT_CELL_SELECT_R = crate::FieldReader;
#[doc = "Field `PVT_PUMP_LIMIT` reader - Use to configure voltage monitor limit for charge pump"]
pub type PVT_PUMP_LIMIT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Power glitch monitor threthold."]
    #[inline(always)]
    pub fn pvt_limit(&self) -> PVT_LIMIT_R {
        PVT_LIMIT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Power glitch monitor PVT cell select."]
    #[inline(always)]
    pub fn pvt_cell_select(&self) -> PVT_CELL_SELECT_R {
        PVT_CELL_SELECT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:30 - Use to configure voltage monitor limit for charge pump"]
    #[inline(always)]
    pub fn pvt_pump_limit(&self) -> PVT_PUMP_LIMIT_R {
        PVT_PUMP_LIMIT_R::new(((self.bits >> 23) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS2")
            .field("pvt_limit", &self.pvt_limit())
            .field("pvt_cell_select", &self.pvt_cell_select())
            .field("pvt_pump_limit", &self.pvt_pump_limit())
            .finish()
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS2_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys2::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS2_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SYS2 to value 0"]
impl crate::Resettable for RD_MAC_SYS2_SPEC {}
