#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `STATUS` reader - The HUK generation status. 0: HUK is not generated. 1: HUK is generated and valid. 2: HUK is generated but invalid. 3: reserved."]
pub type STATUS_R = crate::FieldReader;
#[doc = "Field `RISK_LEVEL` reader - The risk level of HUK. 0-6: the higher the risk level is, the more error bits there are in the PUF SRAM. 7: Error Level, HUK is invalid."]
pub type RISK_LEVEL_R = crate::FieldReader;
#[doc = "Field `UPDATE_REQ` reader - The update request of HUK info. 0: User can update HUK info according to the risk level. 1: The HUK info is expired, and user need to update it."]
pub type UPDATE_REQ_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - The HUK generation status. 0: HUK is not generated. 1: HUK is generated and valid. 2: HUK is generated but invalid. 3: reserved."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - The risk level of HUK. 0-6: the higher the risk level is, the more error bits there are in the PUF SRAM. 7: Error Level, HUK is invalid."]
    #[inline(always)]
    pub fn risk_level(&self) -> RISK_LEVEL_R {
        RISK_LEVEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - The update request of HUK info. 0: User can update HUK info according to the risk level. 1: The HUK info is expired, and user need to update it."]
    #[inline(always)]
    pub fn update_req(&self) -> UPDATE_REQ_R {
        UPDATE_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("status", &self.status())
            .field("risk_level", &self.risk_level())
            .field("update_req", &self.update_req())
            .finish()
    }
}
#[doc = "HUK Generator HUK status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {}
