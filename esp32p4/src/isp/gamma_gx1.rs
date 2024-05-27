///Register `GAMMA_GX1` reader
pub type R = crate::R<GAMMA_GX1_SPEC>;
///Register `GAMMA_GX1` writer
pub type W = crate::W<GAMMA_GX1_SPEC>;
///Field `GAMMA_G_X07` reader - this field configures the point 7 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X07_R = crate::FieldReader;
///Field `GAMMA_G_X07` writer - this field configures the point 7 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X07_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GAMMA_G_X06` reader - this field configures the point 6 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X06_R = crate::FieldReader;
///Field `GAMMA_G_X06` writer - this field configures the point 6 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X06_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GAMMA_G_X05` reader - this field configures the point 5 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X05_R = crate::FieldReader;
///Field `GAMMA_G_X05` writer - this field configures the point 5 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X05_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GAMMA_G_X04` reader - this field configures the point 4 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X04_R = crate::FieldReader;
///Field `GAMMA_G_X04` writer - this field configures the point 4 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X04_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GAMMA_G_X03` reader - this field configures the point 3 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X03_R = crate::FieldReader;
///Field `GAMMA_G_X03` writer - this field configures the point 3 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X03_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GAMMA_G_X02` reader - this field configures the point 2 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X02_R = crate::FieldReader;
///Field `GAMMA_G_X02` writer - this field configures the point 2 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X02_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GAMMA_G_X01` reader - this field configures the point 1 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X01_R = crate::FieldReader;
///Field `GAMMA_G_X01` writer - this field configures the point 1 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X01_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GAMMA_G_X00` reader - this field configures the point 0 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X00_R = crate::FieldReader;
///Field `GAMMA_G_X00` writer - this field configures the point 0 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
pub type GAMMA_G_X00_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - this field configures the point 7 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    pub fn gamma_g_x07(&self) -> GAMMA_G_X07_R {
        GAMMA_G_X07_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - this field configures the point 6 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    pub fn gamma_g_x06(&self) -> GAMMA_G_X06_R {
        GAMMA_G_X06_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - this field configures the point 5 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    pub fn gamma_g_x05(&self) -> GAMMA_G_X05_R {
        GAMMA_G_X05_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - this field configures the point 4 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    pub fn gamma_g_x04(&self) -> GAMMA_G_X04_R {
        GAMMA_G_X04_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - this field configures the point 3 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    pub fn gamma_g_x03(&self) -> GAMMA_G_X03_R {
        GAMMA_G_X03_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - this field configures the point 2 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    pub fn gamma_g_x02(&self) -> GAMMA_G_X02_R {
        GAMMA_G_X02_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - this field configures the point 1 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    pub fn gamma_g_x01(&self) -> GAMMA_G_X01_R {
        GAMMA_G_X01_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - this field configures the point 0 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    pub fn gamma_g_x00(&self) -> GAMMA_G_X00_R {
        GAMMA_G_X00_R::new(((self.bits >> 21) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_GX1")
            .field("gamma_g_x07", &self.gamma_g_x07())
            .field("gamma_g_x06", &self.gamma_g_x06())
            .field("gamma_g_x05", &self.gamma_g_x05())
            .field("gamma_g_x04", &self.gamma_g_x04())
            .field("gamma_g_x03", &self.gamma_g_x03())
            .field("gamma_g_x02", &self.gamma_g_x02())
            .field("gamma_g_x01", &self.gamma_g_x01())
            .field("gamma_g_x00", &self.gamma_g_x00())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - this field configures the point 7 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x07(&mut self) -> GAMMA_G_X07_W<GAMMA_GX1_SPEC> {
        GAMMA_G_X07_W::new(self, 0)
    }
    ///Bits 3:5 - this field configures the point 6 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x06(&mut self) -> GAMMA_G_X06_W<GAMMA_GX1_SPEC> {
        GAMMA_G_X06_W::new(self, 3)
    }
    ///Bits 6:8 - this field configures the point 5 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x05(&mut self) -> GAMMA_G_X05_W<GAMMA_GX1_SPEC> {
        GAMMA_G_X05_W::new(self, 6)
    }
    ///Bits 9:11 - this field configures the point 4 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x04(&mut self) -> GAMMA_G_X04_W<GAMMA_GX1_SPEC> {
        GAMMA_G_X04_W::new(self, 9)
    }
    ///Bits 12:14 - this field configures the point 3 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x03(&mut self) -> GAMMA_G_X03_W<GAMMA_GX1_SPEC> {
        GAMMA_G_X03_W::new(self, 12)
    }
    ///Bits 15:17 - this field configures the point 2 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x02(&mut self) -> GAMMA_G_X02_W<GAMMA_GX1_SPEC> {
        GAMMA_G_X02_W::new(self, 15)
    }
    ///Bits 18:20 - this field configures the point 1 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x01(&mut self) -> GAMMA_G_X01_W<GAMMA_GX1_SPEC> {
        GAMMA_G_X01_W::new(self, 18)
    }
    ///Bits 21:23 - this field configures the point 0 of X-axis of g channel gamma curve, it represents the power of the distance from the previous point
    #[inline(always)]
    #[must_use]
    pub fn gamma_g_x00(&mut self) -> GAMMA_G_X00_W<GAMMA_GX1_SPEC> {
        GAMMA_G_X00_W::new(self, 21)
    }
}
/**point of X-axis of g channel gamma curve register 1

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_gx1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gx1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAMMA_GX1_SPEC;
impl crate::RegisterSpec for GAMMA_GX1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gamma_gx1::R`](R) reader structure
impl crate::Readable for GAMMA_GX1_SPEC {}
///`write(|w| ..)` method takes [`gamma_gx1::W`](W) writer structure
impl crate::Writable for GAMMA_GX1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GAMMA_GX1 to value 0x0092_4924
impl crate::Resettable for GAMMA_GX1_SPEC {
    const RESET_VALUE: u32 = 0x0092_4924;
}
