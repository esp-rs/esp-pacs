#[doc = "Register `ICM_DLOCK_STATUS` reader"]
pub type R = crate::R<ICM_DLOCK_STATUS_SPEC>;
#[doc = "Field `DLOCK_MST` reader - "]
pub type DLOCK_MST_R = crate::FieldReader;
#[doc = "Field `DLOCK_SLV` reader - "]
pub type DLOCK_SLV_R = crate::FieldReader;
#[doc = "Field `DLOCK_ID` reader - "]
pub type DLOCK_ID_R = crate::FieldReader;
#[doc = "Field `DLOCK_WR` reader - "]
pub type DLOCK_WR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dlock_mst(&self) -> DLOCK_MST_R {
        DLOCK_MST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dlock_slv(&self) -> DLOCK_SLV_R {
        DLOCK_SLV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn dlock_id(&self) -> DLOCK_ID_R {
        DLOCK_ID_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dlock_wr(&self) -> DLOCK_WR_R {
        DLOCK_WR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_DLOCK_STATUS")
            .field("dlock_mst", &self.dlock_mst())
            .field("dlock_slv", &self.dlock_slv())
            .field("dlock_id", &self.dlock_id())
            .field("dlock_wr", &self.dlock_wr())
            .finish()
    }
}
#[doc = "Deadlock status\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_dlock_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_DLOCK_STATUS_SPEC;
impl crate::RegisterSpec for ICM_DLOCK_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_dlock_status::R`](R) reader structure"]
impl crate::Readable for ICM_DLOCK_STATUS_SPEC {}
#[doc = "`reset()` method sets ICM_DLOCK_STATUS to value 0"]
impl crate::Resettable for ICM_DLOCK_STATUS_SPEC {}
