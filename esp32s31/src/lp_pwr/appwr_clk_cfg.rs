#[doc = "Register `APPWR_CLK_CFG` reader"]
pub type R = crate::R<APPWR_CLK_CFG_SPEC>;
#[doc = "Register `APPWR_CLK_CFG` writer"]
pub type W = crate::W<APPWR_CLK_CFG_SPEC>;
#[doc = "Field `CPU_CLK_MODE` reader - cpu clk mode config register for apsys"]
pub type CPU_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CPU_CLK_MODE` writer - cpu clk mode config register for apsys"]
pub type CPU_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOP_CLK_MODE` reader - top clk mode config register for apsys"]
pub type TOP_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `TOP_CLK_MODE` writer - top clk mode config register for apsys"]
pub type TOP_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNNT_CLK_MODE` reader - cnnt clk mode config register for apsys"]
pub type CNNT_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CNNT_CLK_MODE` writer - cnnt clk mode config register for apsys"]
pub type CNNT_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HPALIVE_CLK_MODE` reader - hpalive clk mode config register for apsys"]
pub type HPALIVE_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `HPALIVE_CLK_MODE` writer - hpalive clk mode config register for apsys"]
pub type HPALIVE_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_0_CLK_MODE` reader - mem_0 clk mode config register for apsys"]
pub type MEM_0_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_0_CLK_MODE` writer - mem_0 clk mode config register for apsys"]
pub type MEM_0_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_1_CLK_MODE` reader - mem_1 clk mode config register for apsys"]
pub type MEM_1_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_1_CLK_MODE` writer - mem_1 clk mode config register for apsys"]
pub type MEM_1_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_2_CLK_MODE` reader - mem_2 clk mode config register for apsys"]
pub type MEM_2_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_2_CLK_MODE` writer - mem_2 clk mode config register for apsys"]
pub type MEM_2_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_3_CLK_MODE` reader - mem_3 clk mode config register for apsys"]
pub type MEM_3_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_3_CLK_MODE` writer - mem_3 clk mode config register for apsys"]
pub type MEM_3_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - cpu clk mode config register for apsys"]
    #[inline(always)]
    pub fn cpu_clk_mode(&self) -> CPU_CLK_MODE_R {
        CPU_CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - top clk mode config register for apsys"]
    #[inline(always)]
    pub fn top_clk_mode(&self) -> TOP_CLK_MODE_R {
        TOP_CLK_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - cnnt clk mode config register for apsys"]
    #[inline(always)]
    pub fn cnnt_clk_mode(&self) -> CNNT_CLK_MODE_R {
        CNNT_CLK_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - hpalive clk mode config register for apsys"]
    #[inline(always)]
    pub fn hpalive_clk_mode(&self) -> HPALIVE_CLK_MODE_R {
        HPALIVE_CLK_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - mem_0 clk mode config register for apsys"]
    #[inline(always)]
    pub fn mem_0_clk_mode(&self) -> MEM_0_CLK_MODE_R {
        MEM_0_CLK_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - mem_1 clk mode config register for apsys"]
    #[inline(always)]
    pub fn mem_1_clk_mode(&self) -> MEM_1_CLK_MODE_R {
        MEM_1_CLK_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - mem_2 clk mode config register for apsys"]
    #[inline(always)]
    pub fn mem_2_clk_mode(&self) -> MEM_2_CLK_MODE_R {
        MEM_2_CLK_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - mem_3 clk mode config register for apsys"]
    #[inline(always)]
    pub fn mem_3_clk_mode(&self) -> MEM_3_CLK_MODE_R {
        MEM_3_CLK_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPWR_CLK_CFG")
            .field("cpu_clk_mode", &self.cpu_clk_mode())
            .field("top_clk_mode", &self.top_clk_mode())
            .field("cnnt_clk_mode", &self.cnnt_clk_mode())
            .field("hpalive_clk_mode", &self.hpalive_clk_mode())
            .field("mem_0_clk_mode", &self.mem_0_clk_mode())
            .field("mem_1_clk_mode", &self.mem_1_clk_mode())
            .field("mem_2_clk_mode", &self.mem_2_clk_mode())
            .field("mem_3_clk_mode", &self.mem_3_clk_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - cpu clk mode config register for apsys"]
    #[inline(always)]
    pub fn cpu_clk_mode(&mut self) -> CPU_CLK_MODE_W<'_, APPWR_CLK_CFG_SPEC> {
        CPU_CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - top clk mode config register for apsys"]
    #[inline(always)]
    pub fn top_clk_mode(&mut self) -> TOP_CLK_MODE_W<'_, APPWR_CLK_CFG_SPEC> {
        TOP_CLK_MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - cnnt clk mode config register for apsys"]
    #[inline(always)]
    pub fn cnnt_clk_mode(&mut self) -> CNNT_CLK_MODE_W<'_, APPWR_CLK_CFG_SPEC> {
        CNNT_CLK_MODE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - hpalive clk mode config register for apsys"]
    #[inline(always)]
    pub fn hpalive_clk_mode(&mut self) -> HPALIVE_CLK_MODE_W<'_, APPWR_CLK_CFG_SPEC> {
        HPALIVE_CLK_MODE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - mem_0 clk mode config register for apsys"]
    #[inline(always)]
    pub fn mem_0_clk_mode(&mut self) -> MEM_0_CLK_MODE_W<'_, APPWR_CLK_CFG_SPEC> {
        MEM_0_CLK_MODE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - mem_1 clk mode config register for apsys"]
    #[inline(always)]
    pub fn mem_1_clk_mode(&mut self) -> MEM_1_CLK_MODE_W<'_, APPWR_CLK_CFG_SPEC> {
        MEM_1_CLK_MODE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - mem_2 clk mode config register for apsys"]
    #[inline(always)]
    pub fn mem_2_clk_mode(&mut self) -> MEM_2_CLK_MODE_W<'_, APPWR_CLK_CFG_SPEC> {
        MEM_2_CLK_MODE_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - mem_3 clk mode config register for apsys"]
    #[inline(always)]
    pub fn mem_3_clk_mode(&mut self) -> MEM_3_CLK_MODE_W<'_, APPWR_CLK_CFG_SPEC> {
        MEM_3_CLK_MODE_W::new(self, 14)
    }
}
#[doc = "config register for apsys pwr and clk mode\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_clk_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_clk_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPWR_CLK_CFG_SPEC;
impl crate::RegisterSpec for APPWR_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appwr_clk_cfg::R`](R) reader structure"]
impl crate::Readable for APPWR_CLK_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`appwr_clk_cfg::W`](W) writer structure"]
impl crate::Writable for APPWR_CLK_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APPWR_CLK_CFG to value 0xffff"]
impl crate::Resettable for APPWR_CLK_CFG_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
