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
pub type CS_HOLD_DLY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CS_HOLD_DLY_PER` reader - "]
pub type CS_HOLD_DLY_PER_R = crate::FieldReader<u16>;
#[doc = "Field `CS_HOLD_DLY_PER` writer - "]
pub type CS_HOLD_DLY_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CS_HOLD_DLY_PER_EN` reader - "]
pub type CS_HOLD_DLY_PER_EN_R = crate::BitReader;
#[doc = "Field `CS_HOLD_DLY_PER_EN` writer - "]
pub type CS_HOLD_DLY_PER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn cs_hold_dly_res(&self) -> CS_HOLD_DLY_RES_R {
        CS_HOLD_DLY_RES_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:20"]
    #[inline(always)]
    pub fn cs_hold_dly_per(&self) -> CS_HOLD_DLY_PER_R {
        CS_HOLD_DLY_PER_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cs_hold_dly_per_en(&self) -> CS_HOLD_DLY_PER_EN_R {
        CS_HOLD_DLY_PER_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("clk_mode", &self.clk_mode())
            .field("cs_hold_dly_res", &self.cs_hold_dly_res())
            .field("cs_hold_dly_per", &self.cs_hold_dly_per())
            .field("cs_hold_dly_per_en", &self.cs_hold_dly_per_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<'_, CTRL1_SPEC> {
        CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn cs_hold_dly_res(&mut self) -> CS_HOLD_DLY_RES_W<'_, CTRL1_SPEC> {
        CS_HOLD_DLY_RES_W::new(self, 2)
    }
    #[doc = "Bits 12:20"]
    #[inline(always)]
    pub fn cs_hold_dly_per(&mut self) -> CS_HOLD_DLY_PER_W<'_, CTRL1_SPEC> {
        CS_HOLD_DLY_PER_W::new(self, 12)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cs_hold_dly_per_en(&mut self) -> CS_HOLD_DLY_PER_EN_W<'_, CTRL1_SPEC> {
        CS_HOLD_DLY_PER_EN_W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
