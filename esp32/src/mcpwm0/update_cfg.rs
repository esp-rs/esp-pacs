#[doc = "Register `UPDATE_CFG` reader"]
pub type R = crate::R<UPDATE_CFG_SPEC>;
#[doc = "Register `UPDATE_CFG` writer"]
pub type W = crate::W<UPDATE_CFG_SPEC>;
#[doc = "Field `GLOBAL_UP_EN` reader - "]
pub type GLOBAL_UP_EN_R = crate::BitReader;
#[doc = "Field `GLOBAL_UP_EN` writer - "]
pub type GLOBAL_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLOBAL_FORCE_UP` reader - "]
pub type GLOBAL_FORCE_UP_R = crate::BitReader;
#[doc = "Field `GLOBAL_FORCE_UP` writer - "]
pub type GLOBAL_FORCE_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_UP_EN` reader - "]
pub type OP0_UP_EN_R = crate::BitReader;
#[doc = "Field `OP0_UP_EN` writer - "]
pub type OP0_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_FORCE_UP` reader - "]
pub type OP0_FORCE_UP_R = crate::BitReader;
#[doc = "Field `OP0_FORCE_UP` writer - "]
pub type OP0_FORCE_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_UP_EN` reader - "]
pub type OP1_UP_EN_R = crate::BitReader;
#[doc = "Field `OP1_UP_EN` writer - "]
pub type OP1_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_FORCE_UP` reader - "]
pub type OP1_FORCE_UP_R = crate::BitReader;
#[doc = "Field `OP1_FORCE_UP` writer - "]
pub type OP1_FORCE_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_UP_EN` reader - "]
pub type OP2_UP_EN_R = crate::BitReader;
#[doc = "Field `OP2_UP_EN` writer - "]
pub type OP2_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_FORCE_UP` reader - "]
pub type OP2_FORCE_UP_R = crate::BitReader;
#[doc = "Field `OP2_FORCE_UP` writer - "]
pub type OP2_FORCE_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn global_up_en(&self) -> GLOBAL_UP_EN_R {
        GLOBAL_UP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn global_force_up(&self) -> GLOBAL_FORCE_UP_R {
        GLOBAL_FORCE_UP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn op0_up_en(&self) -> OP0_UP_EN_R {
        OP0_UP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn op0_force_up(&self) -> OP0_FORCE_UP_R {
        OP0_FORCE_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn op1_up_en(&self) -> OP1_UP_EN_R {
        OP1_UP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn op1_force_up(&self) -> OP1_FORCE_UP_R {
        OP1_FORCE_UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn op2_up_en(&self) -> OP2_UP_EN_R {
        OP2_UP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn op2_force_up(&self) -> OP2_FORCE_UP_R {
        OP2_FORCE_UP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPDATE_CFG")
            .field("global_up_en", &self.global_up_en())
            .field("global_force_up", &self.global_force_up())
            .field("op0_up_en", &self.op0_up_en())
            .field("op0_force_up", &self.op0_force_up())
            .field("op1_up_en", &self.op1_up_en())
            .field("op1_force_up", &self.op1_force_up())
            .field("op2_up_en", &self.op2_up_en())
            .field("op2_force_up", &self.op2_force_up())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn global_up_en(&mut self) -> GLOBAL_UP_EN_W<UPDATE_CFG_SPEC> {
        GLOBAL_UP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn global_force_up(&mut self) -> GLOBAL_FORCE_UP_W<UPDATE_CFG_SPEC> {
        GLOBAL_FORCE_UP_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn op0_up_en(&mut self) -> OP0_UP_EN_W<UPDATE_CFG_SPEC> {
        OP0_UP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn op0_force_up(&mut self) -> OP0_FORCE_UP_W<UPDATE_CFG_SPEC> {
        OP0_FORCE_UP_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn op1_up_en(&mut self) -> OP1_UP_EN_W<UPDATE_CFG_SPEC> {
        OP1_UP_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn op1_force_up(&mut self) -> OP1_FORCE_UP_W<UPDATE_CFG_SPEC> {
        OP1_FORCE_UP_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn op2_up_en(&mut self) -> OP2_UP_EN_W<UPDATE_CFG_SPEC> {
        OP2_UP_EN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn op2_force_up(&mut self) -> OP2_FORCE_UP_W<UPDATE_CFG_SPEC> {
        OP2_FORCE_UP_W::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`update_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`update_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPDATE_CFG_SPEC;
impl crate::RegisterSpec for UPDATE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`update_cfg::R`](R) reader structure"]
impl crate::Readable for UPDATE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`update_cfg::W`](W) writer structure"]
impl crate::Writable for UPDATE_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UPDATE_CFG to value 0x55"]
impl crate::Resettable for UPDATE_CFG_SPEC {
    const RESET_VALUE: u32 = 0x55;
}
