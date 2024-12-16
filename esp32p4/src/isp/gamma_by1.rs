#[doc = "Register `GAMMA_BY1` reader"]
pub type R = crate::R<GAMMA_BY1_SPEC>;
#[doc = "Register `GAMMA_BY1` writer"]
pub type W = crate::W<GAMMA_BY1_SPEC>;
#[doc = "Field `GAMMA_B_Y03` reader - this field configures the point 3 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y03_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y03` writer - this field configures the point 3 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y03_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y02` reader - this field configures the point 2 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y02_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y02` writer - this field configures the point 2 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y02_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y01` reader - this field configures the point 1 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y01_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y01` writer - this field configures the point 1 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y01_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y00` reader - this field configures the point 0 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y00_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y00` writer - this field configures the point 0 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y00_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 3 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y03(&self) -> GAMMA_B_Y03_R {
        GAMMA_B_Y03_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 2 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y02(&self) -> GAMMA_B_Y02_R {
        GAMMA_B_Y02_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 1 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y01(&self) -> GAMMA_B_Y01_R {
        GAMMA_B_Y01_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 0 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y00(&self) -> GAMMA_B_Y00_R {
        GAMMA_B_Y00_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_BY1")
            .field("gamma_b_y03", &self.gamma_b_y03())
            .field("gamma_b_y02", &self.gamma_b_y02())
            .field("gamma_b_y01", &self.gamma_b_y01())
            .field("gamma_b_y00", &self.gamma_b_y00())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 3 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y03(&mut self) -> GAMMA_B_Y03_W<GAMMA_BY1_SPEC> {
        GAMMA_B_Y03_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 2 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y02(&mut self) -> GAMMA_B_Y02_W<GAMMA_BY1_SPEC> {
        GAMMA_B_Y02_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 1 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y01(&mut self) -> GAMMA_B_Y01_W<GAMMA_BY1_SPEC> {
        GAMMA_B_Y01_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 0 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y00(&mut self) -> GAMMA_B_Y00_W<GAMMA_BY1_SPEC> {
        GAMMA_B_Y00_W::new(self, 24)
    }
}
#[doc = "point of Y-axis of b channel gamma curve register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_by1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_by1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAMMA_BY1_SPEC;
impl crate::RegisterSpec for GAMMA_BY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_by1::R`](R) reader structure"]
impl crate::Readable for GAMMA_BY1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gamma_by1::W`](W) writer structure"]
impl crate::Writable for GAMMA_BY1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_BY1 to value 0x1020_3040"]
impl crate::Resettable for GAMMA_BY1_SPEC {
    const RESET_VALUE: u32 = 0x1020_3040;
}
