#[doc = "Register `GAMMA_BX2` reader"]
pub type R = crate::R<GAMMA_BX2_SPEC>;
#[doc = "Register `GAMMA_BX2` writer"]
pub type W = crate::W<GAMMA_BX2_SPEC>;
#[doc = "Field `GAMMA_B_X0F` reader - this field configures the point 15 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0F_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_X0F` writer - this field configures the point 15 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0F_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_B_X0E` reader - this field configures the point 14 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0E_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_X0E` writer - this field configures the point 14 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0E_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_B_X0D` reader - this field configures the point 13 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0D_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_X0D` writer - this field configures the point 13 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0D_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_B_X0C` reader - this field configures the point 12 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0C_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_X0C` writer - this field configures the point 12 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0C_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_B_X0B` reader - this field configures the point 11 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0B_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_X0B` writer - this field configures the point 11 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0B_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_B_X0A` reader - this field configures the point 10 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0A_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_X0A` writer - this field configures the point 10 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X0A_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_B_X09` reader - this field configures the point 9 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X09_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_X09` writer - this field configures the point 9 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X09_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_B_X08` reader - this field configures the point 8 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X08_R = crate::FieldReader;
#[doc = "Field `GAMMA_B_X08` writer - this field configures the point 8 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
pub type GAMMA_B_X08_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - this field configures the point 15 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_b_x0f(&self) -> GAMMA_B_X0F_R {
        GAMMA_B_X0F_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - this field configures the point 14 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_b_x0e(&self) -> GAMMA_B_X0E_R {
        GAMMA_B_X0E_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - this field configures the point 13 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_b_x0d(&self) -> GAMMA_B_X0D_R {
        GAMMA_B_X0D_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - this field configures the point 12 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_b_x0c(&self) -> GAMMA_B_X0C_R {
        GAMMA_B_X0C_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - this field configures the point 11 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_b_x0b(&self) -> GAMMA_B_X0B_R {
        GAMMA_B_X0B_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - this field configures the point 10 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_b_x0a(&self) -> GAMMA_B_X0A_R {
        GAMMA_B_X0A_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - this field configures the point 9 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_b_x09(&self) -> GAMMA_B_X09_R {
        GAMMA_B_X09_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - this field configures the point 8 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    pub fn gamma_b_x08(&self) -> GAMMA_B_X08_R {
        GAMMA_B_X08_R::new(((self.bits >> 21) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_BX2")
            .field(
                "gamma_b_x0f",
                &format_args!("{}", self.gamma_b_x0f().bits()),
            )
            .field(
                "gamma_b_x0e",
                &format_args!("{}", self.gamma_b_x0e().bits()),
            )
            .field(
                "gamma_b_x0d",
                &format_args!("{}", self.gamma_b_x0d().bits()),
            )
            .field(
                "gamma_b_x0c",
                &format_args!("{}", self.gamma_b_x0c().bits()),
            )
            .field(
                "gamma_b_x0b",
                &format_args!("{}", self.gamma_b_x0b().bits()),
            )
            .field(
                "gamma_b_x0a",
                &format_args!("{}", self.gamma_b_x0a().bits()),
            )
            .field(
                "gamma_b_x09",
                &format_args!("{}", self.gamma_b_x09().bits()),
            )
            .field(
                "gamma_b_x08",
                &format_args!("{}", self.gamma_b_x08().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GAMMA_BX2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - this field configures the point 15 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_x0f(&mut self) -> GAMMA_B_X0F_W<GAMMA_BX2_SPEC> {
        GAMMA_B_X0F_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - this field configures the point 14 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_x0e(&mut self) -> GAMMA_B_X0E_W<GAMMA_BX2_SPEC> {
        GAMMA_B_X0E_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - this field configures the point 13 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_x0d(&mut self) -> GAMMA_B_X0D_W<GAMMA_BX2_SPEC> {
        GAMMA_B_X0D_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - this field configures the point 12 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_x0c(&mut self) -> GAMMA_B_X0C_W<GAMMA_BX2_SPEC> {
        GAMMA_B_X0C_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - this field configures the point 11 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_x0b(&mut self) -> GAMMA_B_X0B_W<GAMMA_BX2_SPEC> {
        GAMMA_B_X0B_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - this field configures the point 10 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_x0a(&mut self) -> GAMMA_B_X0A_W<GAMMA_BX2_SPEC> {
        GAMMA_B_X0A_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - this field configures the point 9 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_x09(&mut self) -> GAMMA_B_X09_W<GAMMA_BX2_SPEC> {
        GAMMA_B_X09_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - this field configures the point 8 of X-axis of b channel gamma curve, it represents the power of the distance from the previous point"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_x08(&mut self) -> GAMMA_B_X08_W<GAMMA_BX2_SPEC> {
        GAMMA_B_X08_W::new(self, 21)
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
#[doc = "point of X-axis of b channel gamma curve register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_bx2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_bx2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAMMA_BX2_SPEC;
impl crate::RegisterSpec for GAMMA_BX2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_bx2::R`](R) reader structure"]
impl crate::Readable for GAMMA_BX2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gamma_bx2::W`](W) writer structure"]
impl crate::Writable for GAMMA_BX2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAMMA_BX2 to value 0x0092_4924"]
impl crate::Resettable for GAMMA_BX2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0092_4924;
}
