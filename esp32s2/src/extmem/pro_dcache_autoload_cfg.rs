#[doc = "Register `PRO_DCACHE_AUTOLOAD_CFG` reader"]
pub type R = crate::R<PRO_DCACHE_AUTOLOAD_CFG_SPEC>;
#[doc = "Register `PRO_DCACHE_AUTOLOAD_CFG` writer"]
pub type W = crate::W<PRO_DCACHE_AUTOLOAD_CFG_SPEC>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_MODE` reader - Reserved."]
pub type PRO_DCACHE_AUTOLOAD_MODE_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_MODE` writer - Reserved."]
pub type PRO_DCACHE_AUTOLOAD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_STEP` reader - Reserved."]
pub type PRO_DCACHE_AUTOLOAD_STEP_R = crate::FieldReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_STEP` writer - Reserved."]
pub type PRO_DCACHE_AUTOLOAD_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_ORDER` reader - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
pub type PRO_DCACHE_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_ORDER` writer - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
pub type PRO_DCACHE_AUTOLOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_RQST` reader - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type PRO_DCACHE_AUTOLOAD_RQST_R = crate::FieldReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_RQST` writer - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type PRO_DCACHE_AUTOLOAD_RQST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SIZE` reader - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SIZE_R = crate::FieldReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SIZE` writer - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT0_ENA` reader - The bits are used to enable the second section for conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT0_ENA` writer - The bits are used to enable the second section for conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SCT0_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT1_ENA` reader - The bits are used to enable the first section for conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT1_ENA` writer - The bits are used to enable the first section for conditional pre-load operation."]
pub type PRO_DCACHE_AUTOLOAD_SCT1_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn pro_dcache_autoload_mode(&self) -> PRO_DCACHE_AUTOLOAD_MODE_R {
        PRO_DCACHE_AUTOLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Reserved."]
    #[inline(always)]
    pub fn pro_dcache_autoload_step(&self) -> PRO_DCACHE_AUTOLOAD_STEP_R {
        PRO_DCACHE_AUTOLOAD_STEP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_dcache_autoload_order(&self) -> PRO_DCACHE_AUTOLOAD_ORDER_R {
        PRO_DCACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn pro_dcache_autoload_rqst(&self) -> PRO_DCACHE_AUTOLOAD_RQST_R {
        PRO_DCACHE_AUTOLOAD_RQST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_size(&self) -> PRO_DCACHE_AUTOLOAD_SIZE_R {
        PRO_DCACHE_AUTOLOAD_SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - The bits are used to enable the second section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_sct0_ena(&self) -> PRO_DCACHE_AUTOLOAD_SCT0_ENA_R {
        PRO_DCACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bits are used to enable the first section for conditional pre-load operation."]
    #[inline(always)]
    pub fn pro_dcache_autoload_sct1_ena(&self) -> PRO_DCACHE_AUTOLOAD_SCT1_ENA_R {
        PRO_DCACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_AUTOLOAD_CFG")
            .field("pro_dcache_autoload_mode", &self.pro_dcache_autoload_mode())
            .field("pro_dcache_autoload_step", &self.pro_dcache_autoload_step())
            .field(
                "pro_dcache_autoload_order",
                &self.pro_dcache_autoload_order(),
            )
            .field("pro_dcache_autoload_rqst", &self.pro_dcache_autoload_rqst())
            .field("pro_dcache_autoload_size", &self.pro_dcache_autoload_size())
            .field(
                "pro_dcache_autoload_sct0_ena",
                &self.pro_dcache_autoload_sct0_ena(),
            )
            .field(
                "pro_dcache_autoload_sct1_ena",
                &self.pro_dcache_autoload_sct1_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_mode(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_MODE_W<PRO_DCACHE_AUTOLOAD_CFG_SPEC> {
        PRO_DCACHE_AUTOLOAD_MODE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_step(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_STEP_W<PRO_DCACHE_AUTOLOAD_CFG_SPEC> {
        PRO_DCACHE_AUTOLOAD_STEP_W::new(self, 1)
    }
    #[doc = "Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_order(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_ORDER_W<PRO_DCACHE_AUTOLOAD_CFG_SPEC> {
        PRO_DCACHE_AUTOLOAD_ORDER_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_rqst(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_RQST_W<PRO_DCACHE_AUTOLOAD_CFG_SPEC> {
        PRO_DCACHE_AUTOLOAD_RQST_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_size(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_SIZE_W<PRO_DCACHE_AUTOLOAD_CFG_SPEC> {
        PRO_DCACHE_AUTOLOAD_SIZE_W::new(self, 6)
    }
    #[doc = "Bit 8 - The bits are used to enable the second section for conditional pre-load operation."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_sct0_ena(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_SCT0_ENA_W<PRO_DCACHE_AUTOLOAD_CFG_SPEC> {
        PRO_DCACHE_AUTOLOAD_SCT0_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The bits are used to enable the first section for conditional pre-load operation."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_sct1_ena(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_SCT1_ENA_W<PRO_DCACHE_AUTOLOAD_CFG_SPEC> {
        PRO_DCACHE_AUTOLOAD_SCT1_ENA_W::new(self, 9)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_autoload_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_autoload_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_AUTOLOAD_CFG_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_autoload_cfg::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_AUTOLOAD_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_autoload_cfg::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DCACHE_AUTOLOAD_CFG to value 0"]
impl crate::Resettable for PRO_DCACHE_AUTOLOAD_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
