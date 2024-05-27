#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    ver_date: VER_DATE,
    clk_en: CLK_EN,
    cntl: CNTL,
    hsync_cnt: HSYNC_CNT,
    frame_cfg: FRAME_CFG,
    ccm_coef0: CCM_COEF0,
    ccm_coef1: CCM_COEF1,
    ccm_coef3: CCM_COEF3,
    ccm_coef4: CCM_COEF4,
    ccm_coef5: CCM_COEF5,
    bf_matrix_ctrl: BF_MATRIX_CTRL,
    bf_sigma: BF_SIGMA,
    bf_gau0: BF_GAU0,
    bf_gau1: BF_GAU1,
    dpc_ctrl: DPC_CTRL,
    dpc_conf: DPC_CONF,
    dpc_matrix_ctrl: DPC_MATRIX_CTRL,
    dpc_deadpix_cnt: DPC_DEADPIX_CNT,
    lut_cmd: LUT_CMD,
    lut_wdata: LUT_WDATA,
    lut_rdata: LUT_RDATA,
    lsc_tablesize: LSC_TABLESIZE,
    demosaic_matrix_ctrl: DEMOSAIC_MATRIX_CTRL,
    demosaic_grad_ratio: DEMOSAIC_GRAD_RATIO,
    median_matrix_ctrl: MEDIAN_MATRIX_CTRL,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    gamma_ctrl: GAMMA_CTRL,
    gamma_ry1: GAMMA_RY1,
    gamma_ry2: GAMMA_RY2,
    gamma_ry3: GAMMA_RY3,
    gamma_ry4: GAMMA_RY4,
    gamma_gy1: GAMMA_GY1,
    gamma_gy2: GAMMA_GY2,
    gamma_gy3: GAMMA_GY3,
    gamma_gy4: GAMMA_GY4,
    gamma_by1: GAMMA_BY1,
    gamma_by2: GAMMA_BY2,
    gamma_by3: GAMMA_BY3,
    gamma_by4: GAMMA_BY4,
    gamma_rx1: GAMMA_RX1,
    gamma_rx2: GAMMA_RX2,
    gamma_gx1: GAMMA_GX1,
    gamma_gx2: GAMMA_GX2,
    gamma_bx1: GAMMA_BX1,
    gamma_bx2: GAMMA_BX2,
    ae_ctrl: AE_CTRL,
    ae_monitor: AE_MONITOR,
    ae_bx: AE_BX,
    ae_by: AE_BY,
    ae_winpixnum: AE_WINPIXNUM,
    ae_win_reciprocal: AE_WIN_RECIPROCAL,
    ae_block_mean_0: AE_BLOCK_MEAN_0,
    ae_block_mean_1: AE_BLOCK_MEAN_1,
    ae_block_mean_2: AE_BLOCK_MEAN_2,
    ae_block_mean_3: AE_BLOCK_MEAN_3,
    ae_block_mean_4: AE_BLOCK_MEAN_4,
    ae_block_mean_5: AE_BLOCK_MEAN_5,
    ae_block_mean_6: AE_BLOCK_MEAN_6,
    sharp_ctrl0: SHARP_CTRL0,
    sharp_filter0: SHARP_FILTER0,
    sharp_filter1: SHARP_FILTER1,
    sharp_filter2: SHARP_FILTER2,
    sharp_matrix_ctrl: SHARP_MATRIX_CTRL,
    sharp_ctrl1: SHARP_CTRL1,
    dma_cntl: DMA_CNTL,
    dma_raw_data: DMA_RAW_DATA,
    cam_cntl: CAM_CNTL,
    cam_conf: CAM_CONF,
    af_ctrl0: AF_CTRL0,
    af_ctrl1: AF_CTRL1,
    af_gen_th_ctrl: AF_GEN_TH_CTRL,
    af_env_user_th_sum: AF_ENV_USER_TH_SUM,
    af_env_user_th_lum: AF_ENV_USER_TH_LUM,
    af_threshold: AF_THRESHOLD,
    af_hscale_a: AF_HSCALE_A,
    af_vscale_a: AF_VSCALE_A,
    af_hscale_b: AF_HSCALE_B,
    af_vscale_b: AF_VSCALE_B,
    af_hscale_c: AF_HSCALE_C,
    af_vscale_c: AF_VSCALE_C,
    af_sum_a: AF_SUM_A,
    af_sum_b: AF_SUM_B,
    af_sum_c: AF_SUM_C,
    af_lum_a: AF_LUM_A,
    af_lum_b: AF_LUM_B,
    af_lum_c: AF_LUM_C,
    awb_mode: AWB_MODE,
    awb_hscale: AWB_HSCALE,
    awb_vscale: AWB_VSCALE,
    awb_th_lum: AWB_TH_LUM,
    awb_th_rg: AWB_TH_RG,
    awb_th_bg: AWB_TH_BG,
    awb0_white_cnt: AWB0_WHITE_CNT,
    awb0_acc_r: AWB0_ACC_R,
    awb0_acc_g: AWB0_ACC_G,
    awb0_acc_b: AWB0_ACC_B,
    color_ctrl: COLOR_CTRL,
    blc_value: BLC_VALUE,
    blc_ctrl0: BLC_CTRL0,
    blc_ctrl1: BLC_CTRL1,
    blc_ctrl2: BLC_CTRL2,
    blc_mean: BLC_MEAN,
    hist_mode: HIST_MODE,
    hist_coeff: HIST_COEFF,
    hist_offs: HIST_OFFS,
    hist_size: HIST_SIZE,
    hist_seg0: HIST_SEG0,
    hist_seg1: HIST_SEG1,
    hist_seg2: HIST_SEG2,
    hist_seg3: HIST_SEG3,
    hist_weight0: HIST_WEIGHT0,
    hist_weight1: HIST_WEIGHT1,
    hist_weight2: HIST_WEIGHT2,
    hist_weight3: HIST_WEIGHT3,
    hist_weight4: HIST_WEIGHT4,
    hist_weight5: HIST_WEIGHT5,
    hist_weight6: HIST_WEIGHT6,
    hist_bin0: HIST_BIN0,
    hist_bin1: HIST_BIN1,
    hist_bin2: HIST_BIN2,
    hist_bin3: HIST_BIN3,
    hist_bin4: HIST_BIN4,
    hist_bin5: HIST_BIN5,
    hist_bin6: HIST_BIN6,
    hist_bin7: HIST_BIN7,
    hist_bin8: HIST_BIN8,
    hist_bin9: HIST_BIN9,
    hist_bin10: HIST_BIN10,
    hist_bin11: HIST_BIN11,
    hist_bin12: HIST_BIN12,
    hist_bin13: HIST_BIN13,
    hist_bin14: HIST_BIN14,
    hist_bin15: HIST_BIN15,
    mem_aux_ctrl_0: MEM_AUX_CTRL_0,
    mem_aux_ctrl_1: MEM_AUX_CTRL_1,
    mem_aux_ctrl_2: MEM_AUX_CTRL_2,
    mem_aux_ctrl_3: MEM_AUX_CTRL_3,
    mem_aux_ctrl_4: MEM_AUX_CTRL_4,
    yuv_format: YUV_FORMAT,
    rdn_eco_cs: RDN_ECO_CS,
    rdn_eco_low: RDN_ECO_LOW,
    rdn_eco_high: RDN_ECO_HIGH,
}
impl RegisterBlock {
    ///0x00 - version control register
    #[inline(always)]
    pub const fn ver_date(&self) -> &VER_DATE {
        &self.ver_date
    }
    ///0x04 - isp clk control register
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    ///0x08 - isp module enable control register
    #[inline(always)]
    pub const fn cntl(&self) -> &CNTL {
        &self.cntl
    }
    ///0x0c - header hsync interval control register
    #[inline(always)]
    pub const fn hsync_cnt(&self) -> &HSYNC_CNT {
        &self.hsync_cnt
    }
    ///0x10 - frame control parameter register
    #[inline(always)]
    pub const fn frame_cfg(&self) -> &FRAME_CFG {
        &self.frame_cfg
    }
    ///0x14 - ccm coef register 0
    #[inline(always)]
    pub const fn ccm_coef0(&self) -> &CCM_COEF0 {
        &self.ccm_coef0
    }
    ///0x18 - ccm coef register 1
    #[inline(always)]
    pub const fn ccm_coef1(&self) -> &CCM_COEF1 {
        &self.ccm_coef1
    }
    ///0x1c - ccm coef register 3
    #[inline(always)]
    pub const fn ccm_coef3(&self) -> &CCM_COEF3 {
        &self.ccm_coef3
    }
    ///0x20 - ccm coef register 4
    #[inline(always)]
    pub const fn ccm_coef4(&self) -> &CCM_COEF4 {
        &self.ccm_coef4
    }
    ///0x24 - ccm coef register 5
    #[inline(always)]
    pub const fn ccm_coef5(&self) -> &CCM_COEF5 {
        &self.ccm_coef5
    }
    ///0x28 - bf pix2matrix ctrl
    #[inline(always)]
    pub const fn bf_matrix_ctrl(&self) -> &BF_MATRIX_CTRL {
        &self.bf_matrix_ctrl
    }
    ///0x2c - bf denoising level control register
    #[inline(always)]
    pub const fn bf_sigma(&self) -> &BF_SIGMA {
        &self.bf_sigma
    }
    ///0x30 - bf gau template register 0
    #[inline(always)]
    pub const fn bf_gau0(&self) -> &BF_GAU0 {
        &self.bf_gau0
    }
    ///0x34 - bf gau template register 1
    #[inline(always)]
    pub const fn bf_gau1(&self) -> &BF_GAU1 {
        &self.bf_gau1
    }
    ///0x38 - DPC mode control register
    #[inline(always)]
    pub const fn dpc_ctrl(&self) -> &DPC_CTRL {
        &self.dpc_ctrl
    }
    ///0x3c - DPC parameter config register
    #[inline(always)]
    pub const fn dpc_conf(&self) -> &DPC_CONF {
        &self.dpc_conf
    }
    ///0x40 - dpc pix2matrix ctrl
    #[inline(always)]
    pub const fn dpc_matrix_ctrl(&self) -> &DPC_MATRIX_CTRL {
        &self.dpc_matrix_ctrl
    }
    ///0x44 - DPC dead-pix number register
    #[inline(always)]
    pub const fn dpc_deadpix_cnt(&self) -> &DPC_DEADPIX_CNT {
        &self.dpc_deadpix_cnt
    }
    ///0x48 - LUT command register
    #[inline(always)]
    pub const fn lut_cmd(&self) -> &LUT_CMD {
        &self.lut_cmd
    }
    ///0x4c - LUT write data register
    #[inline(always)]
    pub const fn lut_wdata(&self) -> &LUT_WDATA {
        &self.lut_wdata
    }
    ///0x50 - LUT read data register
    #[inline(always)]
    pub const fn lut_rdata(&self) -> &LUT_RDATA {
        &self.lut_rdata
    }
    ///0x54 - LSC point in x-direction
    #[inline(always)]
    pub const fn lsc_tablesize(&self) -> &LSC_TABLESIZE {
        &self.lsc_tablesize
    }
    ///0x58 - demosaic pix2matrix ctrl
    #[inline(always)]
    pub const fn demosaic_matrix_ctrl(&self) -> &DEMOSAIC_MATRIX_CTRL {
        &self.demosaic_matrix_ctrl
    }
    ///0x5c - demosaic gradient select ratio
    #[inline(always)]
    pub const fn demosaic_grad_ratio(&self) -> &DEMOSAIC_GRAD_RATIO {
        &self.demosaic_grad_ratio
    }
    ///0x60 - median pix2matrix ctrl
    #[inline(always)]
    pub const fn median_matrix_ctrl(&self) -> &MEDIAN_MATRIX_CTRL {
        &self.median_matrix_ctrl
    }
    ///0x64 - raw interrupt register
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x68 - masked interrupt register
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x6c - interrupt enable register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x70 - interrupt clear register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x74 - gamma control register
    #[inline(always)]
    pub const fn gamma_ctrl(&self) -> &GAMMA_CTRL {
        &self.gamma_ctrl
    }
    ///0x78 - point of Y-axis of r channel gamma curve register 1
    #[inline(always)]
    pub const fn gamma_ry1(&self) -> &GAMMA_RY1 {
        &self.gamma_ry1
    }
    ///0x7c - point of Y-axis of r channel gamma curve register 2
    #[inline(always)]
    pub const fn gamma_ry2(&self) -> &GAMMA_RY2 {
        &self.gamma_ry2
    }
    ///0x80 - point of Y-axis of r channel gamma curve register 3
    #[inline(always)]
    pub const fn gamma_ry3(&self) -> &GAMMA_RY3 {
        &self.gamma_ry3
    }
    ///0x84 - point of Y-axis of r channel gamma curve register 4
    #[inline(always)]
    pub const fn gamma_ry4(&self) -> &GAMMA_RY4 {
        &self.gamma_ry4
    }
    ///0x88 - point of Y-axis of g channel gamma curve register 1
    #[inline(always)]
    pub const fn gamma_gy1(&self) -> &GAMMA_GY1 {
        &self.gamma_gy1
    }
    ///0x8c - point of Y-axis of g channel gamma curve register 2
    #[inline(always)]
    pub const fn gamma_gy2(&self) -> &GAMMA_GY2 {
        &self.gamma_gy2
    }
    ///0x90 - point of Y-axis of g channel gamma curve register 3
    #[inline(always)]
    pub const fn gamma_gy3(&self) -> &GAMMA_GY3 {
        &self.gamma_gy3
    }
    ///0x94 - point of Y-axis of g channel gamma curve register 4
    #[inline(always)]
    pub const fn gamma_gy4(&self) -> &GAMMA_GY4 {
        &self.gamma_gy4
    }
    ///0x98 - point of Y-axis of b channel gamma curve register 1
    #[inline(always)]
    pub const fn gamma_by1(&self) -> &GAMMA_BY1 {
        &self.gamma_by1
    }
    ///0x9c - point of Y-axis of b channel gamma curve register 2
    #[inline(always)]
    pub const fn gamma_by2(&self) -> &GAMMA_BY2 {
        &self.gamma_by2
    }
    ///0xa0 - point of Y-axis of b channel gamma curve register 3
    #[inline(always)]
    pub const fn gamma_by3(&self) -> &GAMMA_BY3 {
        &self.gamma_by3
    }
    ///0xa4 - point of Y-axis of b channel gamma curve register 4
    #[inline(always)]
    pub const fn gamma_by4(&self) -> &GAMMA_BY4 {
        &self.gamma_by4
    }
    ///0xa8 - point of X-axis of r channel gamma curve register 1
    #[inline(always)]
    pub const fn gamma_rx1(&self) -> &GAMMA_RX1 {
        &self.gamma_rx1
    }
    ///0xac - point of X-axis of r channel gamma curve register 2
    #[inline(always)]
    pub const fn gamma_rx2(&self) -> &GAMMA_RX2 {
        &self.gamma_rx2
    }
    ///0xb0 - point of X-axis of g channel gamma curve register 1
    #[inline(always)]
    pub const fn gamma_gx1(&self) -> &GAMMA_GX1 {
        &self.gamma_gx1
    }
    ///0xb4 - point of X-axis of g channel gamma curve register 2
    #[inline(always)]
    pub const fn gamma_gx2(&self) -> &GAMMA_GX2 {
        &self.gamma_gx2
    }
    ///0xb8 - point of X-axis of b channel gamma curve register 1
    #[inline(always)]
    pub const fn gamma_bx1(&self) -> &GAMMA_BX1 {
        &self.gamma_bx1
    }
    ///0xbc - point of X-axis of b channel gamma curve register 2
    #[inline(always)]
    pub const fn gamma_bx2(&self) -> &GAMMA_BX2 {
        &self.gamma_bx2
    }
    ///0xc0 - ae control register
    #[inline(always)]
    pub const fn ae_ctrl(&self) -> &AE_CTRL {
        &self.ae_ctrl
    }
    ///0xc4 - ae monitor control register
    #[inline(always)]
    pub const fn ae_monitor(&self) -> &AE_MONITOR {
        &self.ae_monitor
    }
    ///0xc8 - ae window register in x-direction
    #[inline(always)]
    pub const fn ae_bx(&self) -> &AE_BX {
        &self.ae_bx
    }
    ///0xcc - ae window register in y-direction
    #[inline(always)]
    pub const fn ae_by(&self) -> &AE_BY {
        &self.ae_by
    }
    ///0xd0 - ae sub-window pix num register
    #[inline(always)]
    pub const fn ae_winpixnum(&self) -> &AE_WINPIXNUM {
        &self.ae_winpixnum
    }
    ///0xd4 - reciprocal of ae sub-window pixel number
    #[inline(always)]
    pub const fn ae_win_reciprocal(&self) -> &AE_WIN_RECIPROCAL {
        &self.ae_win_reciprocal
    }
    ///0xd8 - ae statistic result register 0
    #[inline(always)]
    pub const fn ae_block_mean_0(&self) -> &AE_BLOCK_MEAN_0 {
        &self.ae_block_mean_0
    }
    ///0xdc - ae statistic result register 1
    #[inline(always)]
    pub const fn ae_block_mean_1(&self) -> &AE_BLOCK_MEAN_1 {
        &self.ae_block_mean_1
    }
    ///0xe0 - ae statistic result register 2
    #[inline(always)]
    pub const fn ae_block_mean_2(&self) -> &AE_BLOCK_MEAN_2 {
        &self.ae_block_mean_2
    }
    ///0xe4 - ae statistic result register 3
    #[inline(always)]
    pub const fn ae_block_mean_3(&self) -> &AE_BLOCK_MEAN_3 {
        &self.ae_block_mean_3
    }
    ///0xe8 - ae statistic result register 4
    #[inline(always)]
    pub const fn ae_block_mean_4(&self) -> &AE_BLOCK_MEAN_4 {
        &self.ae_block_mean_4
    }
    ///0xec - ae statistic result register 5
    #[inline(always)]
    pub const fn ae_block_mean_5(&self) -> &AE_BLOCK_MEAN_5 {
        &self.ae_block_mean_5
    }
    ///0xf0 - ae statistic result register 6
    #[inline(always)]
    pub const fn ae_block_mean_6(&self) -> &AE_BLOCK_MEAN_6 {
        &self.ae_block_mean_6
    }
    ///0xf4 - sharp control register 0
    #[inline(always)]
    pub const fn sharp_ctrl0(&self) -> &SHARP_CTRL0 {
        &self.sharp_ctrl0
    }
    ///0xf8 - sharp usm config register 0
    #[inline(always)]
    pub const fn sharp_filter0(&self) -> &SHARP_FILTER0 {
        &self.sharp_filter0
    }
    ///0xfc - sharp usm config register 1
    #[inline(always)]
    pub const fn sharp_filter1(&self) -> &SHARP_FILTER1 {
        &self.sharp_filter1
    }
    ///0x100 - sharp usm config register 2
    #[inline(always)]
    pub const fn sharp_filter2(&self) -> &SHARP_FILTER2 {
        &self.sharp_filter2
    }
    ///0x104 - sharp pix2matrix ctrl
    #[inline(always)]
    pub const fn sharp_matrix_ctrl(&self) -> &SHARP_MATRIX_CTRL {
        &self.sharp_matrix_ctrl
    }
    ///0x108 - sharp control register 1
    #[inline(always)]
    pub const fn sharp_ctrl1(&self) -> &SHARP_CTRL1 {
        &self.sharp_ctrl1
    }
    ///0x10c - isp dma source trans control register
    #[inline(always)]
    pub const fn dma_cntl(&self) -> &DMA_CNTL {
        &self.dma_cntl
    }
    ///0x110 - isp dma source total raw number set register
    #[inline(always)]
    pub const fn dma_raw_data(&self) -> &DMA_RAW_DATA {
        &self.dma_raw_data
    }
    ///0x114 - isp cam source control register
    #[inline(always)]
    pub const fn cam_cntl(&self) -> &CAM_CNTL {
        &self.cam_cntl
    }
    ///0x118 - isp cam source config register
    #[inline(always)]
    pub const fn cam_conf(&self) -> &CAM_CONF {
        &self.cam_conf
    }
    ///0x11c - af control register 0
    #[inline(always)]
    pub const fn af_ctrl0(&self) -> &AF_CTRL0 {
        &self.af_ctrl0
    }
    ///0x120 - af control register 1
    #[inline(always)]
    pub const fn af_ctrl1(&self) -> &AF_CTRL1 {
        &self.af_ctrl1
    }
    ///0x124 - af gen threshold control register
    #[inline(always)]
    pub const fn af_gen_th_ctrl(&self) -> &AF_GEN_TH_CTRL {
        &self.af_gen_th_ctrl
    }
    ///0x128 - af monitor user sum threshold register
    #[inline(always)]
    pub const fn af_env_user_th_sum(&self) -> &AF_ENV_USER_TH_SUM {
        &self.af_env_user_th_sum
    }
    ///0x12c - af monitor user lum threshold register
    #[inline(always)]
    pub const fn af_env_user_th_lum(&self) -> &AF_ENV_USER_TH_LUM {
        &self.af_env_user_th_lum
    }
    ///0x130 - af threshold register
    #[inline(always)]
    pub const fn af_threshold(&self) -> &AF_THRESHOLD {
        &self.af_threshold
    }
    ///0x134 - h-scale of af window a register
    #[inline(always)]
    pub const fn af_hscale_a(&self) -> &AF_HSCALE_A {
        &self.af_hscale_a
    }
    ///0x138 - v-scale of af window a register
    #[inline(always)]
    pub const fn af_vscale_a(&self) -> &AF_VSCALE_A {
        &self.af_vscale_a
    }
    ///0x13c - h-scale of af window b register
    #[inline(always)]
    pub const fn af_hscale_b(&self) -> &AF_HSCALE_B {
        &self.af_hscale_b
    }
    ///0x140 - v-scale of af window b register
    #[inline(always)]
    pub const fn af_vscale_b(&self) -> &AF_VSCALE_B {
        &self.af_vscale_b
    }
    ///0x144 - v-scale of af window c register
    #[inline(always)]
    pub const fn af_hscale_c(&self) -> &AF_HSCALE_C {
        &self.af_hscale_c
    }
    ///0x148 - v-scale of af window c register
    #[inline(always)]
    pub const fn af_vscale_c(&self) -> &AF_VSCALE_C {
        &self.af_vscale_c
    }
    ///0x14c - result of sum of af window a
    #[inline(always)]
    pub const fn af_sum_a(&self) -> &AF_SUM_A {
        &self.af_sum_a
    }
    ///0x150 - result of sum of af window b
    #[inline(always)]
    pub const fn af_sum_b(&self) -> &AF_SUM_B {
        &self.af_sum_b
    }
    ///0x154 - result of sum of af window c
    #[inline(always)]
    pub const fn af_sum_c(&self) -> &AF_SUM_C {
        &self.af_sum_c
    }
    ///0x158 - result of lum of af window a
    #[inline(always)]
    pub const fn af_lum_a(&self) -> &AF_LUM_A {
        &self.af_lum_a
    }
    ///0x15c - result of lum of af window b
    #[inline(always)]
    pub const fn af_lum_b(&self) -> &AF_LUM_B {
        &self.af_lum_b
    }
    ///0x160 - result of lum of af window c
    #[inline(always)]
    pub const fn af_lum_c(&self) -> &AF_LUM_C {
        &self.af_lum_c
    }
    ///0x164 - awb mode control register
    #[inline(always)]
    pub const fn awb_mode(&self) -> &AWB_MODE {
        &self.awb_mode
    }
    ///0x168 - h-scale of awb window
    #[inline(always)]
    pub const fn awb_hscale(&self) -> &AWB_HSCALE {
        &self.awb_hscale
    }
    ///0x16c - v-scale of awb window
    #[inline(always)]
    pub const fn awb_vscale(&self) -> &AWB_VSCALE {
        &self.awb_vscale
    }
    ///0x170 - awb lum threshold register
    #[inline(always)]
    pub const fn awb_th_lum(&self) -> &AWB_TH_LUM {
        &self.awb_th_lum
    }
    ///0x174 - awb r/g threshold register
    #[inline(always)]
    pub const fn awb_th_rg(&self) -> &AWB_TH_RG {
        &self.awb_th_rg
    }
    ///0x178 - awb b/g threshold register
    #[inline(always)]
    pub const fn awb_th_bg(&self) -> &AWB_TH_BG {
        &self.awb_th_bg
    }
    ///0x17c - result of awb white point number
    #[inline(always)]
    pub const fn awb0_white_cnt(&self) -> &AWB0_WHITE_CNT {
        &self.awb0_white_cnt
    }
    ///0x180 - result of accumulate of r channel of all white points
    #[inline(always)]
    pub const fn awb0_acc_r(&self) -> &AWB0_ACC_R {
        &self.awb0_acc_r
    }
    ///0x184 - result of accumulate of g channel of all white points
    #[inline(always)]
    pub const fn awb0_acc_g(&self) -> &AWB0_ACC_G {
        &self.awb0_acc_g
    }
    ///0x188 - result of accumulate of b channel of all white points
    #[inline(always)]
    pub const fn awb0_acc_b(&self) -> &AWB0_ACC_B {
        &self.awb0_acc_b
    }
    ///0x18c - color control register
    #[inline(always)]
    pub const fn color_ctrl(&self) -> &COLOR_CTRL {
        &self.color_ctrl
    }
    ///0x190 - blc black level register
    #[inline(always)]
    pub const fn blc_value(&self) -> &BLC_VALUE {
        &self.blc_value
    }
    ///0x194 - blc stretch control register
    #[inline(always)]
    pub const fn blc_ctrl0(&self) -> &BLC_CTRL0 {
        &self.blc_ctrl0
    }
    ///0x198 - blc window control register
    #[inline(always)]
    pub const fn blc_ctrl1(&self) -> &BLC_CTRL1 {
        &self.blc_ctrl1
    }
    ///0x19c - blc black threshold control register
    #[inline(always)]
    pub const fn blc_ctrl2(&self) -> &BLC_CTRL2 {
        &self.blc_ctrl2
    }
    ///0x1a0 - results of the average of black window
    #[inline(always)]
    pub const fn blc_mean(&self) -> &BLC_MEAN {
        &self.blc_mean
    }
    ///0x1a4 - histogram mode control register
    #[inline(always)]
    pub const fn hist_mode(&self) -> &HIST_MODE {
        &self.hist_mode
    }
    ///0x1a8 - histogram rgb to gray coefficients register
    #[inline(always)]
    pub const fn hist_coeff(&self) -> &HIST_COEFF {
        &self.hist_coeff
    }
    ///0x1ac - histogram window offsets register
    #[inline(always)]
    pub const fn hist_offs(&self) -> &HIST_OFFS {
        &self.hist_offs
    }
    ///0x1b0 - histogram sub-window size register
    #[inline(always)]
    pub const fn hist_size(&self) -> &HIST_SIZE {
        &self.hist_size
    }
    ///0x1b4 - histogram bin control register 0
    #[inline(always)]
    pub const fn hist_seg0(&self) -> &HIST_SEG0 {
        &self.hist_seg0
    }
    ///0x1b8 - histogram bin control register 1
    #[inline(always)]
    pub const fn hist_seg1(&self) -> &HIST_SEG1 {
        &self.hist_seg1
    }
    ///0x1bc - histogram bin control register 2
    #[inline(always)]
    pub const fn hist_seg2(&self) -> &HIST_SEG2 {
        &self.hist_seg2
    }
    ///0x1c0 - histogram bin control register 3
    #[inline(always)]
    pub const fn hist_seg3(&self) -> &HIST_SEG3 {
        &self.hist_seg3
    }
    ///0x1c4 - histogram sub-window weight register 0
    #[inline(always)]
    pub const fn hist_weight0(&self) -> &HIST_WEIGHT0 {
        &self.hist_weight0
    }
    ///0x1c8 - histogram sub-window weight register 1
    #[inline(always)]
    pub const fn hist_weight1(&self) -> &HIST_WEIGHT1 {
        &self.hist_weight1
    }
    ///0x1cc - histogram sub-window weight register 2
    #[inline(always)]
    pub const fn hist_weight2(&self) -> &HIST_WEIGHT2 {
        &self.hist_weight2
    }
    ///0x1d0 - histogram sub-window weight register 3
    #[inline(always)]
    pub const fn hist_weight3(&self) -> &HIST_WEIGHT3 {
        &self.hist_weight3
    }
    ///0x1d4 - histogram sub-window weight register 4
    #[inline(always)]
    pub const fn hist_weight4(&self) -> &HIST_WEIGHT4 {
        &self.hist_weight4
    }
    ///0x1d8 - histogram sub-window weight register 5
    #[inline(always)]
    pub const fn hist_weight5(&self) -> &HIST_WEIGHT5 {
        &self.hist_weight5
    }
    ///0x1dc - histogram sub-window weight register 6
    #[inline(always)]
    pub const fn hist_weight6(&self) -> &HIST_WEIGHT6 {
        &self.hist_weight6
    }
    ///0x1e0 - result of histogram bin 0
    #[inline(always)]
    pub const fn hist_bin0(&self) -> &HIST_BIN0 {
        &self.hist_bin0
    }
    ///0x1e4 - result of histogram bin 1
    #[inline(always)]
    pub const fn hist_bin1(&self) -> &HIST_BIN1 {
        &self.hist_bin1
    }
    ///0x1e8 - result of histogram bin 2
    #[inline(always)]
    pub const fn hist_bin2(&self) -> &HIST_BIN2 {
        &self.hist_bin2
    }
    ///0x1ec - result of histogram bin 3
    #[inline(always)]
    pub const fn hist_bin3(&self) -> &HIST_BIN3 {
        &self.hist_bin3
    }
    ///0x1f0 - result of histogram bin 4
    #[inline(always)]
    pub const fn hist_bin4(&self) -> &HIST_BIN4 {
        &self.hist_bin4
    }
    ///0x1f4 - result of histogram bin 5
    #[inline(always)]
    pub const fn hist_bin5(&self) -> &HIST_BIN5 {
        &self.hist_bin5
    }
    ///0x1f8 - result of histogram bin 6
    #[inline(always)]
    pub const fn hist_bin6(&self) -> &HIST_BIN6 {
        &self.hist_bin6
    }
    ///0x1fc - result of histogram bin 7
    #[inline(always)]
    pub const fn hist_bin7(&self) -> &HIST_BIN7 {
        &self.hist_bin7
    }
    ///0x200 - result of histogram bin 8
    #[inline(always)]
    pub const fn hist_bin8(&self) -> &HIST_BIN8 {
        &self.hist_bin8
    }
    ///0x204 - result of histogram bin 9
    #[inline(always)]
    pub const fn hist_bin9(&self) -> &HIST_BIN9 {
        &self.hist_bin9
    }
    ///0x208 - result of histogram bin 10
    #[inline(always)]
    pub const fn hist_bin10(&self) -> &HIST_BIN10 {
        &self.hist_bin10
    }
    ///0x20c - result of histogram bin 11
    #[inline(always)]
    pub const fn hist_bin11(&self) -> &HIST_BIN11 {
        &self.hist_bin11
    }
    ///0x210 - result of histogram bin 12
    #[inline(always)]
    pub const fn hist_bin12(&self) -> &HIST_BIN12 {
        &self.hist_bin12
    }
    ///0x214 - result of histogram bin 13
    #[inline(always)]
    pub const fn hist_bin13(&self) -> &HIST_BIN13 {
        &self.hist_bin13
    }
    ///0x218 - result of histogram bin 14
    #[inline(always)]
    pub const fn hist_bin14(&self) -> &HIST_BIN14 {
        &self.hist_bin14
    }
    ///0x21c - result of histogram bin 15
    #[inline(always)]
    pub const fn hist_bin15(&self) -> &HIST_BIN15 {
        &self.hist_bin15
    }
    ///0x220 - mem aux control register 0
    #[inline(always)]
    pub const fn mem_aux_ctrl_0(&self) -> &MEM_AUX_CTRL_0 {
        &self.mem_aux_ctrl_0
    }
    ///0x224 - mem aux control register 1
    #[inline(always)]
    pub const fn mem_aux_ctrl_1(&self) -> &MEM_AUX_CTRL_1 {
        &self.mem_aux_ctrl_1
    }
    ///0x228 - mem aux control register 2
    #[inline(always)]
    pub const fn mem_aux_ctrl_2(&self) -> &MEM_AUX_CTRL_2 {
        &self.mem_aux_ctrl_2
    }
    ///0x22c - mem aux control register 3
    #[inline(always)]
    pub const fn mem_aux_ctrl_3(&self) -> &MEM_AUX_CTRL_3 {
        &self.mem_aux_ctrl_3
    }
    ///0x230 - mem aux control register 4
    #[inline(always)]
    pub const fn mem_aux_ctrl_4(&self) -> &MEM_AUX_CTRL_4 {
        &self.mem_aux_ctrl_4
    }
    ///0x234 - yuv format control register
    #[inline(always)]
    pub const fn yuv_format(&self) -> &YUV_FORMAT {
        &self.yuv_format
    }
    ///0x238 - rdn eco cs register
    #[inline(always)]
    pub const fn rdn_eco_cs(&self) -> &RDN_ECO_CS {
        &self.rdn_eco_cs
    }
    ///0x23c - rdn eco all low register
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> &RDN_ECO_LOW {
        &self.rdn_eco_low
    }
    ///0x240 - rdn eco all high register
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> &RDN_ECO_HIGH {
        &self.rdn_eco_high
    }
}
/**VER_DATE (rw) register accessor: version control register

You can [`read`](crate::generic::Reg::read) this register and get [`ver_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ver_date`] module*/
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
///version control register
pub mod ver_date;
/**CLK_EN (rw) register accessor: isp clk control register

You can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_en`] module*/
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
///isp clk control register
pub mod clk_en;
/**CNTL (rw) register accessor: isp module enable control register

You can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cntl`] module*/
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
///isp module enable control register
pub mod cntl;
/**HSYNC_CNT (rw) register accessor: header hsync interval control register

You can [`read`](crate::generic::Reg::read) this register and get [`hsync_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsync_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hsync_cnt`] module*/
pub type HSYNC_CNT = crate::Reg<hsync_cnt::HSYNC_CNT_SPEC>;
///header hsync interval control register
pub mod hsync_cnt;
/**FRAME_CFG (rw) register accessor: frame control parameter register

You can [`read`](crate::generic::Reg::read) this register and get [`frame_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frame_cfg`] module*/
pub type FRAME_CFG = crate::Reg<frame_cfg::FRAME_CFG_SPEC>;
///frame control parameter register
pub mod frame_cfg;
/**CCM_COEF0 (rw) register accessor: ccm coef register 0

You can [`read`](crate::generic::Reg::read) this register and get [`ccm_coef0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccm_coef0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccm_coef0`] module*/
pub type CCM_COEF0 = crate::Reg<ccm_coef0::CCM_COEF0_SPEC>;
///ccm coef register 0
pub mod ccm_coef0;
/**CCM_COEF1 (rw) register accessor: ccm coef register 1

You can [`read`](crate::generic::Reg::read) this register and get [`ccm_coef1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccm_coef1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccm_coef1`] module*/
pub type CCM_COEF1 = crate::Reg<ccm_coef1::CCM_COEF1_SPEC>;
///ccm coef register 1
pub mod ccm_coef1;
/**CCM_COEF3 (rw) register accessor: ccm coef register 3

You can [`read`](crate::generic::Reg::read) this register and get [`ccm_coef3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccm_coef3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccm_coef3`] module*/
pub type CCM_COEF3 = crate::Reg<ccm_coef3::CCM_COEF3_SPEC>;
///ccm coef register 3
pub mod ccm_coef3;
/**CCM_COEF4 (rw) register accessor: ccm coef register 4

You can [`read`](crate::generic::Reg::read) this register and get [`ccm_coef4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccm_coef4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccm_coef4`] module*/
pub type CCM_COEF4 = crate::Reg<ccm_coef4::CCM_COEF4_SPEC>;
///ccm coef register 4
pub mod ccm_coef4;
/**CCM_COEF5 (rw) register accessor: ccm coef register 5

You can [`read`](crate::generic::Reg::read) this register and get [`ccm_coef5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccm_coef5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccm_coef5`] module*/
pub type CCM_COEF5 = crate::Reg<ccm_coef5::CCM_COEF5_SPEC>;
///ccm coef register 5
pub mod ccm_coef5;
/**BF_MATRIX_CTRL (rw) register accessor: bf pix2matrix ctrl

You can [`read`](crate::generic::Reg::read) this register and get [`bf_matrix_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bf_matrix_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bf_matrix_ctrl`] module*/
pub type BF_MATRIX_CTRL = crate::Reg<bf_matrix_ctrl::BF_MATRIX_CTRL_SPEC>;
///bf pix2matrix ctrl
pub mod bf_matrix_ctrl;
/**BF_SIGMA (rw) register accessor: bf denoising level control register

You can [`read`](crate::generic::Reg::read) this register and get [`bf_sigma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bf_sigma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bf_sigma`] module*/
pub type BF_SIGMA = crate::Reg<bf_sigma::BF_SIGMA_SPEC>;
///bf denoising level control register
pub mod bf_sigma;
/**BF_GAU0 (rw) register accessor: bf gau template register 0

You can [`read`](crate::generic::Reg::read) this register and get [`bf_gau0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bf_gau0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bf_gau0`] module*/
pub type BF_GAU0 = crate::Reg<bf_gau0::BF_GAU0_SPEC>;
///bf gau template register 0
pub mod bf_gau0;
/**BF_GAU1 (rw) register accessor: bf gau template register 1

You can [`read`](crate::generic::Reg::read) this register and get [`bf_gau1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bf_gau1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bf_gau1`] module*/
pub type BF_GAU1 = crate::Reg<bf_gau1::BF_GAU1_SPEC>;
///bf gau template register 1
pub mod bf_gau1;
/**DPC_CTRL (rw) register accessor: DPC mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`dpc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpc_ctrl`] module*/
pub type DPC_CTRL = crate::Reg<dpc_ctrl::DPC_CTRL_SPEC>;
///DPC mode control register
pub mod dpc_ctrl;
/**DPC_CONF (rw) register accessor: DPC parameter config register

You can [`read`](crate::generic::Reg::read) this register and get [`dpc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpc_conf`] module*/
pub type DPC_CONF = crate::Reg<dpc_conf::DPC_CONF_SPEC>;
///DPC parameter config register
pub mod dpc_conf;
/**DPC_MATRIX_CTRL (rw) register accessor: dpc pix2matrix ctrl

You can [`read`](crate::generic::Reg::read) this register and get [`dpc_matrix_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpc_matrix_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpc_matrix_ctrl`] module*/
pub type DPC_MATRIX_CTRL = crate::Reg<dpc_matrix_ctrl::DPC_MATRIX_CTRL_SPEC>;
///dpc pix2matrix ctrl
pub mod dpc_matrix_ctrl;
/**DPC_DEADPIX_CNT (r) register accessor: DPC dead-pix number register

You can [`read`](crate::generic::Reg::read) this register and get [`dpc_deadpix_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpc_deadpix_cnt`] module*/
pub type DPC_DEADPIX_CNT = crate::Reg<dpc_deadpix_cnt::DPC_DEADPIX_CNT_SPEC>;
///DPC dead-pix number register
pub mod dpc_deadpix_cnt;
/**LUT_CMD (w) register accessor: LUT command register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut_cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lut_cmd`] module*/
pub type LUT_CMD = crate::Reg<lut_cmd::LUT_CMD_SPEC>;
///LUT command register
pub mod lut_cmd;
/**LUT_WDATA (rw) register accessor: LUT write data register

You can [`read`](crate::generic::Reg::read) this register and get [`lut_wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut_wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lut_wdata`] module*/
pub type LUT_WDATA = crate::Reg<lut_wdata::LUT_WDATA_SPEC>;
///LUT write data register
pub mod lut_wdata;
/**LUT_RDATA (r) register accessor: LUT read data register

You can [`read`](crate::generic::Reg::read) this register and get [`lut_rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lut_rdata`] module*/
pub type LUT_RDATA = crate::Reg<lut_rdata::LUT_RDATA_SPEC>;
///LUT read data register
pub mod lut_rdata;
/**LSC_TABLESIZE (rw) register accessor: LSC point in x-direction

You can [`read`](crate::generic::Reg::read) this register and get [`lsc_tablesize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_tablesize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lsc_tablesize`] module*/
pub type LSC_TABLESIZE = crate::Reg<lsc_tablesize::LSC_TABLESIZE_SPEC>;
///LSC point in x-direction
pub mod lsc_tablesize;
/**DEMOSAIC_MATRIX_CTRL (rw) register accessor: demosaic pix2matrix ctrl

You can [`read`](crate::generic::Reg::read) this register and get [`demosaic_matrix_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demosaic_matrix_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@demosaic_matrix_ctrl`] module*/
pub type DEMOSAIC_MATRIX_CTRL = crate::Reg<demosaic_matrix_ctrl::DEMOSAIC_MATRIX_CTRL_SPEC>;
///demosaic pix2matrix ctrl
pub mod demosaic_matrix_ctrl;
/**DEMOSAIC_GRAD_RATIO (rw) register accessor: demosaic gradient select ratio

You can [`read`](crate::generic::Reg::read) this register and get [`demosaic_grad_ratio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demosaic_grad_ratio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@demosaic_grad_ratio`] module*/
pub type DEMOSAIC_GRAD_RATIO = crate::Reg<demosaic_grad_ratio::DEMOSAIC_GRAD_RATIO_SPEC>;
///demosaic gradient select ratio
pub mod demosaic_grad_ratio;
/**MEDIAN_MATRIX_CTRL (rw) register accessor: median pix2matrix ctrl

You can [`read`](crate::generic::Reg::read) this register and get [`median_matrix_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`median_matrix_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@median_matrix_ctrl`] module*/
pub type MEDIAN_MATRIX_CTRL = crate::Reg<median_matrix_ctrl::MEDIAN_MATRIX_CTRL_SPEC>;
///median pix2matrix ctrl
pub mod median_matrix_ctrl;
/**INT_RAW (r) register accessor: raw interrupt register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///raw interrupt register
pub mod int_raw;
/**INT_ST (r) register accessor: masked interrupt register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///masked interrupt register
pub mod int_st;
/**INT_ENA (rw) register accessor: interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///interrupt enable register
pub mod int_ena;
/**INT_CLR (w) register accessor: interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///interrupt clear register
pub mod int_clr;
/**GAMMA_CTRL (rw) register accessor: gamma control register

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_ctrl`] module*/
pub type GAMMA_CTRL = crate::Reg<gamma_ctrl::GAMMA_CTRL_SPEC>;
///gamma control register
pub mod gamma_ctrl;
/**GAMMA_RY1 (rw) register accessor: point of Y-axis of r channel gamma curve register 1

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_ry1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_ry1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_ry1`] module*/
pub type GAMMA_RY1 = crate::Reg<gamma_ry1::GAMMA_RY1_SPEC>;
///point of Y-axis of r channel gamma curve register 1
pub mod gamma_ry1;
/**GAMMA_RY2 (rw) register accessor: point of Y-axis of r channel gamma curve register 2

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_ry2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_ry2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_ry2`] module*/
pub type GAMMA_RY2 = crate::Reg<gamma_ry2::GAMMA_RY2_SPEC>;
///point of Y-axis of r channel gamma curve register 2
pub mod gamma_ry2;
/**GAMMA_RY3 (rw) register accessor: point of Y-axis of r channel gamma curve register 3

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_ry3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_ry3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_ry3`] module*/
pub type GAMMA_RY3 = crate::Reg<gamma_ry3::GAMMA_RY3_SPEC>;
///point of Y-axis of r channel gamma curve register 3
pub mod gamma_ry3;
/**GAMMA_RY4 (rw) register accessor: point of Y-axis of r channel gamma curve register 4

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_ry4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_ry4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_ry4`] module*/
pub type GAMMA_RY4 = crate::Reg<gamma_ry4::GAMMA_RY4_SPEC>;
///point of Y-axis of r channel gamma curve register 4
pub mod gamma_ry4;
/**GAMMA_GY1 (rw) register accessor: point of Y-axis of g channel gamma curve register 1

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_gy1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gy1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_gy1`] module*/
pub type GAMMA_GY1 = crate::Reg<gamma_gy1::GAMMA_GY1_SPEC>;
///point of Y-axis of g channel gamma curve register 1
pub mod gamma_gy1;
/**GAMMA_GY2 (rw) register accessor: point of Y-axis of g channel gamma curve register 2

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_gy2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gy2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_gy2`] module*/
pub type GAMMA_GY2 = crate::Reg<gamma_gy2::GAMMA_GY2_SPEC>;
///point of Y-axis of g channel gamma curve register 2
pub mod gamma_gy2;
/**GAMMA_GY3 (rw) register accessor: point of Y-axis of g channel gamma curve register 3

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_gy3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gy3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_gy3`] module*/
pub type GAMMA_GY3 = crate::Reg<gamma_gy3::GAMMA_GY3_SPEC>;
///point of Y-axis of g channel gamma curve register 3
pub mod gamma_gy3;
/**GAMMA_GY4 (rw) register accessor: point of Y-axis of g channel gamma curve register 4

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_gy4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gy4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_gy4`] module*/
pub type GAMMA_GY4 = crate::Reg<gamma_gy4::GAMMA_GY4_SPEC>;
///point of Y-axis of g channel gamma curve register 4
pub mod gamma_gy4;
/**GAMMA_BY1 (rw) register accessor: point of Y-axis of b channel gamma curve register 1

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_by1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_by1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_by1`] module*/
pub type GAMMA_BY1 = crate::Reg<gamma_by1::GAMMA_BY1_SPEC>;
///point of Y-axis of b channel gamma curve register 1
pub mod gamma_by1;
/**GAMMA_BY2 (rw) register accessor: point of Y-axis of b channel gamma curve register 2

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_by2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_by2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_by2`] module*/
pub type GAMMA_BY2 = crate::Reg<gamma_by2::GAMMA_BY2_SPEC>;
///point of Y-axis of b channel gamma curve register 2
pub mod gamma_by2;
/**GAMMA_BY3 (rw) register accessor: point of Y-axis of b channel gamma curve register 3

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_by3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_by3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_by3`] module*/
pub type GAMMA_BY3 = crate::Reg<gamma_by3::GAMMA_BY3_SPEC>;
///point of Y-axis of b channel gamma curve register 3
pub mod gamma_by3;
/**GAMMA_BY4 (rw) register accessor: point of Y-axis of b channel gamma curve register 4

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_by4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_by4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_by4`] module*/
pub type GAMMA_BY4 = crate::Reg<gamma_by4::GAMMA_BY4_SPEC>;
///point of Y-axis of b channel gamma curve register 4
pub mod gamma_by4;
/**GAMMA_RX1 (rw) register accessor: point of X-axis of r channel gamma curve register 1

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_rx1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_rx1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_rx1`] module*/
pub type GAMMA_RX1 = crate::Reg<gamma_rx1::GAMMA_RX1_SPEC>;
///point of X-axis of r channel gamma curve register 1
pub mod gamma_rx1;
/**GAMMA_RX2 (rw) register accessor: point of X-axis of r channel gamma curve register 2

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_rx2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_rx2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_rx2`] module*/
pub type GAMMA_RX2 = crate::Reg<gamma_rx2::GAMMA_RX2_SPEC>;
///point of X-axis of r channel gamma curve register 2
pub mod gamma_rx2;
/**GAMMA_GX1 (rw) register accessor: point of X-axis of g channel gamma curve register 1

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_gx1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gx1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_gx1`] module*/
pub type GAMMA_GX1 = crate::Reg<gamma_gx1::GAMMA_GX1_SPEC>;
///point of X-axis of g channel gamma curve register 1
pub mod gamma_gx1;
/**GAMMA_GX2 (rw) register accessor: point of X-axis of g channel gamma curve register 2

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_gx2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_gx2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_gx2`] module*/
pub type GAMMA_GX2 = crate::Reg<gamma_gx2::GAMMA_GX2_SPEC>;
///point of X-axis of g channel gamma curve register 2
pub mod gamma_gx2;
/**GAMMA_BX1 (rw) register accessor: point of X-axis of b channel gamma curve register 1

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_bx1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_bx1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_bx1`] module*/
pub type GAMMA_BX1 = crate::Reg<gamma_bx1::GAMMA_BX1_SPEC>;
///point of X-axis of b channel gamma curve register 1
pub mod gamma_bx1;
/**GAMMA_BX2 (rw) register accessor: point of X-axis of b channel gamma curve register 2

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_bx2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_bx2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gamma_bx2`] module*/
pub type GAMMA_BX2 = crate::Reg<gamma_bx2::GAMMA_BX2_SPEC>;
///point of X-axis of b channel gamma curve register 2
pub mod gamma_bx2;
/**AE_CTRL (rw) register accessor: ae control register

You can [`read`](crate::generic::Reg::read) this register and get [`ae_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_ctrl`] module*/
pub type AE_CTRL = crate::Reg<ae_ctrl::AE_CTRL_SPEC>;
///ae control register
pub mod ae_ctrl;
/**AE_MONITOR (rw) register accessor: ae monitor control register

You can [`read`](crate::generic::Reg::read) this register and get [`ae_monitor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_monitor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_monitor`] module*/
pub type AE_MONITOR = crate::Reg<ae_monitor::AE_MONITOR_SPEC>;
///ae monitor control register
pub mod ae_monitor;
/**AE_BX (rw) register accessor: ae window register in x-direction

You can [`read`](crate::generic::Reg::read) this register and get [`ae_bx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_bx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_bx`] module*/
pub type AE_BX = crate::Reg<ae_bx::AE_BX_SPEC>;
///ae window register in x-direction
pub mod ae_bx;
/**AE_BY (rw) register accessor: ae window register in y-direction

You can [`read`](crate::generic::Reg::read) this register and get [`ae_by::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_by::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_by`] module*/
pub type AE_BY = crate::Reg<ae_by::AE_BY_SPEC>;
///ae window register in y-direction
pub mod ae_by;
/**AE_WINPIXNUM (rw) register accessor: ae sub-window pix num register

You can [`read`](crate::generic::Reg::read) this register and get [`ae_winpixnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_winpixnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_winpixnum`] module*/
pub type AE_WINPIXNUM = crate::Reg<ae_winpixnum::AE_WINPIXNUM_SPEC>;
///ae sub-window pix num register
pub mod ae_winpixnum;
/**AE_WIN_RECIPROCAL (rw) register accessor: reciprocal of ae sub-window pixel number

You can [`read`](crate::generic::Reg::read) this register and get [`ae_win_reciprocal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_win_reciprocal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_win_reciprocal`] module*/
pub type AE_WIN_RECIPROCAL = crate::Reg<ae_win_reciprocal::AE_WIN_RECIPROCAL_SPEC>;
///reciprocal of ae sub-window pixel number
pub mod ae_win_reciprocal;
/**AE_BLOCK_MEAN_0 (r) register accessor: ae statistic result register 0

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_block_mean_0`] module*/
pub type AE_BLOCK_MEAN_0 = crate::Reg<ae_block_mean_0::AE_BLOCK_MEAN_0_SPEC>;
///ae statistic result register 0
pub mod ae_block_mean_0;
/**AE_BLOCK_MEAN_1 (r) register accessor: ae statistic result register 1

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_block_mean_1`] module*/
pub type AE_BLOCK_MEAN_1 = crate::Reg<ae_block_mean_1::AE_BLOCK_MEAN_1_SPEC>;
///ae statistic result register 1
pub mod ae_block_mean_1;
/**AE_BLOCK_MEAN_2 (r) register accessor: ae statistic result register 2

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_block_mean_2`] module*/
pub type AE_BLOCK_MEAN_2 = crate::Reg<ae_block_mean_2::AE_BLOCK_MEAN_2_SPEC>;
///ae statistic result register 2
pub mod ae_block_mean_2;
/**AE_BLOCK_MEAN_3 (r) register accessor: ae statistic result register 3

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_block_mean_3`] module*/
pub type AE_BLOCK_MEAN_3 = crate::Reg<ae_block_mean_3::AE_BLOCK_MEAN_3_SPEC>;
///ae statistic result register 3
pub mod ae_block_mean_3;
/**AE_BLOCK_MEAN_4 (r) register accessor: ae statistic result register 4

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_block_mean_4`] module*/
pub type AE_BLOCK_MEAN_4 = crate::Reg<ae_block_mean_4::AE_BLOCK_MEAN_4_SPEC>;
///ae statistic result register 4
pub mod ae_block_mean_4;
/**AE_BLOCK_MEAN_5 (r) register accessor: ae statistic result register 5

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_block_mean_5`] module*/
pub type AE_BLOCK_MEAN_5 = crate::Reg<ae_block_mean_5::AE_BLOCK_MEAN_5_SPEC>;
///ae statistic result register 5
pub mod ae_block_mean_5;
/**AE_BLOCK_MEAN_6 (r) register accessor: ae statistic result register 6

You can [`read`](crate::generic::Reg::read) this register and get [`ae_block_mean_6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ae_block_mean_6`] module*/
pub type AE_BLOCK_MEAN_6 = crate::Reg<ae_block_mean_6::AE_BLOCK_MEAN_6_SPEC>;
///ae statistic result register 6
pub mod ae_block_mean_6;
/**SHARP_CTRL0 (rw) register accessor: sharp control register 0

You can [`read`](crate::generic::Reg::read) this register and get [`sharp_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sharp_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sharp_ctrl0`] module*/
pub type SHARP_CTRL0 = crate::Reg<sharp_ctrl0::SHARP_CTRL0_SPEC>;
///sharp control register 0
pub mod sharp_ctrl0;
/**SHARP_FILTER0 (rw) register accessor: sharp usm config register 0

You can [`read`](crate::generic::Reg::read) this register and get [`sharp_filter0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sharp_filter0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sharp_filter0`] module*/
pub type SHARP_FILTER0 = crate::Reg<sharp_filter0::SHARP_FILTER0_SPEC>;
///sharp usm config register 0
pub mod sharp_filter0;
/**SHARP_FILTER1 (rw) register accessor: sharp usm config register 1

You can [`read`](crate::generic::Reg::read) this register and get [`sharp_filter1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sharp_filter1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sharp_filter1`] module*/
pub type SHARP_FILTER1 = crate::Reg<sharp_filter1::SHARP_FILTER1_SPEC>;
///sharp usm config register 1
pub mod sharp_filter1;
/**SHARP_FILTER2 (rw) register accessor: sharp usm config register 2

You can [`read`](crate::generic::Reg::read) this register and get [`sharp_filter2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sharp_filter2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sharp_filter2`] module*/
pub type SHARP_FILTER2 = crate::Reg<sharp_filter2::SHARP_FILTER2_SPEC>;
///sharp usm config register 2
pub mod sharp_filter2;
/**SHARP_MATRIX_CTRL (rw) register accessor: sharp pix2matrix ctrl

You can [`read`](crate::generic::Reg::read) this register and get [`sharp_matrix_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sharp_matrix_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sharp_matrix_ctrl`] module*/
pub type SHARP_MATRIX_CTRL = crate::Reg<sharp_matrix_ctrl::SHARP_MATRIX_CTRL_SPEC>;
///sharp pix2matrix ctrl
pub mod sharp_matrix_ctrl;
/**SHARP_CTRL1 (r) register accessor: sharp control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`sharp_ctrl1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sharp_ctrl1`] module*/
pub type SHARP_CTRL1 = crate::Reg<sharp_ctrl1::SHARP_CTRL1_SPEC>;
///sharp control register 1
pub mod sharp_ctrl1;
/**DMA_CNTL (rw) register accessor: isp dma source trans control register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_cntl`] module*/
pub type DMA_CNTL = crate::Reg<dma_cntl::DMA_CNTL_SPEC>;
///isp dma source trans control register
pub mod dma_cntl;
/**DMA_RAW_DATA (rw) register accessor: isp dma source total raw number set register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_raw_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_raw_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_raw_data`] module*/
pub type DMA_RAW_DATA = crate::Reg<dma_raw_data::DMA_RAW_DATA_SPEC>;
///isp dma source total raw number set register
pub mod dma_raw_data;
/**CAM_CNTL (rw) register accessor: isp cam source control register

You can [`read`](crate::generic::Reg::read) this register and get [`cam_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cam_cntl`] module*/
pub type CAM_CNTL = crate::Reg<cam_cntl::CAM_CNTL_SPEC>;
///isp cam source control register
pub mod cam_cntl;
/**CAM_CONF (rw) register accessor: isp cam source config register

You can [`read`](crate::generic::Reg::read) this register and get [`cam_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cam_conf`] module*/
pub type CAM_CONF = crate::Reg<cam_conf::CAM_CONF_SPEC>;
///isp cam source config register
pub mod cam_conf;
/**AF_CTRL0 (rw) register accessor: af control register 0

You can [`read`](crate::generic::Reg::read) this register and get [`af_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_ctrl0`] module*/
pub type AF_CTRL0 = crate::Reg<af_ctrl0::AF_CTRL0_SPEC>;
///af control register 0
pub mod af_ctrl0;
/**AF_CTRL1 (rw) register accessor: af control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`af_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_ctrl1`] module*/
pub type AF_CTRL1 = crate::Reg<af_ctrl1::AF_CTRL1_SPEC>;
///af control register 1
pub mod af_ctrl1;
/**AF_GEN_TH_CTRL (rw) register accessor: af gen threshold control register

You can [`read`](crate::generic::Reg::read) this register and get [`af_gen_th_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_gen_th_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_gen_th_ctrl`] module*/
pub type AF_GEN_TH_CTRL = crate::Reg<af_gen_th_ctrl::AF_GEN_TH_CTRL_SPEC>;
///af gen threshold control register
pub mod af_gen_th_ctrl;
/**AF_ENV_USER_TH_SUM (rw) register accessor: af monitor user sum threshold register

You can [`read`](crate::generic::Reg::read) this register and get [`af_env_user_th_sum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_env_user_th_sum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_env_user_th_sum`] module*/
pub type AF_ENV_USER_TH_SUM = crate::Reg<af_env_user_th_sum::AF_ENV_USER_TH_SUM_SPEC>;
///af monitor user sum threshold register
pub mod af_env_user_th_sum;
/**AF_ENV_USER_TH_LUM (rw) register accessor: af monitor user lum threshold register

You can [`read`](crate::generic::Reg::read) this register and get [`af_env_user_th_lum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_env_user_th_lum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_env_user_th_lum`] module*/
pub type AF_ENV_USER_TH_LUM = crate::Reg<af_env_user_th_lum::AF_ENV_USER_TH_LUM_SPEC>;
///af monitor user lum threshold register
pub mod af_env_user_th_lum;
/**AF_THRESHOLD (rw) register accessor: af threshold register

You can [`read`](crate::generic::Reg::read) this register and get [`af_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_threshold`] module*/
pub type AF_THRESHOLD = crate::Reg<af_threshold::AF_THRESHOLD_SPEC>;
///af threshold register
pub mod af_threshold;
/**AF_HSCALE_A (rw) register accessor: h-scale of af window a register

You can [`read`](crate::generic::Reg::read) this register and get [`af_hscale_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_hscale_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_hscale_a`] module*/
pub type AF_HSCALE_A = crate::Reg<af_hscale_a::AF_HSCALE_A_SPEC>;
///h-scale of af window a register
pub mod af_hscale_a;
/**AF_VSCALE_A (rw) register accessor: v-scale of af window a register

You can [`read`](crate::generic::Reg::read) this register and get [`af_vscale_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_vscale_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_vscale_a`] module*/
pub type AF_VSCALE_A = crate::Reg<af_vscale_a::AF_VSCALE_A_SPEC>;
///v-scale of af window a register
pub mod af_vscale_a;
/**AF_HSCALE_B (rw) register accessor: h-scale of af window b register

You can [`read`](crate::generic::Reg::read) this register and get [`af_hscale_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_hscale_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_hscale_b`] module*/
pub type AF_HSCALE_B = crate::Reg<af_hscale_b::AF_HSCALE_B_SPEC>;
///h-scale of af window b register
pub mod af_hscale_b;
/**AF_VSCALE_B (rw) register accessor: v-scale of af window b register

You can [`read`](crate::generic::Reg::read) this register and get [`af_vscale_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_vscale_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_vscale_b`] module*/
pub type AF_VSCALE_B = crate::Reg<af_vscale_b::AF_VSCALE_B_SPEC>;
///v-scale of af window b register
pub mod af_vscale_b;
/**AF_HSCALE_C (rw) register accessor: v-scale of af window c register

You can [`read`](crate::generic::Reg::read) this register and get [`af_hscale_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_hscale_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_hscale_c`] module*/
pub type AF_HSCALE_C = crate::Reg<af_hscale_c::AF_HSCALE_C_SPEC>;
///v-scale of af window c register
pub mod af_hscale_c;
/**AF_VSCALE_C (rw) register accessor: v-scale of af window c register

You can [`read`](crate::generic::Reg::read) this register and get [`af_vscale_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_vscale_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_vscale_c`] module*/
pub type AF_VSCALE_C = crate::Reg<af_vscale_c::AF_VSCALE_C_SPEC>;
///v-scale of af window c register
pub mod af_vscale_c;
/**AF_SUM_A (r) register accessor: result of sum of af window a

You can [`read`](crate::generic::Reg::read) this register and get [`af_sum_a::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_sum_a`] module*/
pub type AF_SUM_A = crate::Reg<af_sum_a::AF_SUM_A_SPEC>;
///result of sum of af window a
pub mod af_sum_a;
/**AF_SUM_B (r) register accessor: result of sum of af window b

You can [`read`](crate::generic::Reg::read) this register and get [`af_sum_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_sum_b`] module*/
pub type AF_SUM_B = crate::Reg<af_sum_b::AF_SUM_B_SPEC>;
///result of sum of af window b
pub mod af_sum_b;
/**AF_SUM_C (r) register accessor: result of sum of af window c

You can [`read`](crate::generic::Reg::read) this register and get [`af_sum_c::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_sum_c`] module*/
pub type AF_SUM_C = crate::Reg<af_sum_c::AF_SUM_C_SPEC>;
///result of sum of af window c
pub mod af_sum_c;
/**AF_LUM_A (r) register accessor: result of lum of af window a

You can [`read`](crate::generic::Reg::read) this register and get [`af_lum_a::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_lum_a`] module*/
pub type AF_LUM_A = crate::Reg<af_lum_a::AF_LUM_A_SPEC>;
///result of lum of af window a
pub mod af_lum_a;
/**AF_LUM_B (r) register accessor: result of lum of af window b

You can [`read`](crate::generic::Reg::read) this register and get [`af_lum_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_lum_b`] module*/
pub type AF_LUM_B = crate::Reg<af_lum_b::AF_LUM_B_SPEC>;
///result of lum of af window b
pub mod af_lum_b;
/**AF_LUM_C (r) register accessor: result of lum of af window c

You can [`read`](crate::generic::Reg::read) this register and get [`af_lum_c::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@af_lum_c`] module*/
pub type AF_LUM_C = crate::Reg<af_lum_c::AF_LUM_C_SPEC>;
///result of lum of af window c
pub mod af_lum_c;
/**AWB_MODE (rw) register accessor: awb mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`awb_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb_mode`] module*/
pub type AWB_MODE = crate::Reg<awb_mode::AWB_MODE_SPEC>;
///awb mode control register
pub mod awb_mode;
/**AWB_HSCALE (rw) register accessor: h-scale of awb window

You can [`read`](crate::generic::Reg::read) this register and get [`awb_hscale::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_hscale::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb_hscale`] module*/
pub type AWB_HSCALE = crate::Reg<awb_hscale::AWB_HSCALE_SPEC>;
///h-scale of awb window
pub mod awb_hscale;
/**AWB_VSCALE (rw) register accessor: v-scale of awb window

You can [`read`](crate::generic::Reg::read) this register and get [`awb_vscale::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_vscale::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb_vscale`] module*/
pub type AWB_VSCALE = crate::Reg<awb_vscale::AWB_VSCALE_SPEC>;
///v-scale of awb window
pub mod awb_vscale;
/**AWB_TH_LUM (rw) register accessor: awb lum threshold register

You can [`read`](crate::generic::Reg::read) this register and get [`awb_th_lum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_th_lum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb_th_lum`] module*/
pub type AWB_TH_LUM = crate::Reg<awb_th_lum::AWB_TH_LUM_SPEC>;
///awb lum threshold register
pub mod awb_th_lum;
/**AWB_TH_RG (rw) register accessor: awb r/g threshold register

You can [`read`](crate::generic::Reg::read) this register and get [`awb_th_rg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_th_rg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb_th_rg`] module*/
pub type AWB_TH_RG = crate::Reg<awb_th_rg::AWB_TH_RG_SPEC>;
///awb r/g threshold register
pub mod awb_th_rg;
/**AWB_TH_BG (rw) register accessor: awb b/g threshold register

You can [`read`](crate::generic::Reg::read) this register and get [`awb_th_bg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_th_bg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb_th_bg`] module*/
pub type AWB_TH_BG = crate::Reg<awb_th_bg::AWB_TH_BG_SPEC>;
///awb b/g threshold register
pub mod awb_th_bg;
/**AWB0_WHITE_CNT (r) register accessor: result of awb white point number

You can [`read`](crate::generic::Reg::read) this register and get [`awb0_white_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb0_white_cnt`] module*/
pub type AWB0_WHITE_CNT = crate::Reg<awb0_white_cnt::AWB0_WHITE_CNT_SPEC>;
///result of awb white point number
pub mod awb0_white_cnt;
/**AWB0_ACC_R (r) register accessor: result of accumulate of r channel of all white points

You can [`read`](crate::generic::Reg::read) this register and get [`awb0_acc_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb0_acc_r`] module*/
pub type AWB0_ACC_R = crate::Reg<awb0_acc_r::AWB0_ACC_R_SPEC>;
///result of accumulate of r channel of all white points
pub mod awb0_acc_r;
/**AWB0_ACC_G (r) register accessor: result of accumulate of g channel of all white points

You can [`read`](crate::generic::Reg::read) this register and get [`awb0_acc_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb0_acc_g`] module*/
pub type AWB0_ACC_G = crate::Reg<awb0_acc_g::AWB0_ACC_G_SPEC>;
///result of accumulate of g channel of all white points
pub mod awb0_acc_g;
/**AWB0_ACC_B (r) register accessor: result of accumulate of b channel of all white points

You can [`read`](crate::generic::Reg::read) this register and get [`awb0_acc_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awb0_acc_b`] module*/
pub type AWB0_ACC_B = crate::Reg<awb0_acc_b::AWB0_ACC_B_SPEC>;
///result of accumulate of b channel of all white points
pub mod awb0_acc_b;
/**COLOR_CTRL (rw) register accessor: color control register

You can [`read`](crate::generic::Reg::read) this register and get [`color_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`color_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@color_ctrl`] module*/
pub type COLOR_CTRL = crate::Reg<color_ctrl::COLOR_CTRL_SPEC>;
///color control register
pub mod color_ctrl;
/**BLC_VALUE (rw) register accessor: blc black level register

You can [`read`](crate::generic::Reg::read) this register and get [`blc_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blc_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blc_value`] module*/
pub type BLC_VALUE = crate::Reg<blc_value::BLC_VALUE_SPEC>;
///blc black level register
pub mod blc_value;
/**BLC_CTRL0 (rw) register accessor: blc stretch control register

You can [`read`](crate::generic::Reg::read) this register and get [`blc_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blc_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blc_ctrl0`] module*/
pub type BLC_CTRL0 = crate::Reg<blc_ctrl0::BLC_CTRL0_SPEC>;
///blc stretch control register
pub mod blc_ctrl0;
/**BLC_CTRL1 (rw) register accessor: blc window control register

You can [`read`](crate::generic::Reg::read) this register and get [`blc_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blc_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blc_ctrl1`] module*/
pub type BLC_CTRL1 = crate::Reg<blc_ctrl1::BLC_CTRL1_SPEC>;
///blc window control register
pub mod blc_ctrl1;
/**BLC_CTRL2 (rw) register accessor: blc black threshold control register

You can [`read`](crate::generic::Reg::read) this register and get [`blc_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blc_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blc_ctrl2`] module*/
pub type BLC_CTRL2 = crate::Reg<blc_ctrl2::BLC_CTRL2_SPEC>;
///blc black threshold control register
pub mod blc_ctrl2;
/**BLC_MEAN (r) register accessor: results of the average of black window

You can [`read`](crate::generic::Reg::read) this register and get [`blc_mean::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blc_mean`] module*/
pub type BLC_MEAN = crate::Reg<blc_mean::BLC_MEAN_SPEC>;
///results of the average of black window
pub mod blc_mean;
/**HIST_MODE (rw) register accessor: histogram mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`hist_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_mode`] module*/
pub type HIST_MODE = crate::Reg<hist_mode::HIST_MODE_SPEC>;
///histogram mode control register
pub mod hist_mode;
/**HIST_COEFF (rw) register accessor: histogram rgb to gray coefficients register

You can [`read`](crate::generic::Reg::read) this register and get [`hist_coeff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_coeff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_coeff`] module*/
pub type HIST_COEFF = crate::Reg<hist_coeff::HIST_COEFF_SPEC>;
///histogram rgb to gray coefficients register
pub mod hist_coeff;
/**HIST_OFFS (rw) register accessor: histogram window offsets register

You can [`read`](crate::generic::Reg::read) this register and get [`hist_offs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_offs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_offs`] module*/
pub type HIST_OFFS = crate::Reg<hist_offs::HIST_OFFS_SPEC>;
///histogram window offsets register
pub mod hist_offs;
/**HIST_SIZE (rw) register accessor: histogram sub-window size register

You can [`read`](crate::generic::Reg::read) this register and get [`hist_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_size`] module*/
pub type HIST_SIZE = crate::Reg<hist_size::HIST_SIZE_SPEC>;
///histogram sub-window size register
pub mod hist_size;
/**HIST_SEG0 (rw) register accessor: histogram bin control register 0

You can [`read`](crate::generic::Reg::read) this register and get [`hist_seg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_seg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_seg0`] module*/
pub type HIST_SEG0 = crate::Reg<hist_seg0::HIST_SEG0_SPEC>;
///histogram bin control register 0
pub mod hist_seg0;
/**HIST_SEG1 (rw) register accessor: histogram bin control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`hist_seg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_seg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_seg1`] module*/
pub type HIST_SEG1 = crate::Reg<hist_seg1::HIST_SEG1_SPEC>;
///histogram bin control register 1
pub mod hist_seg1;
/**HIST_SEG2 (rw) register accessor: histogram bin control register 2

You can [`read`](crate::generic::Reg::read) this register and get [`hist_seg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_seg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_seg2`] module*/
pub type HIST_SEG2 = crate::Reg<hist_seg2::HIST_SEG2_SPEC>;
///histogram bin control register 2
pub mod hist_seg2;
/**HIST_SEG3 (rw) register accessor: histogram bin control register 3

You can [`read`](crate::generic::Reg::read) this register and get [`hist_seg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_seg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_seg3`] module*/
pub type HIST_SEG3 = crate::Reg<hist_seg3::HIST_SEG3_SPEC>;
///histogram bin control register 3
pub mod hist_seg3;
/**HIST_WEIGHT0 (rw) register accessor: histogram sub-window weight register 0

You can [`read`](crate::generic::Reg::read) this register and get [`hist_weight0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_weight0`] module*/
pub type HIST_WEIGHT0 = crate::Reg<hist_weight0::HIST_WEIGHT0_SPEC>;
///histogram sub-window weight register 0
pub mod hist_weight0;
/**HIST_WEIGHT1 (rw) register accessor: histogram sub-window weight register 1

You can [`read`](crate::generic::Reg::read) this register and get [`hist_weight1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_weight1`] module*/
pub type HIST_WEIGHT1 = crate::Reg<hist_weight1::HIST_WEIGHT1_SPEC>;
///histogram sub-window weight register 1
pub mod hist_weight1;
/**HIST_WEIGHT2 (rw) register accessor: histogram sub-window weight register 2

You can [`read`](crate::generic::Reg::read) this register and get [`hist_weight2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_weight2`] module*/
pub type HIST_WEIGHT2 = crate::Reg<hist_weight2::HIST_WEIGHT2_SPEC>;
///histogram sub-window weight register 2
pub mod hist_weight2;
/**HIST_WEIGHT3 (rw) register accessor: histogram sub-window weight register 3

You can [`read`](crate::generic::Reg::read) this register and get [`hist_weight3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_weight3`] module*/
pub type HIST_WEIGHT3 = crate::Reg<hist_weight3::HIST_WEIGHT3_SPEC>;
///histogram sub-window weight register 3
pub mod hist_weight3;
/**HIST_WEIGHT4 (rw) register accessor: histogram sub-window weight register 4

You can [`read`](crate::generic::Reg::read) this register and get [`hist_weight4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_weight4`] module*/
pub type HIST_WEIGHT4 = crate::Reg<hist_weight4::HIST_WEIGHT4_SPEC>;
///histogram sub-window weight register 4
pub mod hist_weight4;
/**HIST_WEIGHT5 (rw) register accessor: histogram sub-window weight register 5

You can [`read`](crate::generic::Reg::read) this register and get [`hist_weight5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_weight5`] module*/
pub type HIST_WEIGHT5 = crate::Reg<hist_weight5::HIST_WEIGHT5_SPEC>;
///histogram sub-window weight register 5
pub mod hist_weight5;
/**HIST_WEIGHT6 (rw) register accessor: histogram sub-window weight register 6

You can [`read`](crate::generic::Reg::read) this register and get [`hist_weight6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_weight6`] module*/
pub type HIST_WEIGHT6 = crate::Reg<hist_weight6::HIST_WEIGHT6_SPEC>;
///histogram sub-window weight register 6
pub mod hist_weight6;
/**HIST_BIN0 (r) register accessor: result of histogram bin 0

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin0`] module*/
pub type HIST_BIN0 = crate::Reg<hist_bin0::HIST_BIN0_SPEC>;
///result of histogram bin 0
pub mod hist_bin0;
/**HIST_BIN1 (r) register accessor: result of histogram bin 1

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin1`] module*/
pub type HIST_BIN1 = crate::Reg<hist_bin1::HIST_BIN1_SPEC>;
///result of histogram bin 1
pub mod hist_bin1;
/**HIST_BIN2 (r) register accessor: result of histogram bin 2

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin2`] module*/
pub type HIST_BIN2 = crate::Reg<hist_bin2::HIST_BIN2_SPEC>;
///result of histogram bin 2
pub mod hist_bin2;
/**HIST_BIN3 (r) register accessor: result of histogram bin 3

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin3`] module*/
pub type HIST_BIN3 = crate::Reg<hist_bin3::HIST_BIN3_SPEC>;
///result of histogram bin 3
pub mod hist_bin3;
/**HIST_BIN4 (r) register accessor: result of histogram bin 4

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin4`] module*/
pub type HIST_BIN4 = crate::Reg<hist_bin4::HIST_BIN4_SPEC>;
///result of histogram bin 4
pub mod hist_bin4;
/**HIST_BIN5 (r) register accessor: result of histogram bin 5

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin5`] module*/
pub type HIST_BIN5 = crate::Reg<hist_bin5::HIST_BIN5_SPEC>;
///result of histogram bin 5
pub mod hist_bin5;
/**HIST_BIN6 (r) register accessor: result of histogram bin 6

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin6`] module*/
pub type HIST_BIN6 = crate::Reg<hist_bin6::HIST_BIN6_SPEC>;
///result of histogram bin 6
pub mod hist_bin6;
/**HIST_BIN7 (r) register accessor: result of histogram bin 7

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin7`] module*/
pub type HIST_BIN7 = crate::Reg<hist_bin7::HIST_BIN7_SPEC>;
///result of histogram bin 7
pub mod hist_bin7;
/**HIST_BIN8 (r) register accessor: result of histogram bin 8

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin8`] module*/
pub type HIST_BIN8 = crate::Reg<hist_bin8::HIST_BIN8_SPEC>;
///result of histogram bin 8
pub mod hist_bin8;
/**HIST_BIN9 (r) register accessor: result of histogram bin 9

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin9`] module*/
pub type HIST_BIN9 = crate::Reg<hist_bin9::HIST_BIN9_SPEC>;
///result of histogram bin 9
pub mod hist_bin9;
/**HIST_BIN10 (r) register accessor: result of histogram bin 10

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin10`] module*/
pub type HIST_BIN10 = crate::Reg<hist_bin10::HIST_BIN10_SPEC>;
///result of histogram bin 10
pub mod hist_bin10;
/**HIST_BIN11 (r) register accessor: result of histogram bin 11

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin11`] module*/
pub type HIST_BIN11 = crate::Reg<hist_bin11::HIST_BIN11_SPEC>;
///result of histogram bin 11
pub mod hist_bin11;
/**HIST_BIN12 (r) register accessor: result of histogram bin 12

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin12`] module*/
pub type HIST_BIN12 = crate::Reg<hist_bin12::HIST_BIN12_SPEC>;
///result of histogram bin 12
pub mod hist_bin12;
/**HIST_BIN13 (r) register accessor: result of histogram bin 13

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin13`] module*/
pub type HIST_BIN13 = crate::Reg<hist_bin13::HIST_BIN13_SPEC>;
///result of histogram bin 13
pub mod hist_bin13;
/**HIST_BIN14 (r) register accessor: result of histogram bin 14

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin14`] module*/
pub type HIST_BIN14 = crate::Reg<hist_bin14::HIST_BIN14_SPEC>;
///result of histogram bin 14
pub mod hist_bin14;
/**HIST_BIN15 (r) register accessor: result of histogram bin 15

You can [`read`](crate::generic::Reg::read) this register and get [`hist_bin15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hist_bin15`] module*/
pub type HIST_BIN15 = crate::Reg<hist_bin15::HIST_BIN15_SPEC>;
///result of histogram bin 15
pub mod hist_bin15;
/**MEM_AUX_CTRL_0 (rw) register accessor: mem aux control register 0

You can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_aux_ctrl_0`] module*/
pub type MEM_AUX_CTRL_0 = crate::Reg<mem_aux_ctrl_0::MEM_AUX_CTRL_0_SPEC>;
///mem aux control register 0
pub mod mem_aux_ctrl_0;
/**MEM_AUX_CTRL_1 (rw) register accessor: mem aux control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_aux_ctrl_1`] module*/
pub type MEM_AUX_CTRL_1 = crate::Reg<mem_aux_ctrl_1::MEM_AUX_CTRL_1_SPEC>;
///mem aux control register 1
pub mod mem_aux_ctrl_1;
/**MEM_AUX_CTRL_2 (rw) register accessor: mem aux control register 2

You can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_aux_ctrl_2`] module*/
pub type MEM_AUX_CTRL_2 = crate::Reg<mem_aux_ctrl_2::MEM_AUX_CTRL_2_SPEC>;
///mem aux control register 2
pub mod mem_aux_ctrl_2;
/**MEM_AUX_CTRL_3 (rw) register accessor: mem aux control register 3

You can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_aux_ctrl_3`] module*/
pub type MEM_AUX_CTRL_3 = crate::Reg<mem_aux_ctrl_3::MEM_AUX_CTRL_3_SPEC>;
///mem aux control register 3
pub mod mem_aux_ctrl_3;
/**MEM_AUX_CTRL_4 (rw) register accessor: mem aux control register 4

You can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_aux_ctrl_4`] module*/
pub type MEM_AUX_CTRL_4 = crate::Reg<mem_aux_ctrl_4::MEM_AUX_CTRL_4_SPEC>;
///mem aux control register 4
pub mod mem_aux_ctrl_4;
/**YUV_FORMAT (rw) register accessor: yuv format control register

You can [`read`](crate::generic::Reg::read) this register and get [`yuv_format::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`yuv_format::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@yuv_format`] module*/
pub type YUV_FORMAT = crate::Reg<yuv_format::YUV_FORMAT_SPEC>;
///yuv format control register
pub mod yuv_format;
/**RDN_ECO_CS (rw) register accessor: rdn eco cs register

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdn_eco_cs`] module*/
pub type RDN_ECO_CS = crate::Reg<rdn_eco_cs::RDN_ECO_CS_SPEC>;
///rdn eco cs register
pub mod rdn_eco_cs;
/**RDN_ECO_LOW (rw) register accessor: rdn eco all low register

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdn_eco_low`] module*/
pub type RDN_ECO_LOW = crate::Reg<rdn_eco_low::RDN_ECO_LOW_SPEC>;
///rdn eco all low register
pub mod rdn_eco_low;
/**RDN_ECO_HIGH (rw) register accessor: rdn eco all high register

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdn_eco_high`] module*/
pub type RDN_ECO_HIGH = crate::Reg<rdn_eco_high::RDN_ECO_HIGH_SPEC>;
///rdn eco all high register
pub mod rdn_eco_high;
