#[doc = "Register `GRXSTSR` reader"]
pub type R = crate::R<GRXSTSR_SPEC>;
#[doc = "Field `G_CHNUM` reader - "]
pub type G_CHNUM_R = crate::FieldReader;
#[doc = "Field `G_BCNT` reader - "]
pub type G_BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `G_DPID` reader - "]
pub type G_DPID_R = crate::FieldReader;
#[doc = "Field `G_PKTSTS` reader - "]
pub type G_PKTSTS_R = crate::FieldReader;
#[doc = "Field `G_FN` reader - "]
pub type G_FN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn g_chnum(&self) -> G_CHNUM_R {
        G_CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14"]
    #[inline(always)]
    pub fn g_bcnt(&self) -> G_BCNT_R {
        G_BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn g_dpid(&self) -> G_DPID_R {
        G_DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20"]
    #[inline(always)]
    pub fn g_pktsts(&self) -> G_PKTSTS_R {
        G_PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn g_fn(&self) -> G_FN_R {
        G_FN_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSR")
            .field("g_chnum", &self.g_chnum())
            .field("g_bcnt", &self.g_bcnt())
            .field("g_dpid", &self.g_dpid())
            .field("g_pktsts", &self.g_pktsts())
            .field("g_fn", &self.g_fn())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXSTSR_SPEC;
impl crate::RegisterSpec for GRXSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr::R`](R) reader structure"]
impl crate::Readable for GRXSTSR_SPEC {}
#[doc = "`reset()` method sets GRXSTSR to value 0"]
impl crate::Resettable for GRXSTSR_SPEC {}
