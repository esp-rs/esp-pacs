#[doc = "Register `MEM_AUX_CTRL_0` reader"]
pub type R = crate::R<MEM_AUX_CTRL_0_SPEC>;
#[doc = "Register `MEM_AUX_CTRL_0` writer"]
pub type W = crate::W<MEM_AUX_CTRL_0_SPEC>;
#[doc = "Field `HEADER_MEM_AUX_CTRL` reader - this field configures the mem_aux of isp input buffer memory"]
pub type HEADER_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `HEADER_MEM_AUX_CTRL` writer - this field configures the mem_aux of isp input buffer memory"]
pub type HEADER_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DPC_LUT_MEM_AUX_CTRL` reader - this field represents this field configures the mem_aux of dpc lut memory"]
pub type DPC_LUT_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `DPC_LUT_MEM_AUX_CTRL` writer - this field represents this field configures the mem_aux of dpc lut memory"]
pub type DPC_LUT_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures the mem_aux of isp input buffer memory"]
    #[inline(always)]
    pub fn header_mem_aux_ctrl(&self) -> HEADER_MEM_AUX_CTRL_R {
        HEADER_MEM_AUX_CTRL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - this field represents this field configures the mem_aux of dpc lut memory"]
    #[inline(always)]
    pub fn dpc_lut_mem_aux_ctrl(&self) -> DPC_LUT_MEM_AUX_CTRL_R {
        DPC_LUT_MEM_AUX_CTRL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_AUX_CTRL_0")
            .field(
                "header_mem_aux_ctrl",
                &format_args!("{}", self.header_mem_aux_ctrl().bits()),
            )
            .field(
                "dpc_lut_mem_aux_ctrl",
                &format_args!("{}", self.dpc_lut_mem_aux_ctrl().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_AUX_CTRL_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures the mem_aux of isp input buffer memory"]
    #[inline(always)]
    #[must_use]
    pub fn header_mem_aux_ctrl(&mut self) -> HEADER_MEM_AUX_CTRL_W<MEM_AUX_CTRL_0_SPEC> {
        HEADER_MEM_AUX_CTRL_W::new(self, 0)
    }
    #[doc = "Bits 16:29 - this field represents this field configures the mem_aux of dpc lut memory"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_lut_mem_aux_ctrl(&mut self) -> DPC_LUT_MEM_AUX_CTRL_W<MEM_AUX_CTRL_0_SPEC> {
        DPC_LUT_MEM_AUX_CTRL_W::new(self, 16)
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
#[doc = "mem aux control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_AUX_CTRL_0_SPEC;
impl crate::RegisterSpec for MEM_AUX_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl_0::R`](R) reader structure"]
impl crate::Readable for MEM_AUX_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl_0::W`](W) writer structure"]
impl crate::Writable for MEM_AUX_CTRL_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL_0 to value 0x1320_1320"]
impl crate::Resettable for MEM_AUX_CTRL_0_SPEC {
    const RESET_VALUE: u32 = 0x1320_1320;
}
