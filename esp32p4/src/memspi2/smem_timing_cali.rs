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
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_TIMING_CALI")
            .field("timing_clk_ena", &self.timing_clk_ena())
            .field("timing_cali", &self.timing_cali())
            .field("extra_dummy_cyclelen", &self.extra_dummy_cyclelen())
            .field("dll_timing_cali", &self.dll_timing_cali())
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
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_timing_cali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_timing_cali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets SMEM_TIMING_CALI to value 0"]
impl crate::Resettable for SMEM_TIMING_CALI_SPEC {}
