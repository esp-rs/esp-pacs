#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub type R = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub type W = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Field `OUT_SEL` reader - select one of the 256 output to 40 GPIO"]
pub type OUT_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `OUT_SEL` writer - select one of the 256 output to 40 GPIO"]
pub type OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `INV_SEL` reader - invert the output value if you want to revert the output value setting the value to 1"]
pub type INV_SEL_R = crate::BitReader;
#[doc = "Field `INV_SEL` writer - invert the output value if you want to revert the output value setting the value to 1"]
pub type INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN_SEL` reader - weather using the logical oen signal or not using the value setting by the register"]
pub type OEN_SEL_R = crate::BitReader;
#[doc = "Field `OEN_SEL` writer - weather using the logical oen signal or not using the value setting by the register"]
pub type OEN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN_INV_SEL` reader - invert the output enable value if you want to revert the output enable value setting the value to 1"]
pub type OEN_INV_SEL_R = crate::BitReader;
#[doc = "Field `OEN_INV_SEL` writer - invert the output enable value if you want to revert the output enable value setting the value to 1"]
pub type OEN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - select one of the 256 output to 40 GPIO"]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - invert the output value if you want to revert the output value setting the value to 1"]
    #[inline(always)]
    pub fn inv_sel(&self) -> INV_SEL_R {
        INV_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - weather using the logical oen signal or not using the value setting by the register"]
    #[inline(always)]
    pub fn oen_sel(&self) -> OEN_SEL_R {
        OEN_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - invert the output enable value if you want to revert the output enable value setting the value to 1"]
    #[inline(always)]
    pub fn oen_inv_sel(&self) -> OEN_INV_SEL_R {
        OEN_INV_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_OUT_SEL_CFG")
            .field("out_sel", &self.out_sel())
            .field("inv_sel", &self.inv_sel())
            .field("oen_sel", &self.oen_sel())
            .field("oen_inv_sel", &self.oen_inv_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - select one of the 256 output to 40 GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OUT_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        OUT_SEL_W::new(self, 0)
    }
    #[doc = "Bit 9 - invert the output value if you want to revert the output value setting the value to 1"]
    #[inline(always)]
    #[must_use]
    pub fn inv_sel(&mut self) -> INV_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        INV_SEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - weather using the logical oen signal or not using the value setting by the register"]
    #[inline(always)]
    #[must_use]
    pub fn oen_sel(&mut self) -> OEN_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        OEN_SEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - invert the output enable value if you want to revert the output enable value setting the value to 1"]
    #[inline(always)]
    #[must_use]
    pub fn oen_inv_sel(&mut self) -> OEN_INV_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        OEN_INV_SEL_W::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
