#[doc = "Register `GAMMA_GY1` reader"]
pub type R = crate::R<GAMMA_GY1_SPEC>;
#[doc = "Register `GAMMA_GY1` writer"]
pub type W = crate::W<GAMMA_GY1_SPEC>;
#[doc = "Field `GAMMA_G_Y03` reader - this field configures the point 3 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y03_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y03` writer - this field configures the point 3 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y03_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y02` reader - this field configures the point 2 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y02_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y02` writer - this field configures the point 2 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y02_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y01` reader - this field configures the point 1 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y01_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y01` writer - this field configures the point 1 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y01_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y00` reader - this field configures the point 0 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y00_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y00` writer - this field configures the point 0 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y00_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 3 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y03(&self) -> GAMMA_G_Y03_R {
        GAMMA_G_Y03_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 2 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y02(&self) -> GAMMA_G_Y02_R {
        GAMMA_G_Y02_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 1 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y01(&self) -> GAMMA_G_Y01_R {
        GAMMA_G_Y01_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 0 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y00(&self) -> GAMMA_G_Y00_R {
        GAMMA_G_Y00_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_GY1")
            .field(
                "gamma_g_y03",
                &format_args!("{}", self.gamma_g_y03().bits()),
            )
            .field(
                "gamma_g_y02",
                &format_args!("{}", self.gamma_g_y02().bits()),
            )
            .field(
                "gamma_g_y01",
                &format_args!("{}", self.gamma_g_y01().bits()),
            )
            .field(
                "gamma_g_y00",
                &format_args!("{}", self.gamma_g_y00().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GAMMA_GY1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 3 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_y03(&mut self) -> GAMMA_G_Y03_W<GAMMA_GY1_SPEC> {
        GAMMA_G_Y03_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 2 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_y02(&mut self) -> GAMMA_G_Y02_W<GAMMA_GY1_SPEC> {
        GAMMA_G_Y02_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 1 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_y01(&mut self) -> GAMMA_G_Y01_W<GAMMA_GY1_SPEC> {
        GAMMA_G_Y01_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 0 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_y00(&mut self) -> GAMMA_G_Y00_W<GAMMA_GY1_SPEC> {
        GAMMA_G_Y00_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "point of Y-axis of g channel gamma curve register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_gy1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gy1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAMMA_GY1_SPEC;
impl crate::RegisterSpec for GAMMA_GY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_gy1::R`](R) reader structure"]
impl crate::Readable for GAMMA_GY1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gamma_gy1::W`](W) writer structure"]
impl crate::Writable for GAMMA_GY1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAMMA_GY1 to value 0x1020_3040"]
impl crate::Resettable for GAMMA_GY1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1020_3040;
}
