#[doc = "Register `GAMMA_GY2` reader"]
pub type R = crate::R<GAMMA_GY2_SPEC>;
#[doc = "Register `GAMMA_GY2` writer"]
pub type W = crate::W<GAMMA_GY2_SPEC>;
#[doc = "Field `GAMMA_G_Y07` reader - this field configures the point 7 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y07_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y07` writer - this field configures the point 7 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y07_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y06` reader - this field configures the point 6 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y06_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y06` writer - this field configures the point 6 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y06_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y05` reader - this field configures the point 5 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y05_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y05` writer - this field configures the point 5 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y05_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y04` reader - this field configures the point 4 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y04_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y04` writer - this field configures the point 4 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y04_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 7 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y07(&self) -> GAMMA_G_Y07_R {
        GAMMA_G_Y07_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 6 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y06(&self) -> GAMMA_G_Y06_R {
        GAMMA_G_Y06_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 5 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y05(&self) -> GAMMA_G_Y05_R {
        GAMMA_G_Y05_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 4 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y04(&self) -> GAMMA_G_Y04_R {
        GAMMA_G_Y04_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_GY2")
            .field(
                "gamma_g_y07",
                &format_args!("{}", self.gamma_g_y07().bits()),
            )
            .field(
                "gamma_g_y06",
                &format_args!("{}", self.gamma_g_y06().bits()),
            )
            .field(
                "gamma_g_y05",
                &format_args!("{}", self.gamma_g_y05().bits()),
            )
            .field(
                "gamma_g_y04",
                &format_args!("{}", self.gamma_g_y04().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GAMMA_GY2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 7 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_y07(&mut self) -> GAMMA_G_Y07_W<GAMMA_GY2_SPEC> {
        GAMMA_G_Y07_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 6 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_y06(&mut self) -> GAMMA_G_Y06_W<GAMMA_GY2_SPEC> {
        GAMMA_G_Y06_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 5 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_y05(&mut self) -> GAMMA_G_Y05_W<GAMMA_GY2_SPEC> {
        GAMMA_G_Y05_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 4 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_y04(&mut self) -> GAMMA_G_Y04_W<GAMMA_GY2_SPEC> {
        GAMMA_G_Y04_W::new(self, 24)
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
#[doc = "point of Y-axis of g channel gamma curve register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_gy2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gy2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAMMA_GY2_SPEC;
impl crate::RegisterSpec for GAMMA_GY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_gy2::R`](R) reader structure"]
impl crate::Readable for GAMMA_GY2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gamma_gy2::W`](W) writer structure"]
impl crate::Writable for GAMMA_GY2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAMMA_GY2 to value 0x5060_7080"]
impl crate::Resettable for GAMMA_GY2_SPEC {
    const RESET_VALUE: Self::Ux = 0x5060_7080;
}
