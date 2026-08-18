#[doc = "Register `SMEM_TIMING_CALI` reader"]
pub type R = crate::R<SMEM_TIMING_CALI_SPEC>;
#[doc = "Register `SMEM_TIMING_CALI` writer"]
pub type W = crate::W<SMEM_TIMING_CALI_SPEC>;
#[doc = "Field `TIMING_CLK_ENA` reader - "]
pub type TIMING_CLK_ENA_R = crate::BitReader;
#[doc = "Field `TIMING_CLK_ENA` writer - "]
pub type TIMING_CLK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMING_CALI` reader - "]
pub type TIMING_CALI_R = crate::BitReader;
#[doc = "Field `TIMING_CALI` writer - "]
pub type TIMING_CALI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` reader - "]
pub type EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` writer - "]
pub type EXTRA_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLL_TIMING_CALI` reader - "]
pub type DLL_TIMING_CALI_R = crate::BitReader;
#[doc = "Field `DLL_TIMING_CALI` writer - "]
pub type DLL_TIMING_CALI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS0_270_SEL` reader - "]
pub type DQS0_270_SEL_R = crate::FieldReader;
#[doc = "Field `DQS0_270_SEL` writer - "]
pub type DQS0_270_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQS0_90_SEL` reader - "]
pub type DQS0_90_SEL_R = crate::FieldReader;
#[doc = "Field `DQS0_90_SEL` writer - "]
pub type DQS0_90_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQS1_270_SEL` reader - "]
pub type DQS1_270_SEL_R = crate::FieldReader;
#[doc = "Field `DQS1_270_SEL` writer - "]
pub type DQS1_270_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQS1_90_SEL` reader - "]
pub type DQS1_90_SEL_R = crate::FieldReader;
#[doc = "Field `DQS1_90_SEL` writer - "]
pub type DQS1_90_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timing_clk_ena(&self) -> TIMING_CLK_ENA_R {
        TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timing_cali(&self) -> TIMING_CALI_R {
        TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&self) -> EXTRA_DUMMY_CYCLELEN_R {
        EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dll_timing_cali(&self) -> DLL_TIMING_CALI_R {
        DLL_TIMING_CALI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn dqs0_270_sel(&self) -> DQS0_270_SEL_R {
        DQS0_270_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn dqs0_90_sel(&self) -> DQS0_90_SEL_R {
        DQS0_90_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn dqs1_270_sel(&self) -> DQS1_270_SEL_R {
        DQS1_270_SEL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn dqs1_90_sel(&self) -> DQS1_90_SEL_R {
        DQS1_90_SEL_R::new(((self.bits >> 13) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_TIMING_CALI")
            .field("timing_clk_ena", &self.timing_clk_ena())
            .field("timing_cali", &self.timing_cali())
            .field("extra_dummy_cyclelen", &self.extra_dummy_cyclelen())
            .field("dll_timing_cali", &self.dll_timing_cali())
            .field("dqs0_270_sel", &self.dqs0_270_sel())
            .field("dqs0_90_sel", &self.dqs0_90_sel())
            .field("dqs1_270_sel", &self.dqs1_270_sel())
            .field("dqs1_90_sel", &self.dqs1_90_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timing_clk_ena(&mut self) -> TIMING_CLK_ENA_W<'_, SMEM_TIMING_CALI_SPEC> {
        TIMING_CLK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timing_cali(&mut self) -> TIMING_CALI_W<'_, SMEM_TIMING_CALI_SPEC> {
        TIMING_CALI_W::new(self, 1)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&mut self) -> EXTRA_DUMMY_CYCLELEN_W<'_, SMEM_TIMING_CALI_SPEC> {
        EXTRA_DUMMY_CYCLELEN_W::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dll_timing_cali(&mut self) -> DLL_TIMING_CALI_W<'_, SMEM_TIMING_CALI_SPEC> {
        DLL_TIMING_CALI_W::new(self, 5)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn dqs0_270_sel(&mut self) -> DQS0_270_SEL_W<'_, SMEM_TIMING_CALI_SPEC> {
        DQS0_270_SEL_W::new(self, 7)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn dqs0_90_sel(&mut self) -> DQS0_90_SEL_W<'_, SMEM_TIMING_CALI_SPEC> {
        DQS0_90_SEL_W::new(self, 9)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn dqs1_270_sel(&mut self) -> DQS1_270_SEL_W<'_, SMEM_TIMING_CALI_SPEC> {
        DQS1_270_SEL_W::new(self, 11)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn dqs1_90_sel(&mut self) -> DQS1_90_SEL_W<'_, SMEM_TIMING_CALI_SPEC> {
        DQS1_90_SEL_W::new(self, 13)
    }
}
#[doc = "MSPI external RAM timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_timing_cali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_timing_cali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_TIMING_CALI_SPEC;
impl crate::RegisterSpec for SMEM_TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_timing_cali::R`](R) reader structure"]
impl crate::Readable for SMEM_TIMING_CALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_timing_cali::W`](W) writer structure"]
impl crate::Writable for SMEM_TIMING_CALI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_TIMING_CALI to value 0x2a81"]
impl crate::Resettable for SMEM_TIMING_CALI_SPEC {
    const RESET_VALUE: u32 = 0x2a81;
}
