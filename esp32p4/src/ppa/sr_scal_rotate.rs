///Register `SR_SCAL_ROTATE` reader
pub type R = crate::R<SR_SCAL_ROTATE_SPEC>;
///Register `SR_SCAL_ROTATE` writer
pub type W = crate::W<SR_SCAL_ROTATE_SPEC>;
///Field `SR_SCAL_X_INT` reader - The integrated part of scaling coefficient in X direction.
pub type SR_SCAL_X_INT_R = crate::FieldReader;
///Field `SR_SCAL_X_INT` writer - The integrated part of scaling coefficient in X direction.
pub type SR_SCAL_X_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SR_SCAL_X_FRAG` reader - The fragment part of scaling coefficient in X direction.
pub type SR_SCAL_X_FRAG_R = crate::FieldReader;
///Field `SR_SCAL_X_FRAG` writer - The fragment part of scaling coefficient in X direction.
pub type SR_SCAL_X_FRAG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SR_SCAL_Y_INT` reader - The integrated part of scaling coefficient in Y direction.
pub type SR_SCAL_Y_INT_R = crate::FieldReader;
///Field `SR_SCAL_Y_INT` writer - The integrated part of scaling coefficient in Y direction.
pub type SR_SCAL_Y_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SR_SCAL_Y_FRAG` reader - The fragment part of scaling coefficient in Y direction.
pub type SR_SCAL_Y_FRAG_R = crate::FieldReader;
///Field `SR_SCAL_Y_FRAG` writer - The fragment part of scaling coefficient in Y direction.
pub type SR_SCAL_Y_FRAG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SR_ROTATE_ANGLE` reader - The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
pub type SR_ROTATE_ANGLE_R = crate::FieldReader;
///Field `SR_ROTATE_ANGLE` writer - The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
pub type SR_ROTATE_ANGLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SCAL_ROTATE_RST` reader - Write 1 then write 0 to this bit to reset scaling and rotating engine.
pub type SCAL_ROTATE_RST_R = crate::BitReader;
///Field `SCAL_ROTATE_RST` writer - Write 1 then write 0 to this bit to reset scaling and rotating engine.
pub type SCAL_ROTATE_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCAL_ROTATE_START` writer - Write 1 to enable scaling and rotating engine after parameter is configured.
pub type SCAL_ROTATE_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SR_MIRROR_X` reader - Image mirror in X direction. 0: disable, 1: enable
pub type SR_MIRROR_X_R = crate::BitReader;
///Field `SR_MIRROR_X` writer - Image mirror in X direction. 0: disable, 1: enable
pub type SR_MIRROR_X_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SR_MIRROR_Y` reader - Image mirror in Y direction. 0: disable, 1: enable
pub type SR_MIRROR_Y_R = crate::BitReader;
///Field `SR_MIRROR_Y` writer - Image mirror in Y direction. 0: disable, 1: enable
pub type SR_MIRROR_Y_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - The integrated part of scaling coefficient in X direction.
    #[inline(always)]
    pub fn sr_scal_x_int(&self) -> SR_SCAL_X_INT_R {
        SR_SCAL_X_INT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - The fragment part of scaling coefficient in X direction.
    #[inline(always)]
    pub fn sr_scal_x_frag(&self) -> SR_SCAL_X_FRAG_R {
        SR_SCAL_X_FRAG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:19 - The integrated part of scaling coefficient in Y direction.
    #[inline(always)]
    pub fn sr_scal_y_int(&self) -> SR_SCAL_Y_INT_R {
        SR_SCAL_Y_INT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bits 20:23 - The fragment part of scaling coefficient in Y direction.
    #[inline(always)]
    pub fn sr_scal_y_frag(&self) -> SR_SCAL_Y_FRAG_R {
        SR_SCAL_Y_FRAG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:25 - The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
    #[inline(always)]
    pub fn sr_rotate_angle(&self) -> SR_ROTATE_ANGLE_R {
        SR_ROTATE_ANGLE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Write 1 then write 0 to this bit to reset scaling and rotating engine.
    #[inline(always)]
    pub fn scal_rotate_rst(&self) -> SCAL_ROTATE_RST_R {
        SCAL_ROTATE_RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Image mirror in X direction. 0: disable, 1: enable
    #[inline(always)]
    pub fn sr_mirror_x(&self) -> SR_MIRROR_X_R {
        SR_MIRROR_X_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Image mirror in Y direction. 0: disable, 1: enable
    #[inline(always)]
    pub fn sr_mirror_y(&self) -> SR_MIRROR_Y_R {
        SR_MIRROR_Y_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_SCAL_ROTATE")
            .field("sr_scal_x_int", &self.sr_scal_x_int())
            .field("sr_scal_x_frag", &self.sr_scal_x_frag())
            .field("sr_scal_y_int", &self.sr_scal_y_int())
            .field("sr_scal_y_frag", &self.sr_scal_y_frag())
            .field("sr_rotate_angle", &self.sr_rotate_angle())
            .field("scal_rotate_rst", &self.scal_rotate_rst())
            .field("sr_mirror_x", &self.sr_mirror_x())
            .field("sr_mirror_y", &self.sr_mirror_y())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - The integrated part of scaling coefficient in X direction.
    #[inline(always)]
    #[must_use]
    pub fn sr_scal_x_int(&mut self) -> SR_SCAL_X_INT_W<SR_SCAL_ROTATE_SPEC> {
        SR_SCAL_X_INT_W::new(self, 0)
    }
    ///Bits 8:11 - The fragment part of scaling coefficient in X direction.
    #[inline(always)]
    #[must_use]
    pub fn sr_scal_x_frag(&mut self) -> SR_SCAL_X_FRAG_W<SR_SCAL_ROTATE_SPEC> {
        SR_SCAL_X_FRAG_W::new(self, 8)
    }
    ///Bits 12:19 - The integrated part of scaling coefficient in Y direction.
    #[inline(always)]
    #[must_use]
    pub fn sr_scal_y_int(&mut self) -> SR_SCAL_Y_INT_W<SR_SCAL_ROTATE_SPEC> {
        SR_SCAL_Y_INT_W::new(self, 12)
    }
    ///Bits 20:23 - The fragment part of scaling coefficient in Y direction.
    #[inline(always)]
    #[must_use]
    pub fn sr_scal_y_frag(&mut self) -> SR_SCAL_Y_FRAG_W<SR_SCAL_ROTATE_SPEC> {
        SR_SCAL_Y_FRAG_W::new(self, 20)
    }
    ///Bits 24:25 - The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
    #[inline(always)]
    #[must_use]
    pub fn sr_rotate_angle(&mut self) -> SR_ROTATE_ANGLE_W<SR_SCAL_ROTATE_SPEC> {
        SR_ROTATE_ANGLE_W::new(self, 24)
    }
    ///Bit 26 - Write 1 then write 0 to this bit to reset scaling and rotating engine.
    #[inline(always)]
    #[must_use]
    pub fn scal_rotate_rst(&mut self) -> SCAL_ROTATE_RST_W<SR_SCAL_ROTATE_SPEC> {
        SCAL_ROTATE_RST_W::new(self, 26)
    }
    ///Bit 27 - Write 1 to enable scaling and rotating engine after parameter is configured.
    #[inline(always)]
    #[must_use]
    pub fn scal_rotate_start(&mut self) -> SCAL_ROTATE_START_W<SR_SCAL_ROTATE_SPEC> {
        SCAL_ROTATE_START_W::new(self, 27)
    }
    ///Bit 28 - Image mirror in X direction. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn sr_mirror_x(&mut self) -> SR_MIRROR_X_W<SR_SCAL_ROTATE_SPEC> {
        SR_MIRROR_X_W::new(self, 28)
    }
    ///Bit 29 - Image mirror in Y direction. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn sr_mirror_y(&mut self) -> SR_MIRROR_Y_W<SR_SCAL_ROTATE_SPEC> {
        SR_MIRROR_Y_W::new(self, 29)
    }
}
/**Scaling and rotating coefficient register

You can [`read`](crate::generic::Reg::read) this register and get [`sr_scal_rotate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_scal_rotate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SR_SCAL_ROTATE_SPEC;
impl crate::RegisterSpec for SR_SCAL_ROTATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sr_scal_rotate::R`](R) reader structure
impl crate::Readable for SR_SCAL_ROTATE_SPEC {}
///`write(|w| ..)` method takes [`sr_scal_rotate::W`](W) writer structure
impl crate::Writable for SR_SCAL_ROTATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR_SCAL_ROTATE to value 0x1001
impl crate::Resettable for SR_SCAL_ROTATE_SPEC {
    const RESET_VALUE: u32 = 0x1001;
}
