#[doc = "Register `SLAVE` reader"]
pub type R = crate::R<SLAVE_SPEC>;
#[doc = "Register `SLAVE` writer"]
pub type W = crate::W<SLAVE_SPEC>;
#[doc = "Field `CLK_MODE` reader - "]
pub type CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - "]
pub type CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_MODE_13` reader - "]
pub type CLK_MODE_13_R = crate::BitReader;
#[doc = "Field `CLK_MODE_13` writer - "]
pub type CLK_MODE_13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSCK_DATA_OUT` reader - "]
pub type RSCK_DATA_OUT_R = crate::BitReader;
#[doc = "Field `RSCK_DATA_OUT` writer - "]
pub type RSCK_DATA_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RDBUF_BITLEN_EN` reader - "]
pub type SLV_RDBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDBUF_BITLEN_EN` writer - "]
pub type SLV_RDBUF_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WRBUF_BITLEN_EN` reader - "]
pub type SLV_WRBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRBUF_BITLEN_EN` writer - "]
pub type SLV_WRBUF_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - "]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - "]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFT_RESET` writer - "]
pub type SOFT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_mode_13(&self) -> CLK_MODE_13_R {
        CLK_MODE_13_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsck_data_out(&self) -> RSCK_DATA_OUT_R {
        RSCK_DATA_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rdbuf_bitlen_en(&self) -> SLV_RDBUF_BITLEN_EN_R {
        SLV_RDBUF_BITLEN_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wrbuf_bitlen_en(&self) -> SLV_WRBUF_BITLEN_EN_R {
        SLV_WRBUF_BITLEN_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE")
            .field("clk_mode", &self.clk_mode())
            .field("clk_mode_13", &self.clk_mode_13())
            .field("rsck_data_out", &self.rsck_data_out())
            .field("slv_rdbuf_bitlen_en", &self.slv_rdbuf_bitlen_en())
            .field("slv_wrbuf_bitlen_en", &self.slv_wrbuf_bitlen_en())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<'_, SLAVE_SPEC> {
        CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_mode_13(&mut self) -> CLK_MODE_13_W<'_, SLAVE_SPEC> {
        CLK_MODE_13_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsck_data_out(&mut self) -> RSCK_DATA_OUT_W<'_, SLAVE_SPEC> {
        RSCK_DATA_OUT_W::new(self, 3)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rdbuf_bitlen_en(&mut self) -> SLV_RDBUF_BITLEN_EN_W<'_, SLAVE_SPEC> {
        SLV_RDBUF_BITLEN_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wrbuf_bitlen_en(&mut self) -> SLV_WRBUF_BITLEN_EN_W<'_, SLAVE_SPEC> {
        SLV_WRBUF_BITLEN_EN_W::new(self, 11)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, SLAVE_SPEC> {
        MODE_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<'_, SLAVE_SPEC> {
        SOFT_RESET_W::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`slave::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave::R`](R) reader structure"]
impl crate::Readable for SLAVE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave::W`](W) writer structure"]
impl crate::Writable for SLAVE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLAVE to value 0"]
impl crate::Resettable for SLAVE_SPEC {}
