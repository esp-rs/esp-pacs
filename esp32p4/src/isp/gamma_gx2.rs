#[doc = "Register `GAMMA_GX2` reader"]
pub type R = crate::R<GAMMA_GX2_SPEC>;
#[doc = "Register `GAMMA_GX2` writer"]
pub type W = crate::W<GAMMA_GX2_SPEC>;
#[doc = "Field `GAMMA_G_X0F` reader - this field configures the point 15 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0F_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0F` writer - this field configures the point 15 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0F_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0E` reader - this field configures the point 14 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0E_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0E` writer - this field configures the point 14 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0E_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0D` reader - this field configures the point 13 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0D_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0D` writer - this field configures the point 13 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0D_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0C` reader - this field configures the point 12 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0C_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0C` writer - this field configures the point 12 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0C_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0B` reader - this field configures the point 11 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0B_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0B` writer - this field configures the point 11 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0B_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X0A` reader - this field configures the point 10 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0A_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X0A` writer - this field configures the point 10 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X0A_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X09` reader - this field configures the point 9 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X09_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X09` writer - this field configures the point 9 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X09_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_G_X08` reader - this field configures the point 8 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X08_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_X08` writer - this field configures the point 8 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_G_X08_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - this field configures the point 15 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0f(&self) -> GAMMA_G_X0F_R {
        GAMMA_G_X0F_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - this field configures the point 14 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0e(&self) -> GAMMA_G_X0E_R {
        GAMMA_G_X0E_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - this field configures the point 13 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0d(&self) -> GAMMA_G_X0D_R {
        GAMMA_G_X0D_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - this field configures the point 12 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0c(&self) -> GAMMA_G_X0C_R {
        GAMMA_G_X0C_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - this field configures the point 11 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0b(&self) -> GAMMA_G_X0B_R {
        GAMMA_G_X0B_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - this field configures the point 10 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x0a(&self) -> GAMMA_G_X0A_R {
        GAMMA_G_X0A_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - this field configures the point 9 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x09(&self) -> GAMMA_G_X09_R {
        GAMMA_G_X09_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - this field configures the point 8 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_g_x08(&self) -> GAMMA_G_X08_R {
        GAMMA_G_X08_R::new(((self.bits >> 21) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_GX2")
            .field("gamma_g_x0f", &self.gamma_g_x0f().bits())
            .field("gamma_g_x0e", &self.gamma_g_x0e().bits())
            .field("gamma_g_x0d", &self.gamma_g_x0d().bits())
            .field("gamma_g_x0c", &self.gamma_g_x0c().bits())
            .field("gamma_g_x0b", &self.gamma_g_x0b().bits())
            .field("gamma_g_x0a", &self.gamma_g_x0a().bits())
            .field("gamma_g_x09", &self.gamma_g_x09().bits())
            .field("gamma_g_x08", &self.gamma_g_x08().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GAMMA_GX2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - this field configures the point 15 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x0f(&mut self) -> GAMMA_G_X0F_W<GAMMA_GX2_SPEC> {
        GAMMA_G_X0F_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - this field configures the point 14 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x0e(&mut self) -> GAMMA_G_X0E_W<GAMMA_GX2_SPEC> {
        GAMMA_G_X0E_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - this field configures the point 13 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x0d(&mut self) -> GAMMA_G_X0D_W<GAMMA_GX2_SPEC> {
        GAMMA_G_X0D_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - this field configures the point 12 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x0c(&mut self) -> GAMMA_G_X0C_W<GAMMA_GX2_SPEC> {
        GAMMA_G_X0C_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - this field configures the point 11 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x0b(&mut self) -> GAMMA_G_X0B_W<GAMMA_GX2_SPEC> {
        GAMMA_G_X0B_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - this field configures the point 10 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x0a(&mut self) -> GAMMA_G_X0A_W<GAMMA_GX2_SPEC> {
        GAMMA_G_X0A_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - this field configures the point 9 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x09(&mut self) -> GAMMA_G_X09_W<GAMMA_GX2_SPEC> {
        GAMMA_G_X09_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - this field configures the point 8 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x08(&mut self) -> GAMMA_G_X08_W<GAMMA_GX2_SPEC> {
        GAMMA_G_X08_W::new(self, 21)
    }
}
#[doc = "point of X-axis of g channel gamma curve register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_gx2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gx2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAMMA_GX2_SPEC;
impl crate::RegisterSpec for GAMMA_GX2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_gx2::R`](R) reader structure"]
impl crate::Readable for GAMMA_GX2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gamma_gx2::W`](W) writer structure"]
impl crate::Writable for GAMMA_GX2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_GX2 to value 0x0092_4924"]
impl crate::Resettable for GAMMA_GX2_SPEC {
    const RESET_VALUE: u32 = 0x0092_4924;
}
