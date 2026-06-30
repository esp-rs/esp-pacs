#[doc = "Register `CSR_CFG` reader"]
pub type R = crate::R<CSR_CFG_SPEC>;
#[doc = "Register `CSR_CFG` writer"]
pub type W = crate::W<CSR_CFG_SPEC>;
#[doc = "Field `FUNC` reader - Used to inicate the type of operation currently being performed"]
pub type FUNC_R = crate::FieldReader;
#[doc = "Field `FUNC` writer - Used to inicate the type of operation currently being performed"]
pub type FUNC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRES` reader - Instruct the cordic calculate cycle number"]
pub type PRES_R = crate::FieldReader;
#[doc = "Field `PRES` writer - Instruct the cordic calculate cycle number"]
pub type PRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCALE` reader - The scaling of Input argument"]
pub type SCALE_R = crate::FieldReader;
#[doc = "Field `SCALE` writer - The scaling of Input argument"]
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WORK_MODE` reader - Select Cordic Module data transfer Mode,2'd0: reg_in,reg_out, no zero mode, 2'd1: dma_in, reg_out, no zero mode, 2'd2: dma_loop, 2'd3: reg_in, reg_out, zero mode"]
pub type WORK_MODE_R = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - Select Cordic Module data transfer Mode,2'd0: reg_in,reg_out, no zero mode, 2'd1: dma_in, reg_out, no zero mode, 2'd2: dma_loop, 2'd3: reg_in, reg_out, zero mode"]
pub type WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES_NUM` reader - Result number"]
pub type RES_NUM_R = crate::BitReader;
#[doc = "Field `RES_NUM` writer - Result number"]
pub type RES_NUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARG_NUM` reader - Argument number"]
pub type ARG_NUM_R = crate::BitReader;
#[doc = "Field `ARG_NUM` writer - Argument number"]
pub type ARG_NUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES_SIZE` reader - Result Size, 0: q1.15, 1: q1.31"]
pub type RES_SIZE_R = crate::BitReader;
#[doc = "Field `RES_SIZE` writer - Result Size, 0: q1.15, 1: q1.31"]
pub type RES_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARG_SIZE` reader - Argument Size, 0: q1.15, 1: q1.31"]
pub type ARG_SIZE_R = crate::BitReader;
#[doc = "Field `ARG_SIZE` writer - Argument Size, 0: q1.15, 1: q1.31"]
pub type ARG_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDATE_FLAG` reader - Used to synchronize register data to the computing core and start calculate"]
pub type UPDATE_FLAG_R = crate::BitReader;
#[doc = "Field `UPDATE_FLAG` writer - Used to synchronize register data to the computing core and start calculate"]
pub type UPDATE_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES_ALL_RDY_FLAG` reader - Used to indicate whether the operation all result has been stored in the result register"]
pub type RES_ALL_RDY_FLAG_R = crate::BitReader;
#[doc = "Field `RES_RDY_FLAG` reader - Used to indicate whether the operation result has been stored in the result register"]
pub type RES_RDY_FLAG_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Used to inicate the type of operation currently being performed"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Instruct the cordic calculate cycle number"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The scaling of Input argument"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Select Cordic Module data transfer Mode,2'd0: reg_in,reg_out, no zero mode, 2'd1: dma_in, reg_out, no zero mode, 2'd2: dma_loop, 2'd3: reg_in, reg_out, zero mode"]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Result number"]
    #[inline(always)]
    pub fn res_num(&self) -> RES_NUM_R {
        RES_NUM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Argument number"]
    #[inline(always)]
    pub fn arg_num(&self) -> ARG_NUM_R {
        ARG_NUM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Result Size, 0: q1.15, 1: q1.31"]
    #[inline(always)]
    pub fn res_size(&self) -> RES_SIZE_R {
        RES_SIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Argument Size, 0: q1.15, 1: q1.31"]
    #[inline(always)]
    pub fn arg_size(&self) -> ARG_SIZE_R {
        ARG_SIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 29 - Used to synchronize register data to the computing core and start calculate"]
    #[inline(always)]
    pub fn update_flag(&self) -> UPDATE_FLAG_R {
        UPDATE_FLAG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Used to indicate whether the operation all result has been stored in the result register"]
    #[inline(always)]
    pub fn res_all_rdy_flag(&self) -> RES_ALL_RDY_FLAG_R {
        RES_ALL_RDY_FLAG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Used to indicate whether the operation result has been stored in the result register"]
    #[inline(always)]
    pub fn res_rdy_flag(&self) -> RES_RDY_FLAG_R {
        RES_RDY_FLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR_CFG")
            .field("func", &self.func())
            .field("pres", &self.pres())
            .field("scale", &self.scale())
            .field("work_mode", &self.work_mode())
            .field("res_num", &self.res_num())
            .field("arg_num", &self.arg_num())
            .field("res_size", &self.res_size())
            .field("arg_size", &self.arg_size())
            .field("update_flag", &self.update_flag())
            .field("res_all_rdy_flag", &self.res_all_rdy_flag())
            .field("res_rdy_flag", &self.res_rdy_flag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Used to inicate the type of operation currently being performed"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W<'_, CSR_CFG_SPEC> {
        FUNC_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Instruct the cordic calculate cycle number"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W<'_, CSR_CFG_SPEC> {
        PRES_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - The scaling of Input argument"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<'_, CSR_CFG_SPEC> {
        SCALE_W::new(self, 8)
    }
    #[doc = "Bits 17:18 - Select Cordic Module data transfer Mode,2'd0: reg_in,reg_out, no zero mode, 2'd1: dma_in, reg_out, no zero mode, 2'd2: dma_loop, 2'd3: reg_in, reg_out, zero mode"]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WORK_MODE_W<'_, CSR_CFG_SPEC> {
        WORK_MODE_W::new(self, 17)
    }
    #[doc = "Bit 19 - Result number"]
    #[inline(always)]
    pub fn res_num(&mut self) -> RES_NUM_W<'_, CSR_CFG_SPEC> {
        RES_NUM_W::new(self, 19)
    }
    #[doc = "Bit 20 - Argument number"]
    #[inline(always)]
    pub fn arg_num(&mut self) -> ARG_NUM_W<'_, CSR_CFG_SPEC> {
        ARG_NUM_W::new(self, 20)
    }
    #[doc = "Bit 21 - Result Size, 0: q1.15, 1: q1.31"]
    #[inline(always)]
    pub fn res_size(&mut self) -> RES_SIZE_W<'_, CSR_CFG_SPEC> {
        RES_SIZE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Argument Size, 0: q1.15, 1: q1.31"]
    #[inline(always)]
    pub fn arg_size(&mut self) -> ARG_SIZE_W<'_, CSR_CFG_SPEC> {
        ARG_SIZE_W::new(self, 22)
    }
    #[doc = "Bit 29 - Used to synchronize register data to the computing core and start calculate"]
    #[inline(always)]
    pub fn update_flag(&mut self) -> UPDATE_FLAG_W<'_, CSR_CFG_SPEC> {
        UPDATE_FLAG_W::new(self, 29)
    }
}
#[doc = "Cordic model config register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR_CFG_SPEC;
impl crate::RegisterSpec for CSR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr_cfg::R`](R) reader structure"]
impl crate::Readable for CSR_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr_cfg::W`](W) writer structure"]
impl crate::Writable for CSR_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR_CFG to value 0"]
impl crate::Resettable for CSR_CFG_SPEC {}
