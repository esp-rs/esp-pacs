///Register `GAMMA_BY4` reader
pub type R = crate::R<GAMMA_BY4_SPEC>;
///Register `GAMMA_BY4` writer
pub type W = crate::W<GAMMA_BY4_SPEC>;
///Field `GAMMA_B_Y0F` reader - this field configures the point 15 of Y-axis of b channel gamma curve
pub type GAMMA_B_Y0F_R = crate::FieldReader;
///Field `GAMMA_B_Y0F` writer - this field configures the point 15 of Y-axis of b channel gamma curve
pub type GAMMA_B_Y0F_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GAMMA_B_Y0E` reader - this field configures the point 14 of Y-axis of b channel gamma curve
pub type GAMMA_B_Y0E_R = crate::FieldReader;
///Field `GAMMA_B_Y0E` writer - this field configures the point 14 of Y-axis of b channel gamma curve
pub type GAMMA_B_Y0E_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GAMMA_B_Y0D` reader - this field configures the point 13 of Y-axis of b channel gamma curve
pub type GAMMA_B_Y0D_R = crate::FieldReader;
///Field `GAMMA_B_Y0D` writer - this field configures the point 13 of Y-axis of b channel gamma curve
pub type GAMMA_B_Y0D_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GAMMA_B_Y0C` reader - this field configures the point 12 of Y-axis of b channel gamma curve
pub type GAMMA_B_Y0C_R = crate::FieldReader;
///Field `GAMMA_B_Y0C` writer - this field configures the point 12 of Y-axis of b channel gamma curve
pub type GAMMA_B_Y0C_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - this field configures the point 15 of Y-axis of b channel gamma curve
    #[inline(always)]
    pub fn gamma_b_y0f(&self) -> GAMMA_B_Y0F_R {
        GAMMA_B_Y0F_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - this field configures the point 14 of Y-axis of b channel gamma curve
    #[inline(always)]
    pub fn gamma_b_y0e(&self) -> GAMMA_B_Y0E_R {
        GAMMA_B_Y0E_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - this field configures the point 13 of Y-axis of b channel gamma curve
    #[inline(always)]
    pub fn gamma_b_y0d(&self) -> GAMMA_B_Y0D_R {
        GAMMA_B_Y0D_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - this field configures the point 12 of Y-axis of b channel gamma curve
    #[inline(always)]
    pub fn gamma_b_y0c(&self) -> GAMMA_B_Y0C_R {
        GAMMA_B_Y0C_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_BY4")
            .field("gamma_b_y0f", &self.gamma_b_y0f())
            .field("gamma_b_y0e", &self.gamma_b_y0e())
            .field("gamma_b_y0d", &self.gamma_b_y0d())
            .field("gamma_b_y0c", &self.gamma_b_y0c())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - this field configures the point 15 of Y-axis of b channel gamma curve
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_y0f(&mut self) -> GAMMA_B_Y0F_W<GAMMA_BY4_SPEC> {
        GAMMA_B_Y0F_W::new(self, 0)
    }
    ///Bits 8:15 - this field configures the point 14 of Y-axis of b channel gamma curve
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_y0e(&mut self) -> GAMMA_B_Y0E_W<GAMMA_BY4_SPEC> {
        GAMMA_B_Y0E_W::new(self, 8)
    }
    ///Bits 16:23 - this field configures the point 13 of Y-axis of b channel gamma curve
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_y0d(&mut self) -> GAMMA_B_Y0D_W<GAMMA_BY4_SPEC> {
        GAMMA_B_Y0D_W::new(self, 16)
    }
    ///Bits 24:31 - this field configures the point 12 of Y-axis of b channel gamma curve
    #[inline(always)]
    #[must_use]
    pub fn gamma_b_y0c(&mut self) -> GAMMA_B_Y0C_W<GAMMA_BY4_SPEC> {
        GAMMA_B_Y0C_W::new(self, 24)
    }
}
/**point of Y-axis of b channel gamma curve register 4

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_by4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_by4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAMMA_BY4_SPEC;
impl crate::RegisterSpec for GAMMA_BY4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gamma_by4::R`](R) reader structure
impl crate::Readable for GAMMA_BY4_SPEC {}
///`write(|w| ..)` method takes [`gamma_by4::W`](W) writer structure
impl crate::Writable for GAMMA_BY4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GAMMA_BY4 to value 0xd0e0_f0ff
impl crate::Resettable for GAMMA_BY4_SPEC {
    const RESET_VALUE: u32 = 0xd0e0_f0ff;
}
