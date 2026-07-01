#[doc = "Register `C%s` reader"]
pub type R = crate::R<C_SPEC>;
#[doc = "Register `C%s` writer"]
pub type W = crate::W<C_SPEC>;
#[doc = "Field `DQT_TBL_SEL` reader - choose c0 quntization table id (TBD)"]
pub type DQT_TBL_SEL_R = crate::FieldReader;
#[doc = "Field `DQT_TBL_SEL` writer - choose c0 quntization table id (TBD)"]
pub type DQT_TBL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Y_FACTOR` reader - vertical sampling factor of c0"]
pub type Y_FACTOR_R = crate::FieldReader;
#[doc = "Field `Y_FACTOR` writer - vertical sampling factor of c0"]
pub type Y_FACTOR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `X_FACTOR` reader - horizontal sampling factor of c0"]
pub type X_FACTOR_R = crate::FieldReader;
#[doc = "Field `X_FACTOR` writer - horizontal sampling factor of c0"]
pub type X_FACTOR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ID` reader - the identifier of c0"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - the identifier of c0"]
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - choose c0 quntization table id (TBD)"]
    #[inline(always)]
    pub fn dqt_tbl_sel(&self) -> DQT_TBL_SEL_R {
        DQT_TBL_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - vertical sampling factor of c0"]
    #[inline(always)]
    pub fn y_factor(&self) -> Y_FACTOR_R {
        Y_FACTOR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - horizontal sampling factor of c0"]
    #[inline(always)]
    pub fn x_factor(&self) -> X_FACTOR_R {
        X_FACTOR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - the identifier of c0"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C")
            .field("dqt_tbl_sel", &self.dqt_tbl_sel())
            .field("y_factor", &self.y_factor())
            .field("x_factor", &self.x_factor())
            .field("id", &self.id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - choose c0 quntization table id (TBD)"]
    #[inline(always)]
    pub fn dqt_tbl_sel(&mut self) -> DQT_TBL_SEL_W<'_, C_SPEC> {
        DQT_TBL_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - vertical sampling factor of c0"]
    #[inline(always)]
    pub fn y_factor(&mut self) -> Y_FACTOR_W<'_, C_SPEC> {
        Y_FACTOR_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - horizontal sampling factor of c0"]
    #[inline(always)]
    pub fn x_factor(&mut self) -> X_FACTOR_W<'_, C_SPEC> {
        X_FACTOR_W::new(self, 12)
    }
    #[doc = "Bits 16:23 - the identifier of c0"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W<'_, C_SPEC> {
        ID_W::new(self, 16)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C_SPEC;
impl crate::RegisterSpec for C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c::R`](R) reader structure"]
impl crate::Readable for C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c::W`](W) writer structure"]
impl crate::Writable for C_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C%s to value 0x1100"]
impl crate::Resettable for C_SPEC {
    const RESET_VALUE: u32 = 0x1100;
}
