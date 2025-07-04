#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CLK_MODE` reader - "]
pub type CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - "]
pub type CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CS_HOLD_DLY_RES` reader - "]
pub type CS_HOLD_DLY_RES_R = crate::FieldReader<u16>;
#[doc = "Field `CS_HOLD_DLY_RES` writer - "]
pub type CS_HOLD_DLY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CS_HOLD_DLY` reader - "]
pub type CS_HOLD_DLY_R = crate::FieldReader<u16>;
#[doc = "Field `CS_HOLD_DLY` writer - "]
pub type CS_HOLD_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CS_DLY_NUM` reader - "]
pub type CS_DLY_NUM_R = crate::FieldReader;
#[doc = "Field `CS_DLY_NUM` writer - "]
pub type CS_DLY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CS_DLY_MODE` reader - "]
pub type CS_DLY_MODE_R = crate::FieldReader;
#[doc = "Field `CS_DLY_MODE` writer - "]
pub type CS_DLY_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CS_DLY_EDGE` reader - "]
pub type CS_DLY_EDGE_R = crate::BitReader;
#[doc = "Field `CS_DLY_EDGE` writer - "]
pub type CS_DLY_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:13"]
    #[inline(always)]
    pub fn cs_hold_dly_res(&self) -> CS_HOLD_DLY_RES_R {
        CS_HOLD_DLY_RES_R::new(((self.bits >> 2) & 0x0fff) as u16)
    }
    #[doc = "Bits 14:25"]
    #[inline(always)]
    pub fn cs_hold_dly(&self) -> CS_HOLD_DLY_R {
        CS_HOLD_DLY_R::new(((self.bits >> 14) & 0x0fff) as u16)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn cs_dly_num(&self) -> CS_DLY_NUM_R {
        CS_DLY_NUM_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn cs_dly_mode(&self) -> CS_DLY_MODE_R {
        CS_DLY_MODE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cs_dly_edge(&self) -> CS_DLY_EDGE_R {
        CS_DLY_EDGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("cs_dly_edge", &self.cs_dly_edge())
            .field("cs_dly_mode", &self.cs_dly_mode())
            .field("cs_dly_num", &self.cs_dly_num())
            .field("cs_hold_dly", &self.cs_hold_dly())
            .field("cs_hold_dly_res", &self.cs_hold_dly_res())
            .field("clk_mode", &self.clk_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<CTRL1_SPEC> {
        CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:13"]
    #[inline(always)]
    pub fn cs_hold_dly_res(&mut self) -> CS_HOLD_DLY_RES_W<CTRL1_SPEC> {
        CS_HOLD_DLY_RES_W::new(self, 2)
    }
    #[doc = "Bits 14:25"]
    #[inline(always)]
    pub fn cs_hold_dly(&mut self) -> CS_HOLD_DLY_W<CTRL1_SPEC> {
        CS_HOLD_DLY_W::new(self, 14)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn cs_dly_num(&mut self) -> CS_DLY_NUM_W<CTRL1_SPEC> {
        CS_DLY_NUM_W::new(self, 26)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn cs_dly_mode(&mut self) -> CS_DLY_MODE_W<CTRL1_SPEC> {
        CS_DLY_MODE_W::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cs_dly_edge(&mut self) -> CS_DLY_EDGE_W<CTRL1_SPEC> {
        CS_DLY_EDGE_W::new(self, 31)
    }
}
#[doc = "SPI Memory Control1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {}
