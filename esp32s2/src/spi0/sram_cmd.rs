#[doc = "Register `SRAM_CMD` reader"]
pub type R = crate::R<SRAM_CMD_SPEC>;
#[doc = "Register `SRAM_CMD` writer"]
pub type W = crate::W<SRAM_CMD_SPEC>;
#[doc = "Field `SCLK_MODE` reader - "]
pub type SCLK_MODE_R = crate::FieldReader;
#[doc = "Field `SCLK_MODE` writer - "]
pub type SCLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWB_MODE` reader - "]
pub type SWB_MODE_R = crate::FieldReader;
#[doc = "Field `SWB_MODE` writer - "]
pub type SWB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDIN_DUAL` reader - "]
pub type SDIN_DUAL_R = crate::BitReader;
#[doc = "Field `SDIN_DUAL` writer - "]
pub type SDIN_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_DUAL` reader - "]
pub type SDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `SDOUT_DUAL` writer - "]
pub type SDOUT_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_DUAL` reader - "]
pub type SADDR_DUAL_R = crate::BitReader;
#[doc = "Field `SADDR_DUAL` writer - "]
pub type SADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMD_DUAL` reader - "]
pub type SCMD_DUAL_R = crate::BitReader;
#[doc = "Field `SCMD_DUAL` writer - "]
pub type SCMD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIN_QUAD` reader - "]
pub type SDIN_QUAD_R = crate::BitReader;
#[doc = "Field `SDIN_QUAD` writer - "]
pub type SDIN_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_QUAD` reader - "]
pub type SDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `SDOUT_QUAD` writer - "]
pub type SDOUT_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_QUAD` reader - "]
pub type SADDR_QUAD_R = crate::BitReader;
#[doc = "Field `SADDR_QUAD` writer - "]
pub type SADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMD_QUAD` reader - "]
pub type SCMD_QUAD_R = crate::BitReader;
#[doc = "Field `SCMD_QUAD` writer - "]
pub type SCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIN_OCT` reader - "]
pub type SDIN_OCT_R = crate::BitReader;
#[doc = "Field `SDIN_OCT` writer - "]
pub type SDIN_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_OCT` reader - "]
pub type SDOUT_OCT_R = crate::BitReader;
#[doc = "Field `SDOUT_OCT` writer - "]
pub type SDOUT_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_OCT` reader - "]
pub type SADDR_OCT_R = crate::BitReader;
#[doc = "Field `SADDR_OCT` writer - "]
pub type SADDR_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMD_OCT` reader - "]
pub type SCMD_OCT_R = crate::BitReader;
#[doc = "Field `SCMD_OCT` writer - "]
pub type SCMD_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDUMMY_OUT` reader - "]
pub type SDUMMY_OUT_R = crate::BitReader;
#[doc = "Field `SDUMMY_OUT` writer - "]
pub type SDUMMY_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sclk_mode(&self) -> SCLK_MODE_R {
        SCLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9"]
    #[inline(always)]
    pub fn swb_mode(&self) -> SWB_MODE_R {
        SWB_MODE_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sdin_dual(&self) -> SDIN_DUAL_R {
        SDIN_DUAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sdout_dual(&self) -> SDOUT_DUAL_R {
        SDOUT_DUAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn saddr_dual(&self) -> SADDR_DUAL_R {
        SADDR_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn scmd_dual(&self) -> SCMD_DUAL_R {
        SCMD_DUAL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sdin_quad(&self) -> SDIN_QUAD_R {
        SDIN_QUAD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sdout_quad(&self) -> SDOUT_QUAD_R {
        SDOUT_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn saddr_quad(&self) -> SADDR_QUAD_R {
        SADDR_QUAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn scmd_quad(&self) -> SCMD_QUAD_R {
        SCMD_QUAD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sdin_oct(&self) -> SDIN_OCT_R {
        SDIN_OCT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sdout_oct(&self) -> SDOUT_OCT_R {
        SDOUT_OCT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn saddr_oct(&self) -> SADDR_OCT_R {
        SADDR_OCT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn scmd_oct(&self) -> SCMD_OCT_R {
        SCMD_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sdummy_out(&self) -> SDUMMY_OUT_R {
        SDUMMY_OUT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CMD")
            .field("sdummy_out", &self.sdummy_out())
            .field("scmd_oct", &self.scmd_oct())
            .field("saddr_oct", &self.saddr_oct())
            .field("sdout_oct", &self.sdout_oct())
            .field("sdin_oct", &self.sdin_oct())
            .field("scmd_quad", &self.scmd_quad())
            .field("saddr_quad", &self.saddr_quad())
            .field("sdout_quad", &self.sdout_quad())
            .field("sdin_quad", &self.sdin_quad())
            .field("scmd_dual", &self.scmd_dual())
            .field("saddr_dual", &self.saddr_dual())
            .field("sdout_dual", &self.sdout_dual())
            .field("sdin_dual", &self.sdin_dual())
            .field("swb_mode", &self.swb_mode())
            .field("sclk_mode", &self.sclk_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sclk_mode(&mut self) -> SCLK_MODE_W<'_, SRAM_CMD_SPEC> {
        SCLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:9"]
    #[inline(always)]
    pub fn swb_mode(&mut self) -> SWB_MODE_W<'_, SRAM_CMD_SPEC> {
        SWB_MODE_W::new(self, 2)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sdin_dual(&mut self) -> SDIN_DUAL_W<'_, SRAM_CMD_SPEC> {
        SDIN_DUAL_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sdout_dual(&mut self) -> SDOUT_DUAL_W<'_, SRAM_CMD_SPEC> {
        SDOUT_DUAL_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn saddr_dual(&mut self) -> SADDR_DUAL_W<'_, SRAM_CMD_SPEC> {
        SADDR_DUAL_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn scmd_dual(&mut self) -> SCMD_DUAL_W<'_, SRAM_CMD_SPEC> {
        SCMD_DUAL_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sdin_quad(&mut self) -> SDIN_QUAD_W<'_, SRAM_CMD_SPEC> {
        SDIN_QUAD_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sdout_quad(&mut self) -> SDOUT_QUAD_W<'_, SRAM_CMD_SPEC> {
        SDOUT_QUAD_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn saddr_quad(&mut self) -> SADDR_QUAD_W<'_, SRAM_CMD_SPEC> {
        SADDR_QUAD_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn scmd_quad(&mut self) -> SCMD_QUAD_W<'_, SRAM_CMD_SPEC> {
        SCMD_QUAD_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sdin_oct(&mut self) -> SDIN_OCT_W<'_, SRAM_CMD_SPEC> {
        SDIN_OCT_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sdout_oct(&mut self) -> SDOUT_OCT_W<'_, SRAM_CMD_SPEC> {
        SDOUT_OCT_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn saddr_oct(&mut self) -> SADDR_OCT_W<'_, SRAM_CMD_SPEC> {
        SADDR_OCT_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn scmd_oct(&mut self) -> SCMD_OCT_W<'_, SRAM_CMD_SPEC> {
        SCMD_OCT_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sdummy_out(&mut self) -> SDUMMY_OUT_W<'_, SRAM_CMD_SPEC> {
        SDUMMY_OUT_W::new(self, 22)
    }
}
#[doc = "SPI Memory SRAM Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_CMD_SPEC;
impl crate::RegisterSpec for SRAM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cmd::R`](R) reader structure"]
impl crate::Readable for SRAM_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_cmd::W`](W) writer structure"]
impl crate::Writable for SRAM_CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CMD to value 0"]
impl crate::Resettable for SRAM_CMD_SPEC {}
