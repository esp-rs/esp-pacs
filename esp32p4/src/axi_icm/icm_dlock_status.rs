#[doc = "Register `ICM_DLOCK_STATUS` reader"]
pub type R = crate::R<ICM_DLOCK_STATUS_SPEC>;
#[doc = "Field `ICM_REG_DLOCK_MST` reader - "]
pub type ICM_REG_DLOCK_MST_R = crate::FieldReader;
#[doc = "Field `ICM_REG_DLOCK_SLV` reader - "]
pub type ICM_REG_DLOCK_SLV_R = crate::FieldReader;
#[doc = "Field `ICM_REG_DLOCK_ID` reader - "]
pub type ICM_REG_DLOCK_ID_R = crate::FieldReader;
#[doc = "Field `ICM_REG_DLOCK_WR` reader - "]
pub type ICM_REG_DLOCK_WR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn icm_reg_dlock_mst(&self) -> ICM_REG_DLOCK_MST_R {
        ICM_REG_DLOCK_MST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn icm_reg_dlock_slv(&self) -> ICM_REG_DLOCK_SLV_R {
        ICM_REG_DLOCK_SLV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn icm_reg_dlock_id(&self) -> ICM_REG_DLOCK_ID_R {
        ICM_REG_DLOCK_ID_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn icm_reg_dlock_wr(&self) -> ICM_REG_DLOCK_WR_R {
        ICM_REG_DLOCK_WR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_DLOCK_STATUS")
            .field("icm_reg_dlock_mst", &self.icm_reg_dlock_mst())
            .field("icm_reg_dlock_slv", &self.icm_reg_dlock_slv())
            .field("icm_reg_dlock_id", &self.icm_reg_dlock_id())
            .field("icm_reg_dlock_wr", &self.icm_reg_dlock_wr())
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
