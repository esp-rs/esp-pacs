#[doc = "Register `GAMMA_BY3` reader"]
pub type R = crate::R<GAMMA_BY3_SPEC>;
#[doc = "Register `GAMMA_BY3` writer"]
pub type W = crate::W<GAMMA_BY3_SPEC>;
#[doc = "Field `GAMMA_B_Y0B` reader - this field configures the point 11 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y0B_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y0B` writer - this field configures the point 11 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y0B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y0A` reader - this field configures the point 10 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y0A_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y0A` writer - this field configures the point 10 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y0A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y09` reader - this field configures the point 9 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y09_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y09` writer - this field configures the point 9 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y09_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_B_Y08` reader - this field configures the point 8 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y08_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_Y08` writer - this field configures the point 8 of Y-axis of b channel gamma curve"]
pub type GAMMA_B_Y08_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 11 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0b(&self) -> GAMMA_B_Y0B_R {
        GAMMA_B_Y0B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 10 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y0a(&self) -> GAMMA_B_Y0A_R {
        GAMMA_B_Y0A_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 9 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y09(&self) -> GAMMA_B_Y09_R {
        GAMMA_B_Y09_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 8 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    pub fn gamma_b_y08(&self) -> GAMMA_B_Y08_R {
        GAMMA_B_Y08_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_BY3")
            .field("gamma_b_y0b", &self.gamma_b_y0b().bits())
            .field("gamma_b_y0a", &self.gamma_b_y0a().bits())
            .field("gamma_b_y09", &self.gamma_b_y09().bits())
            .field("gamma_b_y08", &self.gamma_b_y08().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GAMMA_BY3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 11 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_y0b(&mut self) -> GAMMA_B_Y0B_W<GAMMA_BY3_SPEC> {
        GAMMA_B_Y0B_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 10 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_y0a(&mut self) -> GAMMA_B_Y0A_W<GAMMA_BY3_SPEC> {
        GAMMA_B_Y0A_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 9 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_y09(&mut self) -> GAMMA_B_Y09_W<GAMMA_BY3_SPEC> {
        GAMMA_B_Y09_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 8 of Y-axis of b channel gamma curve"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_y08(&mut self) -> GAMMA_B_Y08_W<GAMMA_BY3_SPEC> {
        GAMMA_B_Y08_W::new(self, 24)
    }
}
#[doc = "point of Y-axis of b channel gamma curve register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_by3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_by3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAMMA_BY3_SPEC;
impl crate::RegisterSpec for GAMMA_BY3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_by3::R`](R) reader structure"]
impl crate::Readable for GAMMA_BY3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gamma_by3::W`](W) writer structure"]
impl crate::Writable for GAMMA_BY3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_BY3 to value 0x90a0_b0c0"]
impl crate::Resettable for GAMMA_BY3_SPEC {
    const RESET_VALUE: u32 = 0x90a0_b0c0;
}
