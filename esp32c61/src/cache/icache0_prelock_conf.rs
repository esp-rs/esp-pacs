#[doc = "Register `ICACHE0_PRELOCK_CONF` reader"]
pub type R = crate::R<ICACHE0_PRELOCK_CONF_SPEC>;
#[doc = "Field `ICACHE0_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on L1-ICache0."]
pub type ICACHE0_PRELOCK_SCT0_EN_R = crate::BitReader;
#[doc = "Field `ICACHE0_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on L1-ICache0."]
pub type ICACHE0_PRELOCK_SCT1_EN_R = crate::BitReader;
#[doc = "Field `ICACHE0_PRELOCK_RGID` reader - The bit is used to set the gid of l1 icache0 prelock."]
pub type ICACHE0_PRELOCK_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-ICache0."]
    #[inline(always)]
    pub fn icache0_prelock_sct0_en(&self) -> ICACHE0_PRELOCK_SCT0_EN_R {
        ICACHE0_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-ICache0."]
    #[inline(always)]
    pub fn icache0_prelock_sct1_en(&self) -> ICACHE0_PRELOCK_SCT1_EN_R {
        ICACHE0_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l1 icache0 prelock."]
    #[inline(always)]
    pub fn icache0_prelock_rgid(&self) -> ICACHE0_PRELOCK_RGID_R {
        ICACHE0_PRELOCK_RGID_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE0_PRELOCK_CONF")
            .field("icache0_prelock_sct0_en", &self.icache0_prelock_sct0_en())
            .field("icache0_prelock_sct1_en", &self.icache0_prelock_sct1_en())
            .field("icache0_prelock_rgid", &self.icache0_prelock_rgid())
            .finish()
    }
}
#[doc = "L1 instruction Cache 0 prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_prelock_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE0_PRELOCK_CONF_SPEC;
impl crate::RegisterSpec for ICACHE0_PRELOCK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache0_prelock_conf::R`](R) reader structure"]
impl crate::Readable for ICACHE0_PRELOCK_CONF_SPEC {}
#[doc = "`reset()` method sets ICACHE0_PRELOCK_CONF to value 0"]
impl crate::Resettable for ICACHE0_PRELOCK_CONF_SPEC {}
