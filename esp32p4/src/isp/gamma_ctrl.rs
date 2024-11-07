#[doc = "Register `GAMMA_CTRL` reader"]
pub type R = crate::R<GAMMA_CTRL_SPEC>;
#[doc = "Register `GAMMA_CTRL` writer"]
pub type W = crate::W<GAMMA_CTRL_SPEC>;
#[doc = "Field `GAMMA_UPDATE` reader - Indicates that gamma register configuration is complete"]
pub type GAMMA_UPDATE_R = crate::BitReader;
#[doc = "Field `GAMMA_UPDATE` writer - Indicates that gamma register configuration is complete"]
pub type GAMMA_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_B_LAST_CORRECT` reader - this bit configures enable of last b segment correcction. 0: disable, 1: enable"]
pub type GAMMA_B_LAST_CORRECT_R = crate::BitReader;
#[doc = "Field `GAMMA_B_LAST_CORRECT` writer - this bit configures enable of last b segment correcction. 0: disable, 1: enable"]
pub type GAMMA_B_LAST_CORRECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_G_LAST_CORRECT` reader - this bit configures enable of last g segment correcction. 0: disable, 1: enable"]
pub type GAMMA_G_LAST_CORRECT_R = crate::BitReader;
#[doc = "Field `GAMMA_G_LAST_CORRECT` writer - this bit configures enable of last g segment correcction. 0: disable, 1: enable"]
pub type GAMMA_G_LAST_CORRECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_R_LAST_CORRECT` reader - this bit configures enable of last r segment correcction. 0: disable, 1: enable"]
pub type GAMMA_R_LAST_CORRECT_R = crate::BitReader;
#[doc = "Field `GAMMA_R_LAST_CORRECT` writer - this bit configures enable of last r segment correcction. 0: disable, 1: enable"]
pub type GAMMA_R_LAST_CORRECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that gamma register configuration is complete"]
    #[inline(always)]
    pub fn gamma_update(&self) -> GAMMA_UPDATE_R {
        GAMMA_UPDATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures enable of last b segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_b_last_correct(&self) -> GAMMA_B_LAST_CORRECT_R {
        GAMMA_B_LAST_CORRECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures enable of last g segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_g_last_correct(&self) -> GAMMA_G_LAST_CORRECT_R {
        GAMMA_G_LAST_CORRECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit configures enable of last r segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_r_last_correct(&self) -> GAMMA_R_LAST_CORRECT_R {
        GAMMA_R_LAST_CORRECT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_CTRL")
            .field("gamma_update", &self.gamma_update())
            .field("gamma_b_last_correct", &self.gamma_b_last_correct())
            .field("gamma_g_last_correct", &self.gamma_g_last_correct())
            .field("gamma_r_last_correct", &self.gamma_r_last_correct())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that gamma register configuration is complete"]
    #[inline(always)]
    pub fn gamma_update(&mut self) -> GAMMA_UPDATE_W<GAMMA_CTRL_SPEC> {
        GAMMA_UPDATE_W::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures enable of last b segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_b_last_correct(&mut self) -> GAMMA_B_LAST_CORRECT_W<GAMMA_CTRL_SPEC> {
        GAMMA_B_LAST_CORRECT_W::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures enable of last g segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_g_last_correct(&mut self) -> GAMMA_G_LAST_CORRECT_W<GAMMA_CTRL_SPEC> {
        GAMMA_G_LAST_CORRECT_W::new(self, 2)
    }
    #[doc = "Bit 3 - this bit configures enable of last r segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_r_last_correct(&mut self) -> GAMMA_R_LAST_CORRECT_W<GAMMA_CTRL_SPEC> {
        GAMMA_R_LAST_CORRECT_W::new(self, 3)
    }
}
#[doc = "gamma control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAMMA_CTRL_SPEC;
impl crate::RegisterSpec for GAMMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_ctrl::R`](R) reader structure"]
impl crate::Readable for GAMMA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gamma_ctrl::W`](W) writer structure"]
impl crate::Writable for GAMMA_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_CTRL to value 0x0e"]
impl crate::Resettable for GAMMA_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0e;
}
