#[doc = "Register `DIG_PWC` reader"]
pub type R = crate::R<DIG_PWC_SPEC>;
#[doc = "Register `DIG_PWC` writer"]
pub type W = crate::W<DIG_PWC_SPEC>;
#[doc = "Field `LSLP_MEM_FORCE_PD` reader - memories in digital core force PD in sleep"]
pub type LSLP_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_FORCE_PD` writer - memories in digital core force PD in sleep"]
pub type LSLP_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSLP_MEM_FORCE_PU` reader - memories in digital core force no PD in sleep"]
pub type LSLP_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_FORCE_PU` writer - memories in digital core force no PD in sleep"]
pub type LSLP_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_FORCE_PD` reader - internal SRAM 2 force power down"]
pub type BT_FORCE_PD_R = crate::BitReader;
#[doc = "Field `BT_FORCE_PD` writer - internal SRAM 2 force power down"]
pub type BT_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_FORCE_PU` reader - internal SRAM 2 force power up"]
pub type BT_FORCE_PU_R = crate::BitReader;
#[doc = "Field `BT_FORCE_PU` writer - internal SRAM 2 force power up"]
pub type BT_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DG_PERI_FORCE_PD` reader - internal SRAM 3 force power down"]
pub type DG_PERI_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DG_PERI_FORCE_PD` writer - internal SRAM 3 force power down"]
pub type DG_PERI_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DG_PERI_FORCE_PU` reader - internal SRAM 3 force power up"]
pub type DG_PERI_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DG_PERI_FORCE_PU` writer - internal SRAM 3 force power up"]
pub type DG_PERI_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_FORCE_PD` reader - wifi force power down"]
pub type WIFI_FORCE_PD_R = crate::BitReader;
#[doc = "Field `WIFI_FORCE_PD` writer - wifi force power down"]
pub type WIFI_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_FORCE_PU` reader - wifi force power up"]
pub type WIFI_FORCE_PU_R = crate::BitReader;
#[doc = "Field `WIFI_FORCE_PU` writer - wifi force power up"]
pub type WIFI_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DG_WRAP_FORCE_PD` reader - digital core force power down"]
pub type DG_WRAP_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_PD` writer - digital core force power down"]
pub type DG_WRAP_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DG_WRAP_FORCE_PU` reader - digital core force power up"]
pub type DG_WRAP_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_PU` writer - digital core force power up"]
pub type DG_WRAP_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_TOP_FORCE_PD` reader - digital dcdc force power down"]
pub type CPU_TOP_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CPU_TOP_FORCE_PD` writer - digital dcdc force power down"]
pub type CPU_TOP_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_TOP_FORCE_PU` reader - digital dcdc force power up"]
pub type CPU_TOP_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CPU_TOP_FORCE_PU` writer - digital dcdc force power up"]
pub type CPU_TOP_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_PD_EN` reader - enable power down internal SRAM 2 in sleep"]
pub type BT_PD_EN_R = crate::BitReader;
#[doc = "Field `BT_PD_EN` writer - enable power down internal SRAM 2 in sleep"]
pub type BT_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DG_PERI_PD_EN` reader - enable power down internal SRAM 3 in sleep"]
pub type DG_PERI_PD_EN_R = crate::BitReader;
#[doc = "Field `DG_PERI_PD_EN` writer - enable power down internal SRAM 3 in sleep"]
pub type DG_PERI_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_TOP_PD_EN` reader - enable power down internal SRAM 4 in sleep"]
pub type CPU_TOP_PD_EN_R = crate::BitReader;
#[doc = "Field `CPU_TOP_PD_EN` writer - enable power down internal SRAM 4 in sleep"]
pub type CPU_TOP_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_PD_EN` reader - enable power down wifi in sleep"]
pub type WIFI_PD_EN_R = crate::BitReader;
#[doc = "Field `WIFI_PD_EN` writer - enable power down wifi in sleep"]
pub type WIFI_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DG_WRAP_PD_EN` reader - enable power down all digital logic"]
pub type DG_WRAP_PD_EN_R = crate::BitReader;
#[doc = "Field `DG_WRAP_PD_EN` writer - enable power down all digital logic"]
pub type DG_WRAP_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - memories in digital core force PD in sleep"]
    #[inline(always)]
    pub fn lslp_mem_force_pd(&self) -> LSLP_MEM_FORCE_PD_R {
        LSLP_MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - memories in digital core force no PD in sleep"]
    #[inline(always)]
    pub fn lslp_mem_force_pu(&self) -> LSLP_MEM_FORCE_PU_R {
        LSLP_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    pub fn bt_force_pd(&self) -> BT_FORCE_PD_R {
        BT_FORCE_PD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    pub fn bt_force_pu(&self) -> BT_FORCE_PU_R {
        BT_FORCE_PU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    pub fn dg_peri_force_pd(&self) -> DG_PERI_FORCE_PD_R {
        DG_PERI_FORCE_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    pub fn dg_peri_force_pu(&self) -> DG_PERI_FORCE_PU_R {
        DG_PERI_FORCE_PU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - wifi force power down"]
    #[inline(always)]
    pub fn wifi_force_pd(&self) -> WIFI_FORCE_PD_R {
        WIFI_FORCE_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - wifi force power up"]
    #[inline(always)]
    pub fn wifi_force_pu(&self) -> WIFI_FORCE_PU_R {
        WIFI_FORCE_PU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - digital core force power down"]
    #[inline(always)]
    pub fn dg_wrap_force_pd(&self) -> DG_WRAP_FORCE_PD_R {
        DG_WRAP_FORCE_PD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - digital core force power up"]
    #[inline(always)]
    pub fn dg_wrap_force_pu(&self) -> DG_WRAP_FORCE_PU_R {
        DG_WRAP_FORCE_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - digital dcdc force power down"]
    #[inline(always)]
    pub fn cpu_top_force_pd(&self) -> CPU_TOP_FORCE_PD_R {
        CPU_TOP_FORCE_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - digital dcdc force power up"]
    #[inline(always)]
    pub fn cpu_top_force_pu(&self) -> CPU_TOP_FORCE_PU_R {
        CPU_TOP_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    pub fn bt_pd_en(&self) -> BT_PD_EN_R {
        BT_PD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    pub fn dg_peri_pd_en(&self) -> DG_PERI_PD_EN_R {
        DG_PERI_PD_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    pub fn cpu_top_pd_en(&self) -> CPU_TOP_PD_EN_R {
        CPU_TOP_PD_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable power down wifi in sleep"]
    #[inline(always)]
    pub fn wifi_pd_en(&self) -> WIFI_PD_EN_R {
        WIFI_PD_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable power down all digital logic"]
    #[inline(always)]
    pub fn dg_wrap_pd_en(&self) -> DG_WRAP_PD_EN_R {
        DG_WRAP_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIG_PWC")
            .field("lslp_mem_force_pd", &self.lslp_mem_force_pd())
            .field("lslp_mem_force_pu", &self.lslp_mem_force_pu())
            .field("bt_force_pd", &self.bt_force_pd())
            .field("bt_force_pu", &self.bt_force_pu())
            .field("dg_peri_force_pd", &self.dg_peri_force_pd())
            .field("dg_peri_force_pu", &self.dg_peri_force_pu())
            .field("wifi_force_pd", &self.wifi_force_pd())
            .field("wifi_force_pu", &self.wifi_force_pu())
            .field("dg_wrap_force_pd", &self.dg_wrap_force_pd())
            .field("dg_wrap_force_pu", &self.dg_wrap_force_pu())
            .field("cpu_top_force_pd", &self.cpu_top_force_pd())
            .field("cpu_top_force_pu", &self.cpu_top_force_pu())
            .field("bt_pd_en", &self.bt_pd_en())
            .field("dg_peri_pd_en", &self.dg_peri_pd_en())
            .field("cpu_top_pd_en", &self.cpu_top_pd_en())
            .field("wifi_pd_en", &self.wifi_pd_en())
            .field("dg_wrap_pd_en", &self.dg_wrap_pd_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - memories in digital core force PD in sleep"]
    #[inline(always)]
    pub fn lslp_mem_force_pd(&mut self) -> LSLP_MEM_FORCE_PD_W<'_, DIG_PWC_SPEC> {
        LSLP_MEM_FORCE_PD_W::new(self, 3)
    }
    #[doc = "Bit 4 - memories in digital core force no PD in sleep"]
    #[inline(always)]
    pub fn lslp_mem_force_pu(&mut self) -> LSLP_MEM_FORCE_PU_W<'_, DIG_PWC_SPEC> {
        LSLP_MEM_FORCE_PU_W::new(self, 4)
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    pub fn bt_force_pd(&mut self) -> BT_FORCE_PD_W<'_, DIG_PWC_SPEC> {
        BT_FORCE_PD_W::new(self, 11)
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    pub fn bt_force_pu(&mut self) -> BT_FORCE_PU_W<'_, DIG_PWC_SPEC> {
        BT_FORCE_PU_W::new(self, 12)
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    pub fn dg_peri_force_pd(&mut self) -> DG_PERI_FORCE_PD_W<'_, DIG_PWC_SPEC> {
        DG_PERI_FORCE_PD_W::new(self, 13)
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    pub fn dg_peri_force_pu(&mut self) -> DG_PERI_FORCE_PU_W<'_, DIG_PWC_SPEC> {
        DG_PERI_FORCE_PU_W::new(self, 14)
    }
    #[doc = "Bit 17 - wifi force power down"]
    #[inline(always)]
    pub fn wifi_force_pd(&mut self) -> WIFI_FORCE_PD_W<'_, DIG_PWC_SPEC> {
        WIFI_FORCE_PD_W::new(self, 17)
    }
    #[doc = "Bit 18 - wifi force power up"]
    #[inline(always)]
    pub fn wifi_force_pu(&mut self) -> WIFI_FORCE_PU_W<'_, DIG_PWC_SPEC> {
        WIFI_FORCE_PU_W::new(self, 18)
    }
    #[doc = "Bit 19 - digital core force power down"]
    #[inline(always)]
    pub fn dg_wrap_force_pd(&mut self) -> DG_WRAP_FORCE_PD_W<'_, DIG_PWC_SPEC> {
        DG_WRAP_FORCE_PD_W::new(self, 19)
    }
    #[doc = "Bit 20 - digital core force power up"]
    #[inline(always)]
    pub fn dg_wrap_force_pu(&mut self) -> DG_WRAP_FORCE_PU_W<'_, DIG_PWC_SPEC> {
        DG_WRAP_FORCE_PU_W::new(self, 20)
    }
    #[doc = "Bit 21 - digital dcdc force power down"]
    #[inline(always)]
    pub fn cpu_top_force_pd(&mut self) -> CPU_TOP_FORCE_PD_W<'_, DIG_PWC_SPEC> {
        CPU_TOP_FORCE_PD_W::new(self, 21)
    }
    #[doc = "Bit 22 - digital dcdc force power up"]
    #[inline(always)]
    pub fn cpu_top_force_pu(&mut self) -> CPU_TOP_FORCE_PU_W<'_, DIG_PWC_SPEC> {
        CPU_TOP_FORCE_PU_W::new(self, 22)
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    pub fn bt_pd_en(&mut self) -> BT_PD_EN_W<'_, DIG_PWC_SPEC> {
        BT_PD_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    pub fn dg_peri_pd_en(&mut self) -> DG_PERI_PD_EN_W<'_, DIG_PWC_SPEC> {
        DG_PERI_PD_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    pub fn cpu_top_pd_en(&mut self) -> CPU_TOP_PD_EN_W<'_, DIG_PWC_SPEC> {
        CPU_TOP_PD_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - enable power down wifi in sleep"]
    #[inline(always)]
    pub fn wifi_pd_en(&mut self) -> WIFI_PD_EN_W<'_, DIG_PWC_SPEC> {
        WIFI_PD_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable power down all digital logic"]
    #[inline(always)]
    pub fn dg_wrap_pd_en(&mut self) -> DG_WRAP_PD_EN_W<'_, DIG_PWC_SPEC> {
        DG_WRAP_PD_EN_W::new(self, 31)
    }
}
#[doc = "configure digital power\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_pwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_pwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIG_PWC_SPEC;
impl crate::RegisterSpec for DIG_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_pwc::R`](R) reader structure"]
impl crate::Readable for DIG_PWC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dig_pwc::W`](W) writer structure"]
impl crate::Writable for DIG_PWC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIG_PWC to value 0x0054_5010"]
impl crate::Resettable for DIG_PWC_SPEC {
    const RESET_VALUE: u32 = 0x0054_5010;
}
