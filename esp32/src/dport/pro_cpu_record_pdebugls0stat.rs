#[doc = "Register `PRO_CPU_RECORD_PDEBUGLS0STAT` reader"]
pub type R = crate::R<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>;
#[doc = "Register `PRO_CPU_RECORD_PDEBUGLS0STAT` writer"]
pub type W = crate::W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>;
#[doc = "Field `RECORD_PRO_PDEBUGLS0STAT` reader - "]
pub type RECORD_PRO_PDEBUGLS0STAT_R = crate::FieldReader<u32>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TYPE` reader - "]
pub type RECORD_PDEBUGLS0STAT_TYPE_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TYPE` writer - "]
pub type RECORD_PDEBUGLS0STAT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_SZ` reader - "]
pub type RECORD_PDEBUGLS0STAT_SZ_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_SZ` writer - "]
pub type RECORD_PDEBUGLS0STAT_SZ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DTLBM` reader - "]
pub type RECORD_PDEBUGLS0STAT_DTLBM_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DTLBM` writer - "]
pub type RECORD_PDEBUGLS0STAT_DTLBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DCM` reader - "]
pub type RECORD_PDEBUGLS0STAT_DCM_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DCM` writer - "]
pub type RECORD_PDEBUGLS0STAT_DCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DCH` reader - "]
pub type RECORD_PDEBUGLS0STAT_DCH_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DCH` writer - "]
pub type RECORD_PDEBUGLS0STAT_DCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_UC` reader - "]
pub type RECORD_PDEBUGLS0STAT_UC_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_UC` writer - "]
pub type RECORD_PDEBUGLS0STAT_UC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_WB` reader - "]
pub type RECORD_PDEBUGLS0STAT_WB_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_WB` writer - "]
pub type RECORD_PDEBUGLS0STAT_WB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_COH` reader - "]
pub type RECORD_PDEBUGLS0STAT_COH_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_COH` writer - "]
pub type RECORD_PDEBUGLS0STAT_COH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_STCOH` reader - "]
pub type RECORD_PDEBUGLS0STAT_STCOH_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_STCOH` writer - "]
pub type RECORD_PDEBUGLS0STAT_STCOH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TGT` reader - "]
pub type RECORD_PDEBUGLS0STAT_TGT_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TGT` writer - "]
pub type RECORD_PDEBUGLS0STAT_TGT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebugls0stat(&self) -> RECORD_PRO_PDEBUGLS0STAT_R {
        RECORD_PRO_PDEBUGLS0STAT_R::new(self.bits)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn record_pdebugls0stat_type(&self) -> RECORD_PDEBUGLS0STAT_TYPE_R {
        RECORD_PDEBUGLS0STAT_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn record_pdebugls0stat_sz(&self) -> RECORD_PDEBUGLS0STAT_SZ_R {
        RECORD_PDEBUGLS0STAT_SZ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn record_pdebugls0stat_dtlbm(&self) -> RECORD_PDEBUGLS0STAT_DTLBM_R {
        RECORD_PDEBUGLS0STAT_DTLBM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn record_pdebugls0stat_dcm(&self) -> RECORD_PDEBUGLS0STAT_DCM_R {
        RECORD_PDEBUGLS0STAT_DCM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn record_pdebugls0stat_dch(&self) -> RECORD_PDEBUGLS0STAT_DCH_R {
        RECORD_PDEBUGLS0STAT_DCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn record_pdebugls0stat_uc(&self) -> RECORD_PDEBUGLS0STAT_UC_R {
        RECORD_PDEBUGLS0STAT_UC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn record_pdebugls0stat_wb(&self) -> RECORD_PDEBUGLS0STAT_WB_R {
        RECORD_PDEBUGLS0STAT_WB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn record_pdebugls0stat_coh(&self) -> RECORD_PDEBUGLS0STAT_COH_R {
        RECORD_PDEBUGLS0STAT_COH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn record_pdebugls0stat_stcoh(&self) -> RECORD_PDEBUGLS0STAT_STCOH_R {
        RECORD_PDEBUGLS0STAT_STCOH_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn record_pdebugls0stat_tgt(&self) -> RECORD_PDEBUGLS0STAT_TGT_R {
        RECORD_PDEBUGLS0STAT_TGT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGLS0STAT")
            .field("record_pro_pdebugls0stat", &self.record_pro_pdebugls0stat())
            .field(
                "record_pdebugls0stat_type",
                &self.record_pdebugls0stat_type(),
            )
            .field("record_pdebugls0stat_sz", &self.record_pdebugls0stat_sz())
            .field(
                "record_pdebugls0stat_dtlbm",
                &self.record_pdebugls0stat_dtlbm(),
            )
            .field("record_pdebugls0stat_dcm", &self.record_pdebugls0stat_dcm())
            .field("record_pdebugls0stat_dch", &self.record_pdebugls0stat_dch())
            .field("record_pdebugls0stat_uc", &self.record_pdebugls0stat_uc())
            .field("record_pdebugls0stat_wb", &self.record_pdebugls0stat_wb())
            .field("record_pdebugls0stat_coh", &self.record_pdebugls0stat_coh())
            .field(
                "record_pdebugls0stat_stcoh",
                &self.record_pdebugls0stat_stcoh(),
            )
            .field("record_pdebugls0stat_tgt", &self.record_pdebugls0stat_tgt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn record_pdebugls0stat_type(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_TYPE_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_TYPE_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn record_pdebugls0stat_sz(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_SZ_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_SZ_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn record_pdebugls0stat_dtlbm(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_DTLBM_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_DTLBM_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn record_pdebugls0stat_dcm(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_DCM_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_DCM_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn record_pdebugls0stat_dch(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_DCH_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_DCH_W::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn record_pdebugls0stat_uc(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_UC_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_UC_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn record_pdebugls0stat_wb(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_WB_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_WB_W::new(self, 13)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn record_pdebugls0stat_coh(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_COH_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_COH_W::new(self, 16)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn record_pdebugls0stat_stcoh(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_STCOH_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_STCOH_W::new(self, 17)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn record_pdebugls0stat_tgt(
        &mut self,
    ) -> RECORD_PDEBUGLS0STAT_TGT_W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
        RECORD_PDEBUGLS0STAT_TGT_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cpu_record_pdebugls0stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_cpu_record_pdebugls0stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cpu_record_pdebugls0stat::R`](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cpu_record_pdebugls0stat::W`](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGLS0STAT to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
