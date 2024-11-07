#[doc = "Register `CNTL_DBG_SEL` reader"]
pub type R = crate::R<CNTL_DBG_SEL_SPEC>;
#[doc = "Register `CNTL_DBG_SEL` writer"]
pub type W = crate::W<CNTL_DBG_SEL_SPEC>;
#[doc = "Field `DEBUG_12M_NO_GATING` reader - Need add desc"]
pub type DEBUG_12M_NO_GATING_R = crate::BitReader;
#[doc = "Field `DEBUG_12M_NO_GATING` writer - Need add desc"]
pub type DEBUG_12M_NO_GATING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUG_BIT_SEL` reader - Need add desc"]
pub type DEBUG_BIT_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_BIT_SEL` writer - Need add desc"]
pub type DEBUG_BIT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_SEL0` reader - Need add desc"]
pub type DEBUG_SEL0_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL0` writer - Need add desc"]
pub type DEBUG_SEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_SEL1` reader - Need add desc"]
pub type DEBUG_SEL1_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL1` writer - Need add desc"]
pub type DEBUG_SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_SEL2` reader - Need add desc"]
pub type DEBUG_SEL2_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL2` writer - Need add desc"]
pub type DEBUG_SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_SEL3` reader - Need add desc"]
pub type DEBUG_SEL3_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL3` writer - Need add desc"]
pub type DEBUG_SEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_SEL4` reader - Need add desc"]
pub type DEBUG_SEL4_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL4` writer - Need add desc"]
pub type DEBUG_SEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn debug_12m_no_gating(&self) -> DEBUG_12M_NO_GATING_R {
        DEBUG_12M_NO_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Need add desc"]
    #[inline(always)]
    pub fn debug_bit_sel(&self) -> DEBUG_BIT_SEL_R {
        DEBUG_BIT_SEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel0(&self) -> DEBUG_SEL0_R {
        DEBUG_SEL0_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel1(&self) -> DEBUG_SEL1_R {
        DEBUG_SEL1_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel2(&self) -> DEBUG_SEL2_R {
        DEBUG_SEL2_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel3(&self) -> DEBUG_SEL3_R {
        DEBUG_SEL3_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel4(&self) -> DEBUG_SEL4_R {
        DEBUG_SEL4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL_DBG_SEL")
            .field("debug_12m_no_gating", &self.debug_12m_no_gating())
            .field("debug_bit_sel", &self.debug_bit_sel())
            .field("debug_sel0", &self.debug_sel0())
            .field("debug_sel1", &self.debug_sel1())
            .field("debug_sel2", &self.debug_sel2())
            .field("debug_sel3", &self.debug_sel3())
            .field("debug_sel4", &self.debug_sel4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn debug_12m_no_gating(&mut self) -> DEBUG_12M_NO_GATING_W<CNTL_DBG_SEL_SPEC> {
        DEBUG_12M_NO_GATING_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Need add desc"]
    #[inline(always)]
    pub fn debug_bit_sel(&mut self) -> DEBUG_BIT_SEL_W<CNTL_DBG_SEL_SPEC> {
        DEBUG_BIT_SEL_W::new(self, 2)
    }
    #[doc = "Bits 7:11 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel0(&mut self) -> DEBUG_SEL0_W<CNTL_DBG_SEL_SPEC> {
        DEBUG_SEL0_W::new(self, 7)
    }
    #[doc = "Bits 12:16 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel1(&mut self) -> DEBUG_SEL1_W<CNTL_DBG_SEL_SPEC> {
        DEBUG_SEL1_W::new(self, 12)
    }
    #[doc = "Bits 17:21 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel2(&mut self) -> DEBUG_SEL2_W<CNTL_DBG_SEL_SPEC> {
        DEBUG_SEL2_W::new(self, 17)
    }
    #[doc = "Bits 22:26 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel3(&mut self) -> DEBUG_SEL3_W<CNTL_DBG_SEL_SPEC> {
        DEBUG_SEL3_W::new(self, 22)
    }
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel4(&mut self) -> DEBUG_SEL4_W<CNTL_DBG_SEL_SPEC> {
        DEBUG_SEL4_W::new(self, 27)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_dbg_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_dbg_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTL_DBG_SEL_SPEC;
impl crate::RegisterSpec for CNTL_DBG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl_dbg_sel::R`](R) reader structure"]
impl crate::Readable for CNTL_DBG_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntl_dbg_sel::W`](W) writer structure"]
impl crate::Writable for CNTL_DBG_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTL_DBG_SEL to value 0"]
impl crate::Resettable for CNTL_DBG_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
